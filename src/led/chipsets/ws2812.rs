use embedded_hal::{delay::DelayNs, digital::OutputPin};
use fugit::NanosDurationU32 as Nanoseconds;

use crate::{
    led::clockless::{ClocklessDelayDriver, LedClockless},
    LedDriver, RgbOrder,
};

pub struct Ws2812;

// WS2812B docs:
// - https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf
impl LedClockless for Ws2812 {
    const T_0H: Nanoseconds = Nanoseconds::nanos(400);
    const T_0L: Nanoseconds = Nanoseconds::nanos(850);
    const T_1H: Nanoseconds = Nanoseconds::nanos(800);
    const T_1L: Nanoseconds = Nanoseconds::nanos(450);
    const T_RESET: Nanoseconds = Nanoseconds::micros(50);
}

pub struct Ws2812Delay<Pin: OutputPin, Delay: DelayNs> {
    driver: ClocklessDelayDriver<Ws2812, Pin, Delay>,
}

impl<Pin, Delay> Ws2812Delay<Pin, Delay>
where
    Pin: OutputPin,
    Delay: DelayNs,
{
    pub fn new(pin: Pin, delay: Delay) -> Result<Self, Pin::Error> {
        Ok(Self {
            driver: ClocklessDelayDriver::new(pin, delay, RgbOrder::GRB)?,
        })
    }
}

impl<Pin, Delay> LedDriver for Ws2812Delay<Pin, Delay>
where
    Pin: OutputPin,
    Delay: DelayNs,
{
    type Error = Pin::Error;
    type Color = <ClocklessDelayDriver<Ws2812, Pin, Delay> as LedDriver>::Color;

    fn write<C, const N: usize>(&mut self, pixels: [C; N]) -> Result<(), Self::Error>
    where
        C: palette::IntoColor<Self::Color>,
    {
        self.driver.write(pixels)
    }
}
