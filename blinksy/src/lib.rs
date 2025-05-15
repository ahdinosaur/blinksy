#![no_std]

//! # Blinksy
//!
//! Blinksy is a no-std, no-alloc LED control library designed for 1D, 2D, and 3D (audio-reactive)
//! LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).
//!
//! ## How Blinksy works
//!
//! - Define your LED [`layout`] in 1D, 2D, or 3D space
//! - Create your visual [`pattern`] (effect), or choose from our built-in [`patterns`] library
//!   - The pattern will compute colors for each LED based on its position
//! - Setup a [`driver`] to send each frame of colors to your LEDs, using our built-in [`drivers`] library.
//!
//! ## Features
//!
//! - **No-std, No-alloc:** Designed to run on embedded targets.
//! - **Layout Abstraction:** Define 1D, 2D, or 3D LED positions with shapes (grids, lines, arcs, points, etc).
//! - **Pattern (Effect) Library:**
//!   - **Rainbow**
//!   - **Noise**
//!   - [Make an issue](https://github.com/ahdinosaur/blinksy/issues) if you want help to port a pattern from FastLED / WLED to Rust!
//! - **Multi‑Chipset Support:**
//!   - **APA102**
//!   - **WS2812B**
//!   - [Make an issue](https://github.com/ahdinosaur/blinksy/issues) if you want help to support a new chipset!
//! - **Board Support Packages**:
//!   - **Gledopto**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
//!   - (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards
//! - **Desktop Simulation:** Run a simulation of a layout and pattern on your computer to experiment with ideas.
//! - (TODO) **Audio-Reactive**: Easily integrate audio reactivity into visual patterns.
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - [`color`]: Color types and utilities
//! - [`control`]: Control system
//! - [`dimension`]: Dimension type-level markers
//! - [`driver`]: LED driver interface
//! - [`drivers`]: LED driver implementations
//! - [`layout`]: LED layout abstractions
//! - [`pattern`]: Pattern abstraction
//! - [`patterns`]: Pattern implementations
//! - [`time`]: Timing utilities
//!
//! ## Quick Start
//!
//! ### 1D Strip with Rainbow Pattern
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
//!     control.tick(/* current time in milliseconds */).unwrap();
//! }
//! ```
//!
//! ### 2D Grid with Noise Pattern
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     layout::{Shape2d, Vec2},
//!     layout2d,
//!     patterns::{noise_fns, Noise2d, NoiseParams},
//! };
//!
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         row_end: Vec2::new(-1., 1.),
//!         col_end: Vec2::new(1., -1.),
//!         row_pixel_count: 16,
//!         col_pixel_count: 16,
//!         serpentine: true,
//!     }]
//! );
//! let mut control = ControlBuilder::new_2d()
//!     .with_layout::<Layout>()
//!     .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
//!     .with_driver(/* insert your LED driver here */)
//!     .build();
//!
//! control.set_brightness(0.5);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).unwrap();
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
