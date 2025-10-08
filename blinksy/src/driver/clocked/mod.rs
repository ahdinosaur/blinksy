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
//! - [`ClockedWriterAsync`]: Trait for how to write data for a clocked protocol, asynchronously
//!
//! ## Drivers
//!
//! - [`ClockedDriver`]: Generic driver for clocked LEDs and writers.
//! - [`ClockedDelayDriver`]: Driver using GPIO bit-banging with a delay timer
//! - [`ClockedSpiDriver`]: (Recommended) Driver using a hardware SPI peripheral
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{
//!     color::{ColorCorrection, FromColor, LedRgb, LinearSrgb},
//!     driver::{ClockedLed, ClockedWriter},
//! };
//!
//! // Define a new LED chipset with specific protocol requirements
//! struct MyLed;
//!
//! impl ClockedLed for MyLed {
//!     type Word = u8;
//!     type Color = LinearSrgb;
//!
//!     fn start() -> impl IntoIterator<Item = Self::Word> {
//!         // Start frame
//!         [0x00, 0x00, 0x00, 0x00]
//!     }
//!
//!     fn led(
//!         color: Self::Color,
//!         brightness: f32,
//!         correction: ColorCorrection,
//!     ) -> impl IntoIterator<Item = Self::Word> {
//!         // Color data for one LED
//!         let linear_srgb = LinearSrgb::from_color(color);
//!         let rgb = LedRgb::from_linear_srgb(linear_srgb, brightness, correction);
//!         [0x80, rgb[0], rgb[1], rgb[2]]
//!     }
//!
//!     fn end(_: usize) -> impl IntoIterator<Item = Self::Word> {
//!         // End frame
//!         [0xFF, 0xFF, 0xFF, 0xFF]
//!     }
//! }
//! ```

use core::marker::PhantomData;

use heapless::Vec;

use crate::color::{ColorCorrection, FromColor};
use crate::driver::Driver;
#[cfg(feature = "async")]
use crate::driver::DriverAsync;

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

    /// Writes an iterator of words to the protocol.
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Self::Word]>;
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
    /// Writes an iterator of words to the protocol, asynchronously.
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    async fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
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

    /// The type for the color system used by the LED.
    type Color;

    /// The type for the beginning of a transmission.
    type StartFrame: AsRef<[Self::Word]>;

    /// A start frame to begin a transmission.
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn start() -> Self::StartFrame;

    /// The type for a single LED in a transmission.
    type LedFrame: AsRef<[Self::Word]>;

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
    fn led(color: Self::Color, brightness: f32, correction: ColorCorrection) -> Self::LedFrame;

    /// The type for the conclusion of a transmission.
    type EndFrame: AsRef<[Self::Word]>;

    /// An end frame to conclude a transmission.
    ///
    /// # Arguments
    ///
    /// * `pixel_count` - The number of LEDs that were written
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn end(pixel_count: usize) -> Self::EndFrame;
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

impl<Led, Writer> Driver for ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter<Word = Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;
    type Framebuffer<const PIXEL_COUNT: usize> = Vec<Led::LedFrame, PIXEL_COUNT>;

    /// Writes a complete sequence of colors to the LED chain.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if any write operation fails
    fn framebuffer<const PIXEL_COUNT: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<Self::Framebuffer<PIXEL_COUNT>, Writer::Error>
    where
        I: IntoIterator<Item = C>,
        Led::Color: FromColor<C>,
    {
        let pixels = pixels.into_iter().map(Led::Color::from_color);
        let led_frames = pixels.map(|pixel| Led::led(pixel, brightness, correction));
        Ok(Vec::from_iter(led_frames))
    }

    fn render<const PIXEL_COUNT: usize>(
        &mut self,
        frame: Self::Framebuffer<PIXEL_COUNT>,
    ) -> Result<(), Self::Error> {
        self.writer.write(Led::start())?;
        self.writer.write(as_bytes(frame))?;
        self.writer.write(Led::end(PIXEL_COUNT))?;
        Ok(())
    }
}
fn as_bytes<const N: usize>(v: &HVec<[u8; 4], N>) -> &[u8] {
    // Safe: u8 has alignment 1; head and tail are always empty here.
    let (head, bytes, tail) = v.as_slice().align_to::<u8>();
    debug_assert!(head.is_empty() && tail.is_empty());
    bytes
}

#[cfg(feature = "async")]
impl<Led, Writer> DriverAsync for ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriterAsync<Word = Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;

    /// Writes a complete sequence of colors to the LED chain.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if any write operation fails
    async fn write<const PIXEL_COUNT: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>
    where
        I: IntoIterator<Item = C>,
        Led::Color: FromColor<C>,
    {
        let pixels = pixels.into_iter().map(Led::Color::from_color);
        self.writer
            .write(Led::update(pixels, brightness, correction, PIXEL_COUNT))
            .await
    }
}
