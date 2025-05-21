//! # Time Utilities
//!
//! This module helps you with time on an ESP microcontroller.
//!
//! ## Example
//!
//! ```rust,no_run
//! use blinksy_esp::time::elapsed;
//!
//! loop {
//!     // Get the current time in milliseconds
//!     let elapsed_in_ms = elapsed().as_millis();
//!
//!     // Use this time to update your animations
//!     // control.tick(elapsed_in_ms);
//! }
//! ```

use esp_hal::time::{Duration, Instant};

/// Returns the elapsed time since system boot.
pub fn elapsed() -> Duration {
    Instant::now().duration_since_epoch()
}
