use super::{ColorComponent, ColorCorrection};

pub struct OutputRgb<C: ColorComponent> {
    red: C,
    green: C,
    blue: C,
}

impl<C: ColorComponent> OutputRgb<C> {
    pub fn as_array(self) -> [C; 3] {
        [self.red, self.green, self.blue]
    }
}

pub struct OutputRgbw<C: ColorComponent> {
    red: C,
    green: C,
    blue: C,
    white: C,
}

impl<C: ColorComponent> OutputRgbw<C> {
    pub fn as_array(self) -> [C; 4] {
        [self.red, self.green, self.blue, self.white]
    }
}

pub trait OutputColor {
    fn to_rgb<C: ColorComponent>(
        &self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> OutputRgb<C>;

    fn to_rgbw<C: ColorComponent>(
        &self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> OutputRgbw<C>;

    fn to_channels<C: ColorComponent + Copy>(
        &self,
        channels: ColorChannels,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> ColorArray<C> {
        match channels {
            ColorChannels::Rgb(rgb_order) => {
                let rgb = self.to_rgb(brightness, gamma, correction);
                ColorArray::Rgb(rgb_order.reorder(rgb.as_array()))
            }
            ColorChannels::Rgbw(rgbw_order) => {
                let rgbw = self.to_rgbw(brightness, gamma, correction);
                ColorArray::Rgbw(rgbw_order.reorder(rgbw.as_array()))
            }
        }
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
