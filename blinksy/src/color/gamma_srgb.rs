use super::LinearSrgb;
#[allow(unused_imports)]
use num_traits::Float;

/// # Gamma-corrected RGB Color Space
///
/// `GammaSrgb` represents colors in a gamma-corrected RGB color space with a customizable
/// gamma value. This is useful for working with display systems that use different gamma
/// correction factors than the standard sRGB specification.
///
/// ## Color Space Properties
///
/// - **Gamma Encoding**: Uses a simple power-law gamma encoding (γ)
/// - **RGB Primaries**: Same as sRGB primaries defined in IEC 61966-2-1
/// - **White Point**: D65 (6500K)
///
/// Unlike the standard sRGB transfer function which uses a piecewise curve,
/// GammaSrgb uses a simple power function: C_gamma = C_linear^(1/gamma)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GammaSrgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
    /// Gamma correction factor
    pub gamma: f32,
}

impl GammaSrgb {
    /// Creates a new gamma-encoded sRGB color
    ///
    /// # Arguments
    ///
    /// * `red` - Red component (0.0 to 1.0)
    /// * `green` - Green component (0.0 to 1.0)
    /// * `blue` - Blue component (0.0 to 1.0)
    /// * `gamma` - Gamma correction factor
    ///
    /// # Example
    ///
    /// ```
    /// use blinksy::color::GammaSrgb;
    ///
    /// let red = GammaSrgb::new(1.0, 0.0, 0.0, 2.2);
    /// let green = GammaSrgb::new(0.0, 1.0, 0.0, 2.2);
    /// let blue = GammaSrgb::new(0.0, 0.0, 1.0, 2.2);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32, gamma: f32) -> Self {
        Self {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
            gamma,
        }
    }

    /// Creates a gamma-encoded color from linear RGB values
    ///
    /// # Arguments
    ///
    /// * `linear_srgb` - Linear RGB color
    /// * `gamma` - Gamma correction factor
    pub fn from_linear_srgb(linear_srgb: LinearSrgb, gamma: f32) -> Self {
        Self {
            red: gamma_encode(linear_srgb.red, gamma),
            green: gamma_encode(linear_srgb.green, gamma),
            blue: gamma_encode(linear_srgb.blue, gamma),
            gamma,
        }
    }

    /// Converts back to linear RGB by removing gamma encoding
    pub fn to_linear_srgb(self) -> LinearSrgb {
        LinearSrgb {
            red: gamma_decode(self.red, self.gamma),
            green: gamma_decode(self.green, self.gamma),
            blue: gamma_decode(self.blue, self.gamma),
        }
    }
}

/// Convert gamma-encoded value to linear value using standard power law
///
/// For gamma-encoded value c_gamma:
/// - c_linear = c_gamma^gamma
#[inline]
fn gamma_decode(c: f32, gamma: f32) -> f32 {
    c.powf(gamma)
}

/// Convert linear value to gamma-encoded value using standard power law
///
/// For linear value c_linear:
/// - c_gamma = c_linear^(1/gamma)
#[inline]
fn gamma_encode(c: f32, gamma: f32) -> f32 {
    c.powf(1.0 / gamma)
}
