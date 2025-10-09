use embedded_hal::{delay::DelayNs, digital::OutputPin};
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as DelayNsAsync;
use num_traits::{PrimInt, ToBytes};

#[cfg(feature = "async")]
use crate::driver::DriverAsync;
use crate::{
    time::{Megahertz, Nanoseconds},
    util::bits::{bits_of, BitOrder},
};

use super::ClockedWriter;
#[cfg(feature = "async")]
use super::ClockedWriterAsync;

/// Writer for clocked LEDs using GPIO bit-banging with a delay timer.
///
/// - Separate GPIO pins for data and clock
/// - A delay provider for timing control
///
/// ## Usage
///
/// ```rust
/// use embedded_hal::digital::OutputPin;
/// use embedded_hal::delay::DelayNs;
/// use blinksy::{driver::ClockedDelayDriver, drivers::apa102::Apa102Led};
/// use blinksy::time::Megahertz;
///
/// fn setup_leds<D, C, Delay>(
///     data_pin: D,
///     clock_pin: C,
///     delay: Delay
/// ) -> ClockedDriver<Apa102Led, ClockedDelay<D, C, Delay>>
/// where
///     D: OutputPin,
///     C: OutputPin,
///     Delay: DelayNs,
/// {
///     // Create a new APA102 driver with 2 MHz data rate
///     ClockedDriver::default()
///         .with_led::<Apa102Led>()
///         .with_writer(ClockedDelay::new(
///             data_pin,
///             clock_pin,
///             delay,
///             Megahertz::MHz(2)
///         ))
/// }
/// ```
///
/// This type handles the low-level bit-banging of data and clock pins
/// to transmit data using a clocked protocol.
#[derive(Debug)]
pub struct ClockedDelay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
{
    /// GPIO pin for data transmission
    data: Data,
    /// GPIO pin for clock signal
    clock: Clock,
    /// Delay provider for timing control
    delay: Delay,
    /// Half-cycle duration in nanoseconds
    t_half_cycle_ns: u32,
}

impl<Data, Clock, Delay> ClockedDelay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
{
    /// Creates a new ClockedDelay.
    ///
    /// # Arguments
    ///
    /// * `data` - The GPIO pin for data output
    /// * `clock` - The GPIO pin for clock output
    /// * `delay` - The delay provider for timing control
    /// * `data_rate` - The clock frequency in MHz
    ///
    /// # Returns
    ///
    /// A new ClockedDelay instance
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

/// Error type for the ClockedDelay.
///
/// This enum wraps errors from the data and clock pins to provide
/// a unified error type for the writer.
#[derive(Debug)]
pub enum ClockedDelayError<Data, Clock>
where
    Data: OutputPin,
    Clock: OutputPin,
{
    /// Error from the data pin
    Data(Data::Error),
    /// Error from the clock pin
    Clock(Clock::Error),
}

impl<Data, Clock, Delay> ClockedWriter for ClockedDelay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error = ClockedDelayError<Data, Clock>;

    /// Writes an iterator of bytes using the bit-banging technique.
    ///
    /// For each bit:
    /// 1. Sets the data line to the bit value
    /// 2. Waits for half a clock cycle
    /// 3. Sets the clock line high
    /// 4. Waits for half a clock cycle
    /// 5. Sets the clock line low
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    fn write<Word, Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Word]>,
        Word: ToBytes,
        Word::Bytes: IntoIterator<Item = u8>,
    {
        for word in words.as_ref() {
            for bit in bits_of(word, BitOrder::MostSignificantBit) {
                match bit {
                    false => self.data.set_low(),
                    true => self.data.set_high(),
                }
                .map_err(ClockedDelayError::Data)?;

                self.delay.delay_ns(self.t_half_cycle_ns);
                self.clock.set_high().map_err(ClockedDelayError::Clock)?;
                self.delay.delay_ns(self.t_half_cycle_ns);
                self.clock.set_low().map_err(ClockedDelayError::Clock)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<Word, Data, Clock, Delay> ClockedWriterAsync<Word> for ClockedDelay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNsAsync,
{
    type Error = ClockedDelayError<Data, Clock>;
    type Word = u8;

    /// Writes an iterator of bytes using the bit-banging technique, asynchronously.
    ///
    /// For each bit:
    /// 1. Sets the data line to the bit value
    /// 2. Waits for half a clock cycle
    /// 3. Sets the clock line high
    /// 4. Waits for half a clock cycle
    /// 5. Sets the clock line low
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    async fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Self::Word]>,
    {
        for word in words.as_ref() {
            for bit in bits_of(word, BitOrder::MostSignificantBit) {
                match bit {
                    false => self.data.set_low(),
                    true => self.data.set_high(),
                }
                .map_err(ClockedDelayError::Data)?;

                self.delay.delay_ns(self.t_half_cycle_ns).await;
                self.clock.set_high().map_err(ClockedDelayError::Clock)?;
                self.delay.delay_ns(self.t_half_cycle_ns).await;
                self.clock.set_low().map_err(ClockedDelayError::Clock)?;
            }
        }

        Ok(())
    }
}
