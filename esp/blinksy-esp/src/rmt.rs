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
use blinksy::driver::ClocklessWriterAsync;
use blinksy::{
    driver::{clockless::ClocklessLed, ClocklessWriter},
    util::bits::{word_to_bits_msb, Word},
};
use core::{fmt::Debug, iter::once, marker::PhantomData};
#[cfg(feature = "async")]
use esp_hal::Async;
use esp_hal::{
    clock::Clocks,
    gpio::{interconnect::PeripheralOutput, Level},
    rmt::{
        Channel, Error as RmtError, PulseCode, Tx, TxChannelConfig, TxChannelCreator,
        CHANNEL_RAM_SIZE,
    },
    Blocking, DriverMode,
};
use heapless::Vec;

use crate::util::chunked;

pub const fn rmt_buffer_size<Led: ClocklessLed>(pixel_count: usize) -> usize {
    pixel_count * Led::LED_CHANNELS.channel_count() * 8 + 1
}

/// All types of errors that can happen during the conversion and transmission
/// of LED commands
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClocklessRmtError {
    /// Raised in the event that the provided data container is not large enough
    BufferSizeExceeded,
    /// Raised if something goes wrong in the transmission
    TransmissionError(RmtError),
}

pub struct ClocklessRmtBuilder<const RMT_BUFFER_SIZE: usize, Led, Chan, Pin> {
    led: PhantomData<Led>,
    channel: Chan,
    pin: Pin,
}

impl Default for ClocklessRmtBuilder<CHANNEL_RAM_SIZE, (), (), ()> {
    fn default() -> ClocklessRmtBuilder<CHANNEL_RAM_SIZE, (), (), ()> {
        ClocklessRmtBuilder {
            led: PhantomData,
            channel: (),
            pin: (),
        }
    }
}

impl<Led, Chan, Pin> ClocklessRmtBuilder<CHANNEL_RAM_SIZE, Led, Chan, Pin> {
    pub fn with_rmt_buffer_size<const RMT_BUFFER_SIZE: usize>(
        self,
    ) -> ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, Chan, Pin> {
        ClocklessRmtBuilder {
            led: self.led,
            channel: self.channel,
            pin: self.pin,
        }
    }
}

impl<const RMT_BUFFER_SIZE: usize, Chan, Pin> ClocklessRmtBuilder<RMT_BUFFER_SIZE, (), Chan, Pin> {
    pub fn with_led<Led>(self) -> ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, Chan, Pin> {
        ClocklessRmtBuilder {
            led: PhantomData,
            channel: self.channel,
            pin: self.pin,
        }
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led, Pin> ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, (), Pin> {
    pub fn with_channel<Chan>(
        self,
        channel: Chan,
    ) -> ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, Chan, Pin> {
        ClocklessRmtBuilder {
            led: self.led,
            channel,
            pin: self.pin,
        }
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led, Chan> ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, Chan, ()> {
    pub fn with_pin<Pin>(self, pin: Pin) -> ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, Chan, Pin> {
        ClocklessRmtBuilder {
            led: self.led,
            channel: self.channel,
            pin,
        }
    }
}

impl<const RMT_BUFFER_SIZE: usize, Led, Chan, Pin>
    ClocklessRmtBuilder<RMT_BUFFER_SIZE, Led, Chan, Pin>
where
    Led: ClocklessLed,
    Led::Word: Word,
{
    pub fn build<'ch, Dm>(self) -> ClocklessRmt<RMT_BUFFER_SIZE, Led, Channel<'ch, Dm, Tx>>
    where
        Chan: TxChannelCreator<'ch, Dm>,
        Pin: PeripheralOutput<'ch>,
        Dm: DriverMode,
    {
        ClocklessRmt::new(self.channel, self.pin)
    }
}

/// RMT-based driver for clockless LED protocols.
///
/// This driver uses the ESP32's RMT peripheral to generate precisely timed
/// signals required by protocols like WS2812.
///
/// # Type Parameters
///
/// - `RMT_BUFFER_SIZE` - Size of the RMT buffer
/// - `Led` - The LED protocol implementation (must implement ClocklessLed)
/// - `TxChannel` - The RMT transmit channel
pub struct ClocklessRmt<const RMT_BUFFER_SIZE: usize, Led, TxChannel>
where
    Led: ClocklessLed,
{
    led: PhantomData<Led>,
    channel: Option<TxChannel>,
    pulses: (PulseCode, PulseCode, PulseCode),
}

impl<const RMT_BUFFER_SIZE: usize, Led, TxChannel> ClocklessRmt<RMT_BUFFER_SIZE, Led, TxChannel>
where
    Led: ClocklessLed,
    Led::Word: Word,
{
    fn clock_divider() -> u8 {
        1
    }

    fn setup_pulses() -> (PulseCode, PulseCode, PulseCode) {
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
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> impl Iterator<Item = PulseCode> {
        let pulses = self.pulses;
        frame.into_iter().flat_map(move |word| {
            word_to_bits_msb(word).map(move |bit| match bit {
                false => pulses.0,
                true => pulses.1,
            })
        })
    }

    fn rmt_end(&self) -> impl IntoIterator<Item = PulseCode> {
        once(self.pulses.2)
    }

    fn rmt<const FRAME_BUFFER_SIZE: usize>(
        &self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> impl Iterator<Item = PulseCode> {
        self.rmt_led(frame).chain(self.rmt_end())
    }
}

impl<'ch, const RMT_BUFFER_SIZE: usize, Led, Dm>
    ClocklessRmt<RMT_BUFFER_SIZE, Led, Channel<'ch, Dm, Tx>>
where
    Led: ClocklessLed,
    Led::Word: Word,
    Dm: DriverMode,
{
    /// Create a new adapter object that drives the pin using the RMT channel.
    ///
    /// # Arguments
    ///
    /// - `channel` - RMT transmit channel creator
    /// - `pin` - GPIO pin connected to the LED data line
    ///
    /// # Returns
    ///
    /// A configured ClocklessRmt instance
    pub fn new<C, O>(channel: C, pin: O) -> Self
    where
        C: TxChannelCreator<'ch, Dm>,
        O: PeripheralOutput<'ch>,
    {
        let config = TxChannelConfig::default()
            .with_clk_divider(Self::clock_divider())
            .with_idle_output_level(Level::Low)
            .with_idle_output(true);
        let channel = channel.configure_tx(pin, config).unwrap();
        let pulses = Self::setup_pulses();

        Self {
            led: PhantomData,
            channel: Some(channel),
            pulses,
        }
    }
}

impl<'ch, const RMT_BUFFER_SIZE: usize, Led>
    ClocklessRmt<RMT_BUFFER_SIZE, Led, Channel<'ch, Blocking, Tx>>
where
    Led: ClocklessLed,
{
    /// Transmit buffer using RMT, blocking.
    ///
    /// # Arguments
    ///
    /// - `buffer` - Buffer to be transmitted
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn transmit_blocking(&mut self, buffer: &[PulseCode]) -> Result<(), ClocklessRmtError> {
        let channel = self.channel.take().unwrap();
        match channel.transmit(buffer).unwrap().wait() {
            Ok(chan) => {
                self.channel = Some(chan);
                Ok(())
            }
            Err((e, chan)) => {
                self.channel = Some(chan);
                Err(ClocklessRmtError::TransmissionError(e))
            }
        }
    }
}

#[cfg(feature = "async")]
impl<'ch, const RMT_BUFFER_SIZE: usize, Led>
    ClocklessRmt<RMT_BUFFER_SIZE, Led, Channel<'ch, Async, Tx>>
where
    Led: ClocklessLed,
{
    /// Transmit buffer using RMT, async.
    ///
    /// # Arguments
    ///
    /// - `buffer` - Buffer to be transmitted
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    async fn transmit_async(&mut self, buffer: &[PulseCode]) -> Result<(), ClocklessRmtError> {
        let channel = self.channel.as_mut().unwrap();
        channel
            .transmit(buffer)
            .await
            .map_err(ClocklessRmtError::TransmissionError)
    }
}

impl<'ch, const RMT_BUFFER_SIZE: usize, Led> ClocklessWriter<Led>
    for ClocklessRmt<RMT_BUFFER_SIZE, Led, Channel<'ch, Blocking, Tx>>
where
    Led: ClocklessLed,
    Led::Word: Word,
{
    type Error = ClocklessRmtError;

    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        let rmt_pulses = self.rmt(frame);
        for mut rmt_buffer in chunked::<_, RMT_BUFFER_SIZE>(rmt_pulses, RMT_BUFFER_SIZE - 1) {
            // RMT buffer must end with 0.
            rmt_buffer.push(PulseCode::end_marker()).unwrap();
            self.transmit_blocking(&rmt_buffer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<'ch, const RMT_BUFFER_SIZE: usize, Led> ClocklessWriterAsync<Led>
    for ClocklessRmt<RMT_BUFFER_SIZE, Led, Channel<'ch, Async, Tx>>
where
    Led: ClocklessLed,
    Led::Word: Word,
{
    type Error = ClocklessRmtError;

    async fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for mut rmt_buffer in chunked::<_, RMT_BUFFER_SIZE>(self.rmt(frame), RMT_BUFFER_SIZE - 1) {
            // RMT buffer must end with 0.
            rmt_buffer.push(PulseCode::end_marker()).unwrap();
            self.transmit_async(&rmt_buffer).await?;
        }

        Ok(())
    }
}
