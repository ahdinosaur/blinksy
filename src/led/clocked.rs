use crate::time::{Megahertz, Nanoseconds};
use embedded_hal::{delay::DelayNs, digital::OutputPin, spi::SpiBus};

pub trait ClockedWriter {
    type Word: Copy + 'static;
    type Error;

    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error>;
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
pub enum ClockedGpioError<Data, Clock>
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
    type Error = ClockedGpioError<Data, Clock>;
    type Word = u8;

    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error> {
        for byte in words {
            for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
                match byte & bit_position {
                    0 => self.data.set_low(),
                    _ => self.data.set_high(),
                }
                .map_err(ClockedGpioError::Data)?;

                self.delay.delay_ns(self.t_half_cycle_ns);

                self.clock.set_high().map_err(ClockedGpioError::Clock)?;

                self.delay.delay_ns(self.t_half_cycle_ns);

                self.clock.set_low().map_err(ClockedGpioError::Clock)?;
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
