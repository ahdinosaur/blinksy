use super::LinearSrgb;

/// # CIE XYZ color space
///
/// The CIE XYZ color space is a device-independent color space that models human color
/// perception. It serves as a standard reference space for other color spaces.
///
/// ## Color Space Assumptions
///
/// - **White Point**: D65 (6500K)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Xyz {
    /// X component (mix of cone responses, roughly red)
    pub x: f32,
    /// Y component (luminance, matches human brightness perception)
    pub y: f32,
    /// Z component (quasi-equal to blue stimulation)
    pub z: f32,
}

impl Xyz {
    /// Creates a new XYZ color.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Xyz { x, y, z }
    }

    /// Converts a linear sRGB color into an XYZ color.
    ///
    /// Uses the standard RGB to XYZ transformation matrix defined in the sRGB specification.
    /// This assumes the D65 white point used in the sRGB standard.
    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        const LINEAR_SRGB_TO_XYZ: [[f32; 3]; 3] = [
            [0.4124564, 0.3575761, 0.1804375],
            [0.2126729, 0.7151522, 0.0721750],
            [0.0193339, 0.1191920, 0.9503041],
        ];

        let LinearSrgb { red, green, blue } = linear_srgb;

        let x = LINEAR_SRGB_TO_XYZ[0][0] * red
            + LINEAR_SRGB_TO_XYZ[0][1] * green
            + LINEAR_SRGB_TO_XYZ[0][2] * blue;
        let y = LINEAR_SRGB_TO_XYZ[1][0] * red
            + LINEAR_SRGB_TO_XYZ[1][1] * green
            + LINEAR_SRGB_TO_XYZ[1][2] * blue;
        let z = LINEAR_SRGB_TO_XYZ[2][0] * red
            + LINEAR_SRGB_TO_XYZ[2][1] * green
            + LINEAR_SRGB_TO_XYZ[2][2] * blue;

        Xyz { x, y, z }
    }

    /// Converts an XYZ color into a linear sRGB color.
    ///
    /// Uses the standard XYZ to RGB transformation matrix defined in the sRGB specification.
    /// This assumes the D65 white point used in the sRGB standard.
    /// Note that the resulting RGB values may be outside the displayable sRGB gamut.
    pub fn to_linear_srgb(self) -> LinearSrgb {
        const XYZ_TO_LINEAR_SRGB: [[f32; 3]; 3] = [
            [3.2404542, -1.5371385, -0.4985314],
            [-0.9692660, 1.8760108, 0.0415560],
            [0.0556434, -0.2040259, 1.0572252],
        ];

        let Xyz { x, y, z } = self;

        let r = XYZ_TO_LINEAR_SRGB[0][0] * x
            + XYZ_TO_LINEAR_SRGB[0][1] * y
            + XYZ_TO_LINEAR_SRGB[0][2] * z;
        let g = XYZ_TO_LINEAR_SRGB[1][0] * x
            + XYZ_TO_LINEAR_SRGB[1][1] * y
            + XYZ_TO_LINEAR_SRGB[1][2] * z;
        let b = XYZ_TO_LINEAR_SRGB[2][0] * x
            + XYZ_TO_LINEAR_SRGB[2][1] * y
            + XYZ_TO_LINEAR_SRGB[2][2] * z;

        LinearSrgb::new(r, g, b)
    }
}
