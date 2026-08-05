#![no_std]
#![no_main]

use blinksy::{
    color::Okhsv,
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    leds::Ws2812,
    markers::Dim2d,
    patterns::noise::{
        noise_fns::{OpenSimplex2, Perlin, Simplex},
        Noise2d, NoiseParams,
    },
    ControlBuilder,
};
use gledopto::{board, bootloader, function_button, main};

bootloader!();

layout2d!(
    #[derive(Debug, Copy, Clone)]
    Layout2,
    [Shape2d::Grid {
        start: Vec2::new(-1., -1.),
        horizontal_end: Vec2::new(1., -1.),
        vertical_end: Vec2::new(-1., 1.),
        horizontal_pixel_count: 16,
        vertical_pixel_count: 16,
        serpentine: true,
    }]
);

type Noise1 = Noise2d<OpenSimplex2>;
type Noise2 = Noise2d<Perlin>;
type Noise3 = Noise2d<Simplex>;

blinksy::composite_pattern! {
    // NOTE! This is a macro, not a struct; arguments must appear in strict order.
    name: Composite,
    color: Okhsv,
    dims: Dim2d,
    layout: Layout2d,
    patterns: [Noise1, Noise2, Noise3]
}

const MS_PER_S: f32 = 1e3;

const NOISE2_PARAMS: NoiseParams = NoiseParams {
    time_scalar: 1. / MS_PER_S,
    position_scalar: 0.7,
};

const NOISE3_PARAMS: NoiseParams = NoiseParams {
    time_scalar: 0.5 / MS_PER_S,
    position_scalar: 0.4,
};

impl CompositeParams<Layout2> {
    fn next(&self) -> Self {
        match self {
            CompositeParams::Noise1(_) => CompositeParams::Noise2(NOISE2_PARAMS),
            CompositeParams::Noise2(_) => CompositeParams::Noise3(NOISE3_PARAMS),
            CompositeParams::Noise3(_) => CompositeParams::Noise1(NoiseParams::default()),
        }
    }
}

#[main]
fn main() -> ! {
    let p = board!();

    let mut params = CompositeParams::Noise1(NoiseParams::default());
    let mut control = ControlBuilder::new_2d()
        .with_layout::<Layout2, { Layout2::PIXEL_COUNT }>()
        .with_pattern::<Composite<Layout2>>(params.clone())
        .with_driver(gledopto::ws2812!(p, Layout2::PIXEL_COUNT))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout2::PIXEL_COUNT) }>()
        .build();

    let mut button = function_button!(p);
    control.set_brightness(0.05);

    loop {
        button.tick();
        if button.is_clicked() || button.held_time().is_some() {
            params = params.next();
            control.set_pattern_params(params.clone());
        }
        button.reset();
        control.tick(gledopto::elapsed().as_millis()).unwrap();
    }
}
