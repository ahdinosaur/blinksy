#![no_std]
#![no_main]

use core::iter;

use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    leds::Ws2812,
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    util::map_range,
    ControlBuilder,
};
use gledopto::{board, bootloader, elapsed, main, ws2812};

bootloader!();

struct VolumeCubeLayout;

impl Layout3d for VolumeCubeLayout {
    const PIXEL_COUNT: usize = 5 * 5 * 5;

    fn shapes() -> impl Iterator<Item = Shape3d> {
        let mut index: usize = 0;

        fn map(n: usize) -> f32 {
            map_range(n as f32, 0., 4., -1., 1.)
        }

        iter::from_fn(move || {
            if index >= 5 * 5 * 5 {
                return None;
            }

            let x = map(index % 5);
            let z = map(index / 5 % 5);
            let y = map(index / 5 / 5);

            index += 1;

            Some(Shape3d::Point(Vec3::new(x, y, z)))
        })
    }
}

#[main]
fn main() -> ! {
    let p = board!();

    let mut control = ControlBuilder::new_3d()
        .with_layout::<VolumeCubeLayout, { VolumeCubeLayout::PIXEL_COUNT }>()
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            time_scalar: 0.25 / 1e3,
            position_scalar: 0.25,
        })
        .with_driver(ws2812!(p, VolumeCubeLayout::PIXEL_COUNT, {
            Layout::PIXEL_COUNT * 3 * 8 + 1
        }))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    control.set_brightness(0.1);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
