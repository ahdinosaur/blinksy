use core::marker::PhantomData;

use super::{LinearRgb, LinearRgbw, OutputColor};

pub struct Hue<M: HueMap = HueRainbow> {
    map: PhantomData<M>,
    inner: f32,
}

pub struct Hsi<M: HueMap = HueRainbow> {
    hue: Hue<M>,
    saturation: f32,
    intensity: f32,
}

impl<M: HueMap> OutputColor for Hsi<M> {
    fn to_linear_rgb(self) -> LinearRgb {
        todo!()
    }
    fn to_linear_rgbw(self) -> LinearRgbw {
        todo!()
    }
}

pub struct HueRainbow;

pub struct HueSpectrum;

pub trait HueMap {
    // ???
}
