//! # Control System
//!
//! [`Control`] is the central control system for Blinksy: connecting a layout, pattern,
//! and driver together to form a complete LED control pipeline.
//!
//! As [`Control`] has a complex generic type signature, [`ControlBuilder`] is a builder to help
//! you create [`Control`] instances.

use core::marker::PhantomData;

use heapless::Vec;

use crate::{
    color::{ColorCorrection, FromColor},
    driver::Driver as DriverTrait,
    layout::LayoutForDim,
    markers::{Blocking, Dim1d, Dim2d, Dim3d},
    pattern::Pattern as PatternTrait,
};
#[cfg(feature = "async")]
use crate::{driver::DriverAsync as DriverAsyncTrait, markers::Async};

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
/// * `PIXEL_COUNT` - The number of LEDs in the layout
/// * `Dim` - The dimension marker ([`Dim1d`] or [`Dim2d`] or [`Dim3d`])
/// * `Layout` - The [`layout`](crate::layout) type
/// * `Pattern` - The [`pattern`](crate::pattern) type
/// * `Driver` - The LED [`driver`](crate::driver) type
///
/// # Example (Blocking)
///
/// ```rust,ignore
/// use blinksy::{
///     ControlBuilder,
///     layout::Layout1d,
///     layout1d,
///     patterns::rainbow::{Rainbow, RainbowParams}
/// };
///
/// // Define a 1d layout of 60 LEDs
/// layout1d!(Layout, 60);
///
/// // Create a control system
/// let mut control = ControlBuilder::new_1d()
///     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
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
///
/// # Example (Async)
///
/// ```rust,ignore
/// use blinksy::{
///     ControlBuilder,
///     layout::Layout1d,
///     layout1d,
///     patterns::rainbow::{Rainbow, RainbowParams}
/// };
///
/// // Define a 1d layout of 60 LEDs
/// layout1d!(Layout, 60);
///
/// // Create a control system
/// let mut control = ControlBuilder::new_1d_async()
///     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
///     .with_pattern::<Rainbow>(RainbowParams::default())
///     .with_driver(/* LED driver */)
///     .build();
///
/// // Use the control system
/// control.set_brightness(0.5);
///
/// // Main control loop
/// loop {
///     control.tick(/* current time in milliseconds */).await.unwrap();
/// }
/// ```
pub struct Control<const PIXEL_COUNT: usize, Dim, Exec, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
{
    dim: PhantomData<Dim>,
    exec: PhantomData<Exec>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
    brightness: f32,
    correction: ColorCorrection,
}

impl<const PIXEL_COUNT: usize, Dim, Exec, Layout, Pattern, Driver>
    Control<PIXEL_COUNT, Dim, Exec, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
{
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
            brightness: 1.0,
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

impl<const PIXEL_COUNT: usize, Dim, Layout, Pattern, Driver>
    Control<PIXEL_COUNT, Dim, Blocking, Layout, Pattern, Driver>
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
        let frame =
            self.driver
                .frame::<PIXEL_COUNT, _, _>(pixels, self.brightness, self.correction)?;
        self.driver.write(frame)
    }
}

#[cfg(feature = "async")]
impl<const PIXEL_COUNT: usize, Dim, Layout, Pattern, Driver>
    Control<PIXEL_COUNT, Dim, Async, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverAsyncTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    /// Updates the LED state based on the current time, asynchronously.
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
        // Write colors from Pattern to pixel buffer.
        self.pixels.extend(self.pattern.tick(time_in_ms));
        // Write colors in pixel buffer to Driver.
        self.driver
            .write::<PIXEL_COUNT, _, _>(
                self.pixels.drain(0..PIXEL_COUNT),
                self.brightness,
                self.correction,
            )
            .await
    }
}

///
/// The builder allows your to build up your [`Control`] system one-by-one
/// and handles the combination of generic types and contraints that [`Control`] expects.
pub struct ControlBuilder<const PIXEL_COUNT: usize, Dim, Exec, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    exec: PhantomData<Exec>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl ControlBuilder<0, (), (), (), (), ()> {
    /// Starts building a one-dimensional blocking control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 1D, blocking
    pub fn new_1d() -> ControlBuilder<0, Dim1d, Blocking, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

#[cfg(feature = "async")]
impl ControlBuilder<0, (), (), (), (), ()> {
    /// Starts building a one-dimensional asynchronous control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 1D, async
    pub fn new_1d_async() -> ControlBuilder<0, Dim1d, Async, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl ControlBuilder<0, (), (), (), (), ()> {
    /// Starts building a two-dimensional blocking control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 2D, blocking
    pub fn new_2d() -> ControlBuilder<0, Dim2d, Blocking, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

#[cfg(feature = "async")]
impl ControlBuilder<0, (), (), (), (), ()> {
    /// Starts building a two-dimensional asynchronous control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 2D, async
    pub fn new_2d_async() -> ControlBuilder<0, Dim2d, Async, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl ControlBuilder<0, (), (), (), (), ()> {
    /// Starts building a three-dimensional blocking control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 3D, blocking
    pub fn new_3d() -> ControlBuilder<0, Dim3d, Blocking, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

#[cfg(feature = "async")]
impl ControlBuilder<0, (), (), (), (), ()> {
    /// Starts building a three-dimensional asynchronous control system.
    ///
    /// # Returns
    ///
    /// A builder initialized for 3D, async
    pub fn new_3d_async() -> ControlBuilder<0, Dim3d, Async, (), (), ()> {
        ControlBuilder {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Dim, Exec, Pattern, Driver> ControlBuilder<0, Dim, Exec, (), Pattern, Driver> {
    /// Specifies the layout type for the control system.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout that corresponds to Dim
    /// * `PIXEL_COUNT` - A constant for the number of pixels (`Layout::PIXEL_COUNT`)
    ///
    /// # Returns
    ///
    /// Builder with layout type specified
    pub fn with_layout<Layout, const PIXEL_COUNT: usize>(
        self,
    ) -> ControlBuilder<PIXEL_COUNT, Dim, Exec, Layout, Pattern, Driver>
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

impl<const PIXEL_COUNT: usize, Dim, Exec, Layout, Driver>
    ControlBuilder<PIXEL_COUNT, Dim, Exec, Layout, (), Driver>
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
    ) -> ControlBuilder<PIXEL_COUNT, Dim, Exec, Layout, Pattern, Driver>
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

impl<const PIXEL_COUNT: usize, Dim, Layout, Pattern>
    ControlBuilder<PIXEL_COUNT, Dim, Blocking, Layout, Pattern, ()>
{
    /// Specifies the LED driver for the control system (blocking).
    ///
    /// # Arguments
    ///
    /// * `driver` - The LED driver instance (blocking)
    ///
    /// # Returns
    ///
    /// Builder with driver specified
    pub fn with_driver<Driver>(
        self,
        driver: Driver,
    ) -> ControlBuilder<PIXEL_COUNT, Dim, Blocking, Layout, Pattern, Driver>
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

#[cfg(feature = "async")]
impl<const PIXEL_COUNT: usize, Dim, Layout, Pattern>
    ControlBuilder<PIXEL_COUNT, Dim, Async, Layout, Pattern, ()>
{
    /// Specifies the LED driver for the control system (async).
    ///
    /// # Arguments
    ///
    /// * `driver` - The LED driver instance (async)
    ///
    /// # Returns
    ///
    /// Builder with driver specified
    pub fn with_driver<Driver>(
        self,
        driver: Driver,
    ) -> ControlBuilder<PIXEL_COUNT, Dim, Async, Layout, Pattern, Driver>
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

impl<const PIXEL_COUNT: usize, Dim, Layout, Pattern, Driver>
    ControlBuilder<PIXEL_COUNT, Dim, Blocking, Layout, Pattern, Driver>
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
    pub fn build(self) -> Control<PIXEL_COUNT, Dim, Blocking, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}

#[cfg(feature = "async")]
impl<const PIXEL_COUNT: usize, Dim, Layout, Pattern, Driver>
    ControlBuilder<PIXEL_COUNT, Dim, Async, Layout, Pattern, Driver>
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
    pub fn build(self) -> Control<PIXEL_COUNT, Dim, Async, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}
