#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_alloc as _;
use esp_hal::{self as hal, delay::Delay};
use esp_hal::main;

use blinksy::{
    ControlBuilder,
    driver::ClocklessDriver,
    layout::Layout1d,
    layout1d,
    leds::Ws2812,
    patterns::rainbow::{Rainbow, RainbowParams},
};
use blinksy_esp::{rmt::ClocklessRmtBuilder, time::elapsed};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let cpu_clock = hal::clock::CpuClock::max();
    let config = hal::Config::default().with_cpu_clock(cpu_clock);
    let p = hal::init(config);

    // layout1d!(Layout, 60 * 5);
    layout1d!(Layout, 1);

    let ws2812_driver = {
        let data_pin = p.GPIO48;
        let rmt_clk_freq = hal::time::Rate::from_mhz(80);

        let rmt = hal::rmt::Rmt::new(p.RMT, rmt_clk_freq).unwrap();
        let rmt_channel = rmt.channel0;

        ClocklessDriver::default().with_led::<Ws2812>().with_writer(
            ClocklessRmtBuilder::default()
                .with_rmt_buffer_size::<{ Layout::PIXEL_COUNT * 3 * 8 + 1 }>()
                .with_led::<Ws2812>()
                .with_channel(rmt_channel)
                .with_pin(data_pin)
                .build(),
        )
    };

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(ws2812_driver)
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();
    control.set_color_correction(blinksy::color::ColorCorrection {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    });
    control.set_brightness(0.5); // Set initial brightness (0.0 to 1.0)

    let mut led_on_off = false;
    let delay = Delay::new();

    loop {
        if led_on_off {
            control.set_color_correction(blinksy::color::ColorCorrection {
                red: 0.0,
                green: 0.0,
                blue: 1.0,
            });
        } else {
            control.set_color_correction(blinksy::color::ColorCorrection {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            });
        }
        led_on_off = !led_on_off;
        let elapsed_in_ms = elapsed().as_micros();
        control.tick(elapsed_in_ms).unwrap();
        delay.delay_millis(500);
    }
}
