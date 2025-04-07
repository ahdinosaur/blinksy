use core::ops::{Add, Div};

use num_traits::FromPrimitive;
use palette::{stimulus::IntoStimulus, FromColor, LinSrgb, Srgb};
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
pub enum ColorChannels {
    Rgb(RgbChannels),
    Rgbw(RgbwChannels),
}

#[derive(Debug)]
pub enum RgbChannels {
    RGB,
    RBG,
    GRB,
    GBR,
    BRG,
    BGR,
}

#[derive(Debug)]
pub enum RgbwChannels {
    // RGB
    WRGB,
    RWGB,
    RGWB,
    RGBW,

    // RBG
    WRBG,
    RWBG,
    RBWG,
    RBGW,

    // GRB
    WGRB,
    GWRB,
    GRWB,
    GRBW,

    // GBR
    WGBR,
    GWBR,
    GBWR,
    GBRW,

    // BRG
    WBRG,
    BWRG,
    BRWG,
    BRGW,

    // BGR
    WBGR,
    BWGR,
    BGWR,
    BGRW,
}

#[derive(Debug)]
pub enum ColorArray<Word> {
    Rgb([Word; 3]),
    Rgbw([Word; 4]),
}

impl RgbChannels {
    pub fn reorder<Word: Copy>(&self, rgb: [Word; 3]) -> [Word; 3] {
        use RgbChannels::*;
        match self {
            RGB => [rgb[0], rgb[1], rgb[2]],
            RBG => [rgb[0], rgb[2], rgb[1]],
            GRB => [rgb[1], rgb[0], rgb[2]],
            GBR => [rgb[1], rgb[2], rgb[0]],
            BRG => [rgb[2], rgb[0], rgb[1]],
            BGR => [rgb[2], rgb[1], rgb[0]],
        }
    }
}

impl RgbwChannels {
    pub fn reorder<Word: Copy>(&self, rgbw: [Word; 4]) -> [Word; 4] {
        use RgbwChannels::*;
        match self {
            // RGB
            WRGB => [rgbw[3], rgbw[0], rgbw[1], rgbw[2]],
            RWGB => [rgbw[0], rgbw[3], rgbw[1], rgbw[2]],
            RGWB => [rgbw[0], rgbw[1], rgbw[3], rgbw[2]],
            RGBW => [rgbw[0], rgbw[1], rgbw[2], rgbw[3]],

            // RBG
            WRBG => [rgbw[3], rgbw[0], rgbw[2], rgbw[1]],
            RWBG => [rgbw[0], rgbw[3], rgbw[2], rgbw[1]],
            RBWG => [rgbw[0], rgbw[2], rgbw[3], rgbw[1]],
            RBGW => [rgbw[0], rgbw[2], rgbw[1], rgbw[3]],

            // GRB
            WGRB => [rgbw[3], rgbw[1], rgbw[0], rgbw[2]],
            GWRB => [rgbw[1], rgbw[3], rgbw[0], rgbw[2]],
            GRWB => [rgbw[1], rgbw[0], rgbw[3], rgbw[2]],
            GRBW => [rgbw[1], rgbw[0], rgbw[2], rgbw[3]],

            // GBR
            WGBR => [rgbw[3], rgbw[1], rgbw[2], rgbw[0]],
            GWBR => [rgbw[1], rgbw[3], rgbw[2], rgbw[0]],
            GBWR => [rgbw[1], rgbw[2], rgbw[3], rgbw[0]],
            GBRW => [rgbw[1], rgbw[2], rgbw[0], rgbw[3]],

            // BRG
            WBRG => [rgbw[3], rgbw[2], rgbw[0], rgbw[1]],
            BWRG => [rgbw[2], rgbw[3], rgbw[0], rgbw[1]],
            BRWG => [rgbw[2], rgbw[0], rgbw[3], rgbw[1]],
            BRGW => [rgbw[2], rgbw[0], rgbw[1], rgbw[3]],

            // BGR
            WBGR => [rgbw[3], rgbw[2], rgbw[1], rgbw[0]],
            BWGR => [rgbw[2], rgbw[3], rgbw[1], rgbw[0]],
            BGWR => [rgbw[2], rgbw[1], rgbw[3], rgbw[0]],
            BGRW => [rgbw[2], rgbw[1], rgbw[0], rgbw[3]],
        }
    }
}

impl ColorChannels {
    pub fn reorder<Word: Copy>(&self, color: ColorArray<Word>) -> ColorArray<Word> {
        match (self, color) {
            (ColorChannels::Rgb(rgb_order), ColorArray::Rgb(rgb)) => {
                ColorArray::Rgb(rgb_order.reorder(rgb))
            }
            (ColorChannels::Rgbw(rgbw_order), ColorArray::Rgbw(rgbw)) => {
                ColorArray::Rgbw(rgbw_order.reorder(rgbw))
            }
            _ => panic!("Mismatched color array type and color channel type"),
        }
    }

    pub fn to_array<Word>(&self, color: Srgb<f32>) -> ColorArray<Word>
    where
        f32: IntoStimulus<Word>,
        Word: Copy + FromPrimitive + Add<Output = Word> + Div<Output = Word> + From<u8>,
    {
        let color: LinSrgb<Word> = Srgb::from_color(color).into_linear().into_format();
        let rgb = color.into_components();
        match self {
            ColorChannels::Rgb(rgb_order) => ColorArray::Rgb(rgb_order.reorder(rgb.into())),
            ColorChannels::Rgbw(rgbw_order) => {
                let w = compute_white_channel(rgb.into());
                let rgbw = [rgb.0, rgb.1, rgb.2, w];
                ColorArray::Rgbw(rgbw_order.reorder(rgbw))
            }
        }
    }
}

fn compute_white_channel<Word>(rgb: [Word; 3]) -> Word
where
    Word: Copy + FromPrimitive + Add<Output = Word> + Div<Output = Word> + From<u8>,
{
    let sum = rgb[0] + rgb[1] + rgb[2];
    sum / Word::from_u8(3_u8).unwrap()
}
