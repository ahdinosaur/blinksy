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
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     layout1d,
//!     patterns::{Rainbow, RainbowParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(Layout, 60);
//!
//! // Create a Rainbow pattern with custom parameters
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<Layout>()
//!     .with_pattern::<Rainbow>(RainbowParams {
//!         time_scalar: 0.1,
//!         position_scalar: 1.0,
//!     })
//!     .with_driver(/* your driver */)
//!     .build();
//! ```

use crate::{
    color::Okhsv,
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
}

impl Default for RainbowParams {
    fn default() -> Self {
        const MILLISECONDS_PER_SECOND: f32 = 1e3;
        Self {
            time_scalar: 0.3 / MILLISECONDS_PER_SECOND,
            position_scalar: 1.,
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
    type Color = Okhsv;

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
        } = params;

        let time = time_in_ms as f32 * time_scalar;
        let step = 0.5 * position_scalar;

        Layout::points().map(move |x| {
            let hue = x * step + time;
            let saturation = 1.;
            let value = 1.;
            Okhsv::new(hue, saturation, value)
        })
    }
}

impl<Layout> Pattern<Dim2d, Layout> for Rainbow
where
    Layout: Layout2d,
{
    type Params = RainbowParams;
    type Color = Okhsv;

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
        } = params;

        let time = time_in_ms as f32 * time_scalar;
        let step = 0.5 * position_scalar;

        Layout::points().map(move |point| {
            let hue = point.x * step + time;
            let saturation = 1.;
            let value = 1.;
            Okhsv::new(hue, saturation, value)
        })
    }
}
