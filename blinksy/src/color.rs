//! # Color Types and Utilities

use core::ops::Sub;
use palette::{
    cast::into_array,
    encoding::{
        gamma::{Gamma, Number},
        Srgb as SrgbEncoding,
    },
    rgb::Rgb,
    stimulus::IntoStimulus,
};

/// sRGB color representation.
///
/// This is the standard RGB color space used for most applications and displays.
pub use palette::Srgb;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct F2p8;

impl Number for F2p8 {
    const VALUE: f64 = 2.8;
}

/// This is the default RGB color space used by FastLED, with gamma correction at power of 2.8
pub type GammaSrgb<T = f32> = Rgb<Gamma<SrgbEncoding, F2p8>, T>;

/// Linear sRGB color representation.
///
/// Linear color space is used for accurate color mixing and transformations.
pub use palette::LinSrgb;

/// Hue-Saturation-Value color representation.
///
/// A more intuitive way to work with colors, especially for animations and patterns.
pub use palette::Hsv;

/// RGB hue component.
///
/// Represents the hue component (color angle) in RGB-based color spaces.
pub use palette::RgbHue;

/// Conversion trait for converting between color types.
pub use palette::FromColor;

/// Conversion trait for converting colors to other color spaces.
pub use palette::IntoColor;

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
    /// Converts a gamma-corrected sRGB color to a properly ordered array for the specified color channels.
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
    pub fn to_array<Word>(&self, color: GammaSrgb<f32>) -> ColorArray<Word>
    where
        f32: IntoStimulus<Word>,
        Word: Copy + PartialOrd + Sub<Output = Word>,
    {
        let color: GammaSrgb<Word> = color.into_format();
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
