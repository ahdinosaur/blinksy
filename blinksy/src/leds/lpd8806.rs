use core::iter::repeat_n;

use crate::{
    color::{ColorCorrection, LinearSrgb, RgbChannels},
    driver::clocked::ClockedLed,
    util::component::Component,
};

pub trait Lpd8806Order {
    fn reorder(bytes: [u8; 3]) -> [u8; 3];
}

pub struct OrderGrb;
pub struct OrderBrg;

impl Lpd8806Order for OrderGrb {
    #[inline]
    fn reorder(bytes: [u8; 3]) -> [u8; 3] {
        RgbChannels::GRB.reorder(bytes)
    }
}

impl Lpd8806Order for OrderBrg {
    #[inline]
    fn reorder(bytes: [u8; 3]) -> [u8; 3] {
        RgbChannels::BRG.reorder(bytes)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Lpd8806Generic<O: Lpd8806Order>(core::marker::PhantomData<O>);

pub type Lpd8806 = Lpd8806Generic<OrderGrb>;
pub type Lpd8806Brg = Lpd8806Generic<OrderBrg>;

impl<O: Lpd8806Order> Lpd8806Generic<O> {
    pub const fn frame_buffer_size(pixel_count: usize) -> usize {
        4 + pixel_count * 3 + (pixel_count - 1).div_ceil(16)
    }
}

impl<O: Lpd8806Order> ClockedLed for Lpd8806Generic<O> {
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
            let mut v = if x == 0 { 0 } else if x >= 0xff00 { 0xff } else { ((x + 128) >> 8) as u8 };
            v >>= 1;
            0x80 | v
        };

        let bytes = O::reorder([
            to_7bit(r16),
            to_7bit(g16),
            to_7bit(b16),
        ]);

        [bytes[0], bytes[1], bytes[2]]
    }

    fn end(pixel_count: usize) -> impl IntoIterator<Item = Self::Word> {
        let num_bytes = (pixel_count - 1).div_ceil(16);
        repeat_n(0u8, num_bytes)
    }
}
