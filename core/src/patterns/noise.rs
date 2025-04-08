use noise::NoiseFn;
use palette::Hsv;

use crate::{
    dimension::{Dim1d, Dim2d},
    layout::{Layout1d, Layout2d},
    pattern::Pattern,
};

pub mod noise_fns {
    pub use noise::{OpenSimplex, Perlin, Simplex};
}

#[derive(Debug)]
pub struct NoiseParams {
    pub time_scalar: f64,
    pub position_scalar: f64,
    pub brightness: f32,
}

impl Default for NoiseParams {
    fn default() -> Self {
        const MILLISECONDS_PER_SECOND: f64 = 1e3;
        Self {
            time_scalar: 0.75 / MILLISECONDS_PER_SECOND,
            position_scalar: 0.5,
            brightness: 1.,
        }
    }
}

#[derive(Debug)]
pub struct Noise1d<Noise>
where
    Noise: NoiseFn<f64, 2>,
{
    noise: Noise,
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim1d, Layout> for Noise1d<Noise>
where
    Layout: Layout1d,
    Noise: NoiseFn<f64, 2> + Default,
{
    type Params = NoiseParams;
    type Color = Hsv;

    fn new(params: Self::Params) -> Self {
        Self {
            noise: Noise::default(),
            params,
        }
    }

    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { noise, params } = self;
        let NoiseParams {
            time_scalar,
            position_scalar,
            brightness,
        } = params;

        let noise_time = time_in_ms as f64 * time_scalar;

        (0..Layout::PIXEL_COUNT).map(move |index| {
            let noise = noise.get([position_scalar * index as f64, noise_time]);

            let hue = 360. * noise as f32;
            let saturation = 1.;

            Hsv::new_srgb(hue, saturation, *brightness)
        })
    }
}

#[derive(Debug)]
pub struct Noise2d<Noise>
where
    Noise: NoiseFn<f64, 3>,
{
    noise: Noise,
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim2d, Layout> for Noise2d<Noise>
where
    Layout: Layout2d,
    Noise: NoiseFn<f64, 3> + Default,
{
    type Params = NoiseParams;
    type Color = Hsv;

    fn new(params: Self::Params) -> Self {
        Self {
            noise: Noise::default(),
            params,
        }
    }

    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { noise, params } = self;
        let NoiseParams {
            time_scalar,
            position_scalar,
            brightness,
        } = params;

        let noise_time = time_in_ms as f64 * time_scalar;

        Layout::points().map(move |point| {
            let noise = noise.get([
                position_scalar * point.x as f64,
                position_scalar * point.y as f64,
                noise_time,
            ]);

            let hue = 360. * noise as f32;
            let saturation = 1.;

            Hsv::new_srgb(hue, saturation, *brightness)
        })
    }
}
