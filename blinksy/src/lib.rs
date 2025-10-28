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
//!   - The pattern will compute colors for each LED based on its position in space
//! - Setup a [`driver`] to send each frame of colors to your [`leds`]
//!
//! ## Features
//!
//! - **No-std, no-alloc**: Designed for embedded targets.
//! - **Spatial in 1D, 2D, or 3D**: Map out the shape of your LEDs in space.
//! - **Async support**: Supports blocking or asynchronous execution.
//! - **Full color support**: Supports modern and classic color spaces.
//! - **Global settings**: Control overall brightness and color correction.
//! - **Desktop simulation**: Simulate your LEDs on your desktop to play with ideas.
//! - **RGB+W support**: Supports RGB + White color channels
//!
//! ### LED Support
//!
//! #### [Clockless](crate::driver::clockless): One-wire (only data, no clock)
//!
//! - **[WS2812B]**: Affordable RGB LED, aka NeoPixel
//! - **[SK6812]**: RGBW LED
//!
//! #### [Clocked](crate::driver::clocked): Two-wire (data and clock)
//!
//! - **[APA102]**: High-FPS RGB LED, aka DotStar
//!
//! If you want help to support a new LED chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [WS2812B]: leds::Ws2812
//! [SK6812]: leds::Sk6812
//! [APA102]: leds::Apa102
//!
//! ### Pattern (Effect) Library:
//!
//! - **[Rainbow]**: A basic scrolling rainbow
//! - **[Noise]**: A flow through random noise functions
//!
//! If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [Rainbow]: patterns::rainbow
//! [Noise]: patterns::noise
//!
//! ### Microcontroller Family Support
//!
//! **Clocked LED support (e.g. APA102):**
//!
//! | Micro | HAL            | Blinksy     | Recommended Writer | Backup Writer
//! |-------|----------------|-------------|--------------------|------------------|
//! | ALL   | [embedded-hal] | [blinksy]   | [embedded_hal::spi::SpiBus] / [embedded_hal_async::spi::SpiBus] | [Delay][clocked-delay] |
//!
//! [embedded-hal]: embedded_hal
//! [blinksy]: crate
//! [clocked-delay]: crate::driver::clocked::ClockedDelay
//!
//! **Clockless LED support (e.g. WS2812):**
//!
//! | Micro          | HAL         | Blinksy       | Recommended Driver     | Backup Driver |
//! |----------------|-------------|---------------|------------------------|---------------|
//! | ALL            | [embedded-hal] | [blinksy]  | -                      | TODO [Spi #12][clockless-spi] |
//! | ESP32          | [esp-hal]   | [blinksy-esp] | [Rmt][rmt]             | - |
//! | RP (2040/2350) | [rp-hal]    | TODO          | TODO [#36][rp-issue]   | - |
//! | STM32          | [stm32-hal] | TODO          | TODO [#78][stm32-issue] | - |
//! | nRF            | [nrf-hal]   | TODO          | TODO [#77][nrf-issue]  | - |
//! | atsamd         | [atsamd]    | TODO          | TODO [#67][atsamd-issue] | - |
//! | AVR (Arduino)  | [avr-hal]   | TODO          | TODO [#79][avr-issue]  | - |
//! | CH32           | [ch32-hal]  | TODO          | TODO [#80][ch32-issue] | - |
//! | ???            | -           | -             | -                      | - |
//!
//! [clockless-spi]: https://github.com/ahdinosaur/blinksy/issues/12
//! [esp-hal]: https://docs.espressif.com/projects/rust/esp-hal/latest/
//! [blinksy-esp]: https://docs.rs/blinksy-esp/0.11/
//! [rmt]: https://docs.espressif.com/projects/rust/esp-hal/latest/
//! [rp-hal]: https://github.com/rp-rs/rp-hal/
//! [rp-issue]: https://github.com/ahdinosaur/blinksy/issues/36
//! [stm32-hal]: https://github.com/David-OConnor/stm32-hal
//! [stm32-issue]: https://github.com/ahdinosaur/blinksy/issues/78
//! [nrf-hal]: https://github.com/nrf-rs/nrf-hal
//! [nrf-issue]: https://github.com/ahdinosaur/blinksy/issues/77
//! [atsamd]: https://github.com/atsamd-rs/atsamd
//! [atsamd-issue]: https://github.com/ahdinosaur/blinksy/issues/67
//! [avr-hal]: https://github.com/Rahix/avr-hal
//! [avr-issue]: https://github.com/ahdinosaur/blinksy/issues/79
//! [ch32-hal]: https://github.com/ch32-rs/ch32-hal
//! [ch32-issue]: https://github.com/ahdinosaur/blinksy/issues/80
//!
//! If you want help to support a new microcontroller family, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! ### Board Support Packages
//!
//! - **[Gledopto]**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
//! - (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards
//!
//! If you want help to support a new target, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [Gledopto]: https://docs.rs/gledopto/0.11/gledopto
//!
//! ## Quick Start
//!
//! To quickstart a project, see:
//!
//! - [`blinksy-quickstart-1d-rope`][blinksy-quickstart-1d-rope]
//! - [`blinksy-quickstart-3d-grid`][blinksy-quickstart-3d-grid]
//!
//! To start using the library, see [control][control].
//!
//! [blinksy-quickstart-1d-rope]: https://github.com/ahdinosaur/blinksy-quickstart-1d-rope
//! [blinksy-quickstart-3d-grid]: https://github.com/ahdinosaur/blinksy-quickstart-3d-grid
//! [control]: https://docs.rs/blinksy/0.11/blinksy/control/index.html
//!
//! ### 1D Strip with Rainbow Pattern (Blocking)
//!
//! ```rust,ignore
//! # use blinksy::{ControlBuilder, layout::Layout1d, layout1d, patterns::rainbow::{Rainbow, RainbowParams}};
//! #
//! // Define a 1D layout with 60 LEDs
//! layout1d!(Layout, 60);
//!
//! let mut control = ControlBuilder::new_1d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Rainbow>(RainbowParams::default())
//!     .with_driver(/* Insert your LED driver here */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//!
//! control.set_brightness(0.5);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).unwrap();
//! }
//! ```
//!
//! ### 1D Strip with Rainbow Pattern (Async)
//!
//! ```rust,ignore
//! # use blinksy::{ControlBuilder, layout::Layout1d, layout1d, patterns::rainbow::{Rainbow, RainbowParams}};
//! #
//! // Define a 1D layout with 60 LEDs
//! layout1d!(Layout, 60);
//!
//! let mut control = ControlBuilder::new_1d_async()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Rainbow>(RainbowParams::default())
//!     .with_driver(/* Insert your LED driver here */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//!
//! control.set_brightness(0.5);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).await.unwrap();
//! }
//! ```
//!
//! ### 2D Grid with Noise Pattern (Blocking)
//!
//! ```rust,ignore
//! # use blinksy::{
//! #     ControlBuilder,
//! #     layout::{Layout2d, Shape2d, Vec2},
//! #     layout2d,
//! #     patterns::noise::{noise_fns, Noise2d, NoiseParams},
//! # };
//! #
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
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
//!     .with_driver(/* Insert your LED driver here */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//!
//! control.set_brightness(0.5);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).unwrap();
//! }
//! ```
//!
//! ### 3D Cube with Noise Pattern (Blocking)
//!
//! ```rust,ignore
//! # use blinksy::{
//! #     layout::{Layout3d, Shape3d, Vec3},
//! #     layout3d,
//! #     patterns::noise::{noise_fns, Noise3d, NoiseParams},
//! #     ControlBuilder,
//! # };
//! #
//! layout3d!(
//!     Layout,
//!     [
//!         // bottom face
//!         Shape3d::Grid {
//!             start: Vec3::new(1., -1., 1.),           // right bottom front
//!             horizontal_end: Vec3::new(-1., -1., 1.), // left bottom front
//!             vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // back face
//!         Shape3d::Grid {
//!             start: Vec3::new(-1., -1., -1.),         // left bottom back
//!             horizontal_end: Vec3::new(-1., 1., -1.), // left top back
//!             vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // right face
//!         Shape3d::Grid {
//!             start: Vec3::new(1., 1., -1.),         // right top back
//!             horizontal_end: Vec3::new(1., 1., 1.), // right top front
//!             vertical_end: Vec3::new(1., -1., -1.), // right bottom back
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // front face
//!         Shape3d::Grid {
//!             start: Vec3::new(-1., -1., 1.),         // left bottom front
//!             horizontal_end: Vec3::new(1., -1., 1.), // right bottom front
//!             vertical_end: Vec3::new(-1., 1., 1.),   // left top front
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // left face
//!         Shape3d::Grid {
////!             start: Vec3::new(-1., 1., -1.),           // left top back
//!             horizontal_end: Vec3::new(-1., -1., -1.), // left bottom back
//!             vertical_end: Vec3::new(-1., 1., 1.),     // left top front
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // top face
//!         Shape3d::Grid {
//!             start: Vec3::new(1., 1., 1.),           // right top front
//!             horizontal_end: Vec3::new(1., 1., -1.), // right top back
//!             vertical_end: Vec3::new(-1., 1., 1.),   // left top front
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         }
//!     ]
//! );
//!
//! let mut control = ControlBuilder::new_3d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams::default())
//!     .with_driver(/* Insert your LED driver here */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//!
//! control.set_brightness(0.2);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).unwrap();
//! }
//! ```
//!
//! ### 2D Grid with Noise Pattern (Blocking)
//!
//! ```rust,ignore
//! # use blinksy::{
//! #     ControlBuilder,
//! #     layout::{Layout2d, Shape2d, Vec2},
//! #     layout2d,
//! #     patterns::noise::{noise_fns, Noise2d, NoiseParams},
//! # };
//! #
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
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
//!     .with_driver(/* Insert your LED driver here */)
//!     .build();
//!
//! control.set_brightness(0.5);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).unwrap();
//! }
//! ```
//!
//! ### 3D Cube with Noise Pattern (Blocking)
//!
//! ```rust,ignore
//! # use blinksy::{
//! #     layout::{Layout3d, Shape3d, Vec3},
//! #     layout3d,
//! #     patterns::noise::{noise_fns, Noise3d, NoiseParams},
//! #     ControlBuilder,
//! # };
//! #
//! layout3d!(
//!     Layout,
//!     [
//!         // bottom face
//!         Shape3d::Grid {
//!             start: Vec3::new(1., -1., 1.),           // right bottom front
//!             horizontal_end: Vec3::new(-1., -1., 1.), // left bottom front
//!             vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // back face
//!         Shape3d::Grid {
//!             start: Vec3::new(-1., -1., -1.),         // left bottom back
//!             horizontal_end: Vec3::new(-1., 1., -1.), // left top back
//!             vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // right face
//!         Shape3d::Grid {
//!             start: Vec3::new(1., 1., -1.),         // right top back
//!             horizontal_end: Vec3::new(1., 1., 1.), // right top front
//!             vertical_end: Vec3::new(1., -1., -1.), // right bottom back
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // front face
//!         Shape3d::Grid {
//!             start: Vec3::new(-1., -1., 1.),         // left bottom front
//!             horizontal_end: Vec3::new(1., -1., 1.), // right bottom front
//!             vertical_end: Vec3::new(-1., 1., 1.),   // left top front
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // left face
//!         Shape3d::Grid {
//!             start: Vec3::new(-1., 1., -1.),           // left top back
//!             horizontal_end: Vec3::new(-1., -1., -1.), // left bottom back
//!             vertical_end: Vec3::new(-1., 1., 1.),     // left top front
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         },
//!         // top face
//!         Shape3d::Grid {
//!             start: Vec3::new(1., 1., 1.),           // right top front
//!             horizontal_end: Vec3::new(1., 1., -1.), // right top back
//!             vertical_end: Vec3::new(-1., 1., 1.),   // left top front
//!             horizontal_pixel_count: 16,
//!             vertical_pixel_count: 16,
//!             serpentine: true,
//!         }
//!     ]
//! );
//!
//! let mut control = ControlBuilder::new_3d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams::default())
//!     .with_driver(/* Insert your LED driver here */)
//!     .build();
//!
//! control.set_brightness(0.2);
//!
//! loop {
//!     control.tick(/* current time in milliseconds */).unwrap();
//! }
//! ```
//!

pub mod color;
pub mod control;
pub mod driver;
pub mod layout;
pub mod leds;
pub mod markers;
pub mod pattern;
pub mod patterns;
pub mod time;
pub mod util;

pub use self::control::*;
