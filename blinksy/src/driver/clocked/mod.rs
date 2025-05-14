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
//! - [`ClockedSpiDriver`]: Driver using a hardware SPI peripheral
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{color::{ColorCorrection, LinearSrgb}, driver::{ClockedLed, ClockedWriter}};
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
//!         let led_rgb = &LedRgb::from_linear_srgb(linear_srgb, brightness, correction);
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
use crate::color::{ColorCorrection, FromColor};

use super::LedDriver;

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
    /// * `words` - Slice of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error>;
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
    /// # Arguments
    ///
    /// * `writer` - The writer implementation to use
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
    ) -> Result<(), Writer::Error>;

    /// Writes a single color frame for one LED.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer implementation to use
    /// * `color` - The color to write
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn color<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
        color: Self::Color,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>;

    /// Performs any necessary reset operations mid-transmission.
    ///
    /// Some protocols may require special handling between LED data frames.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer implementation to use
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn reset<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
    ) -> Result<(), Writer::Error>;

    /// Writes an end frame to conclude a transmission.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer implementation to use
    /// * `pixel_count` - The number of LEDs that were written
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn end<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
        pixel_count: usize,
    ) -> Result<(), Writer::Error>;

    /// Writes a complete sequence of colors to the LED chain.
    ///
    /// This method orchestrates the process of:
    /// 1. Writing the start frame
    /// 2. Writing each LED color
    /// 3. Performing any reset operations
    /// 4. Writing the end frame
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer implementation to use
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if any write operation fails
    fn clocked_write<Writer, I, C>(
        writer: &mut Writer,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>
    where
        Writer: ClockedWriter<Word = Self::Word>,
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Self::start(writer)?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Self::Color::from_color(color);
            Self::color(writer, color, brightness, correction)?;
            pixel_count += 1;
        }

        Self::reset(writer)?;
        Self::end(writer, pixel_count)?;

        Ok(())
    }
}
