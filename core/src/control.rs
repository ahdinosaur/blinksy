use core::marker::PhantomData;
use palette::FromColor;

use crate::{
    dimension::{Dim1d, Dim2d},
    driver::LedDriver,
    layout::{Layout1d, Layout2d},
    pattern::Pattern as PatternTrait,
};

pub struct Control<Dim, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
    brightness: f32,
}

impl<Dim, Layout, Pattern, Driver> Control<Dim, Layout, Pattern, Driver> {
    pub fn new(pattern: Pattern, driver: Driver) -> Self {
        Self {
            dim: PhantomData,
            layout: PhantomData,
            pattern,
            driver,
            brightness: 1.,
        }
    }

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
    pub fn tick(&mut self, time_in_ms: u64) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver.write(pixels, self.brightness)
    }
}

pub struct ControlBuilder<Dim, Layout, Pattern, Driver> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl ControlBuilder<(), (), (), ()> {
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
    pub fn build(self) -> Control<Dim2d, Layout, Pattern, Driver> {
        Control::new(self.pattern, self.driver)
    }
}
