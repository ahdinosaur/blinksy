//! # Control System
//!
//! [`Control`] is the central control system for Blinksy: connecting a layout, pattern,
//! and driver together to form a complete LED control pipeline.
//!
//! As [`Control`] has a complex generic type signature, [`ControlBuilder`] is a builder to help
//! you create [`Control`] instances.

use core::marker::PhantomData;

use crate::{
    color::{ColorCorrection, FromColor},
    driver::{Driver as DriverTrait, DriverAsync as DriverAsyncTrait},
    layout::LayoutForDim,
    markers::{Async, Blocking, Dim1d, Dim2d},
    pattern::Pattern as PatternTrait,
};

/// Central LED control system.
///
/// A [`Control`] is made up of:
///
/// - A [`layout`](crate::layout)
/// - A [`pattern`](crate::pattern)
/// - A [`driver`](crate::driver)
///
/// You can use [`Control`] to
///
/// - Set a global brightness
/// - Set a global color correction.
/// - Send a frame of colors from the pattern to the driver.
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
///     patterns::rainbow::{Rainbow, RainbowParams}
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
pub struct Control<Dim, Exec, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    exec: PhantomData<Exec>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
    brightness: f32,
    correction: ColorCorrection,
}

impl<Dim, Exec, Layout, Pattern, Driver> Control<Dim, Exec, Layout, Pattern, Driver> {
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
            exec: PhantomData,
            layout: PhantomData,
            pattern,
            driver,
            brightness: 1.,
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

    /// Sets a color correction.
    ///
    /// # Arguments
    ///
    /// * `correction` - Color correction factors
    pub fn set_color_correction(&mut self, correction: ColorCorrection) {
        self.correction = correction;
    }
}

impl<Dim, Layout, Pattern, Driver> Control<Dim, Blocking, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverTrait,
    Driver::Color: FromColor<Pattern::Color>,
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
        self.driver.write(pixels, self.brightness, self.correction)
    }
}

impl<Dim, Layout, Pattern, Driver> Control<Dim, Async, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverAsyncTrait,
    Driver::Color: FromColor<Pattern::Color>,
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
    pub async fn tick(&mut self, time_in_ms: u64) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver
            .write(pixels, self.brightness, self.correction)
            .await
    }
}

///
/// The builder allows your to build up your [`Control`] system one-by-one
/// and handles the combination of generic types and contraints that [`Control`] expects.
pub struct ControlBuilder<Dim, Exec, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    exec: PhantomData<Exec>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl ControlBuilder<(), (), (), (), ()> {
    /// Starts building a one-dimensional control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 1D
    pub fn new_1d() -> ControlBuilder<Dim1d, (), (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl ControlBuilder<(), (), (), (), ()> {
    /// Starts building a two-dimensional control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 2D
    pub fn new_2d() -> ControlBuilder<Dim2d, (), (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Dim> ControlBuilder<Dim, (), (), (), ()> {
    /// Specifies the executor to be blocking.
    ///
    /// # Returns
    ///
    /// Builder with blocking executor specified
    pub fn blocking() -> ControlBuilder<Dim, Blocking, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Dim> ControlBuilder<Dim, (), (), (), ()> {
    /// Specifies the executor to be async.
    ///
    /// # Returns
    ///
    /// Builder with async executor specified
    pub fn asynch() -> ControlBuilder<Dim, Async, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Dim, Exec, Pattern, Driver> ControlBuilder<Dim, Exec, (), Pattern, Driver> {
    /// Specifies the layout type for the control system.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout that corresponds to Dim
    ///
    /// # Returns
    ///
    /// Builder with layout type specified
    pub fn with_layout<Layout>(self) -> ControlBuilder<Dim, Exec, Layout, Pattern, Driver>
    where
        Layout: LayoutForDim<Dim>,
    {
        ControlBuilder {
            dim: self.dim,
            exec: self.exec,
            layout: PhantomData,
            pattern: self.pattern,
            driver: self.driver,
        }
    }
}

impl<Dim, Exec, Layout, Driver> ControlBuilder<Dim, Exec, Layout, (), Driver>
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
    ) -> ControlBuilder<Dim, Exec, Layout, Pattern, Driver>
    where
        Pattern: PatternTrait<Dim, Layout>,
    {
        let pattern = Pattern::new(params);
        ControlBuilder {
            dim: self.dim,
            exec: self.exec,
            layout: self.layout,
            pattern,
            driver: self.driver,
        }
    }
}

impl<Dim, Layout, Pattern> ControlBuilder<Dim, Blocking, Layout, Pattern, ()> {
    /// Specifies the LED driver for the control system (blocking).
    ///
    /// # Arguments
    ///
    /// * `driver` - The LED driver instance (blocking executor)
    ///
    /// # Returns
    ///
    /// Builder with driver specified
    pub fn with_driver<Driver>(
        self,
        driver: Driver,
    ) -> ControlBuilder<Dim, Blocking, Layout, Pattern, Driver>
    where
        Driver: DriverTrait,
    {
        ControlBuilder {
            dim: self.dim,
            exec: self.exec,
            layout: self.layout,
            pattern: self.pattern,
            driver,
        }
    }
}

impl<Dim, Layout, Pattern> ControlBuilder<Dim, Async, Layout, Pattern, ()> {
    /// Specifies the LED driver for the control system (async).
    ///
    /// # Arguments
    ///
    /// * `driver` - The LED driver instance (async executor)
    ///
    /// # Returns
    ///
    /// Builder with driver specified
    pub fn with_driver<Driver>(
        self,
        driver: Driver,
    ) -> ControlBuilder<Dim, Async, Layout, Pattern, Driver>
    where
        Driver: DriverAsyncTrait,
    {
        ControlBuilder {
            dim: self.dim,
            exec: self.exec,
            layout: self.layout,
            pattern: self.pattern,
            driver,
        }
    }
}

impl<Dim, Layout, Pattern, Driver> ControlBuilder<Dim, Blocking, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Builds the final [`Control`] struct.
    ///
    /// # Returns
    ///
    /// A fully configured Control instance
    pub fn build(self) -> Control<Dim, Blocking, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}

impl<Dim, Layout, Pattern, Driver> ControlBuilder<Dim, Async, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverAsyncTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Builds the final [`Control`] struct.
    ///
    /// # Returns
    ///
    /// A fully configured Control instance
    pub fn build(self) -> Control<Dim, Async, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}
