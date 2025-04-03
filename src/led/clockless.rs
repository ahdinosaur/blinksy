use core::marker::PhantomData;

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use fugit::NanosDurationU32 as Nanoseconds;
use palette::{FromColor, LinSrgb, Srgb};

use super::{LedDriver, RgbOrder};

pub trait LedClockless {
    const T_0H: Nanoseconds;
    const T_0L: Nanoseconds;
    const T_1H: Nanoseconds;
    const T_1L: Nanoseconds;
    const T_RESET: Nanoseconds;

    fn t_cycle() -> Nanoseconds {
        (Self::T_0H + Self::T_0L).max(Self::T_1H + Self::T_1L)
    }
}

pub struct ClocklessDelayDriver<Led: LedClockless, Pin: OutputPin, Delay: DelayNs> {
    led: PhantomData<Led>,
    pin: Pin,
    delay: Delay,
    rgb_order: RgbOrder,
}

impl<Led, Pin, Delay> ClocklessDelayDriver<Led, Pin, Delay>
where
    Led: LedClockless,
    Pin: OutputPin,
    Delay: DelayNs,
{
    pub fn new(mut pin: Pin, delay: Delay, rgb_order: RgbOrder) -> Result<Self, Pin::Error> {
        pin.set_low()?;

        Ok(Self {
            led: PhantomData,
            delay,
            pin,
            rgb_order,
        })
    }

    pub fn write_bit(&mut self, bit: bool) -> Result<(), Pin::Error> {
        if !bit {
            self.pin.set_high()?;
            self.delay.delay_ns(Led::T_0H.to_nanos());
            self.pin.set_low()?;
            self.delay.delay_ns(Led::T_0L.to_nanos());
        } else {
            self.pin.set_high()?;
            self.delay.delay_ns(Led::T_1H.to_nanos());
            self.pin.set_low()?;
            self.delay.delay_ns(Led::T_1L.to_nanos());
        }
        Ok(())
    }

    pub fn write_byte(&mut self, byte: &u8) -> Result<(), Pin::Error> {
        for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
            match byte & bit_position {
                0 => self.write_bit(false)?,
                _ => self.write_bit(true)?,
            }
        }
        Ok(())
    }

    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), Pin::Error> {
        for byte in buffer {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn delay_for_reset(&mut self) {
        self.delay.delay_ns(Led::T_RESET.to_nanos())
    }
}

impl<Led, Pin, Delay> LedDriver for ClocklessDelayDriver<Led, Pin, Delay>
where
    Led: LedClockless,
    Pin: OutputPin,
    Delay: DelayNs,
{
    type Error = Pin::Error;
    type Color = Srgb;

    fn write<I, C>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        for color in pixels {
            let color: LinSrgb<u8> = Srgb::from_color(color).into_linear().into_format();
            let buffer = self.rgb_order.reorder(color.red, color.green, color.blue);
            self.write_buffer(&buffer)?;
        }
        self.delay_for_reset();
        Ok(())
    }
}
