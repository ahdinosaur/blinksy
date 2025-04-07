use crate::time::{Megahertz, Nanoseconds};
use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use palette::FromColor;

use super::{ClockedLed, ClockedWriter, LedDriver};

#[derive(Debug)]
pub struct ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    led: PhantomData<Led>,
    writer: ClockedDelayWriter<Data, Clock, Delay>,
}

impl<Led, Data, Clock, Delay> ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    pub fn new(data: Data, clock: Clock, delay: Delay, data_rate: Megahertz) -> Self {
        Self {
            led: PhantomData,
            writer: ClockedDelayWriter::new(data, clock, delay, data_rate),
        }
    }
}

impl<Led, Data, Clock, Delay> LedDriver for ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed<Word = u8>,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error = <ClockedDelayWriter<Data, Clock, Delay> as ClockedWriter>::Error;
    type Color = Led::Color;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Led::clocked_write(&mut self.writer, pixels, brightness)
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
