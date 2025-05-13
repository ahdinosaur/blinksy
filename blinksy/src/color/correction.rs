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
///
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
    /// * `red` - Scaling factor for the red channel
    /// * `green` - Scaling factor for the green channel
    /// * `blue` - Scaling factor for the blue channel
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
    /// * `temperature` - Approximate color temperature in Kelvin (e.g., 2700K, 6500K)
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
    pub fn from_temperature(temperature: u32) -> Self {
        // Simple approximation of color temperature correction
        // This is a very basic model and could be improved
        let temp = temperature.clamp(1000, 40000) as f32;
        let temp = (temp - 1000.0) / 39000.0;

        let r = if temp < 6600.0 {
            1.0
        } else {
            1.0 - (temp * 0.3)
        };
        let b = if temp > 6600.0 {
            1.0
        } else {
            0.7 + (temp * 0.3)
        };
        let g = if temp < 6600.0 {
            0.8 + (temp * 0.2)
        } else {
            1.0 - (temp * 0.2)
        };

        ColorCorrection::new(r, g, b)
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
