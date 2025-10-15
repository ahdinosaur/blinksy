#![no_std]
#![no_main]

use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    layout3d,
    leds::Ws2812,
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    ControlBuilder,
};
use gledopto::{board, bootloader, elapsed, main, ws2812};

bootloader!();

#[main]
fn main() -> ! {
    let p = board!();

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

    let mut control = ControlBuilder::new_3d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            ..Default::default()
        })
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT, {
            gledopto::hal::rmt::CHANNEL_RAM_SIZE
        }))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
