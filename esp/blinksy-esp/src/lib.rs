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
//! ## Usage
//!
//! This crate is typically used via the gledopto board support package:
//!
//! ```rust
//! use gledopto::{board, ws2812, main};
//! use blinksy::ControlBuilder;
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

pub mod rmt;

use crate::rmt::ClocklessRmtDriver;
use blinksy::drivers::ws2812::Ws2812Led;

/// WS2812 LED driver using the ESP32 RMT peripheral.
///
/// This driver provides efficient, hardware-accelerated control of WS2812 LEDs.
///
/// # Type Parameters
///
/// * `Tx` - RMT transmit channel type
/// * `BUFFER_SIZE` - Size of the RMT buffer
pub type Ws2812Rmt<Tx, const BUFFER_SIZE: usize> = ClocklessRmtDriver<Ws2812Led, Tx, BUFFER_SIZE>;
