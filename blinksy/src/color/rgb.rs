pub struct Rgb<T> {
    red: f32,
    green: f32,
    blue: f32,
}

impl Rgb {
    pub fn into_linear<T>(self) -> LinRgb<T> {
        todo!()
    }
}

pub struct LinRgb<T = u8> {
    red: T,
    green: T,
    blue: T,
}

/// Convert sRGB to linear RGB (inverse sRGB companding)
/// Verified here: http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
fn srgb_to_linear(c: f32, gamma: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(gamma)
    }
}

/// Convert linear RGB to gamma RGB
/// Verified here: http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / gamma) - 0.055
    }
}

const GAMMA: f32 = 2.4;

mod test {}
