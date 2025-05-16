//! # Time Utilities
//!
//! This module helps you with time in the desktop simulation environment.
//!
//! ## Example
//!
//! ```rust,no_run
//! use blinksy_desktop::time::elapsed_in_ms;
//!
//! loop {
//!     // Get the current time in milliseconds
//!     let current_time = elapsed_in_ms();
//!
//!     // Use this time to update your animations
//!     // control.tick(current_time);
//!
//!     // Sleep to limit frame rate
//!     std::thread::sleep(std::time::Duration::from_millis(16));
//! }
//! ```

use std::sync::OnceLock;
use std::time::Instant;

static START_TIME: OnceLock<Instant> = OnceLock::new();

/// Returns the number of milliseconds elapsed since the program started. This is useful to pass
/// into `control.tick`.
///
/// It initializes a static timer on first call and then measures elapsed time
/// from that point.
///
/// # Returns
///
/// The number of milliseconds since program start or first call to this function
pub fn elapsed_in_ms() -> u64 {
    let start = START_TIME.get_or_init(Instant::now);
    start.elapsed().as_millis() as u64
}
