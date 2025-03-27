use core::marker::PhantomData;

use defmt::info;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use palette::Srgb;

use crate::time::{Megahertz, Nanoseconds};

use super::LedDriver;

pub trait ClockedLed {
    const DEFAULT_DATA_RATE: Megahertz;
}

#[derive(Debug)]
pub enum ClockedDelayDriverError<Data: OutputPin, Clock: OutputPin> {
    Data(Data::Error),
    Clock(Clock::Error),
}

#[derive(Debug)]
pub struct ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    led: PhantomData<Led>,
    data: Data,
    clock: Clock,
    delay: Delay,
    t_half_cycle_ns: u32,
}

impl<Led, Data, Clock, Delay> ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    pub fn new(data: Data, clock: Clock, delay: Delay, data_rate: Option<Megahertz>) -> Self {
        let t_cycle: Nanoseconds = data_rate.unwrap_or(Self::DEFAULT_DATA_RATE).into_duration();
        let t_half_cycle = t_cycle / 2;
        let t_half_cycle_ns = t_half_cycle.to_nanos();

        Self {
            led: PhantomData,
            data,
            clock,
            delay,
            t_half_cycle_ns,
        }
    }
}

impl<Led, Data, Clock, Delay> LedDriver for ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error = ClockedDelayDriverError<Data, Clock>;
    type Color = Srgb;

    fn write<const N: usize>(&mut self, pixels: [Self::Color; N]) -> Result<(), Self::Error> {
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

#[derive(Debug)]
pub struct Clocked<Writer: ClockedWriter> {
    writer: Writer,
    brightness: f32,
    rgb_order: RgbOrder,
}

impl<Writer: ClockedWriter> Apa102<Writer> {
    pub fn new(writer: Writer, brightness: f32) -> Self {
        Self {
            writer,
            brightness,
            rgb_order: RgbOrder::BGR,
        }
    }
    pub fn new_with_rgb_order(writer: Writer, brightness: f32, rgb_order: RgbOrder) -> Self {
        Self {
            writer,
            brightness,
            rgb_order,
        }
    }
}

impl<Writer> LedDriver for Apa102<Writer>
where
    Writer: ClockedWriter<Word = u8>,
{
    type Error = <Writer as ClockedWriter>::Error;
    type Color = Srgb;

    fn write<Color, const N: usize>(&mut self, pixels: [Color; N]) -> Result<(), Self::Error>
    where
        Color: IntoColor<Self::Color>,
    {
        self.writer.write(&[0x00, 0x00, 0x00, 0x00])?;

        // TODO handle brightness how APA102HD works in FastLED

        let brightness = 0b11100000 | (map_f32_to_u8_range(self.brightness, 31) & 0b00011111);

        for color in pixels.into_iter() {
            let color: Srgb = color.into_color();
            let color: LinSrgb = color.into_color();
            let color: LinSrgb<u8> = color.into_format();
            let led_frame = self.rgb_order.reorder(color.red, color.green, color.blue);
            self.writer.write(&[brightness])?;
            self.writer.write(&led_frame)?;
        }

        let end_frame_length = (N - 1).div_ceil(16);
        for _ in 0..end_frame_length {
            self.writer.write(&[0x00])?
        }

        Ok(())
    }
}
