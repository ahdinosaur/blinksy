#![no_std]
#![no_main]

use blinksy::{
    drivers::ws2812::Ws2812Led,
    layout::Layout1d,
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use gledopto::{board, bootloader, elapsed, main, ws2812};

bootloader!();

#[main]
fn main() -> ! {
    let p = board!();

    layout1d!(Layout, 60);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Rainbow>(RainbowParams::default())
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT, 1024))
        .with_frame_buffer_size::<{ Ws2812Led::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
