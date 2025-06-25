#![no_std]
#![no_main]

use blinksy::{
    layout::Layout1d,
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{board, elapsed, main_embassy, ws2812_async};

#[main_embassy]
async fn main(_spawner: Spawner) {
    let p = board!();

    layout1d!(Layout, 60 * 5);

    let mut control = ControlBuilder::new_1d_async()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            position_scalar: 1.,
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
