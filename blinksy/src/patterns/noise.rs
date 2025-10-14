//! # Noise Patterns
//!
//! The noise pattern creates flowing animations based on a noise function.
//!
//! # What is a noise function?
//!
//! A noise function is given a position in 1d, 2d, 3d, or 4d space and returns
//! a random value between -1.0 and 1.0, where values between nearbly positions are
//! smoothly interpolated.
//!
//! For example, a common use of noise functions is to procedurally generate terrain.
//! You could give a 2d noise function an (x, y) position and use the resulting value
//! as an elevation.
//!
//! In our case, we will use noise functions to generate `hue` and `value` for [Okhsv]
//! colors. To animate through time, rather than adding time to our position, we will
//! input the time to the noise function as an additonal dimension. So a 1d layout will
//! use a 2d noise function, a 2d layout a 3d noise function, and so on.
//!
//! This pattern is the same concept as what you see on [mikey.nz](https://mikey.nz/).
//!
//! ## Example
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Layout2d, Shape2d, Vec2},
//!     patterns::noise::{Noise2d, noise_fns, NoiseParams}
//! };
//!
//! // Define a 2D layout
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
//!
//! // Create a 2D noise pattern with Perlin noise
//! let control = ControlBuilder::new_2d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
//!         time_scalar: 0.001,
//!         position_scalar: 0.1,
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//! ```
//!
//! [`Okhsv`]: crate::color::Okhsv
//! [mikey.nz]: https://mikey.nz

use noise_functions::{modifiers::Seeded, Noise as NoiseTrait, Sample};

use crate::{
    color::Okhsv,
    layout::{Layout1d, Layout2d, Layout3d},
    markers::{Dim1d, Dim2d, Dim3d},
    pattern::Pattern,
};

/// Re-exports of noise functions from the noise crate.
pub mod noise_fns {
    pub use noise_functions::{OpenSimplex2, Perlin, Simplex};
}

/// Configuration parameters for noise patterns.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NoiseParams {
    /// Controls the speed of animation (higher = faster)
    pub time_scalar: f32,
    /// Controls the spatial scale of the noise (higher = more compressed)
    pub position_scalar: f32,
}

impl Default for NoiseParams {
    fn default() -> Self {
        const MILLISECONDS_PER_SECOND: f32 = 1e3;
        Self {
            time_scalar: 0.75 / MILLISECONDS_PER_SECOND,
            position_scalar: 0.5,
        }
    }
}

/// One-dimensional noise pattern implementation.
///
/// Creates flowing animations based on a 2D noise function, using
/// time and the 1D position for the input coordinates.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Noise1d<Noise> {
    /// The noise function used to get hue
    hue_noise: Seeded<Noise>,
    /// The noise function used to get value
    value_noise: Seeded<Noise>,
    /// Configuration parameters
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim1d, Layout> for Noise1d<Noise>
where
    Layout: Layout1d,
    Noise: NoiseTrait + Sample<2> + Default,
{
    type Params = NoiseParams;
    type Color = Okhsv;

    /// Creates a new Noise1d pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self {
            hue_noise: Noise::default().seed(0),
            value_noise: Noise::default().seed(1),
            params,
        }
    }

    /// Generates colors for a 1D layout using noise.
    ///
    /// The pattern uses the LED position and time as inputs to a 2D noise function,
    /// mapping the noise value to a hue in the Okhsv color space.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self {
            hue_noise,
            value_noise,
            params,
        } = self;

        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;

        Layout::points().map(move |x| {
            let noise_args = [position_scalar * x, noise_time];
            let hue = hue_noise.sample2(noise_args);
            let saturation = 1.;
            let value = 0.75 + 0.25 * value_noise.sample2(noise_args);
            Okhsv::new(hue, saturation, value)
        })
    }
}

/// Two-dimensional noise pattern implementation.
///
/// Creates flowing animations based on a 3D noise function, using
/// time and the 2D position for the input coordinates.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Noise2d<Noise> {
    /// The noise function used to get hue
    hue_noise: Seeded<Noise>,
    /// The noise function used to get value
    value_noise: Seeded<Noise>,
    /// Configuration parameters
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim2d, Layout> for Noise2d<Noise>
where
    Layout: Layout2d,
    Noise: NoiseTrait + Sample<3> + Default,
{
    type Params = NoiseParams;
    type Color = Okhsv;

    /// Creates a new Noise2d pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self {
            hue_noise: Noise::default().seed(0),
            value_noise: Noise::default().seed(1),
            params,
        }
    }

    /// Generates colors for a 2D layout using noise.
    ///
    /// The pattern uses the LED x,y position and time as inputs to a 3D noise function,
    /// mapping the noise value to a hue in the Okhsv color space.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self {
            hue_noise,
            value_noise,
            params,
        } = self;

        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;

        Layout::points().map(move |point| {
            let noise_args = [
                position_scalar * point.x,
                position_scalar * point.y,
                noise_time,
            ];
            let hue = hue_noise.sample3(noise_args);
            let saturation = 1.;
            let value = 0.75 + 0.25 * value_noise.sample3(noise_args);
            Okhsv::new(hue, saturation, value)
        })
    }
}

/// Three-dimensional noise pattern implementation.
///
/// Creates flowing animations based on a 4D noise function, using
/// time and the 3D position for the input coordinates.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Noise3d<Noise>
where
    Noise: NoiseTrait,
{
    /// The noise function used to get hue
    hue_noise: Seeded<Noise>,
    /// The noise function used to get value
    value_noise: Seeded<Noise>,
    /// Configuration parameters
    params: NoiseParams,
}

impl<Layout, Noise> Pattern<Dim3d, Layout> for Noise3d<Noise>
where
    Layout: Layout3d,
    Noise: NoiseTrait + Sample<4> + Default,
{
    type Params = NoiseParams;
    type Color = Okhsv;

    /// Creates a new Noise2d pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self {
            hue_noise: Noise::default().seed(0),
            value_noise: Noise::default().seed(1),
            params,
        }
    }

    /// Generates colors for a 3D layout using noise.
    ///
    /// The pattern uses the LED x,y,z position and time as inputs to a 4D noise function,
    /// mapping the noise value to a hue in the HSV color space.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self {
            hue_noise,
            value_noise,
            params,
        } = self;

        let NoiseParams {
            time_scalar,
            position_scalar,
        } = params;

        let noise_time = time_in_ms as f32 * time_scalar;

        Layout::points().map(move |point| {
            let noise_args = [
                position_scalar * point.x,
                position_scalar * point.y,
                position_scalar * point.z,
                noise_time,
            ];
            let hue = hue_noise.sample4(noise_args);
            let saturation = 1.;
            let value = 0.75 + 0.25 * value_noise.sample4(noise_args);
            Okhsv::new(hue, saturation, value)
        })
    }
}
