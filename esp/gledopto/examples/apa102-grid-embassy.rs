#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use blinksy::{
    layout::{Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{apa102_async, board, elapsed, main_embassy};

#[main_embassy]
async fn main(_spawner: Spawner) {
    let p = board!();

    layout2d!(
        Layout,
        [Shape2d::Grid {
            start: Vec2::new(-1., -1.),
            horizontal_end: Vec2::new(1., -1.),
            vertical_end: Vec2::new(-1., 1.),
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        }]
    );
    let mut control = ControlBuilder::new_2d_async()
        .with_layout::<Layout>()
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
            ..Default::default()
        })
        .with_driver(apa102_async!(p))
        .build();

    control.set_brightness(0.02);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
