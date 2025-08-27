//! # LED Layouts
//!
//! A layout defines the physical or logical positions of the LEDs in your setup, as
//! arrangements in 1D, 2D, and 3D space.
//!
//! - For 1D, use [`layout1d!`] to define a type that implements [`Layout1d`]
//! - For 2D, use [`layout2d!`] to define a type that implements [`Layout2d`]
//! - For 3D, use [`layout3d!`] to define a type that implements [`Layout3d`]
//!
//! The layout traits provide a `PIXEL_COUNT` constant, which is the number of LEDs, and a
//! `.points()`. method, which maps each LED pixel into a 1D, 2D, or 3D space between -1.0 and
//! 1.0.
//!
//! ## 1D Layouts
//!
//! For simple linear arrangements, use the [`layout1d!`] macro:
//!
//! ```rust
//! use blinksy::layout1d;
//!
//! // Define a strip with 60 LEDs
//! layout1d!(Layout, 60);
//! ```
//!
//! ## 2D Layouts
//!
//! For 2D layouts, use the [`layout2d!`] macro with one or more [`Shape2d`] definitions:
//!
//! ```rust
//! use blinksy::{layout2d, layout::Shape2d, layout::Vec2};
//!
//! // Define a 16x16 LED grid
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
//! ```
//!
//! ## 3D Layouts
//!
//! For 3D layouts, use the [`layout3d!`] macro with one or more [`Shape3d`] definitions.
//!
//! [`layout1d!`]: crate::layout1d!
//! [`layout2d!`]: crate::layout2d!
//! [`layout3d!`]: crate::layout3d!

use crate::markers::{Dim1d, Dim2d, Dim3d};

mod iterators;
mod layout1d;
mod layout2d;
mod layout3d;

pub use iterators::*;
pub use layout1d::*;
pub use layout2d::*;
pub use layout3d::*;

/// Trait for associating layout types with dimension markers.
///
/// This trait creates the relationship between a layout type and its dimensionality,
/// which helps enforce correct combinations at compile time.
pub trait LayoutForDim<Dim> {}

/// All types implementing Layout1d are compatible with Dim1d.
impl<T> LayoutForDim<Dim1d> for T where T: Layout1d {}

/// All types implementing Layout2d are compatible with Dim2d.
impl<T> LayoutForDim<Dim2d> for T where T: Layout2d {}

/// All types implementing Layout3d are compatible with Dim3d.
impl<T> LayoutForDim<Dim3d> for T where T: Layout3d {}
