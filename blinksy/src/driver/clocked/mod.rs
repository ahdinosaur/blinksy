//! # Clocked LED Driver
//!
//! This module provides abstractions for driving "clocked" LED protocols, such as
//! APA102 (DotStar), SK9822, and similar. These protocols are based on
//! [SPI](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface), where chipsets
//! have a data line and a clock line.
//!
//! ## Clocked Protocol
//!
//! Unlike the clockless protocol, the clocked protocol:
//!
//! - Uses separate data and clock lines
//! - The output device determines the clock rate, not the LEDs
//! - Doesn't rely on precise timing (only clock frequency matters)
//! - Often provides more control over brightness and color precision
//!
//! ## Traits
//!
//! - [`ClockedLed`]: Trait defining protocol specifics for a clocked LED chipset
//! - [`ClockedWriter`]: Trait for how to write data for a clocked protocol
//! - [`ClockedWriterAsync`]: Trait for how to write data for a clocked protocol, asynchronously
//!
//! ## Driver
//!
//! - [`ClockedDriver`]: Generic driver for clocked LEDs and writers.
//!
//! ## Writers
//!
//! - [`ClockedDelay`]
//! - [`embedded_hal::spi::SpiBus`] / [`embedded_hal_async::spi::SpiBus`]
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{
//!     color::{ColorCorrection, FromColor, LinearSrgb, RgbChannels},
//!     driver::ClockedLed,
//!     util::component::Component,
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
//!         let linear = LinearSrgb::from_color(color);
//!         let (mut r, mut g, mut b) = (linear.red, linear.green, linear.blue);
//!         r = r * brightness * correction.red;
//!         g = g * brightness * correction.green;
//!         b = b * brightness * correction.blue;
//!         let (r_u8, g_u8, b_u8) = (
//!             Component::from_normalized_f32(r),
//!             Component::from_normalized_f32(g),
//!             Component::from_normalized_f32(b),
//!         );
//!         let bytes = RgbChannels::RGB.reorder([r_u8, g_u8, b_u8]);
//!         [0x80, bytes[0], bytes[1], bytes[2]]
//!     }
//!
//!     fn end(_: usize) -> impl IntoIterator<Item = Self::Word> {
//!         // End frame
//!         [0xFF, 0xFF, 0xFF, 0xFF]
//!     }
//! }
//! ```

use core::fmt::Debug;
use core::marker::PhantomData;

use heapless::Vec;

use crate::color::{ColorCorrection, FromColor};
use crate::driver::Driver;
#[cfg(feature = "async")]
use crate::driver::DriverAsync;

mod delay;
mod spi;

pub use self::delay::*;

/// Trait that defines the protocol specifics for a clocked LED chipset.
///
/// Implementors of this trait specify how to generate the protocol-specific
/// frames for a particular clocked LED chipset.
///
/// # Type Parameters
///
/// - `Word` - The basic data unit type (typically u8)
/// - `Color` - The color representation type
pub trait ClockedLed {
    /// The word type (typically u8).
    type Word;

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
    /// - `color` - The color to write
    /// - `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// - `correction` - Color correction factors
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
    /// - `pixel_count` - The number of LEDs that were written
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn end(pixel_count: usize) -> impl IntoIterator<Item = Self::Word>;

    /// A complete update frame:
    ///
    /// 1. Start frame
    /// 2. For each pixel: Led frame
    /// 3. End frame
    ///
    /// # Arguments
    ///
    /// - `pixel` - The pixel color to write
    /// - `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// - `correction` - Color correction factors
    /// - `pixel_count` - The number of LEDs that were written
    ///
    /// # Returns
    ///
    /// An iterator of words to write
    fn update<I>(
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
        pixel_count: usize,
    ) -> impl IntoIterator<Item = Self::Word>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        Self::start()
            .into_iter()
            .chain(
                pixels
                    .into_iter()
                    .flat_map(move |color| Self::led(color, brightness, correction).into_iter()),
            )
            .chain(Self::end(pixel_count))
    }
}

/// Trait for types that can write data words to a clocked protocol.
///
/// This trait abstracts over different implementation methods for writing data
/// to a clocked protocol, such as bit-banging with GPIOs or using hardware SPI.
pub trait ClockedWriter<Word> {
    /// The error type that may be returned by write operations.
    type Error;

    /// Writes an iterator of words to the protocol.
    ///
    /// # Arguments
    ///
    /// - `words` - Iterator of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Word]>;
}

#[cfg(feature = "async")]
/// Async trait for types that can write data words to a clocked protocol.
///
/// This trait abstracts over different implementation methods for writing data
/// to a clocked protocol, such as bit-banging with GPIOs or using hardware SPI.
pub trait ClockedWriterAsync<Word> {
    /// The error type that may be returned by write operations.
    type Error;

    // See note about allow(async_fn_in_trait) in smart-leds-trait:
    //   https://github.com/smart-leds-rs/smart-leds-trait/blob/faad5eba0f9c9aa80b1dd17e078e4644f11e7ee0/src/lib.rs#L59-L68
    #[allow(async_fn_in_trait)]
    /// Writes an iterator of words to the protocol, asynchronously.
    ///
    /// # Arguments
    ///
    /// - `words` - Iterator of words to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if the write fails
    async fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Word]>;
}

/// A generic driver for clocked LEDs and writers.
///
/// For available writers, see [clocked module](crate::driver::clocked).
///
/// # Type Parameters
///
/// - `Led` - The LED protocol implementation (must implement ClockedLed)
/// - `Writer` - The clocked writer
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockedDriver<Led, Writer> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,
    /// Writer implementation for the clocked protocol
    writer: Writer,
}

impl Default for ClockedDriver<(), ()> {
    fn default() -> Self {
        ClockedDriver {
            led: PhantomData,
            writer: (),
        }
    }
}

impl<Writer> ClockedDriver<(), Writer> {
    pub fn with_led<Led>(self) -> ClockedDriver<Led, Writer> {
        ClockedDriver {
            led: PhantomData,
            writer: self.writer,
        }
    }
}

impl<Led> ClockedDriver<Led, ()> {
    pub fn with_writer<Writer>(self, writer: Writer) -> ClockedDriver<Led, Writer> {
        ClockedDriver {
            led: self.led,
            writer,
        }
    }
}

impl<Led, Writer> Driver for ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter<Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;
    type Word = Led::Word;

    fn encode<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        I: IntoIterator<Item = C>,
        Led::Color: FromColor<C>,
    {
        let pixels = pixels.into_iter().map(Led::Color::from_color);
        let frame: Vec<_, FRAME_BUFFER_SIZE> =
            Vec::from_iter(Led::update(pixels, brightness, correction, PIXEL_COUNT));
        frame
    }

    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Self::Word, FRAME_BUFFER_SIZE>,
        _brightness: f32,
        _correction: ColorCorrection,
    ) -> Result<(), Self::Error> {
        self.writer.write(frame)
    }
}

#[cfg(feature = "async")]
impl<Led, Writer> DriverAsync for ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriterAsync<Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;
    type Word = Led::Word;

    fn encode<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        I: IntoIterator<Item = C>,
        Led::Color: FromColor<C>,
    {
        let pixels = pixels.into_iter().map(Led::Color::from_color);
        let frame: Vec<_, FRAME_BUFFER_SIZE> =
            Vec::from_iter(Led::update(pixels, brightness, correction, PIXEL_COUNT));
        frame
    }

    async fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Self::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        self.writer.write(frame).await
    }
}
