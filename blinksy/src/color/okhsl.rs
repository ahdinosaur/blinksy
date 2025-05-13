use super::{LinearSrgb, Oklab};

/// Okhsl color space representation.
///
/// A color space based on Oklab that uses the more intuitive
/// hue, saturation, and lightness components.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Okhsl {
    /// Hue component [0.0, 1.0) where 0 and 1 both represent red
    pub h: f32,
    /// Saturation component [0.0, 1.0]
    pub s: f32,
    /// Lightness component [0.0, 1.0]
    pub l: f32,
}

impl Okhsl {
    /// Creates a new Okhsl color with the specified components.
    /// All parameters are clamped to their valid ranges.
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Okhsl {
            h: h.rem_euclid(1.0),
            s: s.max(0.0).min(1.0),
            l: l.max(0.0).min(1.0),
        }
    }

    /// Converts Okhsl to Oklab.
    pub fn to_oklab(&self) -> Oklab {
        let l = self.l;

        // Calculate max chroma for this lightness
        let max_c = if l < 0.5 { 0.4 * l } else { 0.4 * (1.0 - l) };

        // Calculate chroma
        let c = self.s * max_c;

        // Convert hue and chroma to a, b components
        let angle = 2.0 * core::f32::consts::PI * self.h;
        let a = c * angle.cos();
        let b = c * angle.sin();

        Oklab { l, a, b }
    }

    /// Converts Okhsl to linear RGB.
    pub fn to_linear_rgb(&self) -> LinearSrgb {
        self.to_oklab().to_linear_srgb()
    }
}
