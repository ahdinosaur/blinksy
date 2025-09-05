//! # Blinksy Desktop Simulation
//!
//! This crate provides a desktop simulation environment for the Blinksy LED control library.
//! It allows you to visualize LED layouts and patterns in a 3D graphical window,
//! making development and testing possible without physical LED hardware.
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
//! use blinksy_desktop::{driver::Desktop, time::elapsed_in_ms};
//!
//! // Define your layout
//! layout2d!(
//!     PanelLayout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         horizontal_end: Vec2::new(1., -1.),
//!         vertical_end: Vec2::new(-1., 1.),
//!         horizontal_pixel_count: 16,
//!         vertical_pixel_count: 16,
//!         serpentine: true,
//!     }]
//! );
//!
//! // Create the Desktop simulator
//! Desktop::new_2d::<PanelLayout>().start(|driver| {
//!     // Create a control using the desktop driver instead of physical hardware
//!     let mut control = ControlBuilder::new_2d()
//!         .with_layout::<PanelLayout>()
//!         .with_pattern::<Rainbow>(RainbowParams::default())
//!         .with_driver(driver)
//!         .build();
//!
//!     // Run your normal animation loop
//!     loop {
//!         control.tick(elapsed_in_ms()).unwrap();
//!
//!         // Sleep on every frame (16 ms per frame ~= 60 frames per second)
//!         std::thread::sleep(std::time::Duration::from_millis(16));
//!     }
//! });
//! ```

/// Desktop LED simulation
pub mod driver;

/// Time utilities
pub mod time;
