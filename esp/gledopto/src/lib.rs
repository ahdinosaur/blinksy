//! # Gledopto Board Support Package
//!
//! Rust **no-std** [embedded](https://github.com/rust-embedded/awesome-embedded-rust) board support crate for Gledopto ESP32 Digital LED controllers.
//!
//! Uses [Blinksy](https://github.com/ahdinosaur/blinksy): an LED control library for 1D, 2D, and soon 3D LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).
//!
//! ## Supported Boards
//!
//! Currently this library only supports one board:
//!
//! - [x] [Gledopto GL-C-016WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_1), `gl_c_016wl_d`
//!
//! Select the board by using its respective feature.
//!
//! ## Features
//!
//! - [x] LED control using [`blinksy`](https://github.com/ahdinosaur/blinksy)
//! - [x] Built-in "Function" button
//! - [ ] Alternative "IO33" button
//! - [ ] Built-in microphone
//!
//!
//!
//! ## Getting started
//!
//! For help to get started, see ["Getting Started"][getting-started] section in README.
//!
//! [getting-started]: https://github.com/ahdinosaur/blinksy/blob/gledopto/v0.3.1/esp/gledopto/README.md
//!
//! ## Examples
//!
//! ### 1D WS2812 Strip with Rainbow Pattern
//!
//! ```rust,no-run
//! #![no_std]
//! #![no_main]
//!
//! use blinksy::{
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::rainbow::{Rainbow, RainbowParams},
//!     ControlBuilder,
//! };
//! use gledopto::{board, elapsed, main, ws2812};
//!
//! #[main]
//! fn main() -> ! {
//!     let p = board!();
//!
//!     layout1d!(Layout, 60 * 5);
//!
//!     let mut control = ControlBuilder::new_1d()
//!         .with_layout::<Layout>()
//!         .with_pattern::<Rainbow>(RainbowParams {
//!             ..Default::default()
//!         })
//!         .with_driver(ws2812!(p, Layout::PIXEL_COUNT))
//!         .build();
//!
//!     control.set_brightness(0.2);
//!
//!     loop {
//!         let elapsed_in_ms = elapsed().as_millis();
//!         control.tick(elapsed_in_ms).unwrap();
//!     }
//! }
//! ```
//!
//! ### 2D APA102 Grid with Noise Pattern
//!
//! ```rust,no-run
//! #![no_std]
//! #![no_main]
//!
//! use blinksy::{
//!     layout::{Shape2d, Vec2},
//!     layout2d,
//!     patterns::noise::{noise_fns, Noise2d, NoiseParams},
//!     ControlBuilder,
//! };
//! use gledopto::{apa102, board, elapsed, main};
//!
//! #[main]
//! fn main() -> ! {
//!     let p = board!();
//!
//!     layout2d!(
//!         Layout,
//!         [Shape2d::Grid {
//!             start: Vec2::new(-1., -1.),
//!             row_end: Vec2::new(1., -1.),
//!             col_end: Vec2::new(-1., 1.),
//!             row_pixel_count: 16,
//!             col_pixel_count: 16,
//!             serpentine: true,
//!         }]
//!     );
//!     let mut control = ControlBuilder::new_2d()
//!         .with_layout::<Layout>()
//!         .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
//!             ..Default::default()
//!         })
//!         .with_driver(apa102!(p))
//!         .build();
//!
//!     control.set_brightness(0.1);
//!
//!     loop {
//!         let elapsed_in_ms = elapsed().as_millis();
//!         control.tick(elapsed_in_ms).unwrap();
//!     }
//! }
//! ```

#![no_std]

/// Re-export of the core Blinksy library
pub use blinksy;

/// Re-export of the ESP32-specific Blinksy extensions
pub use blinksy_esp;
pub use blinksy_esp::time::elapsed;

/// Re-export of the ESP32 HAL
pub use esp_hal as hal;

/// Re-export the main macro from esp_hal for entry point definition
pub use hal::main;

/// Re-export the ESP32 heap allocator
pub use esp_alloc as alloc;

// These modules provide error handling and debug printing
use esp_backtrace as _;
use esp_println as _;

/// Button handling functionality
pub mod button;

/// Initializes the heap allocator with a 72KB heap.
///
/// This is required for ESP32 targets that need dynamic memory allocation.
#[macro_export]
macro_rules! heap_allocator {
    () => {
        $crate::alloc::heap_allocator!(size: 72 * 1024);
    };
}

/// Initializes the ESP32 board with optimal settings.
///
/// Configures the CPU clock to the maximum frequency for best performance.
#[macro_export]
macro_rules! board {
    () => {{
        let cpu_clock = $crate::hal::clock::CpuClock::max();
        let config = $crate::hal::Config::default().with_cpu_clock(cpu_clock);
        $crate::hal::init(config)
    }};
}

/// Creates a function button instance connected to GPIO0.
///
/// The function button can be used for mode selection, brightness control, etc.
#[macro_export]
macro_rules! function_button {
    ($peripherals:ident) => {
        $crate::button::FunctionButton::new($peripherals.GPIO0)
    };
}

/// Creates an APA102 LED driver using the SPI interface.
///
/// # Arguments
///
/// * `$peripherals` - The ESP32 peripherals instance
///
/// # Returns
///
/// An APA102 driver configured for the Gledopto board
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

        $crate::blinksy::drivers::apa102::Apa102Spi::new(spi)
    }};
}

/// Creates a WS2812 LED driver using the RMT peripheral.
///
/// # Arguments
///
/// * `$peripherals` - The ESP32 peripherals instance
/// * `$num_leds` - The number of LEDs in the strip
///
/// # Returns
///
/// A WS2812 driver configured for the Gledopto board
#[macro_export]
macro_rules! ws2812 {
    ($peripherals:ident, $num_leds:expr) => {{
        let led_pin = $peripherals.GPIO16;
        let freq = $crate::hal::time::Rate::from_mhz(80);
        let rmt = $crate::hal::rmt::Rmt::new($peripherals.RMT, freq).unwrap();

        const CHANNEL_COUNT: usize = <
                        $crate::blinksy::drivers::ws2812::Ws2812Led as $crate::blinksy::driver::ClocklessLed
                    >::LED_CHANNELS.channel_count();

        let rmt_buffer = $crate::blinksy_esp::create_rmt_buffer!($num_leds, CHANNEL_COUNT);

        $crate::blinksy_esp::Ws2812Rmt::new(rmt.channel0, led_pin, rmt_buffer)
    }};
}
