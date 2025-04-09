use palette::FromColor;

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
    type Color;
    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
    ) -> Result<(), Writer::Error>;
    fn color<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
        color: Self::Color,
        brightness: f32,
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
    ) -> Result<(), Writer::Error>
    where
        Writer: ClockedWriter<Word = Self::Word>,
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Self::start(writer)?;

        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Self::Color::from_color(color);
            Self::color(writer, color, brightness)?;
            pixel_count += 1;
        }

        Self::reset(writer)?;
        Self::end(writer, pixel_count)?;

        Ok(())
    }
}
