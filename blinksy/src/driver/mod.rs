//! # LED Driver Interface
//!
//! This module defines the core abstractions for driving LED hardware.
//! It provides traits and implementations for interfacing with different
//! LED chipsets and protocols.
//!
//! The main components are:
//!
//! - [`LedDriver`]: The core trait for all LED drivers
//! - [`clocked`]: Implementations for clocked protocols (like APA102)
//! - [`clockless`]: Implementations for clockless protocols (like WS2812)

use crate::color::{ColorCorrection, FromColor};

pub mod clocked;
pub mod clockless;

pub use clocked::*;
pub use clockless::*;

/// Core trait for all LED drivers.
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
/// use blinksy::{color::{ColorCorrection, FromColor, LinearSrgb}, driver::LedDriver};
///
/// struct MyDriver {
///     // Implementation details
/// }
///
/// impl LedDriver for MyDriver {
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
pub trait LedDriver {
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
