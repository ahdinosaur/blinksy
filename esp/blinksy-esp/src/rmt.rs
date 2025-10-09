//! # RMT-based LED Driver
//!
//! This module provides a driver for clockless LED protocols (like WS2812)
//! using the ESP32's RMT (Remote Control Module) peripheral. The RMT
//! peripheral provides hardware acceleration for generating precisely timed
//! signals, which is ideal for LED protocols.
//!
//! ## Features
//!
//! - Hardware-accelerated LED control
//! - Precise timing for WS2812 and similar protocols
//! - Blocking and async (feature "async") APIs with equivalent behavior
//!
//! ## Technical Details
//!
//! The RMT peripheral translates LED color data into a sequence of timed
//! pulses that match the protocol requirements. This implementation converts
//! each bit of color data into the corresponding high/low pulse durations
//! required by the specific LED protocol.

#[cfg(feature = "async")]
use blinksy::driver::DriverAsync;
use blinksy::{
    color::{ColorCorrection, FromColor, LinearSrgb},
    driver::{clockless::ClocklessLed, Driver},
    util::bits::{u8_to_bits, BitOrder},
};
use core::{fmt::Debug, marker::PhantomData};
use esp_hal::{
    clock::Clocks,
    gpio::{interconnect::PeripheralOutput, Level},
    rmt::{
        Channel, Error as RmtError, PulseCode, RawChannelAccess, TxChannel, TxChannelConfig,
        TxChannelCreator, TxChannelInternal,
    },
    Blocking, DriverMode,
};
#[cfg(feature = "async")]
use esp_hal::{rmt::TxChannelAsync, Async};
use heapless::Vec;

use crate::util::chunked;

/// All types of errors that can happen during the conversion and transmission
/// of LED commands
#[derive(Debug, defmt::Format)]
pub enum ClocklessRmtDriverError {
    /// Raised in the event that the provided data container is not large enough
    BufferSizeExceeded,
    /// Raised if something goes wrong in the transmission
    TransmissionError(RmtError),
}

pub struct ClocklessRmt<const RMT_BUFFER_SIZE: usize, Led> {
    led: PhantomData<Led>,
}

impl ClocklessRmt<0, ()> {
    pub fn new() -> ClocklessRmt<0, ()> {
        ClocklessRmt { led: PhantomData }
    }
}

impl<Led> ClocklessRmt<0, Led> {
    pub fn with_rmt_buffer_size<const RMT_BUFFER_SIZE: usize>(
        self,
    ) -> ClocklessRmt<RMT_BUFFER_SIZE, Led> {
        ClocklessRmt { led: PhantomData }
    }
}

impl<const RMT_BUFFER_SIZE: usize> ClocklessRmt<RMT_BUFFER_SIZE, ()> {
    pub fn with_led<Led>(self) -> ClocklessRmt<RMT_BUFFER_SIZE, Led> {
        ClocklessRmt { led: PhantomData }
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led> ClocklessRmt<RMT_BUFFER_SIZE, Led>
where
    Led: ClocklessLed,
{
    pub fn build<'d, Dm, Tx, C, O>(
        self,
        channel: C,
        pin: O,
    ) -> ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Dm, Tx>>
    where
        Dm: DriverMode,
        Tx: RawChannelAccess + TxChannelInternal + 'static,
        C: TxChannelCreator<'d, Dm, Raw = Tx>,
        O: PeripheralOutput<'d>,
    {
        ClocklessRmtDriver::new(channel, pin)
    }
}

/// RMT-based driver for clockless LED protocols.
///
/// This driver uses the ESP32's RMT peripheral to generate precisely timed
/// signals required by protocols like WS2812.
///
/// # Type Parameters
///
/// * `RMT_BUFFER_SIZE` - Size of the RMT buffer
/// * `Led` - The LED protocol implementation (must implement ClocklessLed)
/// * `TxChannel` - The RMT transmit channel
pub struct ClocklessRmtDriver<const RMT_BUFFER_SIZE: usize, Led, TxChannel>
where
    Led: ClocklessLed,
{
    led: PhantomData<Led>,
    channel: Option<TxChannel>,
    pulses: (u32, u32, u32),
}

impl<const RMT_BUFFER_SIZE: usize, Led, TxChannel>
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, TxChannel>
where
    Led: ClocklessLed,
{
    fn clock_divider() -> u8 {
        1
    }

    fn tx_channel_config() -> TxChannelConfig {
        TxChannelConfig::default()
            .with_clk_divider(Self::clock_divider())
            .with_idle_output_level(Level::Low)
            .with_idle_output(true)
            .with_carrier_modulation(false)
            // 64 items per memory block, max of 8
            .with_memsize((RMT_BUFFER_SIZE / 64).min(8) as u8)
    }

    fn setup_pulses() -> (u32, u32, u32) {
        let clocks = Clocks::get();
        let freq_hz = clocks.apb_clock.as_hz() / Self::clock_divider() as u32;
        let freq_mhz = freq_hz / 1_000_000;

        let t_0h = ((Led::T_0H.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_0l = ((Led::T_0L.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_1h = ((Led::T_1H.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_1l = ((Led::T_1L.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_reset = ((Led::T_RESET.to_nanos() * freq_mhz) / 1_000) as u16;

        (
            PulseCode::new(Level::High, t_0h, Level::Low, t_0l),
            PulseCode::new(Level::High, t_1h, Level::Low, t_1l),
            PulseCode::new(Level::Low, t_reset, Level::Low, 0),
        )
    }

    fn rmt_led<const FRAME_BUFFER_SIZE: usize>(
        &self,
        framebuffer: Vec<u8, FRAME_BUFFER_SIZE>,
    ) -> impl Iterator<Item = u32> {
        let pulses = self.pulses.clone();
        framebuffer.into_iter().flat_map(move |byte| {
            u8_to_bits(&byte, BitOrder::MostSignificantBit)
                .into_iter()
                .map(move |bit| match bit {
                    false => pulses.0,
                    true => pulses.1,
                })
        })
    }

    fn rmt_end(&self) -> impl IntoIterator<Item = u32> {
        [self.pulses.2, 0]
    }

    fn rmt<const FRAME_BUFFER_SIZE: usize>(
        &self,
        framebuffer: Vec<u8, FRAME_BUFFER_SIZE>,
    ) -> impl Iterator<Item = u32> {
        self.rmt_led(framebuffer).into_iter().chain(self.rmt_end())
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led, Dm, Tx>
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Dm, Tx>>
where
    Led: ClocklessLed,
    Dm: DriverMode,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    /// Create a new adapter object that drives the pin using the RMT channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - RMT transmit channel creator
    /// * `pin` - GPIO pin connected to the LED data line
    ///
    /// # Returns
    ///
    /// A configured ClocklessRmtDriver instance
    pub fn new<'d, C, O>(channel: C, pin: O) -> Self
    where
        C: TxChannelCreator<'d, Dm, Raw = Tx>,
        O: PeripheralOutput<'d>,
    {
        let config = Self::tx_channel_config();
        let channel = channel.configure_tx(pin, config).unwrap();
        let pulses = Self::setup_pulses();

        Self {
            led: PhantomData,
            channel: Some(channel),
            pulses,
        }
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led, Dm, Tx>
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Dm, Tx>>
where
    Led: ClocklessLed,
    Dm: DriverMode,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
}

impl<const RMT_BUFFER_SIZE: usize, Led, Tx>
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Blocking, Tx>>
where
    Led: ClocklessLed,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    /// Transmit buffer using RMT, blocking.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Buffer to be transmitted
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn transmit_blocking(&mut self, buffer: &[u32]) -> Result<(), ClocklessRmtDriverError> {
        let channel = self.channel.take().unwrap();
        match channel.transmit(buffer).unwrap().wait() {
            Ok(chan) => {
                self.channel = Some(chan);
                Ok(())
            }
            Err((e, chan)) => {
                self.channel = Some(chan);
                Err(ClocklessRmtDriverError::TransmissionError(e))
            }
        }
    }
}

#[cfg(feature = "async")]
impl<Led, Tx, const RMT_BUFFER_SIZE: usize>
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Async, Tx>>
where
    Led: ClocklessLed,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    /// Transmit buffer using RMT, async.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Buffer to be transmitted
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    async fn transmit_async(&mut self, buffer: &[u32]) -> Result<(), ClocklessRmtDriverError> {
        let channel = self.channel.as_mut().unwrap();
        channel
            .transmit(buffer)
            .await
            .map_err(ClocklessRmtDriverError::TransmissionError)
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led, Tx> Driver
    for ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Blocking, Tx>>
where
    Led: ClocklessLed<Word = u8>,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    type Error = ClocklessRmtDriverError;
    type Color = LinearSrgb;
    type Word = Led::Word;

    fn encode<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Led::encode::<PIXEL_COUNT, FRAME_BUFFER_SIZE, _, _>(pixels, brightness, correction)
    }

    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Self::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for mut rmt_buffer in chunked::<_, RMT_BUFFER_SIZE>(self.rmt(frame), RMT_BUFFER_SIZE - 1) {
            // RMT buffer must end with 0.
            rmt_buffer.push(0).unwrap();
            self.transmit_blocking(&rmt_buffer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<const RMT_BUFFER_SIZE: usize, Led, Tx> DriverAsync
    for ClocklessRmtDriver<RMT_BUFFER_SIZE, Led, Channel<Async, Tx>>
where
    Led: ClocklessLed<Word = u8>,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    type Error = ClocklessRmtDriverError;
    type Color = LinearSrgb;
    type Word = Led::Word;

    fn encode<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Led::encode::<PIXEL_COUNT, FRAME_BUFFER_SIZE, _, _>(pixels, brightness, correction)
    }

    async fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Self::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for mut rmt_buffer in chunked::<_, RMT_BUFFER_SIZE>(self.rmt(frame), RMT_BUFFER_SIZE - 1) {
            // RMT buffer must end with 0.
            rmt_buffer.push(0).unwrap();
            self.transmit_async(&rmt_buffer).await?;
        }

        Ok(())
    }
}
