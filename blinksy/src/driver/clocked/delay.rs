use crate::{
    color::{ColorCorrection, FromColor},
    time::{Megahertz, Nanoseconds},
};

use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

use super::{ClockedLed, ClockedWriter, Driver};

/// Driver for clocked LEDs using GPIO bit-banging with a delay timer.
///
/// - Separate GPIO pins for data and clock
/// - A delay provider for timing control
/// - Parameters defined by a ClockedLed implementation
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
/// ) -> ClockedDelayDriver<Apa102Led, D, C, Delay>
/// where
///     D: OutputPin,
///     C: OutputPin,
///     Delay: DelayNs,
/// {
///     // Create a new APA102 driver with 2 MHz data rate
///     ClockedDelayDriver::<Apa102Led, _, _, _>::new(
///         data_pin,
///         clock_pin,
///         delay,
///         Megahertz::MHz(2)
///     )
/// }
/// ```
///
/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClockedLed)
/// * `Data` - The GPIO pin type for data output
/// * `Clock` - The GPIO pin type for clock output
/// * `Delay` - The delay provider
#[derive(Debug)]
pub struct ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    /// Marker for the LED protocol type
    led: PhantomData<Led>,
    /// Writer implementation for the clocked protocol
    writer: ClockedDelayWriter<Data, Clock, Delay>,
}

impl<Led, Data, Clock, Delay> ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    /// Creates a new clocked LED driver.
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
    /// A new ClockedDelayDriver instance
    pub fn new(data: Data, clock: Clock, delay: Delay, data_rate: Megahertz) -> Self {
        Self {
            led: PhantomData,
            writer: ClockedDelayWriter::new(data, clock, delay, data_rate),
        }
    }
}

impl<Led, Data, Clock, Delay> Driver for ClockedDelayDriver<Led, Data, Clock, Delay>
where
    Led: ClockedLed<Word = u8>,
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error = <ClockedDelayWriter<Data, Clock, Delay> as ClockedWriter>::Error;
    type Color = Led::Color;

    /// Writes a sequence of colors to the LED chain.
    ///
    /// Delegates to the Led::clocked_write method to handle the protocol-specific details.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `gamma` - Gamma correction factor
    /// * `correction` - Color correction factors
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
        Led::clocked_write(&mut self.writer, pixels, brightness, correction)
    }
}

/// Implementation of ClockedWriter using GPIO bit-banging with delays.
///
/// This type handles the low-level bit-banging of data and clock pins
/// to transmit data using a clocked protocol.
#[derive(Debug)]
pub struct ClockedDelayWriter<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
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

impl<Data, Clock, Delay> ClockedDelayWriter<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    /// Creates a new ClockedDelayWriter.
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
    /// A new ClockedDelayWriter instance
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

/// Error type for the ClockedDelayWriter.
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

impl<Data, Clock, Delay> ClockedWriter for ClockedDelayWriter<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error = ClockedDelayError<Data, Clock>;
    type Word = u8;

    /// Writes a slice of bytes using the bit-banging technique.
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
    /// * `words` - Slice of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if pin operation fails
    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error> {
        for byte in words {
            for bit_position in [128, 64, 32, 16, 8, 4, 2, 1] {
                match byte & bit_position {
                    0 => self.data.set_low(),
                    _ => self.data.set_high(),
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
