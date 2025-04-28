//! # Control System
//!
//! This module provides the central control system for Blinksy. It connects layouts, patterns,
//! and drivers together to form a complete LED control pipeline.
//!
//! The main components are:
//!
//! - [`Control`]: The core struct that manages the LED update cycle
//! - [`ControlBuilder`]: A builder pattern implementation for creating Control instances
//!
//! The control system is generic over dimension, layout, pattern, and driver types,
//! allowing for type-safe combinations of these components.

use core::marker::PhantomData;
use palette::FromColor;

use crate::{
    dimension::{Dim1d, Dim2d},
    driver::LedDriver,
    layout::{Layout1d, Layout2d},
    pattern::Pattern as PatternTrait,
};

/// Central control system for LED management.
///
/// This struct orchestrates the flow of data from patterns to LED drivers,
/// handling timing, color conversion, and brightness control.
///
/// # Type Parameters
///
/// * `Dim` - The dimension marker (Dim1d or Dim2d)
/// * `Layout` - The specific layout type
/// * `Pattern` - The pattern implementation
/// * `Driver` - The LED driver implementation
///
/// # Example
///
/// ```rust
/// use blinksy::{
///     ControlBuilder,
///     layout1d,
///     patterns::{Rainbow, RainbowParams}
/// };
///
/// // Define a layout
/// layout1d!(MyStrip, 60);
///
/// // Create a control system
/// let mut control = ControlBuilder::new_1d()
///     .with_layout::<MyStrip>()
///     .with_pattern::<Rainbow>(RainbowParams {
///         position_scalar: 1.0,
///         ..Default::default()
///     })
///     .with_driver(/* LED driver */)
///     .build();
///
/// // Use the control system
/// control.set_brightness(0.5);
/// control.tick(0); // Update with time 0 ms
/// ```
pub struct Control<Dim, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
    brightness: f32,
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
        }
    }

    /// Sets the master brightness level.
    ///
    /// # Arguments
    ///
    /// * `brightness` - Brightness level from 0.0 (off) to 1.0 (full)
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
    }
}

impl<Layout, Pattern, Driver> Control<Dim1d, Layout, Pattern, Driver>
where
    Layout: Layout1d,
    Pattern: PatternTrait<Dim1d, Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Updates the LED state based on the current time.
    ///
    /// This method:
    /// 1. Calls the pattern to generate colors
    /// 2. Passes the colors to the driver with brightness applied
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
        self.driver.write(pixels, self.brightness)
    }
}

impl<Layout, Pattern, Driver> Control<Dim2d, Layout, Pattern, Driver>
where
    Layout: Layout2d,
    Pattern: PatternTrait<Dim2d, Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Updates the LED state based on the current time.
    ///
    /// This method:
    /// 1. Calls the pattern to generate colors
    /// 2. Passes the colors to the driver with brightness applied
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
        self.driver.write(pixels, self.brightness)
    }
}

/// Builder for constructing Control instances.
///
/// This struct provides a fluent API for building Control instances with
/// type safety and proper initialization.
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
    /// A builder initialized for 1D layout
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
    /// A builder initialized for 2D layout
    pub fn new_2d() -> ControlBuilder<Dim2d, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Pattern, Driver> ControlBuilder<Dim1d, (), Pattern, Driver> {
    /// Specifies the layout type for a 1D control system.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout1d
    ///
    /// # Returns
    ///
    /// Builder with layout type specified
    pub fn with_layout<Layout>(self) -> ControlBuilder<Dim1d, Layout, Pattern, Driver>
    where
        Layout: Layout1d,
    {
        ControlBuilder {
            dim: PhantomData,
            layout: PhantomData,
            pattern: self.pattern,
            driver: self.driver,
        }
    }
}

impl<Pattern, Driver> ControlBuilder<Dim2d, (), Pattern, Driver> {
    /// Specifies the layout type for a 2D control system.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout2d
    ///
    /// # Returns
    ///
    /// Builder with layout type specified
    pub fn with_layout<Layout>(self) -> ControlBuilder<Dim2d, Layout, Pattern, Driver>
    where
        Layout: Layout2d,
    {
        ControlBuilder {
            dim: PhantomData,
            layout: PhantomData,
            pattern: self.pattern,
            driver: self.driver,
        }
    }
}

impl<Layout, Driver> ControlBuilder<Dim1d, Layout, (), Driver>
where
    Layout: Layout1d,
{
    /// Specifies the pattern and its parameters for a 1D control system.
    ///
    /// # Type Parameters
    ///
    /// * `Pattern` - The pattern type implementing Pattern<Dim1d, Layout>
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
    ) -> ControlBuilder<Dim1d, Layout, Pattern, Driver>
    where
        Pattern: PatternTrait<Dim1d, Layout>,
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

impl<Layout, Driver> ControlBuilder<Dim2d, Layout, (), Driver>
where
    Layout: Layout2d,
{
    /// Specifies the pattern and its parameters for a 2D control system.
    ///
    /// # Type Parameters
    ///
    /// * `Pattern` - The pattern type implementing Pattern<Dim2d, Layout>
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
    ) -> ControlBuilder<Dim2d, Layout, Pattern, Driver>
    where
        Pattern: PatternTrait<Dim2d, Layout>,
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

impl<Layout, Pattern, Driver> ControlBuilder<Dim1d, Layout, Pattern, Driver>
where
    Layout: Layout1d,
    Pattern: PatternTrait<Dim1d, Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Builds the final 1D control system.
    ///
    /// # Returns
    ///
    /// A fully configured Control instance
    pub fn build(self) -> Control<Dim1d, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}

impl<Layout, Pattern, Driver> ControlBuilder<Dim2d, Layout, Pattern, Driver>
where
    Layout: Layout2d,
    Pattern: PatternTrait<Dim2d, Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Builds the final 2D control system.
    ///
    /// # Returns
    ///
    /// A fully configured Control instance
    pub fn build(self) -> Control<Dim2d, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}
