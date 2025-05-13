use super::{ColorCorrection, FromColor, GammaSrgb, Srgb};

/// # Linear RGB Color Space
///
/// `LinearSrgb` represents colors in a linear RGB color space, where values are directly
/// proportional to light intensity (not gamma-encoded).
///
/// ## Color Space Properties
///
/// - **No Gamma Encoding**: Values are linearly proportional to light intensity
/// - **RGB Primaries**: Same as sRGB (IEC 61966-2-1)
/// - **White Point**: D65 (6500K)
///
/// ## When to Use
///
/// Use `LinearSrgb` when:
/// - Performing color calculations that should be physically accurate
/// - Blending, mixing, or interpolating between colors
/// - Working with lighting simulations or physically-based rendering
/// - Processing before final display output
///
/// Mathematical operations on linear RGB values (like averaging or interpolation) will
/// produce physically correct results, unlike operations on gamma-encoded sRGB values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearSrgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
}

impl LinearSrgb {
    /// Creates a new LinearSrgb color
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
    /// use blinksy::color::LinearSrgb;
    ///
    /// let red = LinearSrgb::new(1.0, 0.0, 0.0);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        LinearSrgb {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    pub fn apply_brightness(&mut self, brightness: f32) {
        self.red = self.red * brightness;
        self.green = self.green * brightness;
        self.blue = self.blue * brightness;
    }

    pub fn apply_color_correction(&mut self, correction: ColorCorrection) {
        self.red = self.red * correction.red;
        self.green = self.green * correction.green;
        self.blue = self.blue * correction.blue;
    }

    pub fn clamp(&mut self) {
        self.red = self.red.clamp(0., 1.);
        self.green = self.green.clamp(0., 1.);
        self.blue = self.blue.clamp(0., 1.);
    }

    /// Converts from linear RGB to sRGB color space
    ///
    /// This applies gamma encoding to make the color values perceptually uniform.
    pub fn to_srgb(self) -> Srgb {
        Srgb::from_linear_srgb(self)
    }

    pub fn to_gamma_srgb(self, gamma: f32) -> GammaSrgb {
        GammaSrgb::from_linear_srgb(self, gamma)
    }

    /// Converts to RGBW by extracting a white component
    ///
    /// This extracts the common part of R, G, and B as the white component,
    /// which can be more efficient for RGBW LEDs.
    pub fn to_linear_srgbw(self) -> LinearSrgbw {}
}

/// # Linear RGBW Color Space
///
/// `LinearSrgbw` represents colors in a linear RGB color space with an additional
/// white channel. This is particularly useful for RGBW LED strips.
///
/// The white channel represents a dedicated white LED that can be used to enhance
/// brightness and efficiency for neutral/white colors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearSrgbw {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
    /// White component (0.0 to 1.0)
    pub white: f32,
}

impl LinearSrgbw {
    /// Creates a new LinearSrgbw color
    ///
    /// # Arguments
    ///
    /// * `red` - Red component (0.0 to 1.0)
    /// * `green` - Green component (0.0 to 1.0)
    /// * `blue` - Blue component (0.0 to 1.0)
    /// * `white` - White component (0.0 to 1.0)
    pub fn new(red: f32, green: f32, blue: f32, white: f32) -> Self {
        LinearSrgbw {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
            white: white.clamp(0.0, 1.0),
        }
    }

    pub fn apply_brightness(&mut self, brightness: f32) {
        self.red = self.red * brightness;
        self.green = self.green * brightness;
        self.blue = self.blue * brightness;
    }

    pub fn apply_color_correction(&mut self, correction: ColorCorrection) {
        self.red = self.red * correction.red;
        self.green = self.green * correction.green;
        self.blue = self.blue * correction.blue;
    }

    pub fn clamp(&mut self) {
        self.red = self.red.clamp(0., 1.);
        self.green = self.green.clamp(0., 1.);
        self.blue = self.blue.clamp(0., 1.);
    }

    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        let LinearSrgb { red, green, blue } = linear_srgb;

        // Extract white as the minimum of R, G, B
        let white = red.min(green).min(blue);

        // Subtract white from RGB components
        LinearSrgbw {
            red: red - white,
            green: green - white,
            blue: blue - white,
            white,
        }
    }
}

impl FromColor<LinearSrgb> for LinearSrgbw {
    fn from_color(color: LinearSrgb) -> Self {
        color.to_linear_srgbw()
    }
}
