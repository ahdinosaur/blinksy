use super::LinearSrgb;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GammaSrgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
    /// Gamma
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
    ///
    /// # Example
    ///
    /// ```
    /// use blinksy::color::Srgb;
    ///
    /// let red = GammaSrgb::new(1.0, 0.0, 0.0);
    /// let green = GammaSrgb::new(0.0, 1.0, 0.0);
    /// let blue = GammaSrgb::new(0.0, 0.0, 1.0);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32, gamma: f32) -> Self {
        Self {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
            gamma,
        }
    }

    pub fn from_linear_srgb(linear_srgb: LinearSrgb, gamma: f32) -> Self {
        Self {
            red: gamma_encode(linear_srgb.red, gamma),
            green: gamma_encode(linear_srgb.green, gamma),
            blue: gamma_encode(linear_srgb.blue, gamma),
            gamma,
        }
    }

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
///
/// The gamma value is typically in the range 1.8-2.2, where 2.2 is common.
#[inline]
fn gamma_decode(c: f32, gamma: f32) -> f32 {
    c.powf(gamma)
}

/// Convert linear value to gamma-encoded value using standard power law
///
/// For linear value c_linear:
/// - c_gamma = c_linear^(1/gamma)
///
/// The gamma value is typically in the range 1.8-2.2, where 2.2 is common.
#[inline]
fn gamma_encode(c: f32, gamma: f32) -> f32 {
    c.powf(1.0 / gamma)
}
