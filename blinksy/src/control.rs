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
/// * `NUM_PIXELS` - The number of LEDs in the layout
/// * `Dim` - The dimension marker ([`Dim1d`] or [`Dim2d`])
/// * `Layout` - The [`layout`](crate::layout) type
/// * `Pattern` - The [`pattern`](crate::pattern) type
/// * `Driver` - The LED [`driver`](crate::driver) type
pub struct Control<const NUM_PIXELS: usize, Dim, Exec, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
{
    dim: PhantomData<Dim>,
    exec: PhantomData<Exec>,
    layout: PhantomData<Layout>,
    pixels: Vec<Pattern::Color, NUM_PIXELS>,
    pattern: Pattern,
    driver: Driver,
    brightness: f32,
    correction: ColorCorrection,
}

impl<const NUM_PIXELS: usize, Dim, Exec, Layout, Pattern, Driver>
    Control<NUM_PIXELS, Dim, Exec, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
{
    pub fn new(pattern: Pattern, driver: Driver) -> Self {
        Self {
            dim: PhantomData,
            exec: PhantomData,
            layout: PhantomData,
            pixels: Vec::new(),
            pattern,
            driver,
            brightness: 1.0,
            correction: ColorCorrection::default(),
        }
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
    }

    pub fn set_color_correction(&mut self, correction: ColorCorrection) {
        self.correction = correction;
    }
}

impl<const NUM_PIXELS: usize, Dim, Layout, Pattern, Driver>
    Control<NUM_PIXELS, Dim, Blocking, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn tick(&mut self, time_in_ms: u64) -> Result<(), Driver::Error> {
        self.pixels = self.pattern.tick(time_in_ms).collect();

        self.driver.write(
            self.pixels.drain(0..NUM_PIXELS),
            self.brightness,
            self.correction,
        )
    }
}

#[cfg(feature = "async")]
impl<const NUM_PIXELS: usize, Dim, Layout, Pattern, Driver>
    Control<NUM_PIXELS, Dim, Async, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverAsyncTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub async fn tick(&mut self, time_in_ms: u64) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver
            .write(pixels, self.brightness, self.correction)
            .await
    }
}

///
/// The builder allows you to build up your [`Control`] system one-by-one
/// and handles the combination of generic types and constraints that [`Control`] expects.
pub struct ControlBuilder<const NUM_PIXELS: usize, Dim, Exec, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    exec: PhantomData<Exec>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl ControlBuilder<0, (), (), (), (), ()> {
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
    pub fn with_layout<Layout, const NUM_PIXELS: usize>(
        self,
    ) -> ControlBuilder<NUM_PIXELS, Dim, Exec, Layout, Pattern, Driver>
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

impl<const NUM_PIXELS: usize, Dim, Exec, Layout, Driver>
    ControlBuilder<NUM_PIXELS, Dim, Exec, Layout, (), Driver>
where
    Layout: LayoutForDim<Dim>,
{
    pub fn with_pattern<Pattern>(
        self,
        params: Pattern::Params,
    ) -> ControlBuilder<NUM_PIXELS, Dim, Exec, Layout, Pattern, Driver>
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

impl<const NUM_PIXELS: usize, Dim, Layout, Pattern>
    ControlBuilder<NUM_PIXELS, Dim, Blocking, Layout, Pattern, ()>
{
    pub fn with_driver<Driver>(
        self,
        driver: Driver,
    ) -> ControlBuilder<NUM_PIXELS, Dim, Blocking, Layout, Pattern, Driver>
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
impl<const NUM_PIXELS: usize, Dim, Layout, Pattern>
    ControlBuilder<NUM_PIXELS, Dim, Async, Layout, Pattern, ()>
{
    pub fn with_driver<Driver>(
        self,
        driver: Driver,
    ) -> ControlBuilder<NUM_PIXELS, Dim, Async, Layout, Pattern, Driver>
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

impl<const NUM_PIXELS: usize, Dim, Layout, Pattern, Driver>
    ControlBuilder<NUM_PIXELS, Dim, Blocking, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn build(self) -> Control<NUM_PIXELS, Dim, Blocking, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}

#[cfg(feature = "async")]
impl<const NUM_PIXELS: usize, Dim, Layout, Pattern, Driver>
    ControlBuilder<NUM_PIXELS, Dim, Async, Layout, Pattern, Driver>
where
    Layout: LayoutForDim<Dim>,
    Pattern: PatternTrait<Dim, Layout>,
    Driver: DriverAsyncTrait,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn build(self) -> Control<NUM_PIXELS, Dim, Async, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}
