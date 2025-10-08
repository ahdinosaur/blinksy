//! # RMT-based LED Driver
//!
//! This module provides a driver for clockless LED protocols (like WS2812) using the
//! ESP32's RMT (Remote Control Module) peripheral. The RMT peripheral provides hardware
//! acceleration for generating precisely timed signals, which is ideal for LED protocols.
//!
//! ## Features
//!
//! - Hardware-accelerated LED control
//! - Precise timing for WS2812 and similar protocols
//!
//! ## Technical Details
//!
//! The RMT peripheral translates LED color data into a sequence of timed pulses that
//! match the protocol requirements. This implementation converts each bit of color data
//! into the corresponding high/low pulse durations required by the specific LED protocol.

#[cfg(feature = "async")]
use blinksy::driver::DriverAsync;
use blinksy::{
    color::{ColorCorrection, FromColor, LinearSrgb},
    driver::{clockless::ClocklessLed, Driver},
    util::bits::{u8_to_bits, BitOrder},
};
use core::{fmt::Debug, iter, marker::PhantomData};
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

/// Macro to allocate a buffer used for RMT transmission sized for one LED frame.
///
/// Attempting to use more than the buffer is configured for will result in
/// an `ClocklessRmtDriverError::BufferSizeExceeded` error.
///
/// # Arguments
///
/// * `$channel_count` - Number of color channels per LED (3 for RGB, 4 for RGBW)
///
/// # Returns
///
/// An array of u32 values sized appropriately for the RMT buffer
#[macro_export]
macro_rules! create_rmt_buffer {
    ($channel_count:expr) => {
        [0u32; $channel_count * 8 + 1]
    };
}

pub const fn rmt_buffer_size(chunk_size: usize, channel_count: usize) -> usize {
    chunk_size * channel_count * 8 + 1
}

pub struct ClocklessRmtDriverBuilder<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize>;

impl<const RMT_BUFFER_SIZE: usize> ClocklessRmtDriverBuilder<0, RMT_BUFFER_SIZE> {
    pub fn with_chunk_size<const CHUNK_SIZE: usize>(
        self,
    ) -> ClocklessRmtDriverBuilder<{ CHUNK_SIZE }, RMT_BUFFER_SIZE> {
        ClocklessRmtDriverBuilder
    }
}

impl<const CHUNK_SIZE: usize> ClocklessRmtDriverBuilder<CHUNK_SIZE, 0> {
    pub fn with_rmt_buffer_size<const RMT_BUFFER_SIZE: usize>(
        self,
    ) -> ClocklessRmtDriverBuilder<CHUNK_SIZE, { RMT_BUFFER_SIZE }> {
        ClocklessRmtDriverBuilder
    }
}

impl<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize>
    ClocklessRmtDriverBuilder<CHUNK_SIZE, RMT_BUFFER_SIZE>
{
    pub fn build<'d, Led, Dm, Tx, C, O>(
        channel: C,
        pin: O,
    ) -> ClocklessRmtDriver<CHUNK_SIZE, RMT_BUFFER_SIZE, Led, Channel<Dm, Tx>>
    where
        Led: ClocklessLed,
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
/// This driver uses the ESP32's RMT peripheral to generate precisely timed signals
/// required by protocols like WS2812.
///
/// # Type Parameters
///
/// * `RMT_BUFFER_SIZE` - Size of the RMT buffer
/// * `Led` - The LED protocol implementation (must implement ClocklessLed)
/// * `TxChannel` - The RMT transmit channel
pub struct ClocklessRmtDriver<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize, Led, TxChannel>
where
    Led: ClocklessLed,
{
    led: PhantomData<Led>,
    channel: Option<TxChannel>,
    pulses: (u32, u32, u32),
}

impl<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize, Led, TxChannel>
    ClocklessRmtDriver<CHUNK_SIZE, RMT_BUFFER_SIZE, Led, TxChannel>
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

impl<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize, Led, Dm, Tx>
    ClocklessRmtDriver<CHUNK_SIZE, RMT_BUFFER_SIZE, Led, Channel<Dm, Tx>>
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
    /// * `rmt_buffer` - Buffer for RMT data
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

impl<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize, Led, Dm, Tx>
    ClocklessRmtDriver<CHUNK_SIZE, RMT_BUFFER_SIZE, Led, Channel<Dm, Tx>>
where
    Led: ClocklessLed,
    Dm: DriverMode,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
}

impl<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize, Led, Tx>
    ClocklessRmtDriver<CHUNK_SIZE, RMT_BUFFER_SIZE, Led, Channel<Blocking, Tx>>
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
    ClocklessRmtDriver<Led, Channel<Async, Tx>, RMT_BUFFER_SIZE>
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

    /// Write pixels to internal RMT buffer, then transmit, asynchronously.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over the pixel colors
    /// * `brightness` - Global brightness factor
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    async fn write_pixels_async<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), ClocklessRmtDriverError>
    where
        I: IntoIterator<Item = C>,
        LinearSrgb: FromColor<C>,
    {
        for color in pixels {
            let mut rmt_iter = self.rmt_led_buffer.iter_mut();
            let color = LinearSrgb::from_color(color);
            Self::write_color_to_rmt(color, &mut rmt_iter, &self.pulses, brightness, correction)?;
            let rmt_led_buffer = self.rmt_led_buffer;
            self.transmit_async(&rmt_led_buffer).await?;
        }

        let rmt_end_buffer = self.rmt_end_buffer;
        self.transmit_async(&rmt_end_buffer).await?;

        Ok(())
    }
}

impl<const CHUNK_SIZE: usize, const RMT_BUFFER_SIZE: usize, Led, Tx> Driver
    for ClocklessRmtDriver<CHUNK_SIZE, RMT_BUFFER_SIZE, Led, Channel<Blocking, Tx>>
where
    Led: ClocklessLed<Word = u8>,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    type Error = ClocklessRmtDriverError;
    type Color = LinearSrgb;
    type Word = Led::Word;

    fn framebuffer<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Led::framebuffer::<PIXEL_COUNT, FRAME_BUFFER_SIZE, _, _>(pixels, brightness, correction)
    }

    fn render<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        framebuffer: Vec<Self::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for rmt_buffer in chunked::<_, CHUNK_SIZE>(self.rmt(framebuffer)) {
            self.transmit_blocking(&rmt_buffer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<Led, Tx, const RMT_BUFFER_SIZE: usize> DriverAsync
    for ClocklessRmtDriver<Led, Channel<Async, Tx>, RMT_BUFFER_SIZE>
where
    Led: ClocklessLed,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    type Error = ClocklessRmtDriverError;
    type Color = LinearSrgb;

    async fn write<const PIXEL_COUNT: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        self.write_pixels_async(pixels, brightness, correction)
            .await
    }
}
