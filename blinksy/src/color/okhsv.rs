use super::{LinearSrgb, Oklab};

/// Okhsv color space representation.
///
/// A color space based on Oklab that uses hue, saturation,
/// and value (brightness) components.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Okhsv {
    /// Hue component [0.0, 1.0] where 0 and 1 both represent red
    pub h: f32,
    /// Saturation component [0.0, 1.0]
    pub s: f32,
    /// Value component [0.0, 1.0]
    pub v: f32,
}

impl Okhsv {
    /// Creates a new Okhsv color with the specified components.
    /// All parameters are clamped to their valid ranges.
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Okhsv {
            h: h.rem_euclid(1.0),
            s: s.max(0.0).min(1.0),
            v: v.max(0.0).min(1.0),
        }
    }

    /// Converts Okhsv to Oklab.
    pub fn to_oklab(&self) -> Oklab {
        let v = self.v;

        // Calculate max chroma for this value
        let max_c = 0.4 * v;

        // Calculate chroma
        let c = self.s * max_c;

        // Convert hue and chroma to a, b components
        let angle = 2.0 * core::f32::consts::PI * self.h;
        let a = c * angle.cos();
        let b = c * angle.sin();

        Oklab { l: v, a, b }
    }

    /// Converts Okhsv to linear RGB.
    pub fn to_linear_rgb(&self) -> LinearSrgb {
        self.to_oklab().to_linear_srgb()
    }
}
