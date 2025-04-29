//! # Gledopto Board Support Package
//!
//! This module provides a board support package (BSP) for the Gledopto GL-C-016WL-D
//! LED controller, which is based on the ESP32 microcontroller. It integrates
//! Blinksy with ESP32-specific functionality for a streamlined development experience.
//!
//! ## Features
//!
//! - Integration with ESP32 HAL for hardware access
//! - Convenient macros for board initialization and peripheral setup
//! - Support for common LED drivers
//! - Function button handling
//! - Time management utilities
//!
//! ## Usage Examples
//!
//! ```rust,no-run
//! #![no_std]
//! #![no_main]
//!
//! use blinksy::{
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::{Rainbow, RainbowParams},
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

#![no_std]

/// Re-export of the core Blinksy library
pub use blinksy;

/// Re-export of the ESP32-specific Blinksy extensions
pub use blinksy_esp;

/// Re-export of the ESP32 HAL
pub use esp_hal as hal;

use esp_hal::time::{Duration, Instant};

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

/// Returns the elapsed time since system boot.
///
/// This function provides a consistent timing reference for animations and patterns.
pub fn elapsed() -> Duration {
    Instant::now().duration_since_epoch()
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

        $crate::blinksy::drivers::Apa102Spi::new(spi)
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
                    $crate::blinksy::drivers::Ws2812Led as $crate::blinksy::driver::ClocklessLed
                >::COLOR_CHANNELS.channel_count();

        let rmt_buffer = $crate::blinksy_esp::create_rmt_buffer!($num_leds, CHANNEL_COUNT);

        $crate::blinksy_esp::drivers::Ws2812Rmt::new(rmt.channel0, led_pin, rmt_buffer)
    }};
}
