use super::{Hue, HueMap, HueRainbow, LinearRgb, LinearRgbw, OutputColor};

/// HSI color model (Hue, Saturation, Intensity)
///
/// HSI is a color model that separates color into:
/// - Hue: The color type (red, green, blue, etc.)
/// - Saturation: The purity of the color (0.0 = grayscale, 1.0 = pure color)
/// - Intensity: The brightness of the color (0.0 = black, 1.0 = maximum brightness)
///
/// This implementation allows different hue mapping algorithms to be used through
/// the type parameter M.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsi<M: HueMap = HueRainbow> {
    /// Hue component
    pub hue: Hue<M>,
    /// Saturation component (0.0 to 1.0)
    pub saturation: f32,
    /// Intensity component (0.0 to 1.0)
    pub intensity: f32,
}

impl<M: HueMap> Hsi<M> {
    /// Creates a new HSI color
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue value (0.0 to 1.0)
    /// * `saturation` - Saturation value (0.0 to 1.0)
    /// * `intensity` - Intensity value (0.0 to 1.0)
    pub fn new(hue: f32, saturation: f32, intensity: f32) -> Self {
        Self {
            hue: Hue::new(hue),
            saturation: saturation.clamp(0.0, 1.0),
            intensity: intensity.clamp(0.0, 1.0),
        }
    }

    /// Creates a new HSI color from an existing Hue object
    ///
    /// # Arguments
    ///
    /// * `hue` - Existing Hue object
    /// * `saturation` - Saturation value (0.0 to 1.0)
    /// * `intensity` - Intensity value (0.0 to 1.0)
    pub fn from_hue(hue: Hue<M>, saturation: f32, intensity: f32) -> Self {
        Self {
            hue,
            saturation: saturation.clamp(0.0, 1.0),
            intensity: intensity.clamp(0.0, 1.0),
        }
    }
}

impl<M: HueMap> OutputColor for Hsi<M> {
    fn to_linear_rgb(self) -> LinearRgb {
        // Special case for zero saturation (grayscale)
        if self.saturation <= 0.0 {
            let v = self.intensity;
            return LinearRgb::new(v, v, v);
        }

        // Special case for zero intensity (black)
        if self.intensity <= 0.0 {
            return LinearRgb::new(0.0, 0.0, 0.0);
        }

        // Get the pure hue color
        let rgb = self.hue.to_rgb();

        // If fully saturated, just scale by intensity
        if self.saturation >= 1.0 {
            return LinearRgb::new(
                rgb.red * self.intensity,
                rgb.green * self.intensity,
                rgb.blue * self.intensity,
            );
        }

        // For partial saturation, blend with gray
        let gray = self.intensity;
        let s = self.saturation;

        LinearRgb::new(
            rgb.red * s * self.intensity + gray * (1.0 - s),
            rgb.green * s * self.intensity + gray * (1.0 - s),
            rgb.blue * s * self.intensity + gray * (1.0 - s),
        )
    }

    fn to_linear_rgbw(self) -> LinearRgbw {
        self.to_linear_rgb().to_linear_rgbw()
    }
}

/// Create a balanced HSI color spectrum for animation
///
/// This function provides a convenient way to create a hue-rotating HSI color
/// with full saturation and intensity.
///
/// # Arguments
///
/// * `phase` - A phase value from 0.0 to 1.0 that will be mapped to the full color spectrum
///
/// # Returns
///
/// An HSI color with the specified hue mapping type M
pub fn rainbow_hue<M: HueMap>(phase: f32) -> Hsi<M> {
    Hsi::new(phase % 1.0, 1.0, 1.0)
}
