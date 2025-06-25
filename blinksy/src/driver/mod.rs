//! # Driver Interface
//!
//! A driver is what tells the LED hardware how to be the colors you want.
//!
//! - [`Driver`] is the core trait for all drivers
//! - The [`clocked`] module provides re-usable implementations for clocked (two-wire) protocols (like [`APA102`](crate::drivers::apa102))
//! - The [`clockless`] module provides re-usable implementations for clockless (one-wire) protocols (like [`WS2812`](crate::drivers::ws2812))

use crate::color::{ColorCorrection, FromColor};

pub mod clocked;
pub mod clockless;

pub use clocked::*;
pub use clockless::*;

/// Core blocking trait for all LED drivers.
///
/// This trait defines the common interface for sending color data to LED hardware,
/// regardless of the specific protocol or chipset being used.
///
/// # Type Parameters
///
/// * `Error` - The error type that may be returned by the driver
/// * `Color` - The color type accepted by the driver
///
/// # Example
///
/// ```rust
/// # use blinksy::{color::{ColorCorrection, FromColor, LinearSrgb}, driver::Driver};
///
/// struct MyDriver {
///     // Implementation details
/// }
///
/// impl Driver for MyDriver {
///     type Error = ();
///     type Color = LinearSrgb;
///
///     fn write<I, C>(&mut self, pixels: I, brightness: f32, correction: ColorCorrection) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = C>,
///         Self::Color: FromColor<C>,
///     {
///         // Implementation of writing colors to the LED hardware
///         Ok(())
///     }
/// }
/// ```
pub trait Driver {
    /// The error type that may be returned by the driver.
    type Error;

    /// The color type accepted by the driver.
    type Color;

    /// Writes a sequence of colors to the LED hardware.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>;
}

#[cfg(feature = "async")]
/// Core async trait for all LED drivers.
///
/// This trait defines the common interface for asynchronously sending color data to LED hardware,
/// regardless of the specific protocol or chipset being used.
///
/// # Type Parameters
///
/// * `Error` - The error type that may be returned by the driver
/// * `Color` - The color type accepted by the driver
///
/// # Example
///
/// ```rust
/// # use blinksy::{color::{ColorCorrection, FromColor, LinearSrgb}, driver::DriverAsync};
///
/// struct MyAsyncDriver {
///     // Implementation details
/// }
///
/// impl DriverAsync for MyAsyncDriver {
///     type Error = ();
///     type Color = LinearSrgb;
///
///     async fn write<I, C>(&mut self, pixels: I, brightness: f32, correction: ColorCorrection) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = C>,
///         Self::Color: FromColor<C>,
///     {
///         // Async implementation of writing colors to the LED hardware
///         Ok(())
///     }
/// }
/// ```
pub trait DriverAsync {
    /// The error type that may be returned by the driver.
    type Error;

    /// The color type accepted by the driver.
    type Color;

    // See note about allow(async_fn_in_trait) in smart-leds-trait:
    //   https://github.com/smart-leds-rs/smart-leds-trait/blob/faad5eba0f9c9aa80b1dd17e078e4644f11e7ee0/src/lib.rs#L59-L68
    #[allow(async_fn_in_trait)]
    /// Writes a sequence of colors to the LED hardware, asynchronously.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Future that resolves to a Result indicating success or an error
    async fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>;
}
