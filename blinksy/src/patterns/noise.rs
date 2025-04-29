//! # Noise Patterns
//!
//! This module provides noise-based visual effects that use noise functions to create
//! organic, flowing patterns. Noise patterns are useful for creating fire, clouds, water,
//! and other natural-looking animations.
//!
//! ## Features
//!
//! - Multiple noise function options (Perlin, Simplex, OpenSimplex)
//! - Configurable animation speed and scale
//! - 1D and 2D variants for different layout types
//! - Creates flowing, organic patterns
//!
//! ## Example
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Shape2d, Vec2},
//!     patterns::{Noise2d, noise_fns, NoiseParams}
//! };
//!
//! // Define a 2D layout
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         row_end: Vec2::new(1., -1.),
//!         col_end: Vec2::new(-1., 1.),
//!         row_pixel_count: 16,
//!         col_pixel_count: 16,
//!         serpentine: true,
//!     }]
//! );
//!
//! // Create a 2D noise pattern with Perlin noise
//! let control = ControlBuilder::new_2d()
//!     .with_layout::<Layout>()
//!     .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
//!         time_scalar: 0.001,
//!         position_scalar: 0.1,
//!     })
//!     .with_driver(/* your driver */)
//!     .build();
//! ```

use noise::NoiseFn;
use palette::Hsv;

use crate::{
    dimension::{Dim1d, Dim2d},
    layout::{Layout1d, Layout2d},
    pattern::Pattern,
};

/// Re-exports of noise functions from the noise crate.
pub mod noise_fns {
    pub use noise::{OpenSimplex, Perlin, Simplex};
}

/// Configuration parameters for noise patterns.
#[derive(Debug)]
pub struct NoiseParams {
    /// Controls the speed of animation (higher = faster)
    pub time_scalar: f64,

    /// Controls the spatial scale of the noise (higher = more compressed)
    pub position_scalar: f64,
}

impl Default for NoiseParams {
    fn default() -> Self {
        const MILLISECONDS_PER_SECOND: f64 = 1e3;
        Self {
            time_scalar: 0.75 / MILLISECONDS_PER_SECOND,
            position_scalar: 0.5,
        }
    }
}

/// One-dimensional noise pattern implementation.
///
/// Creates flowing patterns based on a 2D noise function, using
/// time and the 1D position for the input coordinates.
#[derive(Debug)]
pub struct Noise1d<Noise>
where
    Noise: NoiseFn<f64, 2>,
{
    /// The noise function implementation
    noise: Noise,

    /// Configuration parameters
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim1d, Layout> for Noise1d<Noise>
where
    Layout: Layout1d,
    Noise: NoiseFn<f64, 2> + Default,
{
    type Params = NoiseParams;
    type Color = Hsv;

    /// Creates a new Noise1d pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self {
            noise: Noise::default(),
            params,
        }
    }

    /// Generates colors for a 1D layout using noise.
    ///
    /// The pattern uses the LED position and time as inputs to a 2D noise function,
    /// mapping the noise value to a hue in the HSV color space.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { noise, params } = self;
        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        let noise_time = time_in_ms as f64 * time_scalar;

        (0..Layout::PIXEL_COUNT).map(move |index| {
            let noise = noise.get([position_scalar * index as f64, noise_time]);
            let hue = 360. * noise as f32;
            let saturation = 1.;
            let value = 1.;
            Hsv::new_srgb(hue, saturation, value)
        })
    }
}

/// Two-dimensional noise pattern implementation.
///
/// Creates flowing patterns based on a 3D noise function, using
/// time and the 2D position for the input coordinates.
#[derive(Debug)]
pub struct Noise2d<Noise>
where
    Noise: NoiseFn<f64, 3>,
{
    /// The noise function implementation
    noise: Noise,

    /// Configuration parameters
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim2d, Layout> for Noise2d<Noise>
where
    Layout: Layout2d,
    Noise: NoiseFn<f64, 3> + Default,
{
    type Params = NoiseParams;
    type Color = Hsv;

    /// Creates a new Noise2d pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self {
            noise: Noise::default(),
            params,
        }
    }

    /// Generates colors for a 2D layout using noise.
    ///
    /// The pattern uses the LED x,y position and time as inputs to a 3D noise function,
    /// mapping the noise value to a hue in the HSV color space.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { noise, params } = self;
        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        let noise_time = time_in_ms as f64 * time_scalar;

        Layout::points().map(move |point| {
            let noise = noise.get([
                position_scalar * point.x as f64,
                position_scalar * point.y as f64,
                noise_time,
            ]);
            let hue = 360. * noise as f32;
            let saturation = 1.;
            let value = 1.;
            Hsv::new_srgb(hue, saturation, value)
        })
    }
}
