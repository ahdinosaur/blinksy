use core::marker::PhantomData;
use palette::FromColor;

use crate::{
    driver::LedDriver,
    layout::{Layout1d, Layout2d},
    pattern::{Pattern1d, Pattern2d},
};

pub struct Control1d<Layout, Pattern, Driver>
where
    Layout: Layout1d,
    Pattern: Pattern1d<Layout>,
    Driver: LedDriver,
{
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl<Layout, Pattern, Driver> Control1d<Layout, Pattern, Driver>
where
    Layout: Layout1d,
    Pattern: Pattern1d<Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn new(pattern: Pattern, driver: Driver) -> Self {
        Self {
            layout: PhantomData,
            pattern,
            driver,
        }
    }

    pub fn tick(&mut self, time_in_ms: u64, brightness: f32) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver.write(pixels, brightness)
    }
}

pub struct Control2d<Layout, Pattern, Driver>
where
    Layout: Layout2d,
    Pattern: Pattern2d<Layout>,
    Driver: LedDriver,
{
    layout: PhantomData<Layout>,
    pattern: Pattern,
    driver: Driver,
}

impl<Layout, Pattern, Driver> Control2d<Layout, Pattern, Driver>
where
    Layout: Layout2d,
    Pattern: Pattern2d<Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn new(pattern: Pattern, driver: Driver) -> Self {
        Self {
            layout: PhantomData,
            pattern,
            driver,
        }
    }

    pub fn tick(&mut self, time_in_ms: u64, brightness: f32) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver.write(pixels, brightness)
    }
}

pub struct ControlBuilder<Layout, Pattern, Driver> {
    pub layout: PhantomData<Layout>,
    pub pattern: Pattern,
    pub driver: Driver,
}

#[allow(clippy::new_without_default)]
impl ControlBuilder<(), (), ()> {
    pub fn new() -> Self {
        ControlBuilder {
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Pattern, Driver> ControlBuilder<(), Pattern, Driver> {
    pub fn with_layout<Layout>(self) -> ControlBuilder<Layout, Pattern, Driver> {
        ControlBuilder {
            layout: PhantomData,
            pattern: self.pattern,
            driver: self.driver,
        }
    }
}

impl<Layout, Driver> ControlBuilder<Layout, (), Driver>
where
    Layout: Layout1d,
{
    pub fn with_pattern<Pattern>(
        self,
        params: Pattern::Params,
    ) -> ControlBuilder<Layout, Pattern, Driver>
    where
        Pattern: Pattern1d<Layout>,
    {
        let pattern = Pattern::new(params);
        ControlBuilder {
            layout: self.layout,
            pattern,
            driver: self.driver,
        }
    }
}

impl<Layout, Driver> ControlBuilder<Layout, (), Driver>
where
    Layout: Layout2d,
{
    pub fn with_pattern2d<Pattern>(
        self,
        params: Pattern::Params,
    ) -> ControlBuilder<Layout, Pattern, Driver>
    where
        Pattern: Pattern2d<Layout>,
    {
        let pattern = Pattern::new(params);
        ControlBuilder {
            layout: self.layout,
            pattern,
            driver: self.driver,
        }
    }
}

impl<Layout, Pattern> ControlBuilder<Layout, Pattern, ()> {
    pub fn with_driver<Driver>(self, driver: Driver) -> ControlBuilder<Layout, Pattern, Driver>
    where
        Driver: LedDriver,
    {
        ControlBuilder {
            layout: self.layout,
            pattern: self.pattern,
            driver,
        }
    }
}

impl<Layout, Pattern, Driver> ControlBuilder<Layout, Pattern, Driver>
where
    Layout: Layout1d,
    Pattern: Pattern1d<Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn build1d(self) -> Control1d<Layout, Pattern, Driver> {
        Control1d::new(self.pattern, self.driver)
    }
}

impl<Layout, Pattern, Driver> ControlBuilder<Layout, Pattern, Driver>
where
    Layout: Layout2d,
    Pattern: Pattern2d<Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn build2d(self) -> Control2d<Layout, Pattern, Driver> {
        Control2d::new(self.pattern, self.driver)
    }
}
