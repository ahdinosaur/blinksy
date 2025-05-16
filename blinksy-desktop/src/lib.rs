//! # Blinksy Desktop Simulation
//!
//! This crate provides a desktop simulation environment for the Blinksy LED control library.
//! It allows you to visualize LED layouts and patterns in a 3D graphical window,
//! making development and testing possible without physical LED hardware.
//!
//! ## Features
//!
//! - 3D visualization of LED layouts
//! - Real-time display of patterns and animations
//! - Interactive camera controls for viewing from different angles
//! - Drop-in replacement for physical LED drivers
//! - Compatible with all Blinksy layouts and patterns
//!
//! ## Usage
//!
//! ```rust,no_run
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Shape2d, Vec2},
//!     patterns::rainbow::{Rainbow, RainbowParams}
//! };
//! use blinksy_desktop::{drivers::Desktop, time::elapsed_in_ms};
//!
//! // Define your layout
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         row_end: Vec2::new(1., -1.),
//!         col_end: Vec2::new(-1., 1.),
//!         row_pixel_count: 16,
//!         col_pixel_count: 16,
//!         serpentine: true,
//!     }]
//! );
//!
//! // Create a control using the Desktop driver instead of physical hardware
//! let mut control = ControlBuilder::new_2d()
//!     .with_layout::<Layout>()
//!     .with_pattern::<Rainbow>(RainbowParams::default())
//!     .with_driver(Desktop::new_2d::<Layout>())
//!     .build();
//!
//! // Run your normal animation loop
//! loop {
//!     control.tick(elapsed_in_ms()).unwrap();
//!     std::thread::sleep(std::time::Duration::from_millis(16));
//! }
//! ```
//!
//! ## Modules
//!
//! - [`drivers`]: Desktop LED simulation
//! - [`time`]: Time utilities

mod driver;

/// Time utilities
pub mod time;

/// Desktop LED simulation
pub mod drivers {
    pub use crate::driver::*;
}
