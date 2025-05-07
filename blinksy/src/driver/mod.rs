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
//! - Color channel utilities for handling different RGB/RGBW ordering

use smart_leds_trait::SmartLedsWrite;

pub mod clocked;
pub mod clockless;

pub use clocked::*;
pub use clockless::*;

use crate::color::{FromColor, GammaSrgb};

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
/// use blinksy::driver::LedDriver;
/// use palette::Srgb;
///
/// struct MyDriver {
///     // Implementation details
/// }
///
/// impl LedDriver for MyDriver {
///     type Error = ();
///     type Color = Srgb;
///
///     fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = C>,
///         Self::Color: palette::FromColor<C>,
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
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>;
}

/// Implementation of LedDriver for smart-leds-compatible drivers.
///
/// This allows using any driver implementing the smart-leds-trait interface with Blinksy.
impl<Driver, DriverColor> LedDriver for Driver
where
    Driver: SmartLedsWrite<Color = DriverColor>,
    DriverColor: From<smart_leds_trait::RGB<f32>>,
{
    type Color = GammaSrgb;
    type Error = Driver::Error;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        let iterator = pixels.into_iter().map(|color| {
            let color = GammaSrgb::<f32>::from_color(color);
            let color = color * brightness;
            smart_leds_trait::RGB::<f32>::new(color.red, color.green, color.blue)
        });
        SmartLedsWrite::write(self, iterator)
    }
}
