#![no_std]

mod layout;
mod led;
mod pattern;
pub mod patterns;
mod pixels;
pub mod time;
mod util;

use core::marker::PhantomData;

use palette::FromColor;

pub use crate::layout::*;
pub use crate::led::*;
pub use crate::pattern::*;
pub use crate::pixels::*;

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

    pub fn tick(&mut self, time_in_ms: u64) -> Result<(), Driver::Error> {
        let pixels = self.pattern.tick(time_in_ms);
        self.driver.write(pixels)
    }
}

pub struct Control1dBuilder<Layout, Pattern, Driver> {
    pub layout: PhantomData<Layout>,
    pub pattern: Pattern,
    pub driver: Driver,
}

#[allow(clippy::new_without_default)]
impl Control1dBuilder<(), (), ()> {
    pub fn new() -> Self {
        Control1dBuilder {
            layout: PhantomData,
            pattern: (),
            driver: (),
        }
    }
}

impl<Pattern, Driver> Control1dBuilder<(), Pattern, Driver> {
    pub fn with_layout<Layout>(self) -> Control1dBuilder<Layout, Pattern, Driver> {
        Control1dBuilder {
            layout: PhantomData,
            pattern: self.pattern,
            driver: self.driver,
        }
    }
}

impl<Layout, Driver> Control1dBuilder<Layout, (), Driver>
where
    Layout: Layout1d,
{
    pub fn with_pattern<Pattern>(
        self,
        params: Pattern::Params,
    ) -> Control1dBuilder<Layout, Pattern, Driver>
    where
        Pattern: Pattern1d<Layout>,
    {
        let pattern = Pattern::new(params);
        Control1dBuilder {
            layout: self.layout,
            pattern,
            driver: self.driver,
        }
    }
}

impl<Layout, Pattern> Control1dBuilder<Layout, Pattern, ()> {
    pub fn with_driver<Driver>(self, driver: Driver) -> Control1dBuilder<Layout, Pattern, Driver>
    where
        Driver: LedDriver,
    {
        Control1dBuilder {
            layout: self.layout,
            pattern: self.pattern,
            driver,
        }
    }
}

impl<Layout, Pattern, Driver> Control1dBuilder<Layout, Pattern, Driver>
where
    Layout: Layout1d,
    Pattern: Pattern1d<Layout>,
    Driver: LedDriver,
    Driver::Color: FromColor<Pattern::Color>,
{
    pub fn build(self) -> Control1d<Layout, Pattern, Driver> {
        Control1d::new(self.pattern, self.driver)
    }
}
