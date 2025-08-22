#![no_std]
#![no_main]

use core::iter;

use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    ControlBuilder,
};
use gledopto::{board, elapsed, main, ws2812};

struct VolumeCubeLayout;

impl Layout3d for VolumeCubeLayout {
    const PIXEL_COUNT: usize = 5 * 5 * 5;

    fn shapes() -> impl Iterator<Item = Shape3d> {
        let mut index: usize = 0;

        fn map(n: usize) -> f32 {
            assert!(n <= 4, "Input must be between 0 and 4 inclusive.");
            // Map 0..=5 to 0.0..=1.0
            let normalized = n as f32 / 4.0;
            // Map 0.0..=1.0 to -1.0..=1.0
            normalized * 2.0 - 1.0
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
        .with_layout::<VolumeCubeLayout>()
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            time_scalar: 0.25 / 1e3,
            position_scalar: 0.25,
            ..Default::default()
        })
        .with_driver(ws2812!(p, VolumeCubeLayout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.1);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
