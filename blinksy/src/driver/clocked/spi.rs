use core::marker::PhantomData;

use embedded_hal::spi::SpiBus;

use super::{ClockedLed, ClockedWriter};
use crate::{
    color::{ColorCorrection, OutputColor},
    driver::LedDriver,
};

#[derive(Debug)]
pub struct ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
    led: PhantomData<Led>,
    writer: Spi,
}

impl<Led, Spi> ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
    pub fn new(spi: Spi) -> Self {
        Self {
            led: PhantomData,
            writer: spi,
        }
    }
}

impl<Led, Spi> LedDriver for ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
    type Error = <Spi as ClockedWriter>::Error;

    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        C: OutputColor,
    {
        Led::clocked_write(&mut self.writer, pixels, brightness, gamma, correction)
    }
}

impl<Spi> ClockedWriter for Spi
where
    Spi: SpiBus<u8>,
{
    type Error = Spi::Error;
    type Word = u8;

    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error> {
        self.write(words)
    }
}
