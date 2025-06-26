#![no_std]
#![no_main]

use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use gledopto::{board, elapsed, main, ws2812};

#[main]
fn main() -> ! {
    let p = board!();

    layout1d!(Layout, 60 * 5);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams::default())
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
