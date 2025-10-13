use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as DelayNsAsync;
use heapless::Vec;
use num_traits::ToBytes;

use super::ClocklessLed;
#[cfg(feature = "async")]
use crate::driver::ClocklessWriterAsync;
use crate::{
    driver::ClocklessWriter,
    util::bits::{bits_of, BitOrder},
};

/// Builder for [`ClocklessDelay`].
pub struct ClocklessDelayBuilder<Led, Data, Delay> {
    led: PhantomData<Led>,
    data: Data,
    delay: Delay,
}

impl Default for ClocklessDelayBuilder<(), (), ()> {
    fn default() -> Self {
        Self {
            led: PhantomData,
            data: (),
            delay: (),
        }
    }
}

impl<Data, Delay> ClocklessDelayBuilder<(), Data, Delay> {
    pub fn with_led<Led: ClocklessLed>(self) -> ClocklessDelayBuilder<Led, Data, Delay> {
        ClocklessDelayBuilder {
            led: PhantomData,
            data: self.data,
            delay: self.delay,
        }
    }
}

impl<Led, Delay> ClocklessDelayBuilder<Led, (), Delay> {
    pub fn with_data<Data: OutputPin>(self, data: Data) -> ClocklessDelayBuilder<Led, Data, Delay> {
        ClocklessDelayBuilder {
            led: self.led,
            data,
            delay: self.delay,
        }
    }
}

impl<Led, Data> ClocklessDelayBuilder<Led, Data, ()> {
    pub fn with_delay<Delay>(self, delay: Delay) -> ClocklessDelayBuilder<Led, Data, Delay> {
        ClocklessDelayBuilder {
            led: self.led,
            data: self.data,
            delay,
        }
    }
}

impl<Led, Data, Delay> ClocklessDelayBuilder<Led, Data, Delay>
where
    Data: OutputPin,
    Led: ClocklessLed,
{
    pub fn build(self) -> ClocklessDelay<Led, Data, Delay> {
        ClocklessDelay::new(self.data, self.delay)
    }
}

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
/// use blinksy::{driver::clockless::{ClocklessDriver, ClocklessDelay}, leds::Ws2812};
///
/// fn setup_leds<Data, Delay>(data: Data, delay: Delay)
///     -> ClocklessDriver<Ws2812, ClocklessDelay<Ws2812, Data, Delay>>
/// where
///     Data: OutputPin,
///     Delay: DelayNs,
/// {
///     // Create a new WS2812 driver
///     let writer = ClocklessDelayBuilder::default()
///         .with_led::<Ws2812>()
///         .with_data(data)
///         .with_delay(delay)
///         .build();
///     ClocklessDriver::default()
///         .with_led::<Ws2812>()
///         .with_writer(writer)
/// }
/// ```
///
/// # Type Parameters
///
/// - `Led` - The LED protocol implementation (must implement ClocklessLed)
/// - `Data` - The GPIO pin type for data output (must implement OutputPin)
/// - `Delay` - The delay provider
pub struct ClocklessDelay<Led: ClocklessLed, Data: OutputPin, Delay> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,
    /// GPIO pin for data transmission
    data: Data,
    /// Delay provider for timing control
    delay: Delay,
}

impl<Led, Data, Delay> ClocklessDelay<Led, Data, Delay>
where
    Led: ClocklessLed,
    Data: OutputPin,
{
    /// Creates a new clockless LED driver.
    ///
    /// # Arguments
    ///
    /// - `data` - The GPIO pin for data output
    /// - `delay` - The delay provider for timing control
    ///
    /// Assumes data pin is already LOW.
    ///
    /// # Returns
    ///
    /// A new ClocklessDelayDriver instance or an error if pin initialization fails
    pub fn new(data: Data, delay: Delay) -> Self {
        Self {
            led: PhantomData,
            data,
            delay,
        }
    }
}

impl<Led, Data, Delay> ClocklessWriter<Led> for ClocklessDelay<Led, Data, Delay>
where
    Led: ClocklessLed,
    Led::Word: ToBytes,
    <Led::Word as ToBytes>::Bytes: IntoIterator<Item = u8>,
    Data: OutputPin,
    Delay: DelayNs,
{
    type Error = Data::Error;

    /// Transmits a buffer of bytes.
    ///
    /// # Arguments
    ///
    /// - `buffer` - The byte array to transmit
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
                    self.data.set_high()?;
                    self.delay.delay_ns(Led::T_0H.to_nanos());
                    self.data.set_low()?;
                    self.delay.delay_ns(Led::T_0L.to_nanos());
                } else {
                    // Transmit a '1' bit
                    self.data.set_high()?;
                    self.delay.delay_ns(Led::T_1H.to_nanos());
                    self.data.set_low()?;
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
impl<Led, Data, Delay> ClocklessWriterAsync<Led> for ClocklessDelay<Led, Data, Delay>
where
    Led: ClocklessLed,
    Led::Word: ToBytes,
    <Led::Word as ToBytes>::Bytes: IntoIterator<Item = u8>,
    Data: OutputPin,
    Delay: DelayNsAsync,
{
    type Error = Data::Error;

    /// Transmits a buffer of bytes.
    ///
    /// # Arguments
    ///
    /// - `buffer` - The byte array to transmit
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    async fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        for byte in frame {
            for bit in bits_of(&byte, BitOrder::MostSignificantBit) {
                if !bit {
                    // Transmit a '0' bit
                    self.data.set_high()?;
                    self.delay.delay_ns(Led::T_0H.to_nanos()).await;
                    self.data.set_low()?;
                    self.delay.delay_ns(Led::T_0L.to_nanos()).await;
                } else {
                    // Transmit a '1' bit
                    self.data.set_high()?;
                    self.delay.delay_ns(Led::T_1H.to_nanos()).await;
                    self.data.set_low()?;
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
