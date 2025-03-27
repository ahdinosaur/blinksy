use palette::IntoColor;

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

    fn write<C, const N: usize>(&mut self, pixels: [C; N]) -> Result<(), Self::Error>
    where
        C: IntoColor<Self::Color>;
}

impl<Driver, DriverColor> LedDriver for Driver
where
    Driver: SmartLedsWrite<Color = DriverColor>,
    DriverColor: From<smart_leds_trait::RGB<f32>>,
{
    type Color = palette::Srgb;
    type Error = Driver::Error;

    fn write<C, const N: usize>(&mut self, pixels: [C; N]) -> Result<(), Self::Error>
    where
        C: IntoColor<Self::Color>,
    {
        let iterator = pixels.into_iter().map(|item| {
            let item: palette::Srgb = item.into_color();
            let item: palette::LinSrgb = item.into_color();
            smart_leds_trait::RGB::<f32>::new(item.red, item.green, item.blue)
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
