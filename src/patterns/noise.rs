use noise::NoiseFn;
use palette::Hsl;

use crate::{Layout1d, Layout2d, Pattern};

pub mod noise_fns {
    pub use noise::{OpenSimplex, Perlin, Simplex};
}

#[derive(Debug)]
pub struct NoiseParams {
    time_scalar: f64,
    position_scalar: f64,
}

impl Default for NoiseParams {
    fn default() -> Self {
        const NANOSECONDS_PER_SECOND: f64 = 1e9;
        Self {
            time_scalar: 0.75 / NANOSECONDS_PER_SECOND,
            position_scalar: 0.5,
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

impl<Noise, const NUM_PIXELS: usize> Pattern<NUM_PIXELS> for Noise1d<Noise>
where
    Noise: NoiseFn<f64, 2> + Default,
{
    type Params = NoiseParams;
    type Layout = Layout1d;
    type Color = Hsl;

    fn new(params: Self::Params, _layout: Self::Layout) -> Self {
        Self {
            noise: Noise::default(),
            params,
        }
    }

    fn tick(&self, time_in_ms: u64) -> [Self::Color; NUM_PIXELS] {
        let Self { noise, params } = self;
        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        core::array::from_fn(move |index| {
            let noise_time = time_in_ms as f64 * time_scalar;
            let noise = noise.get([position_scalar * index as f64, noise_time]);

            let hue = 360. * noise as f32;
            let saturation = 1.;
            let lightness = 0.5;

            Hsl::new_srgb(hue, saturation, lightness)
        })
    }
}

#[derive(Debug)]
pub struct Noise2d<Noise, const NUM_SHAPES: usize>
where
    Noise: NoiseFn<f64, 3>,
{
    noise: Noise,
    params: NoiseParams,
    layout: Layout2d<NUM_SHAPES>,
}

impl<Noise, const NUM_SHAPES: usize, const NUM_PIXELS: usize> Pattern<NUM_PIXELS>
    for Noise2d<Noise, NUM_SHAPES>
where
    Noise: NoiseFn<f64, 3> + Default,
{
    type Params = NoiseParams;
    type Layout = Layout2d<NUM_SHAPES>;
    type Color = Hsl;

    fn new(params: Self::Params, layout: Self::Layout) -> Self {
        Self {
            noise: Noise::default(),
            params,
            layout,
        }
    }

    fn tick(&self, time_in_ms: u64) -> [Self::Color; NUM_PIXELS] {
        let Self {
            noise,
            params,
            layout,
        } = self;
        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        layout.map_points(|point| {
            let noise_time = time_in_ms as f64 * time_scalar;
            let noise = noise.get([
                position_scalar * point.x as f64,
                position_scalar * point.y as f64,
                noise_time,
            ]);

            let hue = 360. * noise as f32;
            let saturation = 1.;
            let lightness = 0.5;

            Hsl::new_srgb(hue, saturation, lightness)
        })
    }
}
