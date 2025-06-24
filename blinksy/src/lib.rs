#![no_std]

//! # Blinksy
//!
//! Blinksy is a no-std, no-alloc LED control library designed for 1D, 2D, and 3D
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
//! - **No-std, no-alloc**: Designed for embedded targets.
//! - **Spatial in 1D, 2D, or 3D**: Map out the shape of your LEDs in space.
//! - **Full color support**: Supports modern and classic color spaces.
//! - **Global settings**: Control overall brightness and color correction.
//! - **Desktop simulation**: Simulate your LEDs on your desktop to play with ideas.
//! - **RGB+W support**: Supports RGB + White color channels
//!
//! ### Multi‑Chipset Support
//!
//! - [clockless](crate::driver::clockless): One-wire (only data, no clock)
//!   - **[WS2812B]**: Affordable RGB LED, aka NeoPixel
//! - [clocked](crate::driver::clocked): Two-wire (data and clock)
//!   - **[APA102]**: High-FPS RGB LED, aka DotStar
//!
//! If you want help to support a new chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [APA102]: drivers::apa102
//! [WS2812B]: drivers::ws2812
//!
//! ### Pattern (Effect) Library:
//!
//! - **[Rainbow]**: A basic scrolling rainbow
//! - **[Noise]**: A flow through random noise functions.
//!
//! If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [Rainbow]: patterns::rainbow
//! [Noise]: patterns::noise
//!
//! ### Board Support Packages
//!
//! - **[Gledopto]**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
//! - (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards
//!
//! If you want help to support a new target, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [Gledopto]: https://docs.rs/gledopto/0.4/gledopto
//!
//! ## Quick Start
//!
//! To quickstart a project, see [`blinksy-quickstart-gledopto`][blinksy-quickstart-gledopto]
//!
//! To start using the library, see [control].
//!
//! [blinksy-quickstart-gledopto]: https://github.com/ahdinosaur/blinksy-quickstart-gledopto
//!
//! ### 1D Strip with Rainbow Pattern
//!
//! ```rust,ignore
//! use blinksy::{ControlBuilder, layout1d, patterns::rainbow::{Rainbow, RainbowParams}};
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
//!     patterns::noise::{noise_fns, Noise2d, NoiseParams},
//! };
//!
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         horizontal_end: Vec2::new(1., -1.),
//!         vertical_end: Vec2::new(-1., 1.),
//!         horizontal_pixel_count: 16,
//!         vertical_pixel_count: 16,
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
pub mod driver;
pub mod drivers;
pub mod layout;
pub mod markers;
pub mod pattern;
pub mod patterns;
pub mod time;
mod util;

pub use self::control::*;
