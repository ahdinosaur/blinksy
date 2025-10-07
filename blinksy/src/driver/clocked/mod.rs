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
//! - [`ClockedLedAsync`]: Trait defining protocol specifics for a clocked LED chipset, asynchronously
//! - [`ClockedWriter`]: Trait for how to write data for a clocked protocol
//! - [`ClockedWriterAsync`]: Trait for how to write data for a clocked protocol, asynchronously
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
//!     fn end<W: ClockedWriter<Word = Self::Word>>(writer: &mut W, _: usize) -> Result<(), W::Error> {
//!         // Write end frame
//!         writer.write(&[0xFF, 0xFF, 0xFF, 0xFF])
//!     }
//! }
//! ```

use core::marker::PhantomData;

use crate::color::LinearSrgb;
use crate::color::{ColorCorrection, FromColor};

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
    fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: IntoIterator<Item = Self::Word>;
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

    /// A start frame to begin a transmission.
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn start() -> impl IntoIterator<Item = Self::Word>;

    /// A color frame for a single LED.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to write
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn led(
        color: Self::Color,
        brightness: f32,
        correction: ColorCorrection,
    ) -> impl IntoIterator<Item = Self::Word>;

    /// An end frame to conclude a transmission.
    ///
    /// # Arguments
    ///
    /// * `pixel_count` - The number of LEDs that were written
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn end(pixel_count: usize) -> impl IntoIterator<Item = Self::Word>;
}

/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClockedLed)
/// * `Writer` - The clocked writer
#[derive(Debug)]
pub struct ClockedDriver<Led, Writer> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,
    /// Writer implementation for the clocked protocol
    writer: Writer,
}

impl<Led, Writer> ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter,
{
    /// Writes a complete sequence of colors to the LED chain.
    ///
    /// This method orchestrates the process of:
    /// 1. Writing the start frame
    /// 2. Writing each LED frame
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
    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>
    where
        Writer: ClockedWriter<Word = Self::Word>,
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        writer.write(Self::start())?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Self::Color::from_color(color);
            writer.write(Self::led(color, brightness, correction));
            pixel_count += 1;
        }

        writer.write(Self::end(pixel_count))?;

        Ok(())
    }
}

#[cfg(feature = "async")]
/// Async trait for types that can write data words to a clocked protocol.
///
/// This trait abstracts over different implementation methods for writing data
/// to a clocked protocol, such as bit-banging with GPIOs or using hardware SPI.
pub trait ClockedWriterAsync {
    /// The word type (typically u8).
    type Word: Copy + 'static;

    /// The error type that may be returned by write operations.
    type Error;

    // See note about allow(async_fn_in_trait) in smart-leds-trait:
    //   https://github.com/smart-leds-rs/smart-leds-trait/blob/faad5eba0f9c9aa80b1dd17e078e4644f11e7ee0/src/lib.rs#L59-L68
    #[allow(async_fn_in_trait)]
    /// Writes a slice of words to the protocol, asynchronously.
    ///
    /// # Arguments
    ///
    /// * `words` - Slice of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    async fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error>;
}

#[cfg(feature = "async")]
// See note about allow(async_fn_in_trait) in smart-leds-trait:
//   https://github.com/smart-leds-rs/smart-leds-trait/blob/faad5eba0f9c9aa80b1dd17e078e4644f11e7ee0/src/lib.rs#L59-L68
#[allow(async_fn_in_trait)]
/// Async trait that defines the protocol specifics for a clocked LED chipset.
///
/// Implementors of this trait specify how to generate the protocol-specific
/// frames for a particular clocked LED chipset.
///
/// # Type Parameters
///
/// * `Word` - The basic data unit type (typically u8)
/// * `Color` - The color representation type
pub trait ClockedLedAsync {
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
    async fn start<Writer: ClockedWriterAsync<Word = Self::Word>>(
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
    async fn color<Writer: ClockedWriterAsync<Word = Self::Word>>(
        writer: &mut Writer,
        color: Self::Color,
        brightness: f32,
        correction: ColorCorrection,
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
    async fn end<Writer: ClockedWriterAsync<Word = Self::Word>>(
        writer: &mut Writer,
        pixel_count: usize,
    ) -> Result<(), Writer::Error>;

    /// Writes a complete sequence of colors to the LED chain.
    ///
    /// This method orchestrates the process of:
    /// 1. Writing the start frame
    /// 2. Writing each LED color
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
    async fn clocked_write<Writer, I, C>(
        writer: &mut Writer,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>
    where
        Writer: ClockedWriterAsync<Word = Self::Word>,
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Self::start(writer).await?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Self::Color::from_color(color);
            Self::color(writer, color, brightness, correction).await?;
            pixel_count += 1;
        }

        Self::end(writer, pixel_count).await?;
        Ok(())
    }
}
