//! # Control System
//!
//! This module provides the central control system for Blinksy, connecting layouts, patterns,
//! and drivers together to form a complete LED control pipeline.
//!
//! The main components are:
//!
//! - [`Control`]: The core struct that manages the LED control pipeline
//! - [`ControlBuilder`]: A builder for creating Control instances
//!
//! The control system is generic over dimension, layout, pattern, and driver types.

use core::marker::PhantomData;

use crate::{
    color::ColorCorrection,
    dimension::{Dim1d, Dim2d, LayoutForDim},
    driver::LedDriver,
    pattern::Pattern as PatternTrait,
};

/// Central LED control system.
///
/// This struct orchestrates the flow of data from patterns to LED drivers,
/// handling timing, color conversion, and brightness control.
///
/// Tip: Use [`ControlBuilder`] to build your [`Control`] struct.
///
/// # Type Parameters
///
/// * `Dim` - The dimension marker ([`Dim1d`] or [`Dim2d`])
/// * `Layout` - The [`layout`](crate::layout) type
/// * `Pattern` - The [`pattern`](crate::pattern) type
/// * `Driver` - The LED [`driver`](crate::driver) type
///
/// # Example
///
/// ```rust,ignore
/// use blinksy::{
///     ControlBuilder,
///     layout1d,
///     patterns::{Rainbow, RainbowParams}
/// };
///
/// // Define a 1d layout of 60 LEDs
/// layout1d!(Layout, 60);
///
/// // Create a control system
/// let mut control = ControlBuilder::new_1d()
///     .with_layout::<Layout>()
///     .with_pattern::<Rainbow>(RainbowParams::default())
///     .with_driver(/* LED driver */)
///     .build();
///
/// // Use the control system
/// control.set_brightness(0.5);
///
/// // Main control loop
/// loop {
///     control.tick(/* current time in milliseconds */).unwrap();
/// }
/// ```
pub struct Control<Dim, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
    brightness: f32,
    gamma: f32,
    correction: ColorCorrection,
}

impl<Dim, Layout, Pattern, Driver> Control<Dim, Layout, Pattern, Driver> {
    /// Creates a new control system.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to use
    /// * `driver` - The LED driver to use
    ///
    /// # Returns
    ///
    /// A new Control instance with default brightness
    pub fn new(pattern: Pattern, driver: Driver) -> Self {
        Self {
            dim: PhantomData,
            layout: PhantomData,
            pattern,
            driver,
            brightness: 1.,
            gamma: 1.,
            correction: ColorCorrection::default(),
        }
    }

    /// Sets the overall brightness level.
    ///
    /// # Arguments
    ///
    /// * `brightness` - Brightness level from 0.0 (off) to 1.0 (full)
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
    }

    /// Sets an additional gamma correction.
    ///
    /// # Arguments
    ///
    /// * `gamma` - Gamma correction factor
    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    /// Sets a color correction.
    ///
    /// # Arguments
    ///
    /// * `correction` - Color correction factors
    pub fn set_color_correction(&mut self, correction: ColorCorrection) {
        self.correction = correction;
    }
}

impl<Dim, Layout, Pattern, Driver> Control<Dim, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: LedDriver,
{
    /// Updates the LED state based on the current time.
    ///
    /// This method:
    /// 1. Calls the pattern to generate colors
    /// 2. Passes the colors and brightness to the driver
    ///
    /// # Arguments
    ///
    /// * `time_in_ms` - Current time in milliseconds
    ///
    /// # Returns
    ///
    /// Result indicating success or an error from the driver
    pub fn tick(&mut self, time_in_ms: u64) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver
            .write(pixels, self.brightness, self.gamma, self.correction)
    }
}

///
/// The builder allows your to build up your [`Control`] system one-by-one
/// and handles the combination of generic types and contraints that [`Control`] expects.
pub struct ControlBuilder<Dim, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl ControlBuilder<(), (), (), ()> {
    /// Starts building a one-dimensional control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 1D
    pub fn new_1d() -> ControlBuilder<Dim1d, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl ControlBuilder<(), (), (), ()> {
    /// Starts building a two-dimensional control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 2D
    pub fn new_2d() -> ControlBuilder<Dim2d, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Dim, Pattern, Driver> ControlBuilder<Dim, (), Pattern, Driver> {
    /// Specifies the layout type for the control system.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout that corresponds to Dim
    ///
    /// # Returns
    ///
    /// Builder with layout type specified
    pub fn with_layout<Layout>(self) -> ControlBuilder<Dim, Layout, Pattern, Driver>
    where
        Layout: LayoutForDim<Dim>,
    {
        ControlBuilder {
            dim: PhantomData,
            layout: PhantomData,
            pattern: self.pattern,
            driver: self.driver,
        }
    }
}

impl<Dim, Layout, Driver> ControlBuilder<Dim, Layout, (), Driver>
where
    Layout: LayoutForDim<Dim>,
{
    /// Specifies the pattern and its parameters.
    ///
    /// # Type Parameters
    ///
    /// * `Pattern` - The pattern type implementing Pattern<Dim, Layout>
    ///
    /// # Arguments
    ///
    /// * `params` - The pattern parameters
    ///
    /// # Returns
    ///
    /// Builder with pattern specified
    pub fn with_pattern<Pattern>(
        self,
        params: Pattern::Params,
    ) -> ControlBuilder<Dim, Layout, Pattern, Driver>
    where
        Pattern: PatternTrait<Dim, Layout>,
    {
        let pattern = Pattern::new(params);
        ControlBuilder {
            dim: self.dim,
            layout: self.layout,
            pattern,
            driver: self.driver,
        }
    }
}

impl<Dim, Layout, Pattern> ControlBuilder<Dim, Layout, Pattern, ()> {
    /// Specifies the LED driver for the control system.
    ///
    /// # Arguments
    ///
    /// * `driver` - The LED driver instance
    ///
    /// # Returns
    ///
    /// Builder with driver specified
    pub fn with_driver<Driver>(self, driver: Driver) -> ControlBuilder<Dim, Layout, Pattern, Driver>
    where
        Driver: LedDriver,
    {
        ControlBuilder {
            dim: self.dim,
            layout: self.layout,
            pattern: self.pattern,
            driver,
        }
    }
}

impl<Dim, Layout, Pattern, Driver> ControlBuilder<Dim, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: LedDriver,
{
    /// Builds the final [`Control`] struct.
    ///
    /// # Returns
    ///
    /// A fully configured Control instance
    pub fn build(self) -> Control<Dim, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}
