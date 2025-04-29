#![no_std]

//! # ESP32 Blinksy Extensions
//!
//! This crate provides ESP32-specific extensions for the Blinksy LED control library.
//! It adapts the generic Blinksy abstractions to the specific hardware capabilities
//! of the ESP32, particularly focusing on efficient LED driving using the RMT
//! (Remote Control Module) peripheral.
//!
//! ## Features
//!
//! - ESP32 RMT-based driver for clockless (e.g. WS2812) LEDs
//! - Hardware-accelerated LED driving for improved performance
//! - Convenient API matching the core Blinksy interface
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

mod rmt;

/// RMT-based LED driver
pub mod driver {
    pub use crate::rmt::*;
}

/// Concrete driver implementations
pub mod drivers {
    use crate::rmt::ClocklessRmtDriver;
    use blinksy::drivers::Ws2812Led;

    /// WS2812 LED driver using the ESP32 RMT peripheral.
    ///
    /// This driver provides efficient, hardware-accelerated control of WS2812 LEDs.
    ///
    /// # Type Parameters
    ///
    /// * `Tx` - RMT transmit channel type
    /// * `BUFFER_SIZE` - Size of the RMT buffer
    pub type Ws2812Rmt<Tx, const BUFFER_SIZE: usize> =
        ClocklessRmtDriver<Ws2812Led, Tx, BUFFER_SIZE>;
}
