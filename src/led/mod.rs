use palette::IntoColor;

mod chipsets;
mod clocked;
mod clockless;
#[cfg(feature = "esp")]
mod esp;

pub use chipsets::*;
pub use clocked::ClockedGpio;
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
    pub fn reorder<Word>(&self, rgb: (Word, Word, Word)) -> [Word; 3] {
        match self {
            RgbOrder::RGB => [rgb.0, rgb.1, rgb.2],
            RgbOrder::RBG => [rgb.0, rgb.2, rgb.1],
            RgbOrder::GRB => [rgb.1, rgb.0, rgb.2],
            RgbOrder::GBR => [rgb.1, rgb.2, rgb.0],
            RgbOrder::BRG => [rgb.2, rgb.0, rgb.1],
            RgbOrder::BGR => [rgb.2, rgb.1, rgb.0],
        }
    }
}
