// Credit: https://github.com/DaveRichmond/esp-hal-smartled

use blinksy::{
    color::{FromColor, Srgb},
    driver::{clockless::ClocklessLed, ColorArray, LedDriver},
};
use core::{fmt::Debug, marker::PhantomData, slice::IterMut};
use esp_hal::{
    clock::Clocks,
    gpio::{interconnect::PeripheralOutput, Level},
    peripheral::Peripheral,
    rmt::{Error as RmtError, PulseCode, TxChannel, TxChannelConfig, TxChannelCreator},
};

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
    ($led_count:expr, $channel_count:expr) => {
        // The size we're assigning here is calculated as following
        //  (
        //   Nr. of LEDs
        //   * channels (r,g,b -> 3)
        //   * pulses per channel (8)
        //  ) + 1 additional pulse for the end delimiter
        [0u32; $led_count * $channel_count * 8 + 1]
    };
}

pub struct ClocklessRmtDriver<Led, Tx, const BUFFER_SIZE: usize>
where
    Led: ClocklessLed,
    Tx: TxChannel,
{
    led: PhantomData<Led>,
    channel: Option<Tx>,
    rmt_buffer: [u32; BUFFER_SIZE],
    pulses: (u32, u32, u32),
}

impl<'d, Led, Tx, const BUFFER_SIZE: usize> ClocklessRmtDriver<Led, Tx, BUFFER_SIZE>
where
    Led: ClocklessLed,
    Tx: TxChannel,
{
    /// Create a new adapter object that drives the pin using the RMT channel.
    pub fn new<C, P>(
        channel: C,
        pin: impl Peripheral<P = P> + 'd,
        rmt_buffer: [u32; BUFFER_SIZE],
    ) -> Self
    where
        C: TxChannelCreator<'d, Tx, P>,
        P: PeripheralOutput + Peripheral<P = P>,
    {
        let clock_divider = 1;
        let config = TxChannelConfig::default()
            .with_clk_divider(clock_divider)
            .with_idle_output_level(Level::Low)
            .with_idle_output(true)
            .with_carrier_modulation(false);

        let channel = channel.configure(pin, config).unwrap();

        // Assume the RMT peripheral is set up to use the APB clock
        let clocks = Clocks::get();
        let freq_hz = clocks.apb_clock.as_hz() / clock_divider as u32;
        let freq_mhz = freq_hz / 1_000_000;

        let t_0h = ((Led::T_0H.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_0l = ((Led::T_0L.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_1h = ((Led::T_1H.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_1l = ((Led::T_1L.to_nanos() * freq_mhz) / 1_000) as u16;
        let t_reset = ((Led::T_RESET.to_nanos() * freq_mhz) / 1_000) as u16;

        Self {
            led: PhantomData,
            channel: Some(channel),
            rmt_buffer,
            pulses: (
                PulseCode::new(Level::High, t_0h, Level::Low, t_0l),
                PulseCode::new(Level::High, t_1h, Level::Low, t_1l),
                PulseCode::new(Level::Low, t_reset, Level::Low, 0),
            ),
        }
    }

    fn write_color_byte_to_rmt(
        byte: &u8,
        rmt_iter: &mut IterMut<u32>,
        pulses: &(u32, u32, u32),
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

    fn write_color_to_rmt(
        color: Srgb,
        rmt_iter: &mut IterMut<u32>,
        pulses: &(u32, u32, u32),
    ) -> Result<(), ClocklessRmtDriverError> {
        let array = Led::COLOR_CHANNELS.to_array(color);
        match array {
            ColorArray::Rgb(rgb) => {
                Self::write_color_byte_to_rmt(&rgb[0], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgb[1], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgb[2], rmt_iter, pulses)?;
            }
            ColorArray::Rgbw(rgbw) => {
                Self::write_color_byte_to_rmt(&rgbw[0], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgbw[1], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgbw[2], rmt_iter, pulses)?;
                Self::write_color_byte_to_rmt(&rgbw[3], rmt_iter, pulses)?;
            }
        }
        Ok(())
    }

    /// Convert all pixels to the RMT format and
    /// add them to internal buffer, then start a singular RMT operation
    /// based on that buffer.
    pub fn write_pixels<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
    ) -> Result<(), ClocklessRmtDriverError>
    where
        I: IntoIterator<Item = C>,
        Srgb: FromColor<C>,
    {
        // We always start from the beginning of the buffer
        let mut rmt_iter = self.rmt_buffer.iter_mut();

        // Add all converted iterator items to the buffer.
        // This will result in an `BufferSizeExceeded` error in case
        // the iterator provides more elements than the buffer can take.
        for color in pixels {
            let color = Srgb::from_color(color) * brightness;
            Self::write_color_to_rmt(color, &mut rmt_iter, &self.pulses)?;
        }

        // Finally, add the end element.
        *rmt_iter
            .next()
            .ok_or(ClocklessRmtDriverError::BufferSizeExceeded)? = self.pulses.2;

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
    Led: ClocklessLed,
    Tx: TxChannel,
{
    type Error = ClocklessRmtDriverError;
    type Color = Srgb;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        self.write_pixels(pixels, brightness)
    }
}
