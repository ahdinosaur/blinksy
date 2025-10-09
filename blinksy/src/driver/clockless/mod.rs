//! # Clockless LED Driver
//!
//! This module provides abstractions for driving "clockless" LED protocols, such as
//! WS2812 (NeoPixel), SK6812, and similar. These protocols use a single data line
//! with precise timing to encode bits.
//!
//! ## Clockless Protocol
//!
//! The clockless protocol encodes data bits using precise pulse timings on a single data line:
//!
//! - Each bit is represented by a high pulse followed by a low pulse
//! - The duration of the high and low pulses determines whether it's a '0' or '1' bit
//! - After all bits are sent, a longer reset period is required
//!
//! This self-clocked pattern is called [Manchester encoding](https://en.wikipedia.org/wiki/Manchester_code).
//!
//! ## Traits
//!
//! - [`ClocklessLed`]: Trait defining the timing parameters for a clockless LED chipset
//! - [`ClockedWriter`]: Trait for how to write data for the clockless protocol
//! - [`ClockedWriterAsync`]: Trait for how to write data for the clockless protocol, asynchronously
//!
//! ## Driver
//!
//! - [`ClocklessDriver`]: Generic driver for clockless LEDs and writers.
//!
//! ## Writers
//!
//! - [`ClocklessDelay`]: Writer using GPIO bit-banging with a delay timer
//! - [`blinksy-esp::ClocklessRmt`]: Writer using RMT on the ESP32
//!
//! [`blinksy-esp::ClocklessRmt`]: https://docs.rs/blinksy-esp/0.10/blinksy_esp/type.ClocklessRmt.html
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{
//!     color::{LedChannels, RgbChannels},
//!     driver::ClocklessLed,
//!     time::Nanoseconds,
//! };
//!
//! // Define a new LED chipset with specific timing requirements
//! struct MyLed;
//!
//! impl ClocklessLed for MyLed {
//!     // High pulse duration for '0' bit
//!     const T_0H: Nanoseconds = Nanoseconds::nanos(350);
//!     // Low pulse duration for '0' bit
//!     const T_0L: Nanoseconds = Nanoseconds::nanos(800);
//!     // High pulse duration for '1' bit
//!     const T_1H: Nanoseconds = Nanoseconds::nanos(700);
//!     // Low pulse duration for '1' bit
//!     const T_1L: Nanoseconds = Nanoseconds::nanos(600);
//!     // Reset period
//!     const T_RESET: Nanoseconds = Nanoseconds::micros(50);
//!     // Color channel ordering
//!     const LED_CHANNELS: LedChannels = LedChannels::Rgb(RgbChannels::RGB);
//! }
//! ```

use core::marker::PhantomData;
use heapless::Vec;
use num_traits::ToBytes;

#[cfg(feature = "async")]
use crate::driver::DriverAsync;
use crate::{
    color::{ColorCorrection, FromColor, LedChannels, LedColor, LinearSrgb},
    driver::Driver,
    time::Nanoseconds,
    util::component::Component,
};

mod delay;

pub use self::delay::*;

/// Trait that defines the timing parameters and protocol specifics for a clockless LED chipset.
///
/// Implementors of this trait specify the exact timing requirements for a specific
/// LED chipset that uses a clockless protocol.
///
/// # Example
///
/// ```rust
/// use fugit::NanosDurationU32 as Nanoseconds;
/// use blinksy::{color::{LedChannels, RgbChannels}, driver::ClocklessLed};
///
/// struct WS2811Led;
///
/// impl ClocklessLed for WS2811Led {
///     const T_0H: Nanoseconds = Nanoseconds::nanos(250);
///     const T_0L: Nanoseconds = Nanoseconds::nanos(1000);
///     const T_1H: Nanoseconds = Nanoseconds::nanos(600);
///     const T_1L: Nanoseconds = Nanoseconds::nanos(650);
///     const T_RESET: Nanoseconds = Nanoseconds::micros(50);
///     const LED_CHANNELS: LedChannels = LedChannels::Rgb(RgbChannels::RGB);
/// }
/// ```
pub trait ClocklessLed {
    /// The word type (typically u8).
    type Word: ToBytes + Component;

    /// Duration of high signal for transmitting a '0' bit.
    const T_0H: Nanoseconds;

    /// Duration of low signal for transmitting a '0' bit.
    const T_0L: Nanoseconds;

    /// Duration of high signal for transmitting a '1' bit.
    const T_1H: Nanoseconds;

    /// Duration of low signal for transmitting a '1' bit.
    const T_1L: Nanoseconds;

    /// Duration of the reset period at the end of a transmission.
    ///
    /// This low signal period marks the end of a data frame and allows the LEDs
    /// to latch the received data and update their output.
    const T_RESET: Nanoseconds;

    /// Specification of the color channel order and format.
    ///
    /// Different LED chipsets may expect data in different channel orders (e.g., RGB, GRB, RGBW).
    const LED_CHANNELS: LedChannels;

    /// Calculates the total cycle time for a bit transmission.
    ///
    /// Returns the maximum of (T_0H + T_0L) and (T_1H + T_1L) to ensure
    /// timing is correct regardless of bit value.
    fn t_cycle() -> Nanoseconds {
        (Self::T_0H + Self::T_0L).max(Self::T_1H + Self::T_1L)
    }

    /// Encodes a buffer to represent the next frame update.
    ///
    /// This method:
    ///
    /// 1. Converts each input color to linear sRGB.
    /// 2. Applies the global brightness scaling
    /// 3. Reorders color channels according to the LED protocol
    ///
    /// # Type Arguments
    ///
    /// * `PIXEL_COUNT`: Number of pixels
    /// * `BUFFER_SIZE`: Size of the frame buffer
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if transmission fails
    fn encode<const PIXEL_COUNT: usize, const BUFFER_SIZE: usize, I, C>(
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, BUFFER_SIZE>
    where
        I: IntoIterator<Item = C>,
        LinearSrgb: FromColor<C>,
    {
        Vec::from_iter(pixels.into_iter().flat_map(|pixel| {
            let linear_srgb = LinearSrgb::from_color(pixel);
            let data: LedColor<Self::Word> =
                linear_srgb.to_led(Self::LED_CHANNELS, brightness, correction);
            data.into_iter()
        }))
    }
}

/// Trait for types that can write data words to a clockless protocol.
pub trait ClocklessWriter {
    type Error;

    fn write<const FRAME_BUFFER_SIZE: usize, Led>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error>
    where
        Led: ClocklessLed;
}

#[cfg(feature = "async")]
/// Async trait for types that can write data words to a clockless protocol.
pub trait ClocklessWriterAsync {
    type Error;

    fn write<const FRAME_BUFFER_SIZE: usize, Led>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error>
    where
        Led: ClocklessLed;
}

/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClocklessLed)
/// * `Writer` - The clocked writer
#[derive(Debug)]
pub struct ClocklessDriver<Led, Writer> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,
    /// Writer implementation for the clocked protocol
    writer: Writer,
}

impl Default for ClocklessDriver<(), ()> {
    fn default() -> Self {
        ClocklessDriver {
            led: PhantomData,
            writer: (),
        }
    }
}

impl<Writer> ClocklessDriver<(), Writer> {
    pub fn with_led<Led>(self) -> ClocklessDriver<Led, Writer> {
        ClocklessDriver {
            led: PhantomData,
            writer: self.writer,
        }
    }
}

impl<Led> ClocklessDriver<Led, ()> {
    pub fn with_writer<Writer>(self, writer: Writer) -> ClocklessDriver<Led, Writer> {
        ClocklessDriver {
            led: self.led,
            writer,
        }
    }
}

impl<Led, Writer> Driver for ClocklessDriver<Led, Writer>
where
    Led: ClocklessLed,
    Led::Word: ToBytes,
    <Led::Word as ToBytes>::Bytes: IntoIterator<Item = u8>,
    Writer: ClocklessWriter,
{
    type Error = Writer::Error;
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
        self.writer.write::<FRAME_BUFFER_SIZE, Led>(frame)
    }
}

#[cfg(feature = "async")]
impl<Led, Writer> DriverAsync for ClocklessDriver<Led, Writer>
where
    Led: ClocklessLed,
    Led::Word: ToBytes,
    <Led::Word as ToBytes>::Bytes: IntoIterator<Item = u8>,
    Writer: ClocklessWriterAsync,
{
    type Error = Writer::Error;
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
        self.writer.write::<FRAME_BUFFER_SIZE, Led>(frame).await
    }
}
