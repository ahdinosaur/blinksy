/// Color data ready for output to LED hardware
#[derive(Debug)]
pub enum LedColor<C> {
    /// Output RGB color data
    Rgb(LedRgb<C>),
    /// Output RGBW color data
    Rgbw(LedRgbw<C>),
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
pub struct LedRgb<C>([C; 3]);

impl<C> LedRgb<C> {
    pub fn new(red: C, green: C, blue: C) -> Self {
        Self([red, green, blue])
    }
}

impl<C> AsRef<[C]> for LedRgb<C> {
    fn as_ref(&self) -> &[C] {
        &self.0
    }
}

/// RGBW color values ready for output to LED hardware
pub struct LedRgbw<C>([C; 4]);

impl<C> LedRgbw<C> {
    pub fn new(red: C, green: C, blue: C, white: C) -> Self {
        Self([red, green, blue, white])
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
