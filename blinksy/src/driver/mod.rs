use core::ops::Sub;
use palette::{cast::into_array, stimulus::IntoStimulus, FromColor, LinSrgb, Srgb};
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

impl ColorChannels {
    pub const fn channel_count(&self) -> usize {
        match self {
            ColorChannels::Rgb(_) => 3,
            ColorChannels::Rgbw(_) => 4,
        }
    }
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
}

#[derive(Debug)]
pub enum ColorArray<Word> {
    Rgb([Word; 3]),
    Rgbw([Word; 4]),
}

impl<Word> AsRef<[Word]> for ColorArray<Word> {
    fn as_ref(&self) -> &[Word] {
        match self {
            ColorArray::Rgb(rgb) => rgb,
            ColorArray::Rgbw(rgbw) => rgbw,
        }
    }
}

impl ColorChannels {
    pub fn to_array<Word>(&self, color: Srgb<f32>) -> ColorArray<Word>
    where
        f32: IntoStimulus<Word>,
        Word: Copy + PartialOrd + Sub<Output = Word>,
    {
        let color: LinSrgb<Word> = Srgb::from_color(color).into_linear().into_format();
        let rgb = into_array(color);
        match self {
            ColorChannels::Rgb(rgb_order) => ColorArray::Rgb(rgb_order.reorder(rgb)),
            ColorChannels::Rgbw(rgbw_order) => {
                let rgbw = rgb_to_rgbw(rgb);
                ColorArray::Rgbw(rgbw_order.reorder(rgbw))
            }
        }
    }
}

/// Extracts the white component from the RGB values by taking the minimum of R, G, and B.
/// Then subtracts that white component from each channel so the remaining RGB is "color only."
fn rgb_to_rgbw<Word>(rgb: [Word; 3]) -> [Word; 4]
where
    Word: Copy + PartialOrd + Sub<Output = Word>,
{
    // Determine the white component as the minimum value of the three channels.
    let w = if rgb[0] <= rgb[1] && rgb[0] <= rgb[2] {
        rgb[0]
    } else if rgb[1] <= rgb[0] && rgb[1] <= rgb[2] {
        rgb[1]
    } else {
        rgb[2]
    };

    [rgb[0] - w, rgb[1] - w, rgb[2] - w, w]
}
