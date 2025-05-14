use crate::util::Component;

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

/// RGB color values ready for output to LED hardware
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LedRgb<C>([C; 3]);

impl<C: Component> LedRgb<C> {
    pub fn from_linear_srgb(
        linear_srgb: LinearSrgb,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Self {
        let LinearSrgb { red, green, blue } = linear_srgb;

        let red = red * correction.red;
        let green = green * correction.green;
        let blue = blue * correction.blue;

        let red = red * brightness;
        let green = green * brightness;
        let blue = blue * brightness;

        let red = red.clamp(0., 1.);
        let green = green.clamp(0., 1.);
        let blue = blue.clamp(0., 1.);

        Self([
            C::from_normalized_f32(red),
            C::from_normalized_f32(green),
            C::from_normalized_f32(blue),
        ])
    }

    pub fn reorder(self, channels: RgbChannels) -> Self {
        Self(channels.reorder(self.0))
    }
}

impl<C> AsRef<[C]> for LedRgb<C> {
    fn as_ref(&self) -> &[C] {
        &self.0
    }
}

/// RGBW color values ready for output to LED hardware
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LedRgbw<C>([C; 4]);

impl<C: Component> LedRgbw<C> {
    pub fn from_linear_srgb(
        linear_srgb: LinearSrgb,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Self {
        let LinearSrgb { red, green, blue } = linear_srgb;

        let white = red.min(green).min(blue);
        let red = red - white;
        let green = green - white;
        let blue = blue - white;

        let red = red * correction.red;
        let green = green * correction.green;
        let blue = blue * correction.blue;

        let red = red * brightness;
        let green = green * brightness;
        let blue = blue * brightness;
        let white = white * brightness;

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

    pub fn reorder(self, channels: RgbwChannels) -> Self {
        Self(channels.reorder(self.0))
    }
}

impl<C> AsRef<[C]> for LedRgbw<C> {
    fn as_ref(&self) -> &[C] {
        &self.0
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

/// # Linear RGBW Color Space
///
/// `LinearSrgbw` represents colors in a linear RGB color space with an additional
/// white channel. This is particularly useful for RGBW LED strips.
///
/// The white channel represents a dedicated white LED that can be used to enhance
/// brightness and efficiency for neutral/white colors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearSrgbw {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
    /// White component (0.0 to 1.0)
    pub white: f32,
}

impl LinearSrgbw {
    /// Creates a new LinearSrgbw color
    ///
    /// # Arguments
    ///
    /// * `red` - Red component (0.0 to 1.0)
    /// * `green` - Green component (0.0 to 1.0)
    /// * `blue` - Blue component (0.0 to 1.0)
    /// * `white` - White component (0.0 to 1.0)
    pub fn new(red: f32, green: f32, blue: f32, white: f32) -> Self {
        LinearSrgbw {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
            white: white.clamp(0.0, 1.0),
        }
    }

    /// Applies brightness scaling to RGB components (not white)
    pub fn apply_brightness(&mut self, brightness: f32) {
        self.red *= brightness;
        self.green *= brightness;
        self.blue *= brightness;
        self.white *= brightness;
    }

    /// Applies color correction factors to RGB components (not white)
    pub fn apply_color_correction(&mut self, correction: ColorCorrection) {
        self.red *= correction.red;
        self.green *= correction.green;
        self.blue *= correction.blue;
    }

    /// Clamps all components to the range [0.0, 1.0]
    pub fn clamp(&mut self) {
        self.red = self.red.clamp(0.0, 1.0);
        self.green = self.green.clamp(0.0, 1.0);
        self.blue = self.blue.clamp(0.0, 1.0);
        self.white = self.white.clamp(0.0, 1.0);
    }

    /// Creates an RGBW color from RGB by extracting the white component
    ///
    /// This method extracts the minimum of R,G,B as the white component,
    /// which is a common approach for converting RGB to RGBW.
    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        let LinearSrgb { red, green, blue } = linear_srgb;
        let white = red.min(green).min(blue);

        LinearSrgbw {
            red: red - white,
            green: green - white,
            blue: blue - white,
            white,
        }
    }

    /// Converts to LinearSrgb by combining white with RGB channels
    pub fn to_linear_srgb(&self) -> LinearSrgb {
        LinearSrgb {
            red: self.red + self.white,
            green: self.green + self.white,
            blue: self.blue + self.white,
        }
    }
}
