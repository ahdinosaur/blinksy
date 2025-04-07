use palette::{FromColor, LinSrgb, Srgb};
use smart_leds_trait::SmartLedsWrite;

pub mod clocked;
pub mod clockless;

pub use clocked::*;
pub use clockless::*;

pub trait LedDriver {
    type Error;
    type Color;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
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

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        let iterator = pixels.into_iter().map(|color| {
            let color: LinSrgb<f32> = Srgb::from_color(color).into_linear();
            let color = color * brightness;
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
    pub fn reorder<Word: Copy>(&self, rgb: [Word; 3]) -> [Word; 3] {
        match self {
            RgbOrder::RGB => [rgb[0], rgb[1], rgb[2]],
            RgbOrder::RBG => [rgb[0], rgb[2], rgb[1]],
            RgbOrder::GRB => [rgb[1], rgb[0], rgb[2]],
            RgbOrder::GBR => [rgb[1], rgb[2], rgb[0]],
            RgbOrder::BRG => [rgb[2], rgb[0], rgb[1]],
            RgbOrder::BGR => [rgb[2], rgb[1], rgb[0]],
        }
    }
}
