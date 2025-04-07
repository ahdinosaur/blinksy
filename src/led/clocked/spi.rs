use core::marker::PhantomData;

use embedded_hal::spi::SpiBus;
use palette::FromColor;

use crate::LedDriver;

use super::{ClockedLed, ClockedWriter};

#[derive(Debug)]
pub struct ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed,
    Spi: SpiBus<u8>,
{
    led: PhantomData<Led>,
    writer: Spi,
}

impl<Led, Spi> ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed,
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
    Led: ClockedLed,
    Spi: SpiBus<u8>,
{
    type Error = <Spi as ClockedWriter>::Error;
    type Color = Led::Color;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Led::write(self.writer, pixels, brightness)
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
