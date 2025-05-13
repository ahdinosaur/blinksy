use core::marker::PhantomData;

#[allow(unused_imports)]
use num_traits::float::FloatCore;
#[allow(unused_imports)]
use num_traits::Euclid;

use super::LinearRgb;

/// Representation of a color hue with a specific mapping method
///
/// The `Hue` type represents a position on the color wheel using a mapping
/// method (M) to convert between hue values and RGB.
///
/// Different hue maps produce different color distributions when rotating
/// through the entire hue range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hue<M: HueMap = HueRainbow> {
    /// Phantom data to track the hue mapping type
    map: PhantomData<M>,
    /// Hue value (0.0 to 1.0)
    inner: f32,
}

impl<M: HueMap> Hue<M> {
    /// Creates a new hue value
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue value (0.0 to 1.0)
    pub fn new(hue: f32) -> Self {
        Self {
            map: PhantomData,
            inner: Euclid::rem_euclid(&hue, &1.0),
        }
    }

    /// Returns the raw hue value (0.0 to 1.0)
    pub fn inner(self) -> f32 {
        self.inner
    }

    /// Converts the hue to RGB using the specified mapping method
    pub fn to_rgb(&self) -> LinearRgb {
        M::hue_to_rgb(self.inner)
    }
}

/// Trait for hue mapping algorithms
///
/// A hue map defines how a numerical hue value (0.0 to 1.0) is converted
/// to RGB colors. Different mapping approaches produce different visual
/// effects when animating through the hue range.
pub trait HueMap: Sized {
    /// Convert a hue value to RGB
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue value (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A LinearRgb color representing the hue
    fn hue_to_rgb(hue: f32) -> LinearRgb;
}

/// Spectrum hue mapping as used in FastLED's hsv2rgb_spectrum
///
/// This hue mapping produces a mathematically straight spectrum with
/// equal distribution of hues. It has more green and blue, and less
/// yellow and orange.
///
/// ![Spectrum hue mapping](https://raw.githubusercontent.com/FastLED/FastLED/gh-pages/images/HSV-spectrum-with-desc.jpg)
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HueSpectrum;

impl HueMap for HueSpectrum {
    fn hue_to_rgb(hue: f32) -> LinearRgb {
        let h = hue * 3.0; // Scale to 0-3 range
        let section = h.floor() as u8; // Which section: 0, 1, or 2
        let offset = h - h.floor(); // Position within section (0.0-1.0)

        // Calculate rising and falling values
        let rise = offset;
        let fall = 1.0 - offset;

        // Map to RGB based on section
        match section % 3 {
            0 => LinearRgb::new(fall, rise, 0.0), // Red to Green
            1 => LinearRgb::new(0.0, fall, rise), // Green to Blue
            2 => LinearRgb::new(rise, 0.0, fall), // Blue to Red
            _ => unreachable!(),                  // Only for the compiler
        }
    }
}

/// Rainbow hue mapping as used in FastLED's hsv2rgb_rainbow
///
/// This hue mapping produces a visually balanced rainbow effect with
/// enhanced yellow region and other perceptual adjustments.
///
/// ![Rainbow hue mapping](https://raw.githubusercontent.com/FastLED/FastLED/gh-pages/images/HSV-rainbow-with-desc.jpg)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HueRainbow;

impl HueMap for HueRainbow {
    fn hue_to_rgb(hue: f32) -> LinearRgb {
        const FRAC_1_3: f32 = 0.3333333_f32;
        const FRAC_2_3: f32 = 0.6666667_f32;

        let h8 = hue * 8.0; // Scale to 0-8 range
        let section = h8.floor() as u8; // 0-7
        let pos = h8 - h8.floor(); // 0.0-1.0 position within section

        match section % 8 {
            0 => {
                // Red (1,0,0) to Orange (~⅔,⅓,0)
                LinearRgb::new(
                    1.0 - (pos * FRAC_1_3), // R: 1→⅔ (fade to ~⅔)
                    pos * FRAC_1_3,         // G: 0→⅓ (rise to ~⅓)
                    0.0,                    // B: 0
                )
            }
            1 => {
                // Orange (~⅔,⅓,0) to Yellow (~⅔,⅔,0)
                LinearRgb::new(
                    FRAC_2_3,                    // R: stays at ~⅔
                    FRAC_1_3 + (pos * FRAC_1_3), // G: ⅓→⅔ (⅓→⅔)
                    0.0,                         // B: 0
                )
            }
            2 => {
                // Yellow (~⅔,⅔,0) to Green (0,1,0)
                LinearRgb::new(
                    FRAC_2_3 * (1.0 - pos),      // R: ⅔→0 (fade from ⅔ to 0)
                    FRAC_2_3 + (pos * FRAC_1_3), // G: ⅔→1 (rise from ⅔ to 1)
                    0.0,                         // B: 0
                )
            }
            3 => {
                // Green (0,1,0) to Aqua (0,⅔,⅓)
                LinearRgb::new(
                    0.0,                    // R: 0
                    1.0 - (pos * FRAC_1_3), // G: 1→⅔ (fade from 1 to ⅔)
                    pos * FRAC_1_3,         // B: 0→⅓ (rise to ⅓)
                )
            }
            4 => {
                // Aqua (0,⅔,⅓) to Blue (0,0,1)
                LinearRgb::new(
                    0.0,                         // R: 0
                    FRAC_2_3 * (1.0 - pos),      // G: ⅔→0 (fade from ⅔ to 0)
                    FRAC_1_3 + (pos * FRAC_2_3), // B: ⅓→1 (rise from ⅓ to 1)
                )
            }
            5 => {
                // Blue (0,0,1) to Purple (⅓,0,⅔)
                LinearRgb::new(
                    pos * FRAC_1_3,         // R: 0→⅓ (rise to ⅓)
                    0.0,                    // G: 0
                    1.0 - (pos * FRAC_1_3), // B: 1→⅔ (fade from 1 to ⅔)
                )
            }
            6 => {
                // Purple (⅓,0,⅔) to Pink (⅔,0,⅓)
                LinearRgb::new(
                    FRAC_1_3 + (pos * FRAC_1_3), // R: ⅓→⅔ (rise from ⅓ to ⅔)
                    0.0,                         // G: 0
                    FRAC_2_3 - (pos * FRAC_1_3), // B: ⅔→⅓ (fade from ⅔ to ⅓)
                )
            }
            7 => {
                // Pink (⅔,0,⅓) to Red (1,0,0)
                LinearRgb::new(
                    FRAC_2_3 + (pos * FRAC_1_3), // R: ⅔→1 (rise from ⅔ to 1)
                    0.0,                         // G: 0
                    FRAC_1_3 * (1.0 - pos),      // B: ⅓→0 (fade from ⅓ to 0)
                )
            }
            _ => unreachable!(), // Only for the compiler
        }
    }
}
