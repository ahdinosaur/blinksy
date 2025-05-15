use super::LinearSrgb;

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
            [0.412_221_46, 0.536_332_55, 0.051_445_995],
            [0.211_903_5, 0.680_699_5, 0.107_396_96],
            [0.088_302_46, 0.281_718_85, 0.629_978_7],
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
            [4.076_741_7, -3.307_711_6, 0.230_969_94],
            [-1.268_438, 2.609_757_4, -0.341_319_38],
            [-0.0041960863, -0.703_418_6, 1.707_614_7],
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
