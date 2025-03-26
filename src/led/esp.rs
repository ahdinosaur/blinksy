// Credit: https://github.com/DaveRichmond/esp-hal-smartled

use super::{clockless::LedClockless, LedDriver, RgbOrder};
use core::{fmt::Debug, marker::PhantomData, slice::IterMut};
use esp_hal::{
    clock::Clocks,
    gpio::{interconnect::PeripheralOutput, Level},
    peripheral::Peripheral,
    rmt::{Error as RmtError, PulseCode, TxChannel, TxChannelConfig, TxChannelCreator},
};
use palette::{IntoColor, LinSrgb, Srgb};

/// All types of errors that can happen during the conversion and transmission
/// of LED commands
#[derive(Debug, defmt::Format)]
pub enum ClocklessRmtDriverError {
    /// Raised in the event that the provided data container is not large enough
    BufferSizeExceeded,
    /// Raised if something goes wrong in the transmission,
    TransmissionError(RmtError),
}

/// Macro to allocate a buffer sized for a specific number of LEDs to be
/// addressed.
///
/// Attempting to use more LEDs that the buffer is configured for will result in
/// an `LedAdapterError:BufferSizeExceeded` error.
#[macro_export]
macro_rules! create_rmt_buffer {
    ( $buffer_size: literal ) => {
        // The size we're assigning here is calculated as following
        //  (
        //   Nr. of LEDs
        //   * channels (r,g,b -> 3)
        //   * pulses per channel 8)
        //  ) + 1 additional pulse for the end delimiter
        [0u32; $buffer_size * 24 + 1]
    };
}

pub struct ClocklessRmtDriver<Led, Tx, const BUFFER_SIZE: usize>
where
    Led: LedClockless,
    Tx: TxChannel,
{
    led: PhantomData<Led>,
    channel: Option<Tx>,
    rgb_order: RgbOrder,
    rmt_buffer: [u32; BUFFER_SIZE],
    pulses: (u32, u32),
}

impl<'d, Led, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Tx, BUFFER_SIZE>
where
    Led: LedClockless,
    Tx: TxChannel,
{
    /// Create a new adapter object that drives the pin using the RMT channel.
    pub fn new<C, P>(
        channel: C,
        pin: impl Peripheral<P = P> + 'd,
        rmt_buffer: [u32; BUFFER_SIZE],
        rgb_order: RgbOrder,
    ) -> Self
    where
        C: TxChannelCreator<'d, Tx, P>,
        P: PeripheralOutput + Peripheral<P = P>,
    {
        let config = TxChannelConfig::default()
            .with_clk_divider(1)
            .with_idle_output_level(Level::Low)
            .with_carrier_modulation(false)
            .with_idle_output(true);

        let channel = channel.configure(pin, config).unwrap();

        // Assume the RMT peripheral is set up to use the APB clock
        let clocks = Clocks::get();
        let clock_cycle_in_ms = clocks.apb_clock.as_duration().as_micros() as u32;

        Self {
            led: PhantomData,
            rgb_order,
            channel: Some(channel),
            rmt_buffer,
            pulses: (
                PulseCode::new(
                    Level::High,
                    (Led::T_0H.to_micros() / clock_cycle_in_ms) as u16,
                    Level::Low,
                    (Led::T_0L.to_micros() / clock_cycle_in_ms) as u16,
                ),
                PulseCode::new(
                    Level::High,
                    (Led::T_1H.to_micros() / clock_cycle_in_ms) as u16,
                    Level::Low,
                    (Led::T_1L.to_micros() / clock_cycle_in_ms) as u16,
                ),
            ),
        }
    }

    fn write_color_byte_to_rmt(
        byte: &u8,
        rmt_iter: &mut IterMut<u32>,
        pulses: (u32, u32),
    ) -> Result<(), ClocklessRmtDriverError> {
        for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
            *rmt_iter
                .next()
                .ok_or(ClocklessRmtDriverError::BufferSizeExceeded)? = match byte & bit_position {
                0 => pulses.0,
                _ => pulses.1,
            }
        }

        Ok(())
    }

    /// Convert all  items of the iterator to the RMT format and
    /// add them to internal buffer, then start a singular RMT operation
    /// based on that buffer.
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), ClocklessRmtDriverError> {
        // We always start from the beginning of the buffer
        let mut rmt_iter = self.rmt_buffer.iter_mut();

        // Add all converted iterator items to the buffer.
        // This will result in an `BufferSizeExceeded` error in case
        // the iterator provides more elements than the buffer can take.
        for item in buffer {
            Self::write_color_byte_to_rmt(item, &mut rmt_iter, self.pulses)?;
        }

        // Finally, add an end element.
        *rmt_iter
            .next()
            .ok_or(ClocklessRmtDriverError::BufferSizeExceeded)? = PulseCode::empty();

        // Perform the actual RMT operation. We use the u32 values here right away.
        let channel = self.channel.take().unwrap();
        match channel.transmit(&self.rmt_buffer).unwrap().wait() {
            Ok(chan) => {
                self.channel = Some(chan);
                Ok(())
            }
            Err((e, chan)) => {
                self.channel = Some(chan);
                Err(ClocklessRmtDriverError::TransmissionError(e))
            }
        }
    }
}

impl<Led, Tx, const BUFFER_SIZE: usize> LedDriver for ClocklessRmtDriver<Led, Tx, BUFFER_SIZE>
where
    Led: LedClockless,
    Tx: TxChannel,
{
    type Error = ClocklessRmtDriverError;
    type Color = Srgb;

    fn write<C, const N: usize>(&mut self, pixels: [C; N]) -> Result<(), Self::Error>
    where
        C: palette::IntoColor<Self::Color>,
    {
        for color in pixels {
            let color: Srgb = color.into_color();
            let color: LinSrgb = color.into_color();
            let color: LinSrgb<u8> = color.into_format();
            let buffer = match self.rgb_order {
                RgbOrder::RGB => [color.red, color.green, color.blue],
                RgbOrder::RBG => [color.red, color.blue, color.green],
                RgbOrder::GRB => [color.green, color.red, color.blue],
                RgbOrder::GBR => [color.green, color.blue, color.red],
                RgbOrder::BRG => [color.blue, color.red, color.green],
                RgbOrder::BGR => [color.blue, color.green, color.red],
            };
            self.write_buffer(&buffer)?;
        }
        Ok(())
    }
}
