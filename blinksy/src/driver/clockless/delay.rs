use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

use super::ClocklessLed;
use crate::{
    color::{ColorCorrection, OutputColor},
    driver::LedDriver,
};

pub struct ClocklessDelayDriver<Led: ClocklessLed, Pin: OutputPin, Delay: DelayNs> {
    led: PhantomData<Led>,
    pin: Pin,
    delay: Delay,
}

impl<Led, Pin, Delay> ClocklessDelayDriver<Led, Pin, Delay>
where
    Led: ClocklessLed,
    Pin: OutputPin,
    Delay: DelayNs,
{
    pub fn new(mut pin: Pin, delay: Delay) -> Result<Self, Pin::Error> {
        pin.set_low()?;

        Ok(Self {
            led: PhantomData,
            delay,
            pin,
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
    Led: ClocklessLed,
    Pin: OutputPin,
    Delay: DelayNs,
{
    type Error = Pin::Error;

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
        for color in pixels {
            let data = color.to_led(Led::LED_CHANNELS, brightness, gamma, correction);
            self.write_buffer(data.as_ref())?;
        }
        self.delay_for_reset();
        Ok(())
    }
}
