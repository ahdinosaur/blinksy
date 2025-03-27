use palette::{IntoColor, LinSrgb, Srgb};

use crate::led::clocked::ClockedWriter;
use crate::util::map_f32_to_u8_range;
use crate::{LedDriver, RgbOrder};

// Apa102 docs:
// - https://hackaday.com/2014/12/09/digging-into-the-apa102-serial-led-protocol/
// - https://www.pololu.com/product/2554

#[derive(Debug)]
pub struct Apa102<Writer: ClockedWriter> {
    writer: Writer,
    brightness: f32,
    rgb_order: RgbOrder,
}

impl<Writer: ClockedWriter> Apa102<Writer> {
    pub fn new(writer: Writer, brightness: f32) -> Self {
        Self {
            writer,
            brightness,
            rgb_order: RgbOrder::BGR,
        }
    }
    pub fn new_with_rgb_order(writer: Writer, brightness: f32, rgb_order: RgbOrder) -> Self {
        Self {
            writer,
            brightness,
            rgb_order,
        }
    }
}

impl<Writer> LedDriver for Apa102<Writer>
where
    Writer: ClockedWriter<Word = u8>,
{
    type Error = <Writer as ClockedWriter>::Error;
    type Color = Srgb;

    fn write<Color, const N: usize>(&mut self, pixels: [Color; N]) -> Result<(), Self::Error>
    where
        Color: IntoColor<Self::Color>,
    {
        self.writer.write(&[0x00, 0x00, 0x00, 0x00])?;

        // TODO handle brightness how APA102HD works in FastLED

        let brightness = 0b11100000 | (map_f32_to_u8_range(self.brightness, 31) & 0b00011111);

        for color in pixels.into_iter() {
            let color: Srgb = color.into_color();
            let color: LinSrgb = color.into_color();
            let color: LinSrgb<u8> = color.into_format();
            let led_frame = self.rgb_order.reorder(color.red, color.green, color.blue);
            self.writer.write(&[brightness])?;
            self.writer.write(&led_frame)?;
        }

        let end_frame_length = (N - 1).div_ceil(16);
        for _ in 0..end_frame_length {
            self.writer.write(&[0x00])?
        }

        Ok(())
    }
}
