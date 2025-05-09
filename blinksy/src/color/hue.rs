pub struct Hue<M: HueMap = HueRainbow> {
    map: PhantomData<M>,
    inner: f32,
}

pub struct Hsi<M: HueMap = HueRainbow> {
    hue: Hue<M>,
    saturation: f32,
    intensity: f32,
}

impl<M: HueMap> Hsi<M> {
    pub fn to_rgb() -> Rgb {}

    pub fn to_rgbw() -> Rgbw {}
}

pub struct HueRainbow;

pub struct HueSpectrum;

pub trait HueMap {}
