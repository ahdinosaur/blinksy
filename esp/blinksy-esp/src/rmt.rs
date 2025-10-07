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
    color::{ColorCorrection, FromColor, LedColor, LinearSrgb},
    driver::{clockless::ClocklessLed, Driver},
    util::bits::{u8_to_bits, BitOrder},
};
use core::{fmt::Debug, marker::PhantomData, slice::IterMut};
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

/// RMT-based driver for clockless LED protocols.
///
/// This driver uses the ESP32's RMT peripheral to generate precisely timed signals
/// required by protocols like WS2812.
///
/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClocklessLed)
/// * `TxChannel` - The RMT transmit channel
/// * `BUFFER_SIZE` - Size of the RMT buffer
pub struct ClocklessRmtDriver<Led, TxChannel, const BUFFER_SIZE: usize>
where
    Led: ClocklessLed,
{
    led: PhantomData<Led>,
    channel: Option<TxChannel>,
    rmt_led_buffer: [u32; BUFFER_SIZE],
    rmt_end_buffer: [u32; 2],
    pulses: (u32, u32, u32),
}

impl<Led, TxChannel, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, TxChannel, BUFFER_SIZE>
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

    fn setup_rmt_end_buffer(pulses: (u32, u32, u32)) -> [u32; 2] {
        [pulses.2, 0]
    }
}

impl<Led, Dm, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Channel<Dm, Tx>, BUFFER_SIZE>
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
    pub fn new<'d, C, O>(channel: C, pin: O, rmt_buffer: [u32; BUFFER_SIZE]) -> Self
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
            rmt_led_buffer: rmt_buffer,
            rmt_end_buffer: Self::setup_rmt_end_buffer(pulses),
            pulses,
        }
    }
}

impl<Led, Dm, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Channel<Dm, Tx>, BUFFER_SIZE>
where
    Led: ClocklessLed,
    Dm: DriverMode,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    /// Writes a single byte of color data to the RMT buffer.
    ///
    /// # Arguments
    ///
    /// * `byte` - The color byte to write
    /// * `rmt_iter` - Iterator over the RMT buffer
    /// * `pulses` - Tuple of pulse codes for 0-bit, 1-bit, and reset
    ///
    /// # Returns
    ///
    /// Result indicating success or a buffer size exceeded error
    fn write_color_byte_to_rmt(
        byte: &u8,
        rmt_iter: &mut IterMut<u32>,
        pulses: &(u32, u32, u32),
    ) -> Result<(), ClocklessRmtDriverError> {
        for bit in u8_to_bits(byte, BitOrder::MostSignificantBit) {
            *rmt_iter
                .next()
                .ok_or(ClocklessRmtDriverError::BufferSizeExceeded)? = match bit {
                false => pulses.0,
                true => pulses.1,
            }
        }
        Ok(())
    }

    /// Writes a complete color to the RMT buffer.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to write
    /// * `rmt_iter` - Iterator over the RMT buffer
    /// * `pulses` - Tuple of pulse codes for 0-bit, 1-bit, and reset
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn write_color_to_rmt(
        color: LinearSrgb,
        rmt_iter: &mut IterMut<u32>,
        pulses: &(u32, u32, u32),
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), ClocklessRmtDriverError> {
        let led_color = color.to_led(Led::LED_CHANNELS, brightness, correction);

        match led_color {
            LedColor::Rgb(rgb) => {
                Self::write_color_byte_to_rmt(&rgb[0], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgb[1], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgb[2], rmt_iter, pulses)?;
            }
            LedColor::Rgbw(rgbw) => {
                Self::write_color_byte_to_rmt(&rgbw[0], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgbw[1], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgbw[2], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgbw[3], rmt_iter, pulses)?;
            }
        }
        Ok(())
    }
}

impl<Led, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Channel<Blocking, Tx>, BUFFER_SIZE>
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

    /// Write pixels to internal RMT buffer, then transmit.
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
    fn write_pixels<I, C>(
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
            self.transmit_blocking(&rmt_led_buffer)?;
        }

        let rmt_end_buffer = self.rmt_end_buffer;
        self.transmit_blocking(&rmt_end_buffer)?;

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<Led, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Channel<Async, Tx>, BUFFER_SIZE>
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

/// Implementation of Driver trait for ClocklessRmtDriver.
impl<Led, Tx, const BUFFER_SIZE: usize> Driver
    for ClocklessRmtDriver<Led, Channel<Blocking, Tx>, BUFFER_SIZE>
where
    Led: ClocklessLed,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    type Error = ClocklessRmtDriverError;
    type Color = LinearSrgb;

    fn write<const PIXEL_COUNT: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        self.write_pixels(pixels, brightness, correction)
    }
}

#[cfg(feature = "async")]
/// Implementation of DriverAsync trait for ClocklessRmtDriver.
impl<Led, Tx, const BUFFER_SIZE: usize> DriverAsync
    for ClocklessRmtDriver<Led, Channel<Async, Tx>, BUFFER_SIZE>
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
