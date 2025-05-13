use crate::color::Lms;

use super::{FromColor, LinearSrgb};

/// Oklab color space representation.
///
/// https://bottosson.github.io/posts/oklab/
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
/// The `a` and `b` components are theoretically unbounded, but practical
/// colors within typical gamuts (like sRGB) will fall within a finite range
/// (e.g., roughly -0.5 to +0.5).
///
/// Oklab, like many standard color spaces, is based on the D65 whitepoint,
/// which represents average daylight.
///
/// # Why Use Oklab?
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
/// Oklab is designed to be numerically stable and well-behaved for computations.
/// It is a device-independent space, related to the CIE standard observer.
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

    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        let lms = Lms::from_linear_srgb(linear_srgb);
        Self::from_lms(lms)
    }

    pub fn to_linear_srgb(self) -> LinearSrgb {
        let lms = self.to_lms();
        lms.to_linear_srgb()
    }

    pub fn from_lms(lms: Lms) -> Self {
        // LMS^(1/3) → OkLab
        const LMS_TO_OKLAB: [[f32; 3]; 3] = [
            [0.2104542553, 0.7936177850, -0.0040720468],
            [1.9779984951, -2.4285922050, 0.4505937099],
            [0.0259040371, 0.7827717662, -0.8086757660],
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

    pub fn to_lms(self) -> Lms {
        // OkLab → LMS^(1/3)
        const OKLAB_TO_LMS_CBRT: [[f32; 3]; 3] = [
            [1.0, 0.3963377774, 0.2158037573],
            [1.0, -0.1055613458, -0.0638541728],
            [1.0, -0.0894841775, -1.2914855480],
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

impl FromColor<LinearSrgb> for Oklab {
    fn from_color(color: LinearSrgb) -> Self {
        Self::from_linear_srgb(color)
    }
}

impl FromColor<Oklab> for LinearSrgb {
    fn from_color(color: Oklab) -> Self {
        color.to_linear_srgb()
    }
}
