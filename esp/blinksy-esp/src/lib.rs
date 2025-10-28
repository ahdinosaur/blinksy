#![no_std]

//! # ESP32 Blinksy Extensions
//!
//! ESP32-specific extensions for the [Blinksy][blinksy] LED control library using [`esp-hal`][esp_hal].
//!
//! ## Features
//!
//! - ESP-specific driver for clockless (e.g. WS2812) LEDs, using [RMT (Remote Control Module)][RMT] peripheral
//! - ESP-specific elapsed time helper
//!
//! [RMT]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/peripherals/rmt.html
//!
//! ## Example
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! use esp_hal as hal;
//!
//! use blinksy::{
//!     driver::ClocklessLed,
//!     drivers::ws2812::Ws2812Led,
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::rainbow::{Rainbow, RainbowParams},
//!     ControlBuilder,
//! };
//! use blinksy_esp::{create_rmt_buffer, time::elapsed, Ws2812Rmt};
//!
//! #[hal::main]
//! fn main() -> ! {
//!     let cpu_clock = hal::clock::CpuClock::max();
//!     let config = hal::Config::default().with_cpu_clock(cpu_clock);
//!     let p = hal::init(config);
//!
//!     // Define the LED layout (1D strip of 300 pixels)
//!     layout1d!(Layout, 60 * 5);
//!
//!     // Setup the WS2812 driver using RMT.
//!     let ws2812_driver = {
//!         // IMPORTANT: Change `p.GPIO16` to the GPIO pin connected to your WS2812 data line.
//!         let data_pin = p.GPIO16;
//!
//!         // Initialize RMT peripheral (typical base clock 80 MHz).
//!         let rmt_clk_freq = hal::time::Rate::from_mhz(80);
//!         let rmt = hal::rmt::Rmt::new(p.RMT, rmt_clk_freq).unwrap();
//!         let rmt_channel = rmt.channel0;
//!
//!         // Create the driver using the ClocklessRmt builder."]
//!         blinksy::driver::ClocklessDriver::default()
//!             .with_led::<Ws2812>()
//!             .with_writer(ClocklessRmtBuilder::default()
//!                 .with_rmt_buffer_size::<{ Layout::PIXEL_COUNT * 3 * 8 + 1 }>()
//!                 .with_led::<Ws2812>()
//!                 .with_channel(rmt_channel)
//!                 .with_pin(data_pin)
//!                 .build())
//!     };
//!
//!     // Build the Blinky controller
//!     let mut control = ControlBuilder::new_1d()
//!         .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!         .with_pattern::<Rainbow>(RainbowParams {
//!             ..Default::default()
//!         })
//!         .with_driver(ws2812_driver)
//!         .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
//!         .build();
//!
//!     control.set_brightness(0.2); // Set initial brightness (0.0 to 1.0)
//!
//!     loop {
//!         let elapsed_in_ms = elapsed().as_millis();
//!         control.tick(elapsed_in_ms).unwrap();
//!     }
//! }
//! ```
//!
//! ## Getting started
//!
//! For more help to get started, see [`blinksy-quickstart-gledopto`][blinksy-quickstart-gledopto]
//! project template and [`gledopto`][gledopto] library.
//!
//! As the Gledopto controller is an ESP32 board, the project template and library should provide
//! an entry point to understand how to use Blinksy with an ESP board.
//!
//! [blinksy-quickstart-gledopto]: https://github.com/ahdinosaur/blinksy-quickstart-gledopto
//! [gledopto]: https://docs.rs/gledopto/0.11/gledopto

pub mod rmt;
pub mod time;
pub(crate) mod util;

pub use crate::rmt::{ClocklessRmt, ClocklessRmtBuilder, ClocklessRmtError};
