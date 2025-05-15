use super::{LinearSrgb, Oklab};

use num_traits::Euclid;
#[allow(unused_imports)]
use num_traits::Float;

/// # Okhsl Color Space
///
/// A color space based on Oklab that uses the more intuitive hue, saturation,
/// and lightness components. This provides a perceptually uniform alternative
/// to traditional HSL models.
///
/// - `h`: Hue component (0.0 to 1.0) representing the color's position on the color wheel
/// - `s`: Saturation component (0.0 to 1.0) representing the color's intensity/purity
/// - `l`: Lightness component (0.0 to 1.0) representing the color's brightness
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
            h: Euclid::rem_euclid(&h, &1.),
            s: s.clamp(0., 1.),
            l: l.clamp(0., 1.),
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
    pub fn to_linear_srgb(&self) -> LinearSrgb {
        self.to_oklab().to_linear_srgb()
    }
}
