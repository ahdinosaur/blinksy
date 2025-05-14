use super::{LinearSrgb, Lms};

/// # Oklab Color Space
///
/// Oklab is a perceptual color space designed for improved uniformity and
/// blending characteristics compared to traditional spaces like sRGB or
/// CIELAB. Its goal is to make mathematical color operations align more
/// closely with how humans perceive color differences.
///
/// It represents colors using three components:
///
/// - `l`: **Perceptual Lightness**. This value typically ranges from 0.0 (black)
///   to 1.0 (white). Changes in `l` are intended to correspond linearly
///   with perceived changes in brightness.
/// - `a`: Represents the green-red axis. Negative values lean towards green,
///   and positive values lean towards red. A value near zero is neutral grey
///   along this axis.
/// - `b`: Represents the blue-yellow axis. Negative values lean towards blue,
///   and positive values lean towards yellow. A value near zero is neutral grey
///   along this axis.
///
/// ## Properties
///
/// - **White Point**: D65 (6500K), same as sRGB
///
/// Oklab, like many standard color spaces, is based on the D65 whitepoint,
/// which represents average daylight.
///
/// ## Why Use Oklab?
///
/// The primary advantage of Oklab is its **perceptual uniformity**. This means
/// that a small change in the Oklab coordinates (i.e., a small Euclidean
/// distance in the 3D Oklab space) corresponds more closely to a small,
/// equally perceived difference in color by a human observer, regardless
/// of the color's initial hue, lightness, or chroma.
///
/// This property makes Oklab excellent for:
///
/// - **Color Gradients and Interpolation:** Blending colors in Oklab
///   often results in smoother, more natural-looking transitions without
///   undesirable "greyish" or "muddy" intermediate colors sometimes seen
///   when blending in sRGB.
/// - **Image Processing:** Operations like desaturation, adjusting
///   lightness, or manipulating contrast can be performed in Oklab with
///   less risk of affecting the perceived hue or introducing artifacts.
///   For example, simply setting `a` and `b` to zero effectively grayscales
///   a color while preserving its *perceived* lightness.
/// - **Color Picking Interfaces:** Providing a more intuitive way for users
///   to select and manipulate colors based on how they are seen.
///
/// Reference: https://bottosson.github.io/posts/oklab/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Oklab {
    /// Lightness component [0.0, 1.0]
    pub l: f32,
    /// Green-red opponent component
    pub a: f32,
    /// Blue-yellow opponent component
    pub b: f32,
}

impl Oklab {
    /// Creates a new Oklab color.
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Oklab { l, a, b }
    }

    /// Converts from linear sRGB to Oklab color space.
    ///
    /// This conversion goes through the LMS color space, which models
    /// the response of the three types of cones in the human eye.
    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        let lms = Lms::from_linear_srgb(linear_srgb);
        Self::from_lms(lms)
    }

    /// Converts from Oklab to linear sRGB color space.
    ///
    /// Note that the result may contain values outside the standard sRGB gamut.
    pub fn to_linear_srgb(self) -> LinearSrgb {
        let lms = self.to_lms();
        lms.to_linear_srgb()
    }

    /// Converts from LMS cone responses to Oklab.
    ///
    /// This applies a non-linear transformation (cube root) to the LMS values
    /// followed by a linear transformation to get the Oklab components.
    pub fn from_lms(lms: Lms) -> Self {
        const LMS_TO_OKLAB: [[f32; 3]; 3] = [
            [0.210_454_26, 0.793_617_8, -0.004_072_047],
            [1.977_998_5, -2.428_592_2, 0.450_593_7],
            [0.025_904_037, 0.782_771_77, -0.808_675_77],
        ];

        let Lms {
            long,
            medium,
            short,
        } = lms;

        let l_cbrt = long.cbrt();
        let m_cbrt = medium.cbrt();
        let s_cbrt = short.cbrt();

        Oklab {
            l: LMS_TO_OKLAB[0][0] * l_cbrt
                + LMS_TO_OKLAB[0][1] * m_cbrt
                + LMS_TO_OKLAB[0][2] * s_cbrt,
            a: LMS_TO_OKLAB[1][0] * l_cbrt
                + LMS_TO_OKLAB[1][1] * m_cbrt
                + LMS_TO_OKLAB[1][2] * s_cbrt,
            b: LMS_TO_OKLAB[2][0] * l_cbrt
                + LMS_TO_OKLAB[2][1] * m_cbrt
                + LMS_TO_OKLAB[2][2] * s_cbrt,
        }
    }

    /// Converts from Oklab to LMS cone responses.
    ///
    /// This applies the inverse transformation from Oklab to LMS,
    /// followed by cubing the result to undo the non-linearity.
    pub fn to_lms(self) -> Lms {
        const OKLAB_TO_LMS_CBRT: [[f32; 3]; 3] = [
            [1.0, 0.396_337_78, 0.215_803_76],
            [1.0, -0.105_561_346, -0.063_854_17],
            [1.0, -0.089_484_18, -1.291_485_5],
        ];

        let Oklab { l, a, b } = self;

        let l_cbrt =
            OKLAB_TO_LMS_CBRT[0][0] * l + OKLAB_TO_LMS_CBRT[0][1] * a + OKLAB_TO_LMS_CBRT[0][2] * b;
        let m_cbrt =
            OKLAB_TO_LMS_CBRT[1][0] * l + OKLAB_TO_LMS_CBRT[1][1] * a + OKLAB_TO_LMS_CBRT[1][2] * b;
        let s_cbrt =
            OKLAB_TO_LMS_CBRT[2][0] * l + OKLAB_TO_LMS_CBRT[2][1] * a + OKLAB_TO_LMS_CBRT[2][2] * b;

        let long = l_cbrt * l_cbrt * l_cbrt;
        let medium = m_cbrt * m_cbrt * m_cbrt;
        let short = s_cbrt * s_cbrt * s_cbrt;

        Lms::new(long, medium, short)
    }
}
