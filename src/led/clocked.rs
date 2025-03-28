use crate::time::{Megahertz, Nanoseconds};
use embedded_hal::{delay::DelayNs, digital::OutputPin, spi::SpiBus};
use palette::FromColor;

use super::LedDriver;

pub trait ClockedWriter {
    type Word: Copy + 'static;
    type Error;

    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error>;
}

pub trait ClockedLed {
    type Word: Copy + 'static;
    type Color;
    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        length: usize,
    ) -> Result<(), Writer::Error>;
    fn color<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        color: Self::Color,
    ) -> Result<(), Writer::Error>;
    fn reset<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
    ) -> Result<(), Writer::Error>;
    fn end<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        length: usize,
    ) -> Result<(), Writer::Error>;
}

#[derive(Debug)]
pub struct ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter,
{
    led: Led,
    writer: Writer,
}

impl<Led, Writer> ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter,
{
    pub fn new(led: Led, writer: Writer) -> Self {
        Self { led, writer }
    }
}

impl<Led, Writer> LedDriver for ClockedDriver<Led, Writer>
where
    Led: ClockedLed,
    Writer: ClockedWriter<Word = Led::Word>,
{
    type Error = Writer::Error;
    type Color = Led::Color;

    fn write<C, const N: usize>(&mut self, pixels: [C; N]) -> Result<(), Self::Error>
    where
        Self::Color: FromColor<C>,
    {
        self.led.start(&mut self.writer, N)?;

        for color in pixels.into_iter() {
            let color = Self::Color::from_color(color);
            self.led.color(&mut self.writer, color)?;
        }

        self.led.reset(&mut self.writer)?;
        self.led.end(&mut self.writer, N)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ClockedDelayWriter<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    data: Data,
    clock: Clock,
    delay: Delay,
    t_half_cycle_ns: u32,
}

impl<Data, Clock, Delay> ClockedDelayWriter<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    pub fn new(data: Data, clock: Clock, delay: Delay, data_rate: Megahertz) -> Self {
        let t_cycle: Nanoseconds = data_rate.into_duration();
        let t_half_cycle = t_cycle / 2;
        let t_half_cycle_ns = t_half_cycle.to_nanos();

        Self {
            data,
            clock,
            delay,
            t_half_cycle_ns,
        }
    }
}

#[derive(Debug)]
pub enum ClockedDelayError<Data, Clock>
where
    Data: OutputPin,
    Clock: OutputPin,
{
    Data(Data::Error),
    Clock(Clock::Error),
}

impl<Data, Clock, Delay> ClockedWriter for ClockedDelayWriter<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error = ClockedDelayError<Data, Clock>;
    type Word = u8;

    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error> {
        for byte in words {
            for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
                match byte & bit_position {
                    0 => self.data.set_low(),
                    _ => self.data.set_high(),
                }
                .map_err(ClockedDelayError::Data)?;

                self.delay.delay_ns(self.t_half_cycle_ns);

                self.clock.set_high().map_err(ClockedDelayError::Clock)?;

                self.delay.delay_ns(self.t_half_cycle_ns);

                self.clock.set_low().map_err(ClockedDelayError::Clock)?;
            }
        }
        Ok(())
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
