use super::{ColorComponent, OutputColor};

/// Rgb is [sRGB](https://en.wikipedia.org/wiki/SRGB). What you expect when you think "just RGB".
///
/// Has gamma encoding, RGB primaries, whitepoint, etc.
pub struct Rgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl Rgb {
    pub fn into_linear(self, gamma: f32) -> LinearRgb {
        LinearRgb {
            red: rgb_to_linear_component(self.red, gamma),
            green: rgb_to_linear_component(self.green, gamma),
            blue: rgb_to_linear_component(self.blue, gamma),
        }
    }
}

/// LinearRgb is linear, not gamma encoded.
///
/// What is linear?
///
/// Is expected to have the same color standards as sRGB: RGB primaries, whitepoint, etc.
pub struct LinearRgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinearRgb {
    pub fn into_rgb(self, gamma: f32) -> Rgb {
        Rgb {
            red: linear_to_rgb_component(self.red, gamma),
            green: linear_to_rgb_component(self.green, gamma),
            blue: linear_to_rgb_component(self.blue, gamma),
        }
    }
}

pub struct LinearRgbw {
    red: f32,
    green: f32,
    blue: f32,
    white: f32,
}

impl OutputColor for LinearRgb {
    fn to_rgb<C: ColorComponent>(
        &self,
        brightness: f32,
        gamma: f32,
        correction: super::ColorCorrection,
    ) -> super::OutputRgb<C> {
        todo!()
    }

    fn to_rgbw<C: ColorComponent>(
        &self,
        brightness: f32,
        gamma: f32,
        correction: super::ColorCorrection,
    ) -> super::OutputRgb<C> {
        todo!()
    }
}

/// Convert RGB component to linear RGB component (inverse RGB companding)
/// Reference: http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
fn rgb_to_linear_component(c: f32, gamma: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(gamma)
    }
}

/// Convert linear RGB component to RGB component
/// Reference: http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
fn linear_to_rgb_component(c: f32, gamma: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / gamma) - 0.055
    }
}
