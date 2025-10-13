use embedded_hal::{delay::DelayNs, digital::OutputPin};
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as DelayNsAsync;
use num_traits::ToBytes;

use crate::{
    time::{Megahertz, Nanoseconds},
    util::bits::{bits_of, BitOrder, Word as WordTrait},
};

use super::ClockedWriter;
#[cfg(feature = "async")]
use super::ClockedWriterAsync;

/// Builder for [`ClockedDelay`].
pub struct ClockedDelayBuilder<Data, Clock, Delay, DataRate> {
    data: Data,
    clock: Clock,
    delay: Delay,
    data_rate: DataRate,
}

impl Default for ClockedDelayBuilder<(), (), (), ()> {
    fn default() -> Self {
        Self {
            data: (),
            clock: (),
            delay: (),
            data_rate: (),
        }
    }
}

impl<Clock, Delay, DataRate> ClockedDelayBuilder<(), Clock, Delay, DataRate> {
    pub fn with_data<Data: OutputPin>(
        self,
        data: Data,
    ) -> ClockedDelayBuilder<Data, Clock, Delay, DataRate> {
        ClockedDelayBuilder {
            data,
            clock: self.clock,
            delay: self.delay,
            data_rate: self.data_rate,
        }
    }
}
impl<Data, Delay, DataRate> ClockedDelayBuilder<Data, (), Delay, DataRate> {
    pub fn with_clock<Clock: OutputPin>(
        self,
        clock: Clock,
    ) -> ClockedDelayBuilder<Data, Clock, Delay, DataRate> {
        ClockedDelayBuilder {
            data: self.data,
            clock,
            delay: self.delay,
            data_rate: self.data_rate,
        }
    }
}

impl<Data, Clock, DataRate> ClockedDelayBuilder<Data, Clock, (), DataRate> {
    pub fn with_delay<Delay>(
        self,
        delay: Delay,
    ) -> ClockedDelayBuilder<Data, Clock, Delay, DataRate> {
        ClockedDelayBuilder {
            data: self.data,
            clock: self.clock,
            delay,
            data_rate: self.data_rate,
        }
    }
}

impl<Data, Clock, Delay> ClockedDelayBuilder<Data, Clock, Delay, ()> {
    pub fn with_data_rate(
        self,
        data_rate: Megahertz,
    ) -> ClockedDelayBuilder<Data, Clock, Delay, Megahertz> {
        ClockedDelayBuilder {
            data: self.data,
            clock: self.clock,
            delay: self.delay,
            data_rate,
        }
    }
}

impl<Data, Clock, Delay> ClockedDelayBuilder<Data, Clock, Delay, Megahertz>
where
    Data: OutputPin,
    Clock: OutputPin,
{
    pub fn build(self) -> ClockedDelay<Data, Clock, Delay> {
        ClockedDelay::new(self.data, self.clock, self.delay, self.data_rate)
    }
}

/// Writer for clocked LEDs using GPIO bit-banging with a delay timer.
///
/// - Separate GPIO pins for data and clock
/// - A delay provider for timing control
///
/// Tip: Use [`ClockedDelayBuilder`] to build your [`ClockedDelay`] struct.
///
/// ## Usage
///
/// ```rust
/// use embedded_hal::digital::OutputPin;
/// use embedded_hal::delay::DelayNs;
/// use blinksy::{
///     driver::clocked::{ClockedDelay, ClockedDelayBuilder, ClockedDriver},
///     leds::Apa102,
///     time::Megahertz,
/// };
///
/// fn setup_leds<D, C, Delay>(
///     data_pin: D,
///     clock_pin: C,
///     delay: Delay
/// ) -> ClockedDriver<Apa102, ClockedDelay<D, C, Delay>>
/// where
///     D: OutputPin,
///     C: OutputPin,
///     Delay: DelayNs,
/// {
///     // Create a new APA102 driver with 2 MHz data rate
///     ClockedDriver::default()
///         .with_led::<Apa102>()
///         .with_writer(ClockedDelayBuilder::default()
///             .with_data(data_pin)
///             .with_clock(clock_pin)
///             .with_delay(delay)
///             .with_data_rate(Megahertz::MHz(2))
///             .build()
///         )
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
    /// - `data` - The GPIO pin for data output
    /// - `clock` - The GPIO pin for clock output
    /// - `delay` - The delay provider for timing control
    /// - `data_rate` - The clock frequency in MHz
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

impl<Word, Data, Clock, Delay> ClockedWriter<Word> for ClockedDelay<Data, Clock, Delay>
where
    Word: WordTrait,
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
    /// - `words` - Iterator of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Word]>,
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
    Word: WordTrait,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNsAsync,
{
    type Error = ClockedDelayError<Data, Clock>;

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
    /// - `words` - Iterator of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    async fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Word]>,
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
