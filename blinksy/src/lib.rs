//! # Blinksy
//!
//! Blinksy is a no-std, no-alloc LED control library designed for 1D, 2D, and 3D (audio-reactive)
//! LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).
//!
//! - Define LED layouts in 1D, 2D, or 3D space
//! - Choose visual patterns (effects)
//! - Compute colors for each LED based on its position
//! - Drive various LED chipsets with the calculated colors
//!
//! ## Core Features
//!
//! - **No-std, No-alloc:** Designed to run on embedded targets with minimal resources
//! - **Layout Abstraction:** Define 1D, 2D, or 3D LED positions with shapes (grids, lines, arcs, points)
//! - **Pattern Library:** Visual effects like Rainbow, Noise, and more
//! - **Multi-Chipset Support:** Works with APA102, WS2812B, and others
//! - **Board Support Packages:** Ready-to-use configurations for popular LED controllers
//! - **Desktop Simulation:** Run a simulation of a layout and pattern on your computer to experiment with ideas.
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - [`color`]: Color representations and utilities
//! - [`control`]: Orchestration of LED updates
//! - [`dimension`]: Type-level markers for dimensionality
//! - [`driver`]: LED driver abstractions
//! - [`drivers`]: Concrete implementations for specific LED chipsets
//! - [`layout`]: Definitions for LED arrangements
//! - [`pattern`]: Pattern trait definition
//! - [`patterns`]: Collection of visual effects
//! - [`time`]: Timing utilities
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use blinksy::{ControlBuilder, layout1d, patterns::{Rainbow, RainbowParams}};
//!
//! // Define a 1D layout with 60 LEDs
//! layout1d!(Layout, 60);
//!
//! let mut control = ControlBuilder::new_1d()
//!     .with_layout::<Layout>()
//!     .with_pattern::<Rainbow>(RainbowParams::default())
//!     .with_driver(/* insert your LED driver here */)
//!     .build();
//!
//! control.set_brightness(0.5);
//!
//! loop {
//!     let time = /* obtain current time in milliseconds */;
//!     control.tick(time).unwrap();
//! }
//! ```

pub mod color;
pub mod control;
pub mod dimension;
pub mod driver;
pub mod drivers;
pub mod layout;
pub mod pattern;
pub mod patterns;
pub mod time;

mod util;

pub use self::control::*;
