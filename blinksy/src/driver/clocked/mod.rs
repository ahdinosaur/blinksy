use crate::color::ColorCorrection;
use crate::color::OutputColor;

use super::LedDriver;

mod delay;
mod spi;

pub use self::delay::*;
pub use self::spi::*;

pub trait ClockedWriter {
    type Word: Copy + 'static;
    type Error;

    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error>;
}

pub trait ClockedLed {
    type Word: Copy + 'static;

    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
    ) -> Result<(), Writer::Error>;
    fn color<Writer: ClockedWriter<Word = Self::Word>, Color: OutputColor>(
        writer: &mut Writer,
        color: Color,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>;
    fn reset<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
    ) -> Result<(), Writer::Error>;
    fn end<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
        pixel_count: usize,
    ) -> Result<(), Writer::Error>;

    fn clocked_write<Writer, I, C>(
        writer: &mut Writer,
        pixels: I,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error>
    where
        Writer: ClockedWriter<Word = Self::Word>,
        I: IntoIterator<Item = C>,
        C: OutputColor,
    {
        Self::start(writer)?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            Self::color(writer, color, brightness, gamma, correction)?;
            pixel_count += 1;
        }

        Self::reset(writer)?;
        Self::end(writer, pixel_count)?;

        Ok(())
    }
}
