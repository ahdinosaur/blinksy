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

use core::ops::Sub;
use palette::{cast::into_array, stimulus::IntoStimulus, FromColor, LinSrgb, Srgb};
use smart_leds_trait::SmartLedsWrite;

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
    type Color = palette::Srgb;
    type Error = Driver::Error;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        let iterator = pixels.into_iter().map(|color| {
            let color: LinSrgb<f32> = Srgb::from_color(color).into_linear();
            let color = color * brightness;
            smart_leds_trait::RGB::<f32>::new(color.red, color.green, color.blue)
        });
        SmartLedsWrite::write(self, iterator)
    }
}

/// Enumeration of color channel formats.
///
/// Different LED chipsets have different ordering of color channels.
/// This enum represents the possible arrangements.
#[derive(Debug)]
pub enum ColorChannels {
    /// RGB with 3 channels
    Rgb(RgbChannels),
    /// RGBW with 4 channels
    Rgbw(RgbwChannels),
}

impl ColorChannels {
    /// Returns the number of color channels.
    pub const fn channel_count(&self) -> usize {
        match self {
            ColorChannels::Rgb(_) => 3,
            ColorChannels::Rgbw(_) => 4,
        }
    }
}

/// Enumeration of RGB channel orders.
///
/// Different RGB LED chipsets may use different ordering of the R, G, and B channels.
#[derive(Debug)]
pub enum RgbChannels {
    /// Red, Green, Blue
    RGB,
    /// Red, Blue, Green
    RBG,
    /// Green, Red, Blue
    GRB,
    /// Green, Blue, Red
    GBR,
    /// Blue, Red, Green
    BRG,
    /// Blue, Green, Red
    BGR,
}

/// Enumeration of RGBW channel orders.
///
/// Different RGBW LED chipsets may use different ordering of the R, G, B, and W channels.
#[derive(Debug)]
pub enum RgbwChannels {
    // RGB
    WRGB,
    RWGB,
    RGWB,
    RGBW,

    // RBG
    WRBG,
    RWBG,
    RBWG,
    RBGW,

    // GRB
    WGRB,
    GWRB,
    GRWB,
    GRBW,

    // GBR
    WGBR,
    GWBR,
    GBWR,
    GBRW,

    // BRG
    WBRG,
    BWRG,
    BRWG,
    BRGW,

    // BGR
    WBGR,
    BWGR,
    BGWR,
    BGRW,
}

impl RgbChannels {
    /// Reorders RGB values according to the channel order.
    ///
    /// # Arguments
    ///
    /// * `rgb` - Array of [R, G, B] values in canonical order
    ///
    /// # Returns
    ///
    /// Array of values reordered according to the channel specification
    pub fn reorder<Word: Copy>(&self, rgb: [Word; 3]) -> [Word; 3] {
        use RgbChannels::*;
        match self {
            RGB => [rgb[0], rgb[1], rgb[2]],
            RBG => [rgb[0], rgb[2], rgb[1]],
            GRB => [rgb[1], rgb[0], rgb[2]],
            GBR => [rgb[1], rgb[2], rgb[0]],
            BRG => [rgb[2], rgb[0], rgb[1]],
            BGR => [rgb[2], rgb[1], rgb[0]],
        }
    }
}

impl RgbwChannels {
    /// Reorders RGBW values according to the channel order.
    ///
    /// # Arguments
    ///
    /// * `rgbw` - Array of [R, G, B, W] values in canonical order
    ///
    /// # Returns
    ///
    /// Array of values reordered according to the channel specification
    pub fn reorder<Word: Copy>(&self, rgbw: [Word; 4]) -> [Word; 4] {
        use RgbwChannels::*;
        match self {
            // RGB
            WRGB => [rgbw[3], rgbw[0], rgbw[1], rgbw[2]],
            RWGB => [rgbw[0], rgbw[3], rgbw[1], rgbw[2]],
            RGWB => [rgbw[0], rgbw[1], rgbw[3], rgbw[2]],
            RGBW => [rgbw[0], rgbw[1], rgbw[2], rgbw[3]],

            // RBG
            WRBG => [rgbw[3], rgbw[0], rgbw[2], rgbw[1]],
            RWBG => [rgbw[0], rgbw[3], rgbw[2], rgbw[1]],
            RBWG => [rgbw[0], rgbw[2], rgbw[3], rgbw[1]],
            RBGW => [rgbw[0], rgbw[2], rgbw[1], rgbw[3]],

            // GRB
            WGRB => [rgbw[3], rgbw[1], rgbw[0], rgbw[2]],
            GWRB => [rgbw[1], rgbw[3], rgbw[0], rgbw[2]],
            GRWB => [rgbw[1], rgbw[0], rgbw[3], rgbw[2]],
            GRBW => [rgbw[1], rgbw[0], rgbw[2], rgbw[3]],

            // GBR
            WGBR => [rgbw[3], rgbw[1], rgbw[2], rgbw[0]],
            GWBR => [rgbw[1], rgbw[3], rgbw[2], rgbw[0]],
            GBWR => [rgbw[1], rgbw[2], rgbw[3], rgbw[0]],
            GBRW => [rgbw[1], rgbw[2], rgbw[0], rgbw[3]],

            // BRG
            WBRG => [rgbw[3], rgbw[2], rgbw[0], rgbw[1]],
            BWRG => [rgbw[2], rgbw[3], rgbw[0], rgbw[1]],
            BRWG => [rgbw[2], rgbw[0], rgbw[3], rgbw[1]],
            BRGW => [rgbw[2], rgbw[0], rgbw[1], rgbw[3]],

            // BGR
            WBGR => [rgbw[3], rgbw[2], rgbw[1], rgbw[0]],
            BWGR => [rgbw[2], rgbw[3], rgbw[1], rgbw[0]],
            BGWR => [rgbw[2], rgbw[1], rgbw[3], rgbw[0]],
            BGRW => [rgbw[2], rgbw[1], rgbw[0], rgbw[3]],
        }
    }
}

/// Container for color data in various formats.
///
/// This enum provides a convenient way to handle both RGB and RGBW color arrays
/// with the same interface.
#[derive(Debug)]
pub enum ColorArray<Word> {
    /// RGB color data
    Rgb([Word; 3]),
    /// RGBW color data
    Rgbw([Word; 4]),
}

impl<Word> AsRef<[Word]> for ColorArray<Word> {
    fn as_ref(&self) -> &[Word] {
        match self {
            ColorArray::Rgb(rgb) => rgb,
            ColorArray::Rgbw(rgbw) => rgbw,
        }
    }
}

impl ColorChannels {
    /// Converts an sRGB color to a properly ordered array for the specified color channels.
    ///
    /// # Type Parameters
    ///
    /// * `Word` - The numeric type for each color component
    ///
    /// # Arguments
    ///
    /// * `color` - The sRGB color to convert
    ///
    /// # Returns
    ///
    /// A ColorArray containing the color data in the appropriate format and order
    pub fn to_array<Word>(&self, color: Srgb<f32>) -> ColorArray<Word>
    where
        f32: IntoStimulus<Word>,
        Word: Copy + PartialOrd + Sub<Output = Word>,
    {
        let color: LinSrgb<Word> = Srgb::from_color(color).into_linear().into_format();
        let rgb = into_array(color);

        match self {
            ColorChannels::Rgb(rgb_order) => ColorArray::Rgb(rgb_order.reorder(rgb)),
            ColorChannels::Rgbw(rgbw_order) => {
                let rgbw = rgb_to_rgbw(rgb);
                ColorArray::Rgbw(rgbw_order.reorder(rgbw))
            }
        }
    }
}

/// Extracts the white component from the RGB values by taking the minimum of R, G, and B.
/// Then subtracts that white component from each channel so the remaining RGB is "color only."
///
/// # Arguments
///
/// * `rgb` - RGB color values
///
/// # Returns
///
/// RGBW color values with the white component extracted
fn rgb_to_rgbw<Word>(rgb: [Word; 3]) -> [Word; 4]
where
    Word: Copy + PartialOrd + Sub<Output = Word>,
{
    let w = if rgb[0] <= rgb[1] && rgb[0] <= rgb[2] {
        rgb[0]
    } else if rgb[1] <= rgb[0] && rgb[1] <= rgb[2] {
        rgb[1]
    } else {
        rgb[2]
    };

    [rgb[0] - w, rgb[1] - w, rgb[2] - w, w]
}
