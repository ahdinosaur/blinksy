use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

use super::ClocklessLed;
use crate::{
    color::{ColorCorrection, FromColor, LinearSrgb},
    driver::Driver,
};

/// Driver for clockless LEDs using GPIO bit-banging with a delay timer.
///
/// The implementation uses:
///
/// - A single GPIO output pin for data transmission
/// - A delay provider for timing control
/// - Timing parameters defined by a [`ClocklessLed`] implementation
///
/// Note: This will not work unless your delay timer is able to handle microsecond
/// precision, which most microcontrollers cannot do.
///
/// ## Usage
///
/// ```rust
/// use embedded_hal::digital::OutputPin;
/// use embedded_hal::delay::DelayNs;
/// use blinksy::{driver::ClocklessDelayDriver, drivers::ws2812::Ws2812Led};
///
/// fn setup_leds<P, D>(data_pin: P, delay: D) -> ClocklessDelayDriver<Ws2812Led, P, D>
/// where
///     P: OutputPin,
///     D: DelayNs,
/// {
///     // Create a new WS2812 driver
///     ClocklessDelayDriver::<Ws2812Led, _, _>::new(data_pin, delay)
///         .expect("Failed to initialize LED driver")
/// }
/// ```
///
/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClocklessLed)
/// * `Pin` - The GPIO pin type for data output (must implement OutputPin)
/// * `Delay` - The delay provider (must implement DelayNs)
pub struct ClocklessDelayDriver<Led: ClocklessLed, Pin: OutputPin, Delay: DelayNs> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,

    /// GPIO pin for data transmission
    pin: Pin,

    /// Delay provider for timing control
    delay: Delay,
}

impl<Led, Pin, Delay> ClocklessDelayDriver<Led, Pin, Delay>
where
    Led: ClocklessLed,
    Pin: OutputPin,
    Delay: DelayNs,
{
    /// Creates a new clockless LED driver.
    ///
    /// Initializes the data pin to the low state.
    ///
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin for data output
    /// * `delay` - The delay provider for timing control
    ///
    /// # Returns
    ///
    /// A new ClocklessDelayDriver instance or an error if pin initialization fails
    pub fn new(mut pin: Pin, delay: Delay) -> Result<Self, Pin::Error> {
        pin.set_low()?;
        Ok(Self {
            led: PhantomData,
            delay,
            pin,
        })
    }

    /// Transmits a single bit using the timing parameters from Led.
    ///
    /// # Arguments
    ///
    /// * `bit` - The bit value to transmit (true = 1, false = 0)
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    pub fn write_bit(&mut self, bit: bool) -> Result<(), Pin::Error> {
        if !bit {
            // Transmit a '0' bit
            self.pin.set_high()?;
            self.delay.delay_ns(Led::T_0H.to_nanos());
            self.pin.set_low()?;
            self.delay.delay_ns(Led::T_0L.to_nanos());
        } else {
            // Transmit a '1' bit
            self.pin.set_high()?;
            self.delay.delay_ns(Led::T_1H.to_nanos());
            self.pin.set_low()?;
            self.delay.delay_ns(Led::T_1L.to_nanos());
        }
        Ok(())
    }

    /// Transmits a byte, bit by bit, most significant bit first.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte value to transmit
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    pub fn write_byte(&mut self, byte: &u8) -> Result<(), Pin::Error> {
        for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
            match byte & bit_position {
                0 => self.write_bit(false)?,
                _ => self.write_bit(true)?,
            }
        }
        Ok(())
    }

    /// Transmits a buffer of bytes.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The byte array to transmit
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), Pin::Error> {
        for byte in buffer {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    /// Sends the reset signal at the end of a transmission.
    ///
    /// This keeps the data line low for the required reset period, allowing the LEDs
    /// to latch the received data and update their outputs.
    pub fn delay_for_reset(&mut self) {
        self.delay.delay_ns(Led::T_RESET.to_nanos())
    }
}

impl<Led, Pin, Delay> Driver for ClocklessDelayDriver<Led, Pin, Delay>
where
    Led: ClocklessLed,
    Pin: OutputPin,
    Delay: DelayNs,
{
    type Error = Pin::Error;
    type Color = LinearSrgb;

    /// Writes a sequence of colors to the LED chain.
    ///
    /// This method:
    /// 1. Converts each input color to the appropriate format
    /// 2. Applies the global brightness scaling
    /// 3. Reorders color channels according to the LED protocol
    /// 4. Transmits all data
    /// 5. Sends the reset signal
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if transmission fails
    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        for color in pixels {
            let linear_srgb = LinearSrgb::from_color(color);
            let data = linear_srgb.to_led(Led::LED_CHANNELS, brightness, correction);
            self.write_buffer(data.as_ref())?;
        }
        self.delay_for_reset();
        Ok(())
    }
}
