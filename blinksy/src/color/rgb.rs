pub struct Rgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl Rgb {
    pub fn into_linear<T>(self) -> LinRgb<T> {
        todo!()
    }
}

pub struct LinearRgb<T = u8> {
    red: T,
    green: T,
    blue: T,
}

/// Convert RGB component to linear RGB component (inverse RGB companding)
/// Reference: http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
pub fn rgb_component_to_linear_rgb_component(c: f32, gamma: f32) -> f32 {
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
pub fn linear_rgb_component_to_rgb_component(c: f32, gamma: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / gamma) - 0.055
    }
}

mod test {}
