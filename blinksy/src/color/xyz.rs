use super::LinearSrgb;

/// # CIE XYZ Color Space
///
/// The CIE XYZ color space is a device-independent color space that models human color
/// perception. It serves as a standard reference space for other color spaces and
/// is often used as an intermediate step in color conversions.
///
/// ## Color Space Properties
///
/// - **White Point**: D65 (6500K)
/// - **Device-Independent**: Based on human perception
/// - **Linear**: Values are proportional to light intensity
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Xyz {
    /// X component (mix of cone responses, roughly corresponds to red)
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
            [0.412_456_4, 0.357_576_1, 0.180_437_5],
            [0.212_672_9, 0.715_152_2, 0.072_175_0],
            [0.019_333_9, 0.119_192, 0.950_304_1],
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
    ///
    /// Note that the resulting RGB values may be outside the displayable sRGB gamut.
    pub fn to_linear_srgb(self) -> LinearSrgb {
        const XYZ_TO_LINEAR_SRGB: [[f32; 3]; 3] = [
            [3.240_454_2, -1.537_138_5, -0.498_531_4],
            [-0.969_266, 1.876_010_8, 0.041_556_0],
            [0.055_643_4, -0.204_025_9, 1.057_225_2],
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
