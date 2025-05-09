use super::{ColorComponent, ColorCorrection, OutputColor, OutputRgb, OutputRgbw};

/// Rgb is [sRGB](https://en.wikipedia.org/wiki/SRGB). What you expect when you think "just RGB".
///
/// Has gamma encoding, RGB primaries, whitepoint, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl Srgb {
    const GAMMA: f32 = 2.2;

    pub fn to_linear_rgb(self) -> LinearRgb {
        LinearRgb {
            red: gamma_decode(self.red, Self::GAMMA),
            green: gamma_decode(self.green, Self::GAMMA),
            blue: gamma_decode(self.blue, Self::GAMMA),
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

/// LinearRgb is linear, not gamma encoded.
///
/// What is linear?
///
/// Is expected to have the same color standards as sRGB: RGB primaries, whitepoint, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinearRgb {
    pub fn to_srgb(self) -> Rgb {
        Srgb {
            red: gamma_encode(self.red, Srgb::GAMMA),
            green: gamma_encode(self.green, Srgb::GAMMA),
            blue: gamma_encode(self.blue, Srgb::GAMMA),
        }
    }

    pub fn to_output_rgb<C: ColorComponent>(
        self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> OutputRgb<C> {
        todo!()
    }
}

impl OutputColor for LinearRgb {
    fn to_linear_rgb(self) -> LinearRgb {
        self
    }

    fn to_output_rgbw(self) -> LinearRgbw {
        /*
            W = min(R, G, B);
            R = R - W;
            G = G - W;
            B = B - W;
        */
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgbw {
    red: f32,
    green: f32,
    blue: f32,
    white: f32,
}

impl LinearRgbw {
    pub fn to_output_rgbw<C: ColorComponent>(
        self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> OutputRgbw<C> {
        todo!()
    }
}

/// Convert gamma-encoded RGB component to linear RGB component
/// Also know as "compression".
/// Reference: http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
pub(crate) fn gamma_decode(c: f32, gamma: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(gamma)
    }
}

/// Convert linear RGB component to gamma-encoded RGB component
/// Also known as "expansion".
/// Reference: http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
pub(crate) fn gamma_encode(c: f32, gamma: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / gamma) - 0.055
    }
}
