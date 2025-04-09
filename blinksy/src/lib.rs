#![no_std]

//! # Blinksy Core Library
//!
//! Blinksy is the core no-std, no-alloc (audio-reactive) LED control library. It provides the building
//! blocks for defining LED layouts (1D, 2D, 3D), creating visual patterns, driving LED chipsets, and handling
//! timing for animations.
//!
//! The crate is organized into several modules:
//!
//! - **color:** Types and conversions for color representations using the [palette] crate.
//!
//! - **control:** The `Control` structure and `ControlBuilder` facilitate orchestrating LED updates.
//!
//! - **dimension:** Dimension markers and traits used to abstract over 1D and 2D layouts.
//!
//! - **driver:** Abstractions for LED drivers, including clocked and clockless methods.
//!
//! - **drivers:** Concrete implementations for LED chipsets (e.g., APA102, WS2812B).
//!
//! - **layout:** Traits and macros for defining LED layouts such as lines, grids, arcs, etc.
//!
//! - **pattern:** The `Pattern` trait that governs the behavior of visual effects.
//!
//! - **patterns:** Built-in visual effects such as Rainbow and Noise.
//!
//! - **time:** Timing utilities to facilitate animation updates.
//!
//! ## Example Usage
//!
//! Here is a brief example that demonstrates defining a 1D LED layout and driving it with a Rainbow pattern:
//!
//! ```rust
//! use blinksy::{ControlBuilder, layout1d, patterns::{Rainbow, RainbowParams}};
//!
//! // Define a 1D layout with 60 pixels
//! layout1d!(MyStrip, 60);
//!
//! // Build the control for your LED strip
//! let mut control = ControlBuilder::new_1d()
//!     .with_layout::<MyStrip>()
//!     .with_pattern::<Rainbow>(RainbowParams {
//!         position_scalar: 1.0,
//!         ..Default::default()
//!     })
//!     .with_driver(/* insert your LED driver here */)
//!     .build();
//!
//! // Set brightness and update LED state in your main loop
//! control.set_brightness(0.2);
//! loop {
//!     let time = /* obtain current time in ms */;
//!     control.tick(time).unwrap();
//! }
//! ```
//!
//! For more detailed examples, please refer to the example files in the `gledopto/examples` directory.
//!
//! [palette]: https://crates.io/crates/palette

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
