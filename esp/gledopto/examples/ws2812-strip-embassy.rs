#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use blinksy::{
    layout::Layout1d,
    layout1d,
    leds::Ws2812,
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

    layout1d!(Layout, 1500);

    let mut control = ControlBuilder::new_1d_async()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Rainbow>(RainbowParams {
            //time_scalar: 0.,
            ..Default::default()
        })
        .with_driver(ws2812_async!(p, Layout::PIXEL_COUNT))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
