use super::{FromColor, LinearSrgb};

/// CIE XYZ color space representation.
///
/// A device-independent color space that models human color perception.
/// The Y component represents luminance, while X and Z represent chromaticity.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Xyz {
    /// X component (mix of cone responses)
    pub x: f32,
    /// Y component (luminance)
    pub y: f32,
    /// Z component (quasi-equal to blue stimulation)
    pub z: f32,
}

impl Xyz {
    /// Creates a new XYZ color.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Xyz { x, y, z }
    }

    /// Converts a linear sRGB color into a XYZ color.
    pub fn from_linear_srgb(linear_srgb: LinearSrgb) -> Self {
        // Based on sRGB Working Space Matrix
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
        pub const LINEAR_SRGB_TO_XYZ_MATRIX: [[f32; 3]; 3] = [
            [0.4124564, 0.3575761, 0.1804375],
            [0.2126729, 0.7151522, 0.0721750],
            [0.0193339, 0.1191920, 0.9503041],
        ];

        let LinearSrgb { red, green, blue } = linear_srgb;

        let x = LINEAR_SRGB_TO_XYZ_MATRIX[0][0] * red
            + LINEAR_SRGB_TO_XYZ_MATRIX[0][1] * green
            + LINEAR_SRGB_TO_XYZ_MATRIX[0][2] * blue;
        let y = LINEAR_SRGB_TO_XYZ_MATRIX[1][0] * red
            + LINEAR_SRGB_TO_XYZ_MATRIX[1][1] * green
            + LINEAR_SRGB_TO_XYZ_MATRIX[1][2] * blue;
        let z = LINEAR_SRGB_TO_XYZ_MATRIX[2][0] * red
            + LINEAR_SRGB_TO_XYZ_MATRIX[2][1] * green
            + LINEAR_SRGB_TO_XYZ_MATRIX[2][2] * blue;

        Xyz { x, y, z }
    }

    /// Converts an XYZ color into a linear sRGB color
    pub fn to_linear_srgb(self) -> LinearSrgb {
        // Based on sRGB Working Space Matrix
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
        pub const XYZ_TO_LINEAR_SRGB_MATRIX: [[f32; 3]; 3] = [
            [3.2404542, -1.5371385, -0.4985314],
            [-0.9692660, 1.8760108, 0.0415560],
            [0.0556434, -0.2040259, 1.0572252],
        ];

        let Xyz { x, y, z } = self;

        let r = XYZ_TO_LINEAR_SRGB_MATRIX[0][0] * x
            + XYZ_TO_LINEAR_SRGB_MATRIX[0][1] * y
            + XYZ_TO_LINEAR_SRGB_MATRIX[0][2] * z;
        let g = XYZ_TO_LINEAR_SRGB_MATRIX[1][0] * x
            + XYZ_TO_LINEAR_SRGB_MATRIX[1][1] * y
            + XYZ_TO_LINEAR_SRGB_MATRIX[1][2] * z;
        let b = XYZ_TO_LINEAR_SRGB_MATRIX[2][0] * x
            + XYZ_TO_LINEAR_SRGB_MATRIX[2][1] * y
            + XYZ_TO_LINEAR_SRGB_MATRIX[2][2] * z;

        LinearSrgb::new(r, g, b)
    }
}

impl FromColor<Xyz> for LinearSrgb {
    fn from_color(color: Xyz) -> Self {
        color.to_linear_srgb()
    }
}

impl FromColor<LinearSrgb> for Xyz {
    fn from_color(color: LinearSrgb) -> Self {
        Xyz::from_linear_srgb(color)
    }
}
