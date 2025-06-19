#![no_std]
#![no_main]

use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    layout3d,
    patterns::{
        noise::{noise_fns, Noise3d, NoiseParams},
        rainbow::{Rainbow, RainbowParams},
    },
    ControlBuilder,
};
use gledopto::{board, elapsed, main, ws2812};

#[main]
fn main() -> ! {
    let p = board!();

    layout3d!(
        Layout,
        [
            Shape3d::Grid {
                start: Vec3::new(-1., -1., -1.),
                horizontal_end: Vec3::new(-1., 1., -1.),
                vertical_end: Vec3::new(1., -1., -1.),
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            Shape3d::Grid {
                start: Vec3::new(1., 1., -1.),
                horizontal_end: Vec3::new(1., 1., 1.),
                vertical_end: Vec3::new(1., -1., -1.),
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            Shape3d::Grid {
                start: Vec3::new(1., -1., 1.),
                horizontal_end: Vec3::new(-1., -1., 1.),
                vertical_end: Vec3::new(1., -1., -1.),
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            Shape3d::Grid {
                start: Vec3::new(-1., -1., 1.),
                horizontal_end: Vec3::new(1., -1., 1.),
                vertical_end: Vec3::new(-1., 1., 1.),
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            Shape3d::Grid {
                start: Vec3::new(1., 1., 1.),
                horizontal_end: Vec3::new(1., 1., -1.),
                vertical_end: Vec3::new(-1., 1., 1.),
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            Shape3d::Grid {
                start: Vec3::new(-1., 1., -1.),
                horizontal_end: Vec3::new(-1., -1., -1.),
                vertical_end: Vec3::new(-1., 1., 1.),
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            }
        ]
    );

    let mut control = ControlBuilder::new_3d()
        .with_layout::<Layout>()
        /*
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            ..Default::default()
        })
        */
        .with_pattern::<Rainbow>(RainbowParams {
            position_scalar: 1.,
            // time_scalar: 50. / 1e6,
            ..Default::default()
        })
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
