pub struct Rgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl Rgb {
    pub fn into_linear<T: Component>(self, gamma: f32) -> LinearRgb<T> {
        LinearRgb {
            red: rgb_to_linear_component(self.red, gamma),
            green: rgb_to_linear_component(self.green, gamma),
            blue: rgb_to_linear_component(self.blue, gamma),
        }
    }
}

pub struct LinearRgb<T: Component = u8> {
    red: T,
    green: T,
    blue: T,
}

pub trait Component {
    fn to_normalized_f32(self) -> f32;
}

macro_rules! impl_component_for_uint {
    ($T:ident) => {
        impl Component for $T {
            fn to_normalized_f32(self) -> f32 {
                self as f32 / ($T::MAX as f32)
            }
        }
    };
}

impl_component_for_uint!(u8);
impl_component_for_uint!(u16);
impl_component_for_uint!(u32);

impl Component for f32 {
    fn to_normalized_f32(self) -> f32 {
        self.clamp(0., 1.)
    }
}

impl<T: Component> LinearRgb<T> {
    pub fn into_rgb(self, gamma: f32) -> Rgb {
        LinearRgb {
            red: linear_to_rgb_component(self.red, gamma),
            green: linear_to_rgb_component(self.green, gamma),
            blue: linear_to_rgb_component(self.blue, gamma),
        }
    }
}

/// Convert RGB component to linear RGB component (inverse RGB companding)
/// Reference: http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
/// Source: https://github.com/kimtore/colorspace/
#[inline]
pub(crate) fn rgb_to_linear_component(c: f32, gamma: f32) -> f32 {
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
pub(crate) fn linear_to_rgb_component(c: f32, gamma: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / gamma) - 0.055
    }
}

mod test {}
