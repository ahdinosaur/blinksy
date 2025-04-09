#![no_std]

pub use blinksy;
pub use blinksy_esp;
pub use esp_hal as hal;
use esp_hal::time::{Duration, Instant};
pub use hal::main;

pub use esp_alloc as alloc;
use esp_backtrace as _;
use esp_println as _;

pub mod button;

#[macro_export]
macro_rules! heap_allocator {
    () => {
        $crate::alloc::heap_allocator!(size: 72 * 1024);
    }
}

#[macro_export]
macro_rules! board {
    () => {{
        let cpu_clock = $crate::hal::clock::CpuClock::max();
        let config = $crate::hal::Config::default().with_cpu_clock(cpu_clock);
        $crate::hal::init(config)
    }};
}

pub fn elapsed() -> Duration {
    Instant::now().duration_since_epoch()
}

#[macro_export]
macro_rules! function_button {
    ($peripherals:ident) => {
        $crate::button::FunctionButton::new($peripherals.GPIO0)
    };
}

#[macro_export]
macro_rules! apa102 {
    ($peripherals:ident) => {{
        let clock_pin = $peripherals.GPIO16;
        let data_pin = $peripherals.GPIO2;
        let data_rate = $crate::hal::time::Rate::from_mhz(4);
        let mut spi = $crate::hal::spi::master::Spi::new(
            $peripherals.SPI2,
            $crate::hal::spi::master::Config::default()
                .with_frequency(data_rate)
                .with_mode($crate::hal::spi::Mode::_0),
        )
        .expect("Failed to setup SPI")
        .with_sck(clock_pin)
        .with_mosi(data_pin);
        $crate::blinksy::drivers::Apa102Spi::new(spi)
    }};
}

#[macro_export]
macro_rules! ws2812 {
    ($peripherals:ident, $num_leds:expr) => {{
        let led_pin = $peripherals.GPIO16;
        let freq = $crate::hal::time::Rate::from_mhz(80);
        let rmt = $crate::hal::rmt::Rmt::new($peripherals.RMT, freq).unwrap();
        const CHANNEL_COUNT: usize =
            <$crate::blinksy::drivers::Ws2812Led as $crate::blinksy::driver::ClocklessLed>::COLOR_CHANNELS.channel_count();
        let rmt_buffer = $crate::blinksy_esp::create_rmt_buffer!($num_leds, CHANNEL_COUNT);
        $crate::blinksy_esp::drivers::Ws2812Rmt::new(rmt.channel0, led_pin, rmt_buffer)
    }};
}
