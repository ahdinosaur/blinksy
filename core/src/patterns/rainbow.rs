use palette::Hsv;

use crate::{
    layout::{Layout1d, Layout2d},
    pattern::{Pattern1d, Pattern2d},
};

#[derive(Debug)]
pub struct RainbowParams {
    pub time_scalar: f32,
    pub position_scalar: f32,
    pub brightness: f32,
}

impl Default for RainbowParams {
    fn default() -> Self {
        Self {
            time_scalar: 0.1,
            position_scalar: 1.,
            brightness: 1.,
        }
    }
}

#[derive(Debug)]
pub struct Rainbow {
    params: RainbowParams,
}

impl<Layout> Pattern1d<Layout> for Rainbow
where
    Layout: Layout1d,
{
    type Params = RainbowParams;
    type Color = Hsv;

    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let RainbowParams {
            time_scalar,
            position_scalar,
            brightness,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;
        let step = (1. / Layout::PIXEL_COUNT as f32) * 360. * position_scalar;

        (0..Layout::PIXEL_COUNT).map(move |index| {
            let hue = index as f32 * step + noise_time;
            let saturation = 1.;

            Hsv::new_srgb(hue, saturation, *brightness)
        })
    }
}

impl<Layout> Pattern2d<Layout> for Rainbow
where
    Layout: Layout2d,
{
    type Params = RainbowParams;
    type Color = Hsv;

    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let RainbowParams {
            time_scalar,
            position_scalar,
            brightness,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;
        let step = (1. / Layout::PIXEL_COUNT as f32) * 360. * position_scalar;

        Layout::points().map(move |point| {
            let hue = point.x + noise_time * step;
            let saturation = 1.;

            Hsv::new_srgb(hue, saturation, *brightness)
        })
    }
}
