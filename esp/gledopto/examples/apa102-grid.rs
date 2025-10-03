#![no_std]
#![no_main]

use blinksy::{
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use gledopto::{apa102, board, bootloader, elapsed, main};

bootloader!();

#[main]
fn main() -> ! {
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
    let mut control = ControlBuilder::new_2d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
        .with_driver(apa102!(p))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
