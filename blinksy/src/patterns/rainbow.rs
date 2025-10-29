//! # Rainbow Pattern
//!
//! The rainbow pattern creates smooth color transitions across the LED layout.
//! The colors flow through the full [`Hsv`] spectrum, creating a classic rainbow
//! visual.
//!
//! ## Example
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::rainbow::{Rainbow, RainbowParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(Layout, 60);
//!
//! // Create a Rainbow pattern with custom parameters
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Rainbow>(RainbowParams {
//!         time_scalar: 0.1,
//!         position_scalar: 1.0,
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//! ```

use crate::{
    color::{Hsv, HsvHueRainbow},
    layout::{Layout1d, Layout2d, Layout3d},
    markers::{Dim1d, Dim2d, Dim3d},
    pattern::Pattern,
};

/// Configuration parameters for the Rainbow pattern.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Rainbow {
    /// Configuration parameters
    params: RainbowParams,
}

impl<Layout> Pattern<Dim1d, Layout> for Rainbow
where
    Layout: Layout1d,
{
    type Params = RainbowParams;
    type Color = Hsv<HsvHueRainbow>;

    /// Creates a new Rainbow pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Sets an existing Rainbow pattern with the specified parameters.
    fn set(&mut self, params: Self::Params) {
        self.params = params;
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
            Self::Color::new(hue, saturation, value)
        })
    }
}

impl<Layout> Pattern<Dim2d, Layout> for Rainbow
where
    Layout: Layout2d,
{
    type Params = RainbowParams;
    type Color = Hsv<HsvHueRainbow>;

    /// Creates a new Rainbow pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Sets an existing Rainbow pattern with the specified parameters.
    fn set(&mut self, params: Self::Params) {
        self.params = params;
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
            let hue = (point.x + point.y) * step + time;
            let saturation = 1.;
            let value = 1.;
            Self::Color::new(hue, saturation, value)
        })
    }
}

impl<Layout> Pattern<Dim3d, Layout> for Rainbow
where
    Layout: Layout3d,
{
    type Params = RainbowParams;
    type Color = Hsv<HsvHueRainbow>;

    /// Creates a new Rainbow pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Sets an existing Rainbow pattern with the specified parameters.
    fn set(&mut self, params: Self::Params) {
        self.params = params;
    }

    /// Generates colors for a 3D layout.
    ///
    /// In 3D, the rainbow pattern uses the x-coordinate to determine hue,
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
            let hue = (point.x + point.y + point.z) * step + time;
            let saturation = 1.;
            let value = 1.;
            Self::Color::new(hue, saturation, value)
        })
    }
}
