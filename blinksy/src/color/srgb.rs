use super::LinearSrgb;

#[allow(unused_imports)]
use num_traits::Float;

/// # sRGB Color Space
///
/// `Srgb` represents colors in the standard RGB (sRGB) color space, which is
/// the most common color space used for displays, web content, and digital images.
///
/// ## Color Space Properties
///
/// - **Gamma Encoding**: Uses a non-linear transfer function (approximately gamma 2.2)
/// - **RGB Primaries**: Uses the sRGB primaries as defined in IEC 61966-2-1
/// - **White Point**: D65 (6500K)
///
/// sRGB values are non-linear (gamma-encoded) to account for human perception. This means
/// that arithmetic operations on sRGB values (like averaging or interpolation) will not
/// produce perceptually correct results. For such operations, convert to `LinearSrgb` first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
}

impl Srgb {
    /// Creates a new sRGB color
    ///
    /// # Arguments
    ///
    /// * `red` - Red component (0.0 to 1.0)
    /// * `green` - Green component (0.0 to 1.0)
    /// * `blue` - Blue component (0.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use blinksy::color::Srgb;
    ///
    /// let red = Srgb::new(1.0, 0.0, 0.0);
    /// let green = Srgb::new(0.0, 1.0, 0.0);
    /// let blue = Srgb::new(0.0, 0.0, 1.0);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Srgb {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    /// Converts from sRGB to linear RGB color space
    ///
    /// This removes the gamma encoding, making the color values proportional to light intensity.
    /// Linear RGB is necessary for physically accurate color calculations.
    pub fn to_linear_srgb(self) -> LinearSrgb {
        LinearSrgb {
            red: srgb_decode(self.red),
            green: srgb_decode(self.green),
            blue: srgb_decode(self.blue),
        }
    }

    /// Converts from linear RGB to sRGB color space
    ///
    /// This applies the sRGB gamma encoding to make the color values perceptually uniform.
    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        Self {
            red: srgb_encode(linear_srgb.red),
            green: srgb_encode(linear_srgb.green),
            blue: srgb_encode(linear_srgb.blue),
        }
    }
}

/// Convert sRGB gamma-encoded component to linear RGB component
///
/// The sRGB standard uses a piece-wise function that's approximately
/// equivalent to a gamma of 2.2, but with a linear segment near zero.
///
/// For gamma-encoded value C_srgb:
/// - If C_srgb ≤ 0.04045: C_linear = C_srgb / 12.92
/// - If C_srgb > 0.04045: C_linear = ((C_srgb + 0.055) / 1.055)^2.4
///
/// References:
/// - http://color.org/sRGB.pdf
/// - http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
#[inline]
fn srgb_decode(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear RGB component to sRGB gamma-encoded component
///
/// The sRGB standard uses a piece-wise function that's approximately
/// equivalent to a gamma of 2.2, but with a linear segment near zero.
///
/// For linear value C_linear:
/// - If C_linear ≤ 0.0031308: C_srgb = 12.92 * C_linear
/// - If C_linear > 0.0031308: C_srgb = 1.055 * C_linear^(1/2.4) - 0.055
///
/// References:
/// - http://color.org/sRGB.pdf
/// - http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
#[inline]
fn srgb_encode(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}
