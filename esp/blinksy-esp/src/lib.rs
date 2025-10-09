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
//!         // For example, if using GPIO2, change to `p.GPIO2`.
//!         // Ensure the chosen pin is not used for other critical functions (e.g., strapping, JTAG).
//!         let data_pin = p.GPIO16;
//!
//!         // RMT peripheral frequency, typically 80MHz for WS2812 on ESP32.
//!         let rmt_clk_freq = hal::time::Rate::from_mhz(80);
//!
//!         // Initialize RMT peripheral.
//!         let rmt = hal::rmt::Rmt::new(p.RMT, rmt_clk_freq).unwrap();
//!         let rmt_channel = rmt.channel0;
//!
//!         // Create RMT buffer
//!         const CHANNELS_PER_LED: usize = <Ws2812Led as ClocklessLed>::LED_CHANNELS.channel_count(); // Usually 3 (RGB)
//!         let rmt_buffer = create_rmt_buffer!(CHANNELS_PER_LED);
//!
//!         Ws2812Rmt::new(rmt_channel, data_pin, rmt_buffer)
//!     };
//!
//!     // Build the Blinky controller
//!     let mut control = ControlBuilder::new_1d()
//!         .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!         .with_pattern::<Rainbow>(RainbowParams {
//!             ..Default::default()
//!         })
//!         .with_driver(ws2812_driver)
//!         .build();
//!
//!     control.set_brightness(0.2); // Set initial brightness (0.0 to 1.0)
//!
//!     loop {
//!         let elapsed_in_ms = elapsed().as_millis();
//!         control.tick(elapsed_in_ms).unwrap();
//!
//!         // Optional: Add a delay to control the update rate and reduce CPU usage.
//!         // Without an explicit delay, the loop will run as fast as possible.
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
//! [gledopto]: https://docs.rs/gledopto/0.10/gledopto

pub mod rmt;
pub mod time;
pub(crate) mod util;

use crate::rmt::ClocklessRmtDriver;
use blinksy::drivers::sk6812::Sk6812Led;
use blinksy::drivers::ws2812::Ws2812Led;

pub use crate::rmt::ClocklessRmt;

/// WS2812 LED driver using the ESP32 RMT peripheral.
///
/// This driver provides efficient, hardware-accelerated control of WS2812 LEDs.
///
/// # Type Parameters
///
/// * `Tx` - RMT transmit channel
/// * `BUFFER_SIZE` - Size of the RMT buffer
///
/// [`DriverMode`]: esp_hal::DriverMode
pub type Ws2812Rmt<const RMT_BUFFER_SIZE: usize, Tx> =
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Ws2812Led, Tx>;

/// SK6812 LED driver using the ESP32 RMT peripheral.
///
/// This driver provides efficient, hardware-accelerated control of SK6812 LEDs.
///
/// # Type Parameters
///
/// * `Tx` - RMT transmit channel
/// * `BUFFER_SIZE` - Size of the RMT buffer
///
/// [`DriverMode`]: esp_hal::DriverMode
pub type Sk6812Rmt<const RMT_BUFFER_SIZE: usize, Tx> =
    ClocklessRmtDriver<RMT_BUFFER_SIZE, Sk6812Led, Tx>;
