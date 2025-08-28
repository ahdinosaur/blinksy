#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{board, bootloader, elapsed, init_embassy, main_embassy, ws2812_async};

bootloader!();

#[main_embassy]
async fn main(_spawner: Spawner) {
    let p = board!();

    init_embassy!(p);

    layout1d!(Layout, 60 * 5);

    let mut control = ControlBuilder::new_1d_async()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams::default())
        .with_driver(ws2812_async!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
