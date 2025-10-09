use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as DelayNsAsync;
use heapless::Vec;
use num_traits::ToBytes;

use super::ClocklessLed;
#[cfg(feature = "async")]
use crate::driver::DriverAsync;
use crate::{
    driver::ClocklessWriter,
    util::bits::{bits_of, BitOrder},
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
/// * `Delay` - The delay provider
pub struct ClocklessDelay<Led: ClocklessLed, Pin: OutputPin, Delay> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,

    /// GPIO pin for data transmission
    pin: Pin,

    /// Delay provider for timing control
    delay: Delay,
}

impl<Led, Pin, Delay> ClocklessDelay<Led, Pin, Delay>
where
    Led: ClocklessLed,
    Pin: OutputPin,
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
}

impl<Led, Pin, Delay> ClocklessWriter<Led> for ClocklessDelay<Led, Pin, Delay>
where
    Led: ClocklessLed,
    Led::Word: ToBytes,
    <Led::Word as ToBytes>::Bytes: IntoIterator<Item = u8>,
    Pin: OutputPin,
    Delay: DelayNs,
{
    type Error = Pin::Error;

    /// Transmits a buffer of bytes.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The byte array to transmit
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for byte in frame {
            for bit in bits_of(&byte, BitOrder::MostSignificantBit) {
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
            }
        }

        // Sends the reset signal at the end of a transmission.
        //
        // This keeps the data line low for the required reset period, allowing the LEDs
        // to latch the received data and update their outputs.
        self.delay.delay_ns(Led::T_RESET.to_nanos());

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<Led, Pin, Delay> ClocklessWriterAsync<Led> for ClocklessDelay<Led, Pin, Delay>
where
    Led: ClocklessLed,
    Led::Word: ToBytes,
    <Led::Word as ToBytes>::Bytes: IntoIterator<Item = u8>,
    Pin: OutputPin,
    Delay: DelayNs,
{
    type Error = Pin::Error;

    /// Transmits a buffer of bytes.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The byte array to transmit
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for byte in frame {
            for bit in bits_of(&byte, BitOrder::MostSignificantBit) {
                if !bit {
                    // Transmit a '0' bit
                    self.pin.set_high()?;
                    self.delay.delay_ns(Led::T_0H.to_nanos()).await;
                    self.pin.set_low()?;
                    self.delay.delay_ns(Led::T_0L.to_nanos()).await;
                } else {
                    // Transmit a '1' bit
                    self.pin.set_high()?;
                    self.delay.delay_ns(Led::T_1H.to_nanos()).await;
                    self.pin.set_low()?;
                    self.delay.delay_ns(Led::T_1L.to_nanos()).await;
                }
            }
        }

        // Sends the reset signal at the end of a transmission.
        //
        // This keeps the data line low for the required reset period, allowing the LEDs
        // to latch the received data and update their outputs.
        self.delay.delay_ns(Led::T_RESET.to_nanos()).await;

        Ok(())
    }
}
