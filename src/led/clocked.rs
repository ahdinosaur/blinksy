use defmt::info;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use fugit::{MegahertzU32 as Megahertz, NanosDurationU32 as Nanoseconds};

pub trait ClockedWriter {
    type Word: Copy + 'static;
    type Error;

    fn write(&mut self, buf: &[Self::Word]) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct ClockedGpio<Data: OutputPin, Clock: OutputPin, Delay: DelayNs> {
    data: Data,
    clock: Clock,
    delay: Delay,
    t_half_cycle_ns: u32,
}

impl<Data: OutputPin, Clock: OutputPin, Delay: DelayNs> ClockedGpio<Data, Clock, Delay> {
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
pub enum ClockedGpioError<Data: OutputPin, Clock: OutputPin> {
    Data(Data::Error),
    Clock(Clock::Error),
}

impl<Data: OutputPin, Clock: OutputPin, Delay: DelayNs> ClockedWriter
    for ClockedGpio<Data, Clock, Delay>
{
    type Error = ClockedGpioError<Data, Clock>;
    type Word = u8;

    fn write(&mut self, buffer: &[Self::Word]) -> Result<(), Self::Error> {
        info!("write: {}", buffer);

        // For each byte in the buffer, iterate over bit masks in descending order.
        for byte in buffer {
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
