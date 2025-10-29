use core::{iter::repeat_n, marker::PhantomData};

use crate::{
    color::{ColorCorrection, LinearSrgb, RgbOrder, RgbOrderIsGrb},
    driver::clocked::ClockedLed,
    util::component::Component,
};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Lpd8806<Order: RgbOrder = RgbOrderIsGrb> {
    order: PhantomData<Order>,
}

impl<Order> Lpd8806<Order>
where
    Order: RgbOrder,
{
    pub const fn frame_buffer_size(pixel_count: usize) -> usize {
        4 + pixel_count * 3 + (pixel_count - 1).div_ceil(16)
    }
}

impl<Order> ClockedLed for Lpd8806<Order>
where
    Order: RgbOrder,
{
    type Word = u8;
    type Color = LinearSrgb;

    fn start() -> impl IntoIterator<Item = Self::Word> {
        [0x00, 0x00, 0x00, 0x00]
    }

    fn led(
        linear_rgb: LinearSrgb,
        _brightness: f32,
        correction: ColorCorrection,
    ) -> impl IntoIterator<Item = Self::Word> {
        let (r, g, b) = (linear_rgb.red, linear_rgb.green, linear_rgb.blue);

        let r = r * correction.red;
        let g = g * correction.green;
        let b = b * correction.blue;

        let (r16, g16, b16) = (
            Component::from_normalized_f32(r),
            Component::from_normalized_f32(g),
            Component::from_normalized_f32(b),
        );

        let to_7bit = |x: u16| -> u8 {
            let mut v = if x == 0 {
                0
            } else if x >= 0xff00 {
                0xff
            } else {
                ((x + 128) >> 8) as u8
            };
            v >>= 1;
            0x80 | v
        };

        let bytes = Order::RGB_CHANNELS.reorder([to_7bit(r16), to_7bit(g16), to_7bit(b16)]);

        [bytes[0], bytes[1], bytes[2]]
    }

    fn end(pixel_count: usize) -> impl IntoIterator<Item = Self::Word> {
        let num_bytes = (pixel_count - 1).div_ceil(16);
        repeat_n(0u8, num_bytes)
    }
}
