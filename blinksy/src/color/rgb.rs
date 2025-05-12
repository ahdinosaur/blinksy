use super::{ColorComponent, ColorCorrection, LedRgb, LedRgbw, OutputColor};

/// # sRGB Color Space
///
/// `Srgb` represents colors in the standard RGB (sRGB) color space, which is the most common
/// color space used for digital displays and the web.
///
/// ## Color Space Properties
///
/// - **Gamma Encoding**: Uses a non-linear transfer function (approximately gamma 2.2)
/// - **RGB Primaries**: Uses the sRGB primaries as defined in IEC 61966-2-1
/// - **White Point**: D65 (6500K)
///
/// ## When to Use
///
/// Use `Srgb` when:
/// - Working with color values from typical image formats, web colors, or GUI applications
/// - You need a perceptually uniform color space
/// - You want to match colors as they appear on standard displays
///
/// sRGB values are non-linear (gamma-encoded) to account for human perception. This means
/// that arithmetic operations on sRGB values (like averaging or interpolation) will not
/// produce perceptually correct results. For such operations, convert to `LinearRgb` first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
}

impl Srgb {
    /// Creates a new sRGB color
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
    /// let red = Srgb::new(1.0, 0.0, 0.0);
    /// let green = Srgb::new(0.0, 1.0, 0.0);
    /// let blue = Srgb::new(0.0, 0.0, 1.0);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Srgb {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    /// Converts from sRGB to linear RGB color space
    ///
    /// This removes the gamma encoding, making the color values proportional to light intensity.
    /// Linear RGB is necessary for physically accurate color calculations.
    pub fn to_linear_rgb(self) -> LinearRgb {
        LinearRgb {
            red: srgb_decode(self.red),
            green: srgb_decode(self.green),
            blue: srgb_decode(self.blue),
        }
    }
}

impl OutputColor for Srgb {
    fn to_linear_rgb(self) -> LinearRgb {
        self.to_linear_rgb()
    }

    fn to_linear_rgbw(self) -> LinearRgbw {
        self.to_linear_rgb().to_linear_rgbw()
    }
}

/// # Linear RGB Color Space
///
/// `LinearRgb` represents colors in a linear RGB color space, where values are directly
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
/// Use `LinearRgb` when:
/// - Performing color calculations that should be physically accurate
/// - Blending, mixing, or interpolating between colors
/// - Working with lighting simulations or physically-based rendering
/// - Processing before final display output
///
/// Mathematical operations on linear RGB values (like averaging or interpolation) will
/// produce physically correct results, unlike operations on gamma-encoded sRGB values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgb {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
}

impl LinearRgb {
    /// Creates a new LinearRgb color
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
    /// use blinksy::color::LinearRgb;
    ///
    /// let red = LinearRgb::new(1.0, 0.0, 0.0);
    /// ```
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        LinearRgb {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    /// Converts from linear RGB to sRGB color space
    ///
    /// This applies gamma encoding to make the color values perceptually uniform.
    pub fn to_srgb(self) -> Srgb {
        Srgb {
            red: srgb_encode(self.red),
            green: srgb_encode(self.green),
            blue: srgb_encode(self.blue),
        }
    }

    /// Converts to RGBW by extracting a white component
    ///
    /// This extracts the common part of R, G, and B as the white component,
    /// which can be more efficient for RGBW LEDs.
    pub fn to_linear_rgbw(self) -> LinearRgbw {
        // Extract white as the minimum of R, G, B
        let white = self.red.min(self.green).min(self.blue);

        // Subtract white from RGB components
        LinearRgbw {
            red: self.red - white,
            green: self.green - white,
            blue: self.blue - white,
            white,
        }
    }

    /// Converts to output RGB format with applied brightness, gamma, and color correction
    ///
    /// # Arguments
    ///
    /// * `brightness` - Overall brightness scaling factor (0.0 to 1.0)
    /// * `gamma` - Output gamma correction factor (typically 1.0 to 3.0)
    /// * `correction` - Color correction factors for LED hardware
    ///
    /// # Returns
    ///
    /// An `LedRgb<C>` with the specified color component type
    pub fn to_led_rgb<C: ColorComponent>(
        self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> LedRgb<C> {
        // Apply color correction first
        let red = self.red * correction.red;
        let green = self.green * correction.green;
        let blue = self.blue * correction.blue;

        // Apply brightness scaling
        let red = (red * brightness).clamp(0., 1.);
        let green = (green * brightness).clamp(0., 1.);
        let blue = (blue * brightness).clamp(0., 1.);

        // Apply gamma
        let red = gamma_encode(red, gamma);
        let green = gamma_encode(green, gamma);
        let blue = gamma_encode(blue, gamma);

        // Convert to component type and return
        LedRgb::new(
            C::from_normalized_f32(red),
            C::from_normalized_f32(green),
            C::from_normalized_f32(blue),
        )
    }
}

impl OutputColor for LinearRgb {
    fn to_linear_rgb(self) -> LinearRgb {
        self
    }

    fn to_linear_rgbw(self) -> LinearRgbw {
        self.to_linear_rgbw()
    }
}

/// # Linear RGBW Color Space
///
/// `LinearRgbw` represents colors in a linear RGB color space with an additional
/// white channel. This is particularly useful for RGBW LED strips.
///
/// The white channel represents a dedicated white LED that can be used to enhance
/// brightness and efficiency for neutral/white colors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgbw {
    /// Red component (0.0 to 1.0)
    pub red: f32,
    /// Green component (0.0 to 1.0)
    pub green: f32,
    /// Blue component (0.0 to 1.0)
    pub blue: f32,
    /// White component (0.0 to 1.0)
    pub white: f32,
}

impl LinearRgbw {
    /// Creates a new LinearRgbw color
    ///
    /// # Arguments
    ///
    /// * `red` - Red component (0.0 to 1.0)
    /// * `green` - Green component (0.0 to 1.0)
    /// * `blue` - Blue component (0.0 to 1.0)
    /// * `white` - White component (0.0 to 1.0)
    pub fn new(red: f32, green: f32, blue: f32, white: f32) -> Self {
        LinearRgbw {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
            white: white.clamp(0.0, 1.0),
        }
    }

    /// Converts to output RGBW format with applied brightness, gamma, and color correction
    ///
    /// # Arguments
    ///
    /// * `brightness` - Overall brightness scaling factor (0.0 to 1.0)
    /// * `gamma` - Output gamma correction factor (typically 1.0 to 3.0)
    /// * `correction` - Color correction factors for LED hardware
    ///
    /// # Returns
    ///
    /// An `OutputRgbw<C>` with the specified color component type
    pub fn to_output_rgbw<C: ColorComponent>(
        self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> LedRgbw<C> {
        // Apply color correction first
        let red = self.red * correction.red;
        let green = self.green * correction.green;
        let blue = self.blue * correction.blue;
        let white = self.white;

        // Apply brightness scaling
        let red = (red * brightness).clamp(0., 1.);
        let green = (green * brightness).clamp(0., 1.);
        let blue = (blue * brightness).clamp(0., 1.);
        let white = (white * brightness).clamp(0., 1.);

        // Apply gamma
        let red = gamma_encode(red, gamma);
        let green = gamma_encode(green, gamma);
        let blue = gamma_encode(blue, gamma);
        let white = gamma_encode(white, gamma);

        // Convert to component type and return
        LedRgbw::new(
            C::from_normalized_f32(red),
            C::from_normalized_f32(green),
            C::from_normalized_f32(blue),
            C::from_normalized_f32(white),
        )
    }
}

/// Convert sRGB gamma-encoded component to linear RGB component
///
/// The sRGB standard uses a piece-wise function that's approximately
/// equivalent to a gamma of 2.2, but with a linear segment near zero.
///
/// For gamma-encoded value C_srgb:
/// - If C_srgb ≤ 0.04045: C_linear = C_srgb / 12.92
/// - If C_srgb > 0.04045: C_linear = ((C_srgb + 0.055) / 1.055)^2.4
///
/// References:
/// - http://color.org/sRGB.pdf
/// - http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
#[inline]
pub fn srgb_decode(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear RGB component to sRGB gamma-encoded component
///
/// The sRGB standard uses a piece-wise function that's approximately
/// equivalent to a gamma of 2.2, but with a linear segment near zero.
///
/// For linear value C_linear:
/// - If C_linear ≤ 0.0031308: C_srgb = 12.92 * C_linear
/// - If C_linear > 0.0031308: C_srgb = 1.055 * C_linear^(1/2.4) - 0.055
///
/// References:
/// - http://color.org/sRGB.pdf
/// - http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
#[inline]
pub fn srgb_encode(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Convert gamma-encoded value to linear value using standard power law
///
/// For gamma-encoded value c_gamma:
/// - c_linear = c_gamma^gamma
///
/// The gamma value is typically in the range 1.8-2.2, where 2.2 is common.
#[inline]
pub fn gamma_decode(c: f32, gamma: f32) -> f32 {
    c.powf(gamma)
}

/// Convert linear value to gamma-encoded value using standard power law
///
/// For linear value c_linear:
/// - c_gamma = c_linear^(1/gamma)
///
/// The gamma value is typically in the range 1.8-2.2, where 2.2 is common.
#[inline]
pub fn gamma_encode(c: f32, gamma: f32) -> f32 {
    c.powf(1.0 / gamma)
}
