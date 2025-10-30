#![no_std]
#![no_main]

use blinksy::{
    layout::Layout1d,
    layout1d,
    leds::Ws2812,
    patterns::{
        noise::{noise_fns, Noise1d, NoiseParams},
        rainbow::Rainbow,
    },
    ControlBuilder,
};
use gledopto::{board, bootloader, elapsed, function_button, main, ws2812};

bootloader!();

blinksy::pattern_switch! {
    pub mod patterns {
        Rainbow: Rainbow,
        Noise: Noise1d<noise_fns::Perlin>,
    }
}

#[main]
fn main() -> ! {
    let p = board!();

    layout1d!(Layout, 50);

    // Build Control with the wrapper pattern; start with Rainbow active.
    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<patterns::Switch>(patterns::Params::Select(patterns::Active::Rainbow))
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    control.set_brightness(0.2);

    let mut button = function_button!(p);

    loop {
        let t = elapsed().as_millis();
        control.tick(t).unwrap();

        button.tick();

        // Toggle pattern on single click
        if button.is_clicked() {
            control.set_params(patterns::Params::Toggle);
        }

        // Example: change Noise params and select Noise on double click
        if button.is_double_clicked() {
            control.set_params(patterns::Params::Set(patterns::SetParam::Noise(
                NoiseParams {
                    time_scalar: 0.2 / 1e3,
                    position_scalar: 0.4,
                    ..Default::default()
                },
            )));
            control.set_params(patterns::Params::Select(patterns::Active::Noise));
        }

        button.reset();
    }
}
