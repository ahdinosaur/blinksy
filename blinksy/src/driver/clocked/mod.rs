//! # Clocked LED Driver Abstractions
//!
//! This module provides abstractions for driving "clocked" LED protocols, such as
//! APA102 (DotStar), SK9822, and similar. These protocols are based on
//! [SPI](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface), where chipsets
//! have a data line and a clock line.
//!
//! ## Clocked Protocols
//!
//! Unlike clockless protocols, clocked protocols:
//!
//! - Use separate data and clock lines
//! - Don't rely on precise timing (only clock frequency matters)
//! - Often provide more control over brightness and color precision
//! - Can work with hardware SPI peripherals
//!
//! ## Traits
//!
//! - [`ClockedLed`]: Trait defining protocol specifics for a clocked LED chipset
//! - [`ClockedWriter`]: Trait for how to write data for a clocked protocol
//!
//! ## Drivers
//!
//! - [`ClockedDelayDriver`]: Driver using GPIO bit-banging with a delay timer
//! - [`ClockedSpiDriver`]: (Recommended) Driver using a hardware SPI peripheral
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{color::{ColorCorrection, FromColor, LedRgb, LinearSrgb}, driver::{ClockedLed, ClockedWriter}};
//!
//! // Define a new LED chipset with specific protocol requirements
//! struct MyLed;
//!
//! impl ClockedLed for MyLed {
//!     type Word = u8;
//!     type Color = LinearSrgb;
//!
//!     fn start<W: ClockedWriter<Word = Self::Word>>(writer: &mut W) -> Result<(), W::Error> {
//!         // Write start frame
//!         writer.write(&[0x00, 0x00, 0x00, 0x00])
//!     }
//!
//!     fn color<W: ClockedWriter<Word = Self::Word>>(
//!         writer: &mut W,
//!         color: Self::Color,
//!         brightness: f32,
//!         correction: ColorCorrection,
//!     ) -> Result<(), W::Error> {
//!         // Write color data for one LED
//!         let linear_srgb = LinearSrgb::from_color(color);
//!         let rgb = LedRgb::from_linear_srgb(linear_srgb, brightness, correction);
//!         writer.write(&[0x80, rgb[0], rgb[1], rgb[2]])
//!     }
//!
//!     fn reset<W: ClockedWriter<Word = Self::Word>>(_: &mut W) -> Result<(), W::Error> {
//!         // No reset needed
//!         Ok(())
//!     }
//!
//!     fn end<W: ClockedWriter<Word = Self::Word>>(writer: &mut W, _: usize) -> Result<(), W::Error> {
//!         // Write end frame
//!         writer.write(&[0xFF, 0xFF, 0xFF, 0xFF])
//!     }
//! }
//! ```

use core::marker::PhantomData;

use crate::color::{ColorCorrection, FromColor};
use crate::driver::DriverAsync;

use super::Driver;

mod delay;
mod spi;

pub use self::delay::*;
pub use self::spi::*;

/// Trait for types that can write data words to a clocked protocol.
///
/// This trait abstracts over different implementation methods for writing data
/// to a clocked protocol, such as bit-banging with GPIOs or using hardware SPI.
pub trait ClockedWriter {
    /// The word type (typically u8).
    type Word: Copy + 'static;

    /// The error type that may be returned by write operations.
    type Error;

    /// Writes a slice of words to the protocol.
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn write<I>(&mut self, words: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Word>;
}

/// Trait for types that can write data words to a clocked protocol, async.
///
/// This trait abstracts over different implementation methods for writing data
/// to a clocked protocol, such as bit-banging with GPIOs or using hardware SPI.
pub trait ClockedWriterAsync {
    /// The word type (typically u8).
    type Word: Copy + 'static;

    /// The error type that may be returned by write operations.
    type Error;

    /// Writes a slice of words to the protocol.
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    #[allow(async_fn_in_trait)]
    async fn write<I>(&mut self, words: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Word>;
}

/// Trait that defines the protocol specifics for a clocked LED chipset.
///
/// Implementors of this trait specify how to generate the protocol-specific
/// frames for a particular clocked LED chipset.
///
/// # Type Parameters
///
/// * `Word` - The basic data unit type (typically u8)
/// * `Color` - The color representation type
pub trait ClockedLed {
    /// The word type (typically u8).
    type Word: Copy + 'static;

    /// The color representation type.
    type Color;

    /// Writes a start frame to begin a transmission.
    ///
    /// This typically sends some form of header that identifies the beginning
    /// of an LED update sequence.
    ///
    /// # Returns
    ///
    /// Iterator of words to write
    fn start() -> impl IntoIterator<Item = Self::Word>;

    /// Writes a single color frame for one LED.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to write
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Iterator of words to write
    fn color(
        color: Self::Color,
        brightness: f32,
        correction: ColorCorrection,
    ) -> impl IntoIterator<Item = Self::Word>;

    /// Writes an end frame to conclude a transmission.
    ///
    /// # Arguments
    ///
    /// * `pixel_count` - The number of LEDs that were written
    ///
    /// # Returns
    ///
    /// Iterator of words to write
    fn end(pixel_count: usize) -> impl IntoIterator<Item = Self::Word>;
}

#[derive(Debug)]
struct ClockedLedDriver<Led: ClockedLed, Writer> {
    led: PhantomData<Led>,
    writer: Writer,
}

impl<Led, Writer> Driver for ClockedLedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter<Word = Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;

    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        Self::Color: FromColor<C>,
        I: IntoIterator<Item = C>,
    {
        self.writer.write(Led::start())?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Led::Color::from_color(color);
            self.writer
                .write(Led::color(color, brightness, correction))?;
            pixel_count += 1;
        }

        self.writer.write(Led::end(pixel_count))?;
        Ok(())
    }
}

impl<Led, Writer> DriverAsync for ClockedLedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriterAsync<Word = Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;

    async fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        Self::Color: FromColor<C>,
        I: IntoIterator<Item = C>,
    {
        self.writer.write(Led::start()).await?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Led::Color::from_color(color);
            self.writer
                .write(Led::color(color, brightness, correction))
                .await?;
            pixel_count += 1;
        }

        self.writer.write(Led::end(pixel_count)).await?;
        Ok(())
    }
}
