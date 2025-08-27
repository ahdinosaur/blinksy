//! # Button Handling Module
//!
//! This module provides functionality for handling button inputs on the Gledopto controller.
//! It uses the button-driver crate to implement debouncing, click detection, and hold time
//! measurement.
//!
//! ## Features
//!
//! - Debounced button input
//! - Click, double-click, and triple-click detection
//! - Hold time measurement
//! - Combined click-and-hold detection
//!
//! ## Example
//!
//! ```rust
//! use gledopto::{board, function_button, main};
//!
//! #[main]
//! fn main() -> ! {
//!     let p = board!();
//!     let mut button = function_button!(p);
//!
//!     loop {
//!         button.tick();
//!
//!         #[allow(clippy::collapsible_else_if)]
//!         if let Some(dur) = button.held_time() {
//!             info!("Total holding time {:?}", dur);
//!
//!             if button.is_clicked() {
//!                 info!("Clicked + held");
//!             } else if button.is_double_clicked() {
//!                 info!("Double clicked + held");
//!             } else if button.holds() == 2 && button.clicks() > 0 {
//!                 info!("Held twice with {} clicks", button.clicks());
//!             } else if button.holds() == 2 {
//!                 info!("Held twice");
//!             }
//!         } else {
//!             if button.is_clicked() {
//!                 info!("Click");
//!             } else if button.is_double_clicked() {
//!                 info!("Double click");
//!             } else if button.is_triple_clicked() {
//!                 info!("Triple click");
//!             } else if let Some(dur) = button.current_holding_time() {
//!                 info!("Held for {:?}", dur);
//!             }
//!         }
//!
//!         button.reset();
//!     }
//! }
//! ```

use button_driver::{Button, ButtonConfig, InstantProvider, Mode};
use core::ops::{Deref, DerefMut, Sub};
use esp_hal::{
    gpio::{Input, InputConfig, Pull},
    peripherals::GPIO0,
    time::{Duration, Instant},
};

/// Function button implementation for the Gledopto controller.
///
/// This struct wraps the button-driver Button type with ESP32-specific configuration
/// for the function button on the Gledopto board (connected to GPIO0).
pub struct FunctionButton<'a>(Button<Input<'a>, ButtonInstant, Duration>);

impl<'a> FunctionButton<'a> {
    /// Creates a new function button instance.
    ///
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin connected to the button (GPIO0)
    ///
    /// # Returns
    ///
    /// A configured FunctionButton instance
<<<<<<< HEAD
    pub fn new(pin: GPIO0<'static>) -> Self {
=======
    pub fn new(pin: GPIO0<'a>) -> Self {
>>>>>>> 42ca8b6cebc6895423b108608123dca1acb888d1
        let input = Input::new(pin, InputConfig::default().with_pull(Pull::Up));

        let button_config = ButtonConfig::<Duration> {
            mode: Mode::PullUp,
            debounce: Duration::from_micros(900),
            release: Duration::from_millis(150),
            hold: Duration::from_millis(500),
        };

        let button = Button::new(input, button_config);
        Self(button)
    }
}

impl<'a> Deref for FunctionButton<'a> {
    type Target = Button<Input<'a>, ButtonInstant, Duration>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FunctionButton<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// ESP32-specific implementation of button timing.
///
/// This wrapper provides the necessary time-related functionality for the button driver.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ButtonInstant(Instant);

impl Sub for ButtonInstant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl InstantProvider<Duration> for ButtonInstant {
    fn now() -> Self {
        Self(Instant::now())
    }
}
