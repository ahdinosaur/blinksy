use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;
use palette::{FromColor, LinSrgb, Srgb};

use crate::led::clocked::{ClockedDriver, ClockedLed, ClockedWriter};
use crate::time::Megahertz;
use crate::util::map_f32_to_u8_range;
use crate::{ClockedDelayWriter, LedDriver, RgbOrder};

// Apa102 docs:
// - https://hackaday.com/2014/12/09/digging-into-the-apa102-serial-led-protocol/
// - https://www.pololu.com/product/2554

#[derive(Debug)]
pub struct Apa102 {
    brightness: u8,
}

impl Apa102 {
    pub fn new(brightness: f32) -> Self {
        let brightness = 0b11100000 | (map_f32_to_u8_range(brightness, 31) & 0b00011111);
        Self { brightness }
    }
}

impl ClockedLed for Apa102 {
    type Word = u8;
    type Color = Srgb;

    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        _length: usize,
    ) -> Result<(), Writer::Error> {
        writer.write(&[0x00, 0x00, 0x00, 0x00])
    }

    fn color<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        color: Self::Color,
    ) -> Result<(), Writer::Error> {
        let color: LinSrgb<u8> = Srgb::from_color(color).into_linear().into_format();
        let led_frame = RgbOrder::BGR.reorder(color.red, color.green, color.blue);
        writer.write(&[self.brightness])?;
        writer.write(&led_frame)?;
        Ok(())
    }

    fn reset<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        _writer: &mut Writer,
    ) -> Result<(), Writer::Error> {
        Ok(())
    }
    fn end<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        length: usize,
    ) -> Result<(), Writer::Error> {
        let num_bytes = (length - 1).div_ceil(16);
        for _ in 0..num_bytes {
            writer.write(&[0x00])?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Apa102Delay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    driver: ClockedDriver<Apa102, ClockedDelayWriter<Data, Clock, Delay>>,
}

impl<Data, Clock, Delay> Apa102Delay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    pub fn new(
        data: Data,
        clock: Clock,
        delay: Delay,
        brightness: f32,
        data_rate: Megahertz,
    ) -> Self {
        let led = Apa102::new(brightness);
        let writer = ClockedDelayWriter::new(data, clock, delay, data_rate);
        let driver = ClockedDriver::new(led, writer);
        Self { driver }
    }
}

impl<Data, Clock, Delay> LedDriver for Apa102Delay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error =
        <ClockedDriver<Apa102, ClockedDelayWriter<Data, Clock, Delay>> as LedDriver>::Error;
    type Color =
        <ClockedDriver<Apa102, ClockedDelayWriter<Data, Clock, Delay>> as LedDriver>::Color;

    fn write<Color, const N: usize>(&mut self, pixels: [Color; N]) -> Result<(), Self::Error>
    where
        Self::Color: FromColor<Color>,
    {
        self.driver.write(pixels)
    }
}

#[derive(Debug)]
pub struct Apa102Spi<Spi>
where
    Spi: SpiBus,
{
    driver: ClockedDriver<Apa102, Spi>,
}

impl<Spi> Apa102Spi<Spi>
where
    Spi: SpiBus,
{
    pub fn new(spi: Spi, brightness: f32) -> Self {
        let led = Apa102::new(brightness);
        let driver = ClockedDriver::new(led, spi);
        Self { driver }
    }
}

impl<Spi> LedDriver for Apa102Spi<Spi>
where
    Spi: SpiBus,
{
    type Error = <ClockedDriver<Apa102, Spi> as LedDriver>::Error;
    type Color = <ClockedDriver<Apa102, Spi> as LedDriver>::Color;

    fn write<Color, const N: usize>(&mut self, pixels: [Color; N]) -> Result<(), Self::Error>
    where
        Self::Color: FromColor<Color>,
    {
        self.driver.write(pixels)
    }
}
