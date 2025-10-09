//! # Driver Interface
//!
//! A driver is what tells the LED hardware how to be the colors you want.
//!
//! ## Core traits
//!
//! - [`Driver`]: For all blocking drivers
//! - [`DriverAsync`]: For all async drivers
//!
//! ## Re-usable implementations
//!
//! - [`clocked`]: For clocked (two-wire) protocols (like [`APA102`](crate::drivers::apa102))
//! - [`clockless`]: For clockless (one-wire) protocols (like [`WS2812`](crate::drivers::ws2812))

use crate::color::{ColorCorrection, FromColor};

pub mod clocked;
pub mod clockless;

pub use clocked::*;
pub use clockless::*;
use heapless::Vec;

/// Core trait for all blocking LED drivers.
///
/// This trait defines the common interface for sending color data to LED hardware,
/// regardless of the specific protocol or chipset being used.
///
/// # Type Parameters
///
/// * `Error` - The error type that may be returned by the driver
/// * `Color` - The color type accepted by the driver
/// * `Word` - The word type used for the encoded frame buffer
///
/// # Example
///
/// ```rust
/// # use blinksy::{
/// #     color::{ColorCorrection, FromColor, LinearSrgb},
/// #     driver::Driver,
/// # };
///
/// struct MyDriver {
///     // Implementation details
/// }
///
/// impl Driver for MyDriver {
///     type Error = ();
///     type Color = LinearSrgb;
///     type Word = u8;
///
///     fn encode<
///         const PIXEL_COUNT: usize,
///         const FRAME_BUFFER_SIZE: usize,
///         Pixels,
///         C,
///     >(
///         &mut self,
///         pixels: Pixels,
///         brightness: f32,
///         correction: ColorCorrection,
///     ) -> heapless::Vec<Self::Word, FRAME_BUFFER_SIZE>
///     where
///         Pixels: IntoIterator<Item = C>,
///         Self::Color: FromColor<C>,
///     {
///         // Encode pixel data into a frame buffer for the hardware
///         heapless::Vec::new()
///     }
///
///     fn write<const FRAME_BUFFER_SIZE: usize>(
///         &mut self,
///         frame: heapless::Vec<Self::Word, FRAME_BUFFER_SIZE>,
///     ) -> Result<(), Self::Error> {
///         // Send encoded frame buffer to hardware
///         Ok(())
///     }
/// }
/// ```
pub trait Driver {
    /// The error type that may be returned by the driver.
    type Error;

    /// The color type accepted by the driver.
    type Color;

    /// The word of the frame buffer.
    type Word;

    /// Encodes an update frame buffer for the LED hardware.
    ///
    /// # Type Parameters
    ///
    /// * `PIXEL_COUNT` - Number of pixels in frame
    /// * `FRAME_BUFFER_SIZE` - Length of encoded frame buffer, in words.
    /// * `Pixels` - Iterator of colors for each pixel
    /// * `Color` - Type of each pixel
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator of colors for each pixel
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Result with frame buffer
    fn encode<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, Pixels, Color>(
        &mut self,
        pixels: Pixels,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        Pixels: IntoIterator<Item = Color>,
        Self::Color: FromColor<Color>;

    /// Writes frame buffer to the LED hardware.
    ///
    /// # Type Parameters
    ///
    /// * `FRAME_BUFFER_SIZE` - Length of encoded frame buffer, in words.
    ///
    /// # Arguments
    ///
    /// * `frame` - Frame buffer
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Self::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error>;

    /// Shows a frame on the LED hardware.
    ///
    /// # Type Parameters
    ///
    /// * `PIXEL_COUNT` - Number of pixels in frame
    /// * `FRAME_BUFFER_SIZE` - Length of encoded frame buffer, in words.
    /// * `Pixels` - Iterator of colors for each pixel
    /// * `Color` - Type of each pixel
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator of colors for each pixel
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn show<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        let frame_buffer =
            self.encode::<PIXEL_COUNT, FRAME_BUFFER_SIZE, _, _>(pixels, brightness, correction);
        self.write(frame_buffer)
    }
}

/// Core trait for all async LED drivers.
///
/// This trait defines the common interface for asynchronously sending color data
/// to LED hardware, regardless of the specific protocol or chipset being used.
///
/// # Type Parameters
///
/// * `Error` - The error type that may be returned by the driver
/// * `Color` - The color type accepted by the driver
/// * `Word` - The word type used for the encoded frame buffer
///
/// # Example
///
/// ```rust
/// # use blinksy::{
/// #     color::{ColorCorrection, FromColor, LinearSrgb},
/// #     driver::DriverAsync,
/// # };
///
/// struct MyAsyncDriver {
///     // Implementation details
/// }
///
/// impl DriverAsync for MyAsyncDriver {
///     type Error = ();
///     type Color = LinearSrgb;
///     type Word = u8;
///
///     fn encode<
///         const PIXEL_COUNT: usize,
///         const FRAME_BUFFER_SIZE: usize,
///         Pixels,
///         C,
///     >(
///         &mut self,
///         pixels: Pixels,
///         brightness: f32,
///         correction: ColorCorrection,
///     ) -> heapless::Vec<Self::Word, FRAME_BUFFER_SIZE>
///     where
///         Pixels: IntoIterator<Item = C>,
///         Self::Color: FromColor<C>,
///     {
///         heapless::Vec::new()
///     }
///
///     async fn write<const FRAME_BUFFER_SIZE: usize>(
///         &mut self,
///         frame: heapless::Vec<Self::Word, FRAME_BUFFER_SIZE>,
///     ) -> Result<(), Self::Error> {
///         Ok(())
///     }
/// }
/// ```
#[cfg(feature = "async")]
pub trait DriverAsync {
    /// The error type that may be returned by the driver.
    type Error;

    /// The color type accepted by the driver.
    type Color;

    /// The word of the frame buffer.
    type Word;

    /// Encodes an update frame buffer for the LED hardware.
    ///
    /// # Type Parameters
    ///
    /// * `PIXEL_COUNT` - Number of pixels in frame
    /// * `FRAME_BUFFER_SIZE` - Length of encoded frame buffer, in words.
    /// * `Pixels` - Iterator of colors for each pixel
    /// * `Color` - Type of each pixel
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator of colors for each pixel
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Frame buffer for LED hardware
    fn encode<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, Pixels, Color>(
        &mut self,
        pixels: Pixels,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Vec<Self::Word, FRAME_BUFFER_SIZE>
    where
        Pixels: IntoIterator<Item = Color>,
        Self::Color: FromColor<Color>;

    #[allow(async_fn_in_trait)]
    /// Writes frame buffer to the LED hardware, asynchronously.
    ///
    /// # Type Parameters
    ///
    /// * `FRAME_BUFFER_SIZE` - Length of encoded frame buffer, in words.
    ///
    /// # Arguments
    ///
    /// * `frame` - Frame buffer
    ///
    /// # Returns
    ///
    /// Future that resolves to a Result indicating success or an error
    async fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Self::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error>;

    #[allow(async_fn_in_trait)]
    /// Shows a frame on the LED hardware, asynchronously.
    ///
    /// # Type Parameters
    ///
    /// * `PIXEL_COUNT` - Number of pixels in frame
    /// * `FRAME_BUFFER_SIZE` - Length of encoded frame buffer, in words.
    /// * `Pixels` - Iterator of colors for each pixel
    /// * `Color` - Type of each pixel
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator of colors for each pixel
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Future that resolves to a Result indicating success or an error
    async fn show<const PIXEL_COUNT: usize, const FRAME_BUFFER_SIZE: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        let frame_buffer =
            self.encode::<PIXEL_COUNT, FRAME_BUFFER_SIZE, _, _>(pixels, brightness, correction);
        self.write(frame_buffer).await
    }
}
