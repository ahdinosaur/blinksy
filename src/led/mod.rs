use palette::{FromColor, LinSrgb, Srgb};

mod chipsets;
mod clocked;
mod clockless;
#[cfg(feature = "esp")]
mod esp;

pub use chipsets::*;
pub use clocked::ClockedDelayWriter;
use smart_leds_trait::SmartLedsWrite;

pub trait LedDriver {
    type Error;
    type Color;

    fn write<I, C>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>;
}

impl<Driver, DriverColor> LedDriver for Driver
where
    Driver: SmartLedsWrite<Color = DriverColor>,
    DriverColor: From<smart_leds_trait::RGB<f32>>,
{
    type Color = palette::Srgb;
    type Error = Driver::Error;

    fn write<I, C>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        let iterator = pixels.into_iter().map(|color| {
            let color: LinSrgb<f32> = Srgb::from_color(color).into_linear();
            smart_leds_trait::RGB::<f32>::new(color.red, color.green, color.blue)
        });
        SmartLedsWrite::write(self, iterator)
    }
}

#[derive(Debug)]
pub enum RgbOrder {
    RGB,
    RBG,
    GRB,
    GBR,
    BRG,
    BGR,
}

impl RgbOrder {
    pub fn reorder<Word>(&self, red: Word, green: Word, blue: Word) -> [Word; 3] {
        match self {
            RgbOrder::RGB => [red, green, blue],
            RgbOrder::RBG => [red, blue, green],
            RgbOrder::GRB => [green, red, blue],
            RgbOrder::GBR => [green, blue, red],
            RgbOrder::BRG => [blue, red, green],
            RgbOrder::BGR => [blue, green, red],
        }
    }
}
