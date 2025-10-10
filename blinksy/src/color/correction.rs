#[allow(unused_imports)]
use num_traits::Float;

/// Defines color correction factors for LED hardware.
///
/// `ColorCorrection` contains scaling factors for each RGB component to
/// compensate for differences in LED brightness and color balance.
/// This allows for more accurate color reproduction on specific LED hardware.
///
/// # When to Use
///
/// Use `ColorCorrection` when:
/// - Working with LED strips or arrays with unbalanced color output
/// - You need a white point correction for your specific LEDs
/// - Calibrating a display system for accurate color reproduction
/// - Compensating for RGB LED intensity differences
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorCorrection {
    /// Scaling factor for red channel
    pub red: f32,
    /// Scaling factor for green channel
    pub green: f32,
    /// Scaling factor for blue channel
    pub blue: f32,
}

impl ColorCorrection {
    /// Creates a new color correction with the specified scaling factors.
    ///
    /// # Arguments
    ///
    /// - `red` - Scaling factor for the red channel
    /// - `green` - Scaling factor for the green channel
    /// - `blue` - Scaling factor for the blue channel
    ///
    /// # Returns
    ///
    /// A new `ColorCorrection` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use blinksy::color::ColorCorrection;
    ///
    /// // Create a correction for LEDs with strong red, weak blue
    /// let correction = ColorCorrection::new(0.8, 1.0, 1.4);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        ColorCorrection {
            red: red.max(0.0),
            green: green.max(0.0),
            blue: blue.max(0.0),
        }
    }

    /// Creates a color correction from color temperature adjustment.
    ///
    /// This creates a color correction that simulates adjusting the white
    /// point to a specific color temperature.
    ///
    /// # Arguments
    ///
    /// - `temperature` - Approximate color temperature in Kelvin (e.g., 2700K, 6500K)
    ///
    /// # Returns
    ///
    /// A `ColorCorrection` instance that approximates the desired color temperature.
    ///
    /// # Example
    ///
    /// ```
    /// use blinksy::color::ColorCorrection;
    ///
    /// // Create a warm white (incandescent-like) correction
    /// let warm = ColorCorrection::from_temperature(2700);
    ///
    /// // Create a cool white (daylight-like) correction
    /// let cool = ColorCorrection::from_temperature(6500);
    /// ```
    pub fn from_temperature(kelvin: u32) -> Self {
        // Clamp to a reasonable range for the approximation.
        let k = kelvin.clamp(1000, 40000) as f32;

        // Use Tanner Helland's approximation (T in hundreds of Kelvin):
        // https://tannerhelland.com/2012/09/18/convert-temperature-rgb-algorithm-code.html
        let t = k / 100.0;

        let r = if t <= 66.0 {
            255.0
        } else {
            (329.698_73 * (t - 60.0).powf(-0.133_204_76)).clamp(0.0, 255.0)
        };

        let g = if t <= 66.0 {
            (99.470_8 * t.ln() - 161.119_57).clamp(0.0, 255.0)
        } else {
            (288.122_16 * (t - 60.0).powf(-0.075_514_846)).clamp(0.0, 255.0)
        };

        let b = if t >= 66.0 {
            255.0
        } else if t <= 19.0 {
            0.0
        } else {
            (138.517_73 * (t - 10.0).ln() - 305.044_8).clamp(0.0, 255.0)
        };

        ColorCorrection::new(r / 255.0, g / 255.0, b / 255.0)
    }
}

impl Default for ColorCorrection {
    fn default() -> Self {
        Self {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    }
}
