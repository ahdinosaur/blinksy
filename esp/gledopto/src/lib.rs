//! # Gledopto Board Support Package
//!
//! Rust **no-std** [embedded](https://github.com/rust-embedded/awesome-embedded-rust) board support crate for Gledopto ESP32 Digital LED controllers.
//!
//! Uses [Blinksy](https://github.com/ahdinosaur/blinksy): an LED control library for 1D, 2D, and 3D LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).
//!
//! ## Supported Boards
//!
//! - [x] [Gledopto GL-C-016WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_1), `gl_c_016wl_d`
//! - [x] [Gledopto GL-C-017WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_5), `gl_c_017wl_d`
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
//! ## Getting started
//!
//! To quickstart a project, see [`blinksy-quickstart-gledopto`][blinksy-quickstart-gledopto].
//!
//! [blinksy-quickstart-gledopto]: https://github.com/ahdinosaur/blinksy-quickstart-gledopto
//!
//! ## Examples
//!
//! ### 1D WS2812 Strip with Rainbow Pattern
//!
//! ```rust,no_run
//! #![no_std]
//! #![no_main]
//!
//! use blinksy::{
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::rainbow::{Rainbow, RainbowParams},
//!     ControlBuilder,
//! };
//! use gledopto::{board, bootloader, elapsed, main, ws2812};
//!
//! bootloader!();
//!
//! #[main]
//! fn main() -> ! {
//!     let p = board!();
//!
//!     layout1d!(Layout, 60 * 5);
//!
//!     let mut control = ControlBuilder::new_1d()
//!         .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!         .with_pattern::<Rainbow>(RainbowParams {
//!             ..Default::default()
//!         })
//!         .with_driver(ws2812!(p, Layout::PIXEL_COUNT, { Layout:: PIXEL_COUNT * 3 * 8 + 1 }))
//!         .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
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
//! ```rust,no_run
//! #![no_std]
//! #![no_main]
//!
//! use blinksy::{
//!     layout::{Shape2d, Vec2},
//!     layout2d,
//!     patterns::noise::{noise_fns, Noise2d, NoiseParams},
//!     ControlBuilder,
//! };
//! use gledopto::{apa102, board, bootloader, elapsed, main};
//!
//! bootloader!();
//!
//! #[main]
//! fn main() -> ! {
//!     let p = board!();
//!
//!     layout2d!(
//!         Layout,
//!         [Shape2d::Grid {
//!             start: Vec2::new(-1., -1.),
//!             horizontal_end: Vec2::new(1., -1.),
//!             vertical_end: Vec2::new(-1., 1.),
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         }]
//!     );
//!     let mut control = ControlBuilder::new_2d()
//!         .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!         .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
//!             ..Default::default()
//!         })
//!         .with_driver(apa102!(p))
//!         .with_frame_buffer_size::<{ Apa102::frame_buffer_size(Layout::PIXEL_COUNT) }>()
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

#[cfg(feature = "embassy")]
/// Re-export the ESP32 Embassy HAL
pub use esp_hal_embassy as hal_embassy;

#[cfg(feature = "embassy")]
/// Re-export the main macro from esp_hal for entry point definition
pub use hal_embassy::main as main_embassy;

/// Re-export the ESP32 heap allocator
pub use esp_alloc as alloc;

/// Re-export the ESP32 heap allocator
pub use esp_bootloader_esp_idf as bootloader;

// These modules provide error handling and debug printing
pub use esp_backtrace as backtrace;
pub use esp_println as println;

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

/// Populates the bootloader application descriptor
///
/// This is required for espflash.
#[macro_export]
macro_rules! bootloader {
    () => {
        $crate::bootloader::esp_app_desc!();
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

#[cfg(feature = "embassy")]
#[macro_export]
macro_rules! init_embassy {
    ($peripherals:ident) => {{
        let timg0 = $crate::hal::timer::timg::TimerGroup::new($peripherals.TIMG0);
        $crate::hal_embassy::init(timg0.timer0);
    }};
}

#[macro_export]
macro_rules! spi {
    ($peripherals:ident) => {{
        let clock_pin = $peripherals.GPIO16;
        let data_pin = $peripherals.GPIO2;
        let data_rate = $crate::hal::time::Rate::from_mhz(4);

        $crate::hal::spi::master::Spi::new(
            $peripherals.SPI2,
            $crate::hal::spi::master::Config::default()
                .with_frequency(data_rate)
                .with_mode($crate::hal::spi::Mode::_0),
        )
        .expect("Failed to setup SPI")
        .with_sck(clock_pin)
        .with_mosi(data_pin)
    }};
}

/// Creates a clocked LED driver using the SPI interface.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
/// - `$led` - The type of LED
///
/// # Returns
///
/// A clocked driver configured for the Gledopto board
#[macro_export]
macro_rules! clocked {
    ($peripherals:ident, $led:ty) => {{
        let spi = $crate::spi!($peripherals);
        $crate::blinksy::driver::ClockedDriver::default()
            .with_led::<$led>()
            .with_writer(spi)
    }};
}

/// Creates a APA102 LED driver using the SPI interface.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
///
/// # Returns
///
/// An APA102 driver configured for the Gledopto board
#[macro_export]
macro_rules! apa102 {
    ($peripherals:ident) => {{
        $crate::clocked!($peripherals, $crate::blinksy::leds::Apa102)
    }};
}

/// Creates an async clocked LED driver using the SPI interface.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
/// - `$led` - The type of LED
///
/// # Returns
///
/// A clocked driver configured for the Gledopto board
#[cfg(feature = "async")]
#[macro_export]
macro_rules! clocked_async {
    ($peripherals:ident, $led:ty) => {{
        let spi = $crate::spi!($peripherals).into_async();
        $crate::blinksy::driver::ClockedDriver::default()
            .with_led::<$led>()
            .with_writer(spi)
    }};
}

/// Creates an async APA102 LED driver using the SPI interface.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
///
/// # Returns
///
/// An APA102 driver configured for the Gledopto board
#[cfg(feature = "async")]
#[macro_export]
macro_rules! apa102_async {
    ($peripherals:ident) => {{
        $crate::clocked_async!($peripherals, $crate::blinksy::leds::Apa102)
    }};
}

#[macro_export]
macro_rules! rmt {
    ($peripherals:ident) => {{
        let freq = $crate::hal::time::Rate::from_mhz(80);
        $crate::hal::rmt::Rmt::new($peripherals.RMT, freq).unwrap()
    }};
}

/// Creates a clockless LED driver using the RMT peripheral.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
/// - `$pixel_count` - The number of LEDs
/// - `$led` - The type of LED
/// - `$rmt_buffer_size` (Optional) - The length of the RMT buffer
///   - (Can be `buffered` literal to mean RMT buffer size should be complete frame.)
///
/// # Returns
///
/// A clockless driver configured for the LED type on the Gledopto board
#[macro_export]
macro_rules! clockless {
    ($peripherals:ident, $pixel_count:expr, $led:ty) => {{
        $crate::clockless!($peripherals, $pixel_count, $led, 64)
    }};
    ($peripherals:ident, $pixel_count:expr, $led:ty, buffered) => {{
        $crate::clockless!($peripherals, $pixel_count, $led, {
            $crate::blinksy_esp::rmt::rmt_buffer_size::<$led>($pixel_count)
        })
    }};
    ($peripherals:ident, $pixel_count:expr, $led:ty, $rmt_buffer_size:expr) => {{
        let led_pin = $peripherals.GPIO16;
        let rmt = $crate::rmt!($peripherals);

        $crate::blinksy::driver::ClocklessDriver::default()
            .with_led::<$led>()
            .with_writer(
                $crate::blinksy_esp::ClocklessRmtBuilder::default()
                    .with_rmt_buffer_size::<$rmt_buffer_size>()
                    .with_led::<$led>()
                    .with_channel(rmt.channel0)
                    .with_pin(led_pin)
                    .build(),
            )
    }};
}

/// Creates a WS2812 LED driver using the RMT peripheral.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
/// - `$pixel_count` - The number of LEDs in the strip
/// - `$rmt_buffer_size` (Optional) - The length of the RMT buffer
///   - (Can be `buffered` literal to mean RMT buffer size should be complete frame.)
///
/// # Returns
///
/// A WS2812 driver configured for the Gledopto board
#[macro_export]
macro_rules! ws2812 {
    ($peripherals:ident, $pixel_count:expr) => {{
        $crate::clockless!($peripherals, $pixel_count, $crate::blinksy::leds::Ws2812)
    }};
    ($peripherals:ident, $pixel_count:expr, buffered) => {{
        $crate::clockless!(
            $peripherals,
            $pixel_count,
            $crate::blinksy::leds::Ws2812,
            buffered
        )
    }};
    ($peripherals:ident, $pixel_count:expr, $rmt_buffer_size:expr) => {{
        $crate::clockless!(
            $peripherals,
            $pixel_count,
            $crate::blinksy::leds::Ws2812,
            $rmt_buffer_size
        )
    }};
}

/// Creates an async clockless LED driver using the RMT peripheral.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
/// - `$pixel_count` - The number of LEDs
/// - `$led` - The type of LED
///
/// # Returns
///
/// A clockless driver configured for the LED type on the Gledopto board
#[cfg(feature = "async")]
#[macro_export]
macro_rules! clockless_async {
    ($peripherals:ident, $pixel_count:expr, $led:ty) => {{
        $crate::clockless_async!($peripherals, $pixel_count, $led, 64)
    }};
    ($peripherals:ident, $pixel_count:expr, $led:ty, $rmt_buffer_size:expr) => {{
        let led_pin = $peripherals.GPIO16;
        let rmt = $crate::rmt!($peripherals).into_async();

        $crate::blinksy::driver::ClocklessDriver::default()
            .with_led::<$led>()
            .with_writer(
                $crate::blinksy_esp::ClocklessRmtBuilder::default()
                    .with_rmt_buffer_size::<$rmt_buffer_size>()
                    .with_led::<$led>()
                    .with_channel(rmt.channel0)
                    .with_pin(led_pin)
                    .build(),
            )
    }};
}

/// Creates an async WS2812 LED driver using the RMT peripheral.
///
/// # Arguments
///
/// - `$peripherals` - The ESP32 peripherals instance
/// - `$pixel_count` - The number of LEDs in the strip
/// - `$rmt_buffer_size` (Optional) - The length of the RMT buffer
///
/// # Returns
///
/// A WS2812 driver configured for the Gledopto board
#[cfg(feature = "async")]
#[macro_export]
macro_rules! ws2812_async {
    ($peripherals:ident, $pixel_count:expr, $rmt_buffer_size:expr) => {{
        $crate::clockless_async!(
            $peripherals,
            $pixel_count,
            $crate::blinksy::leds::Ws2812,
            $rmt_buffer_size
        )
    }};
    ($peripherals:ident, $pixel_count:expr) => {{
        $crate::clockless_async!($peripherals, $pixel_count, $crate::blinksy::leds::Ws2812)
    }};
}
