use crate::util::component::Component;

use super::{
    ColorCorrection, FromColor, GammaSrgb, LedChannels, LedColor, Lms, Okhsl, Okhsv, Oklab, Srgb,
    Xyz,
};

/// # Linear RGB Color Space
///
/// `LinearSrgb` represents colors in a linear RGB color space, where values are directly
/// proportional to light intensity (not gamma-encoded).
///
/// ## Color Space Properties
///
/// - **No Gamma Encoding**: Values are linearly proportional to light intensity
/// - **RGB Primaries**: Same as sRGB (IEC 61966-2-1)
/// - **White Point**: D65 (6500K)
///
/// Mathematical operations on linear RGB values (like averaging or interpolation) will
/// produce physically correct results, unlike operations on gamma-encoded sRGB values.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LinearSrgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
}

impl LinearSrgb {
    /// Creates a new LinearSrgb color
    ///
    /// # Arguments
    ///
    /// - `red` - Red component (0.0 to 1.0)
    /// - `green` - Green component (0.0 to 1.0)
    /// - `blue` - Blue component (0.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use blinksy::color::LinearSrgb;
    ///
    /// let red = LinearSrgb::new(1.0, 0.0, 0.0);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        LinearSrgb {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    /// Converts from linear RGB to sRGB color space
    ///
    /// This applies gamma encoding to make the color values perceptually uniform.
    pub fn to_srgb(self) -> Srgb {
        Srgb::from_linear_srgb(self)
    }

    /// Converts to gamma-encoded RGB using the specified gamma value
    pub fn to_gamma_srgb(self, gamma: f32) -> GammaSrgb {
        GammaSrgb::from_linear_srgb(self, gamma)
    }

    /// Converts to LED output color values
    ///
    /// # Arguments
    ///
    /// - `channels` - The LED channel format specification
    /// - `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// - `correction` - Color correction factors for the LEDs
    ///
    /// # Returns
    ///
    /// A `LedColor` in the specified component type, ready for output to hardware
    pub fn to_led<C: Component>(
        self,
        channels: LedChannels,
        brightness: f32,
        correction: ColorCorrection,
    ) -> LedColor<C> {
        LedColor::from_linear_srgb(self, channels, brightness, correction)
    }
}

impl FromColor<GammaSrgb> for LinearSrgb {
    fn from_color(color: GammaSrgb) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<Lms> for LinearSrgb {
    fn from_color(color: Lms) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<Okhsv> for LinearSrgb {
    fn from_color(color: Okhsv) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<Okhsl> for LinearSrgb {
    fn from_color(color: Okhsl) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<Oklab> for LinearSrgb {
    fn from_color(color: Oklab) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<Srgb> for LinearSrgb {
    fn from_color(color: Srgb) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<Xyz> for LinearSrgb {
    fn from_color(color: Xyz) -> Self {
        color.to_linear_srgb()
    }
}
