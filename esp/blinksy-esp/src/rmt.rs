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

use blinksy::{
    color::{ColorCorrection, FromColor, LedColor, LinearSrgb},
    driver::{clockless::ClocklessLed, Driver},
};
use core::{fmt::Debug, marker::PhantomData, slice::IterMut};
use esp_hal::{
    clock::Clocks,
    gpio::{interconnect::PeripheralOutput, Level},
    rmt::{
        Channel, Error as RmtError, PulseCode, RawChannelAccess, TxChannel, TxChannelConfig,
        TxChannelCreator, TxChannelInternal,
    },
    Blocking,
};

/// All types of errors that can happen during the conversion and transmission
/// of LED commands
#[derive(Debug, defmt::Format)]
pub enum ClocklessRmtDriverError {
    /// Raised in the event that the provided data container is not large enough
    BufferSizeExceeded,
    /// Raised if something goes wrong in the transmission
    TransmissionError(RmtError),
}

/// Macro to allocate a buffer sized for a specific number of LEDs to be
/// addressed.
///
/// Attempting to use more LEDs than the buffer is configured for will result in
/// an `ClocklessRmtDriverError::BufferSizeExceeded` error.
///
/// # Arguments
///
/// * `$led_count` - Number of LEDs to be controlled
/// * `$channel_count` - Number of color channels per LED (3 for RGB, 4 for RGBW)
///
/// # Returns
///
/// An array of u32 values sized appropriately for the RMT buffer
#[macro_export]
macro_rules! create_rmt_buffer {
    ($led_count:expr, $channel_count:expr) => {
        [0u32; $led_count * $channel_count * 8 + 1]
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
/// * `Tx` - The RMT transmit channel identifier
/// * `BUFFER_SIZE` - Size of the RMT buffer
pub struct ClocklessRmtDriver<Led, Tx, const BUFFER_SIZE: usize>
where
    Led: ClocklessLed,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    led: PhantomData<Led>,
    channel: Option<Channel<Blocking, Tx>>,
    rmt_buffer: [u32; BUFFER_SIZE],
    pulses: (u32, u32, u32),
}

impl<Led, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Tx, BUFFER_SIZE>
where
    Led: ClocklessLed,
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
        C: TxChannelCreator<'d, Blocking, Raw = Tx>,
        O: PeripheralOutput<'d>,
    {
        let clock_divider = 1;
        let config = TxChannelConfig::default()
            .with_clk_divider(clock_divider)
            .with_idle_output_level(Level::Low)
            .with_idle_output(true)
            .with_carrier_modulation(false);

        let channel = channel.configure_tx(pin, config).unwrap();

        let clocks = Clocks::get();
        let freq_hz = clocks.apb_clock.as_hz() / clock_divider as u32;
        let freq_mhz = freq_hz / 1_000_000;

        let t_0h = ((Led::T_0H.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_0l = ((Led::T_0L.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_1h = ((Led::T_1H.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_1l = ((Led::T_1L.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_reset = ((Led::T_RESET.to_nanos() * freq_mhz) / 1_000) as u16;

        Self {
            led: PhantomData,
            channel: Some(channel),
            rmt_buffer,
            pulses: (
                PulseCode::new(Level::High, t_0h, Level::Low, t_0l),
                PulseCode::new(Level::High, t_1h, Level::Low, t_1l),
                PulseCode::new(Level::Low, t_reset, Level::Low, 0),
            ),
        }
    }

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
        for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
            *rmt_iter
                .next()
                .ok_or(ClocklessRmtDriverError::BufferSizeExceeded)? = match byte & bit_position {
                0 => pulses.0,
                _ => pulses.1,
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

    /// Convert all pixels to the RMT format and
    /// add them to internal buffer, then start a singular RMT operation
    /// based on that buffer.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over the pixel colors
    /// * `brightness` - Global brightness factor
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    pub fn write_pixels<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), ClocklessRmtDriverError>
    where
        I: IntoIterator<Item = C>,
        LinearSrgb: FromColor<C>,
    {
        let mut rmt_iter = self.rmt_buffer.iter_mut();

        for color in pixels {
            let color = LinearSrgb::from_color(color);
            Self::write_color_to_rmt(color, &mut rmt_iter, &self.pulses, brightness, correction)?;
        }

        *rmt_iter
            .next()
            .ok_or(ClocklessRmtDriverError::BufferSizeExceeded)? = self.pulses.2;

        let channel = self.channel.take().unwrap();
        match channel.transmit(&self.rmt_buffer).unwrap().wait() {
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

/// Implementation of Driver trait for ClocklessRmtDriver.
///
/// This allows the RMT driver to be used with the Blinksy control system.
impl<Led, Tx, const BUFFER_SIZE: usize> Driver for ClocklessRmtDriver<Led, Tx, BUFFER_SIZE>
where
    Led: ClocklessLed,
    Tx: RawChannelAccess + TxChannelInternal + 'static,
{
    type Error = ClocklessRmtDriverError;
    type Color = LinearSrgb;

    fn write<I, C>(
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
