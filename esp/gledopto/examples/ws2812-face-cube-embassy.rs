#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    layout3d,
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{board, bootloader, elapsed, init_embassy, main_embassy, ws2812_async};

bootloader!();

#[main_embassy]
async fn main(_spawner: Spawner) {
    let p = board!();

    init_embassy!(p);

    layout3d!(
        Layout,
        [
            // bottom face
            Shape3d::Grid {
                start: Vec3::new(1., -1., 1.),           // right bottom front
                horizontal_end: Vec3::new(-1., -1., 1.), // left bottom front
                vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // back face
            Shape3d::Grid {
                start: Vec3::new(-1., -1., -1.),         // left bottom back
                horizontal_end: Vec3::new(-1., 1., -1.), // left top back
                vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // right face
            Shape3d::Grid {
                start: Vec3::new(1., 1., -1.),         // right top back
                horizontal_end: Vec3::new(1., 1., 1.), // right top front
                vertical_end: Vec3::new(1., -1., -1.), // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // front face
            Shape3d::Grid {
                start: Vec3::new(-1., -1., 1.),         // left bottom front
                horizontal_end: Vec3::new(1., -1., 1.), // right bottom front
                vertical_end: Vec3::new(-1., 1., 1.),   // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // left face
            Shape3d::Grid {
                start: Vec3::new(-1., 1., -1.),           // left top back
                horizontal_end: Vec3::new(-1., -1., -1.), // left bottom back
                vertical_end: Vec3::new(-1., 1., 1.),     // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // top face
            Shape3d::Grid {
                start: Vec3::new(1., 1., 1.),           // right top front
                horizontal_end: Vec3::new(1., 1., -1.), // right top back
                vertical_end: Vec3::new(-1., 1., 1.),   // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            }
        ]
    );

    let mut control = ControlBuilder::new_3d_async()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            ..Default::default()
        })
        .with_driver(ws2812_async!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
