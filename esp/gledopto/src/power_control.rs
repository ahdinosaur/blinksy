//! # Power control Handling Module
//!
//! This module provides functionality for controlling output power (V+ out) via
//! on board MOSFET connected to GPIO18
//!
//! ## Features
//!
//! - Turn output power on and off
//!
//! ## Example
//!
//! ```rust
//! use gledopto::{board, power_control, main};
//!
//! #[main]
//! fn main() -> ! {
//!     let p = board!();
//!     let mut power_control = power_control!(p);
//!
//!     power_control.turn_on();
//! }
//! ```

use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    peripherals::GPIO18,
};

pub struct PowerControl<'a> {
    control_pin: Output<'a>,
}

impl<'a> PowerControl<'a> {
    /// Creates a new power control instance.
    ///
    /// # Arguments
    ///
    /// - `pin` - The GPIO pin connected to the power MOSFET (GPIO18)
    ///
    /// # Returns
    ///
    /// A configured PowerControl instance
    pub fn new(pin: GPIO18<'a>) -> Self {
        Self {
            control_pin: Output::new(pin, Level::Low, OutputConfig::default()),
        }
    }

    pub fn turn_on(&mut self) {
        self.control_pin.set_high();
    }

    pub fn turn_off(&mut self) {
        self.control_pin.set_low();
    }
}
