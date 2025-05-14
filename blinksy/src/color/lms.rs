use super::FromColor;
use crate::color::LinearSrgb;

/// # LMS Color Space
///
/// The LMS color space is based on the response of the three types of cones
/// in the human eye:
///
/// - L (Long) cones: Most sensitive to long wavelengths (reddish)
/// - M (Medium) cones: Most sensitive to medium wavelengths (greenish)
/// - S (Short) cones: Most sensitive to short wavelengths (bluish)
///
/// ## Properties
///
/// - **Device-independent**: Based on human perception
/// - **White Point**: D65 (6500K), same as sRGB
/// - **Use Cases**: Color adaptation, vision deficiency simulation
///
/// LMS is primarily used as an intermediate space for color processing algorithms,
/// particularly those that simulate or account for human color vision characteristics.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lms {
    /// Long cone response component
    pub long: f32,
    /// Medium cone response component
    pub medium: f32,
    /// Short cone response component
    pub short: f32,
}

impl Lms {
    /// Creates a new LMS color.
    pub fn new(long: f32, medium: f32, short: f32) -> Self {
        Self {
            long,
            medium,
            short,
        }
    }

    /// Converts from linear sRGB to LMS color space.
    ///
    /// Uses the CAT02 (CIECAM02) transformation matrix which is designed
    /// to accurately model the cone responses of the human eye.
    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        const LINEAR_SRGB_TO_LMS: [[f32; 3]; 3] = [
            [0.4122214708, 0.5363325363, 0.0514459929],
            [0.2119034982, 0.6806995451, 0.1073969566],
            [0.0883024619, 0.2817188376, 0.6299787005],
        ];

        let LinearSrgb { red, green, blue } = linear_srgb;

        let long = LINEAR_SRGB_TO_LMS[0][0] * red
            + LINEAR_SRGB_TO_LMS[0][1] * green
            + LINEAR_SRGB_TO_LMS[0][2] * blue;
        let medium = LINEAR_SRGB_TO_LMS[1][0] * red
            + LINEAR_SRGB_TO_LMS[1][1] * green
            + LINEAR_SRGB_TO_LMS[1][2] * blue;
        let short = LINEAR_SRGB_TO_LMS[2][0] * red
            + LINEAR_SRGB_TO_LMS[2][1] * green
            + LINEAR_SRGB_TO_LMS[2][2] * blue;

        Self::new(long, medium, short)
    }

    /// Converts from LMS to linear sRGB color space.
    ///
    /// Applies the inverse of the CAT02 transformation matrix.
    pub fn to_linear_srgb(self) -> LinearSrgb {
        const LMS_TO_LINEAR_SRGB: [[f32; 3]; 3] = [
            [4.0767416621, -3.3077115913, 0.2309699292],
            [-1.2684380046, 2.6097574011, -0.3413193965],
            [-0.0041960863, -0.7034186147, 1.7076147010],
        ];

        let Self {
            long,
            medium,
            short,
        } = self;

        let red = LMS_TO_LINEAR_SRGB[0][0] * long
            + LMS_TO_LINEAR_SRGB[0][1] * medium
            + LMS_TO_LINEAR_SRGB[0][2] * short;
        let green = LMS_TO_LINEAR_SRGB[1][0] * long
            + LMS_TO_LINEAR_SRGB[1][1] * medium
            + LMS_TO_LINEAR_SRGB[1][2] * short;
        let blue = LMS_TO_LINEAR_SRGB[2][0] * long
            + LMS_TO_LINEAR_SRGB[2][1] * medium
            + LMS_TO_LINEAR_SRGB[2][2] * short;

        LinearSrgb::new(red, green, blue)
    }
}

impl FromColor<LinearSrgb> for Lms {
    fn from_color(color: LinearSrgb) -> Self {
        Self::from_linear_srgb(color)
    }
}

impl FromColor<Lms> for LinearSrgb {
    fn from_color(color: Lms) -> Self {
        color.to_linear_srgb()
    }
}
