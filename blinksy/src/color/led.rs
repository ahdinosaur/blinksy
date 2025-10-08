use core::{array::IntoIter, iter::Iterator, ops::Index};

use crate::util::component::Component;

use super::{ColorCorrection, LinearSrgb};

/// Color data ready for output to LED hardware
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LedColor<C> {
    /// Output RGB color data
    Rgb(LedRgb<C>),
    /// Output RGBW color data
    Rgbw(LedRgbw<C>),
}

impl<C: Component> LedColor<C> {
    /// Creates an output-ready LED color from a linear sRGB color
    ///
    /// # Arguments
    ///
    /// * `linear_srgb` - Linear RGB color to convert
    /// * `channels` - The LED channel format specification
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors for the LEDs
    ///
    /// # Returns
    ///
    /// A `LedColor` ready for output to hardware
    pub fn from_linear_srgb(
        linear_srgb: LinearSrgb,
        channels: LedChannels,
        brightness: f32,
        correction: ColorCorrection,
    ) -> LedColor<C> {
        match channels {
            LedChannels::Rgb(rgb_channels) => {
                let rgb = LedRgb::from_linear_srgb(linear_srgb, brightness, correction);
                LedColor::Rgb(rgb.reorder(rgb_channels))
            }
            LedChannels::Rgbw(rgbw_channels) => {
                let rgbw = LedRgbw::from_linear_srgb(linear_srgb, brightness, correction);
                LedColor::Rgbw(rgbw.reorder(rgbw_channels))
            }
        }
    }
}

impl<C> AsRef<[C]> for LedColor<C> {
    fn as_ref(&self) -> &[C] {
        use LedColor::*;
        match self {
            Rgb(rgb) => rgb.as_ref(),
            Rgbw(rgbw) => rgbw.as_ref(),
        }
    }
}

pub enum LedColorIntoIter<C> {
    Rgb(IntoIter<C, 3>),
    Rgbw(IntoIter<C, 4>),
}

impl<C> Iterator for LedColorIntoIter<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        use LedColorIntoIter::*;
        match self {
            Rgb(rgb_iter) => rgb_iter.next(),
            Rgbw(rgbw_iter) => rgbw_iter.next(),
        }
    }
}

impl<C> IntoIterator for LedColor<C> {
    type Item = C;
    type IntoIter = LedColorIntoIter<C>;

    fn into_iter(self) -> Self::IntoIter {
        use LedColor::*;
        match self {
            Rgb(rgb) => LedColorIntoIter::Rgb(rgb.into_iter()),
            Rgbw(rgbw) => LedColorIntoIter::Rgbw(rgbw.into_iter()),
        }
    }
}

/// RGB color values ready for output to LED hardware
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LedRgb<C>([C; 3]);

impl<C: Component> LedRgb<C> {
    /// Creates RGB LED output values from a linear sRGB color
    ///
    /// # Arguments
    ///
    /// * `linear_srgb` - Linear RGB color to convert
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors for the LEDs
    ///
    /// # Returns
    ///
    /// A `LedRgb` with component values ready for output
    pub fn from_linear_srgb(
        linear_srgb: LinearSrgb,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Self {
        let LinearSrgb { red, green, blue } = linear_srgb;

        // Apply color correction
        let red = red * correction.red;
        let green = green * correction.green;
        let blue = blue * correction.blue;

        // Apply brightness
        let red = red * brightness;
        let green = green * brightness;
        let blue = blue * brightness;

        // Clamp values
        let red = red.clamp(0., 1.);
        let green = green.clamp(0., 1.);
        let blue = blue.clamp(0., 1.);

        Self([
            C::from_normalized_f32(red),
            C::from_normalized_f32(green),
            C::from_normalized_f32(blue),
        ])
    }

    /// Reorders the RGB components according to the specified channel order
    pub fn reorder(self, channels: RgbChannels) -> Self {
        Self(channels.reorder(self.0))
    }
}

impl<C> AsRef<[C]> for LedRgb<C> {
    fn as_ref(&self) -> &[C] {
        &self.0
    }
}

impl<C> IntoIterator for LedRgb<C> {
    type Item = C;
    type IntoIter = IntoIter<C, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<C> Index<usize> for LedRgb<C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// RGBW color values ready for output to LED hardware
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LedRgbw<C>([C; 4]);

impl<C: Component> LedRgbw<C> {
    /// Creates RGBW LED output values from a linear sRGB color
    ///
    /// This performs white channel extraction using the common minimum method,
    /// where the white component is the minimum of R,G,B, and those values are
    /// then subtracted from the RGB components.
    ///
    /// # Arguments
    ///
    /// * `linear_srgb` - Linear RGB color to convert
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors for the LEDs
    ///
    /// # Returns
    ///
    /// A `LedRgbw` with component values ready for output
    pub fn from_linear_srgb(
        linear_srgb: LinearSrgb,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Self {
        let LinearSrgb { red, green, blue } = linear_srgb;

        // Extract white component (minimum of RGB)
        let white = red.min(green).min(blue);

        // Subtract white from RGB to get true RGB components
        let red = red - white;
        let green = green - white;
        let blue = blue - white;

        // Apply color correction
        let red = red * correction.red;
        let green = green * correction.green;
        let blue = blue * correction.blue;

        // Apply brightness
        let red = red * brightness;
        let green = green * brightness;
        let blue = blue * brightness;
        let white = white * brightness;

        // Clamp values
        let red = red.clamp(0., 1.);
        let green = green.clamp(0., 1.);
        let blue = blue.clamp(0., 1.);
        let white = white.clamp(0., 1.);

        Self([
            C::from_normalized_f32(red),
            C::from_normalized_f32(green),
            C::from_normalized_f32(blue),
            C::from_normalized_f32(white),
        ])
    }

    /// Reorders the RGBW components according to the specified channel order
    pub fn reorder(self, channels: RgbwChannels) -> Self {
        Self(channels.reorder(self.0))
    }
}

impl<C> AsRef<[C]> for LedRgbw<C> {
    fn as_ref(&self) -> &[C] {
        &self.0
    }
}

impl<C> IntoIterator for LedRgbw<C> {
    type Item = C;
    type IntoIter = IntoIter<C, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<C> Index<usize> for LedRgbw<C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Enumeration of color channel formats.
///
/// Different LED chipsets have different ordering of color channels.
/// This enum represents the possible arrangements.
#[derive(Debug)]
pub enum LedChannels {
    /// RGB with 3 channels
    Rgb(RgbChannels),
    /// RGBW with 4 channels
    Rgbw(RgbwChannels),
}

impl LedChannels {
    /// Returns the number of color channels.
    pub const fn channel_count(&self) -> usize {
        match self {
            LedChannels::Rgb(_) => 3,
            LedChannels::Rgbw(_) => 4,
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
