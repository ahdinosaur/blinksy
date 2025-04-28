//! # Rainbow Pattern
//!
//! This module provides a rainbow effect pattern that creates smooth color transitions
//! across the LED layout. The colors flow through the full HSV spectrum, creating a
//! classic rainbow visual.
//!
//! ## Features
//!
//! - Smooth transitions through the full color spectrum
//! - Configurable animation speed
//! - Adjustable spatial density (how compressed the rainbow appears)
//! - Works with both 1D and 2D layouts
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{
//!     ControlBuilder,
//!     layout1d,
//!     patterns::{Rainbow, RainbowParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(MyStrip, 60);
//!
//! // Create a Rainbow pattern with custom parameters
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<MyStrip>()
//!     .with_pattern::<Rainbow>(RainbowParams {
//!         time_scalar: 0.1,
//!         position_scalar: 1.0,
//!         brightness: 1.0,
//!     })
//!     .with_driver(/* your driver */)
//!     .build();
//! ```

use palette::Hsv;

use crate::{
    dimension::{Dim1d, Dim2d},
    layout::{Layout1d, Layout2d},
    pattern::Pattern,
};

/// Configuration parameters for the Rainbow pattern.
#[derive(Debug)]
pub struct RainbowParams {
    /// Controls the speed of the animation (higher = faster)
    pub time_scalar: f32,

    /// Controls the spatial density of the rainbow (higher = more compressed)
    pub position_scalar: f32,

    /// Base brightness before global brightness scaling
    pub brightness: f32,
}

impl Default for RainbowParams {
    fn default() -> Self {
        Self {
            time_scalar: 0.1,
            position_scalar: 1.,
            brightness: 1.,
        }
    }
}

/// Rainbow pattern implementation.
///
/// Creates a smooth transition through the full HSV spectrum across the LED layout.
#[derive(Debug)]
pub struct Rainbow {
    /// Configuration parameters
    params: RainbowParams,
}

impl<Layout> Pattern<Dim1d, Layout> for Rainbow
where
    Layout: Layout1d,
{
    type Params = RainbowParams;
    type Color = Hsv;

    /// Creates a new Rainbow pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 1D layout.
    ///
    /// The rainbow pattern creates a smooth transition of hues across the layout,
    /// which shifts over time to create a flowing effect.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let RainbowParams {
            time_scalar,
            position_scalar,
            brightness,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;
        let step = (1. / Layout::PIXEL_COUNT as f32) * 360. * position_scalar;

        (0..Layout::PIXEL_COUNT).map(move |index| {
            let hue = index as f32 * step + noise_time;
            let saturation = 1.;
            Hsv::new_srgb(hue, saturation, *brightness)
        })
    }
}

impl<Layout> Pattern<Dim2d, Layout> for Rainbow
where
    Layout: Layout2d,
{
    type Params = RainbowParams;
    type Color = Hsv;

    /// Creates a new Rainbow pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 2D layout.
    ///
    /// In 2D, the rainbow pattern uses the x-coordinate to determine hue,
    /// creating bands of color that move across the layout over time.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let RainbowParams {
            time_scalar,
            position_scalar,
            brightness,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;
        let step = (1. / Layout::PIXEL_COUNT as f32) * 360. * position_scalar;

        Layout::points().map(move |point| {
            let hue = point.x + noise_time * step;
            let saturation = 1.;
            Hsv::new_srgb(hue, saturation, *brightness)
        })
    }
}
