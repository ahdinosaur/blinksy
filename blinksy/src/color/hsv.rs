use core::marker::PhantomData;

#[allow(unused_imports)]
use num_traits::float::FloatCore;
#[allow(unused_imports)]
use num_traits::Euclid;

use super::{FromColor, LinearSrgb};

/// HSV color model (Hue, Saturation, Value)
///
/// HSV is a color model that separates color into:
///
/// - Hue: The color type (red, green, blue, etc.)
/// - Saturation: The purity of the color (0.0 = grayscale, 1.0 = pure color)
/// - Value: The brightness of the color (0.0 = black, 1.0 = maximum brightness)
///
/// Inspired by [FastLED's HSV], [`Hsv`] receives a generic `M` which implements [`HsvHueMap`], so
/// you can control how a hue is mapped to a color. The default mapping [`HsvHueRainbow`] provides
/// more evenly-spaced color bands, including a band of 'yellow' which is the same width as other
/// colors, and which has an appropriately high inherent brightness.
///
/// [FastLED's HSV]: https://github.com/FastLED/FastLED/wiki/FastLED-HSV-Colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv<M: HsvHueMap = HsvHueRainbow> {
    /// HsvHue component
    pub hue: HsvHue<M>,
    /// Saturation component (0.0 to 1.0)
    pub saturation: f32,
    /// Value component (0.0 to 1.0)
    pub value: f32,
}

impl<M: HsvHueMap> Hsv<M> {
    /// Creates a new HSV color
    ///
    /// # Arguments
    ///
    /// * `hue` - HsvHue component (0.0 to 1.0)
    /// * `saturation` - Saturation component (0.0 to 1.0)
    /// * `value` - Value component (0.0 to 1.0)
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        Self {
            hue: HsvHue::new(hue),
            saturation: saturation.clamp(0.0, 1.0),
            value: value.clamp(0.0, 1.0),
        }
    }

    /// Creates a new HSV color from an existing HsvHue object
    ///
    /// # Arguments
    ///
    /// * `hue` - Existing HsvHue object
    /// * `saturation` - Saturation component (0.0 to 1.0)
    /// * `value` - Value component (0.0 to 1.0)
    pub fn from_hue(hue: HsvHue<M>, saturation: f32, value: f32) -> Self {
        Self {
            hue,
            saturation: saturation.clamp(0.0, 1.0),
            value: value.clamp(0.0, 1.0),
        }
    }
}

impl<M: HsvHueMap> FromColor<Hsv<M>> for LinearSrgb {
    fn from_color(color: Hsv<M>) -> Self {
        // Special case for zero saturation (grayscale)
        if color.saturation <= 0.0 {
            let v = color.value;
            return LinearSrgb::new(v, v, v);
        }

        // Special case for zero value (black)
        if color.value <= 0.0 {
            return LinearSrgb::new(0.0, 0.0, 0.0);
        }

        // Get the pure hue color
        let rgb = color.hue.to_rgb();

        // If fully saturated, just scale by value
        if color.saturation >= 1.0 {
            return LinearSrgb::new(
                rgb.red * color.value,
                rgb.green * color.value,
                rgb.blue * color.value,
            );
        }

        // For partial saturation, blend with gray
        let gray = color.value;
        let s = color.saturation;

        LinearSrgb::new(
            rgb.red * s * color.value + gray * (1.0 - s),
            rgb.green * s * color.value + gray * (1.0 - s),
            rgb.blue * s * color.value + gray * (1.0 - s),
        )
    }
}

/// Representation of a color hue with a specific mapping method
///
/// The [`HsvHue`] type represents a position on the color wheel using a mapping
/// method (M) to convert between hue values and colors.
///
/// Different hue maps produce different color distributions when rotating
/// through the entire hue range. See [FastLED's HSV].
///
/// [FastLED's HSV]: https://github.com/FastLED/FastLED/wiki/FastLED-HSV-Colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsvHue<M: HsvHueMap = HsvHueRainbow> {
    /// Phantom data to track the hue mapping type
    map: PhantomData<M>,
    /// HsvHue value (0.0 to 1.0)
    inner: f32,
}

impl<M: HsvHueMap> HsvHue<M> {
    /// Creates a new hue value
    ///
    /// # Arguments
    ///
    /// * `hue` - HsvHue value (0.0 to 1.0)
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
    pub fn to_rgb(&self) -> LinearSrgb {
        M::hue_to_rgb(self.inner)
    }
}

/// Trait for hue mapping algorithms, inspired by [FastLED's HSV].
///
/// A hue map defines how a numerical hue value (0.0 to 1.0) is converted
/// to RGB colors. Different mapping approaches produce different color
/// distributions when rotating through the entire hue range.
///
/// [FastLED's HSV]: https://github.com/FastLED/FastLED/wiki/FastLED-HSV-Colors
pub trait HsvHueMap: Sized {
    /// Convert a hue value to RGB
    ///
    /// # Arguments
    ///
    /// * `hue` - HsvHue value (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A LinearSrgb color representing the hue
    fn hue_to_rgb(hue: f32) -> LinearSrgb;
}

/// Spectrum hue mapping as used in FastLED's hsv2rgb_spectrum
///
/// This hue mapping produces a mathematically straight spectrum with
/// equal distribution of hues. It has wide red, green and blue bands, with
/// a narrow and muddy yellow band.
///
/// ![Spectrum hue mapping](https://raw.githubusercontent.com/FastLED/FastLED/gh-pages/images/HSV-spectrum-with-desc.jpg)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsvHueSpectrum;

impl HsvHueMap for HsvHueSpectrum {
    fn hue_to_rgb(hue: f32) -> LinearSrgb {
        let h = hue * 3.0; // Scale to 0-3 range
        let section = h.floor() as u8; // Which section: 0, 1, or 2
        let offset = h - h.floor(); // Position within section (0.0-1.0)

        // Calculate rising and falling values
        let rise = offset;
        let fall = 1.0 - offset;

        // Map to RGB based on section
        match section % 3 {
            0 => LinearSrgb::new(fall, rise, 0.0), // Red to Green
            1 => LinearSrgb::new(0.0, fall, rise), // Green to Blue
            2 => LinearSrgb::new(rise, 0.0, fall), // Blue to Red
            _ => unreachable!(),                   // Only for the compiler
        }
    }
}

/// Rainbow hue mapping as used in FastLED's hsv2rgb_rainbow
///
/// This hue mapping produces a visually balanced rainbow effect with
/// enhanced yellow and deep purple.
///
/// ![Rainbow hue mapping](https://raw.githubusercontent.com/FastLED/FastLED/gh-pages/images/HSV-rainbow-with-desc.jpg)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsvHueRainbow;

impl HsvHueMap for HsvHueRainbow {
    fn hue_to_rgb(hue: f32) -> LinearSrgb {
        const FRAC_1_3: f32 = 0.333_333_33_f32;
        const FRAC_2_3: f32 = 0.666_666_7_f32;

        let h8 = hue * 8.0; // Scale to 0-8 range
        let section = h8.floor() as u8; // 0-7
        let pos = h8 - h8.floor(); // 0.0-1.0 position within section

        match section % 8 {
            0 => {
                // Red (1,0,0) to Orange (~⅔,⅓,0)
                LinearSrgb::new(
                    1.0 - (pos * FRAC_1_3), // R: 1→⅔ (fade to ~⅔)
                    pos * FRAC_1_3,         // G: 0→⅓ (rise to ~⅓)
                    0.0,                    // B: 0
                )
            }
            1 => {
                // Orange (~⅔,⅓,0) to Yellow (~⅔,⅔,0)
                LinearSrgb::new(
                    FRAC_2_3,                    // R: stays at ~⅔
                    FRAC_1_3 + (pos * FRAC_1_3), // G: ⅓→⅔ (⅓→⅔)
                    0.0,                         // B: 0
                )
            }
            2 => {
                // Yellow (~⅔,⅔,0) to Green (0,1,0)
                LinearSrgb::new(
                    FRAC_2_3 * (1.0 - pos),      // R: ⅔→0 (fade from ⅔ to 0)
                    FRAC_2_3 + (pos * FRAC_1_3), // G: ⅔→1 (rise from ⅔ to 1)
                    0.0,                         // B: 0
                )
            }
            3 => {
                // Green (0,1,0) to Aqua (0,⅔,⅓)
                LinearSrgb::new(
                    0.0,                    // R: 0
                    1.0 - (pos * FRAC_1_3), // G: 1→⅔ (fade from 1 to ⅔)
                    pos * FRAC_1_3,         // B: 0→⅓ (rise to ⅓)
                )
            }
            4 => {
                // Aqua (0,⅔,⅓) to Blue (0,0,1)
                LinearSrgb::new(
                    0.0,                         // R: 0
                    FRAC_2_3 * (1.0 - pos),      // G: ⅔→0 (fade from ⅔ to 0)
                    FRAC_1_3 + (pos * FRAC_2_3), // B: ⅓→1 (rise from ⅓ to 1)
                )
            }
            5 => {
                // Blue (0,0,1) to Purple (⅓,0,⅔)
                LinearSrgb::new(
                    pos * FRAC_1_3,         // R: 0→⅓ (rise to ⅓)
                    0.0,                    // G: 0
                    1.0 - (pos * FRAC_1_3), // B: 1→⅔ (fade from 1 to ⅔)
                )
            }
            6 => {
                // Purple (⅓,0,⅔) to Pink (⅔,0,⅓)
                LinearSrgb::new(
                    FRAC_1_3 + (pos * FRAC_1_3), // R: ⅓→⅔ (rise from ⅓ to ⅔)
                    0.0,                         // G: 0
                    FRAC_2_3 - (pos * FRAC_1_3), // B: ⅔→⅓ (fade from ⅔ to ⅓)
                )
            }
            7 => {
                // Pink (⅔,0,⅓) to Red (1,0,0)
                LinearSrgb::new(
                    FRAC_2_3 + (pos * FRAC_1_3), // R: ⅔→1 (rise from ⅔ to 1)
                    0.0,                         // G: 0
                    FRAC_1_3 * (1.0 - pos),      // B: ⅓→0 (fade from ⅓ to 0)
                )
            }
            _ => unreachable!(), // Only for the compiler
        }
    }
}
