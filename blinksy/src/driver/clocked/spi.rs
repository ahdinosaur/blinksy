use core::marker::PhantomData;
use embedded_hal::spi::SpiBus;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiBus as SpiBusAsync;

use crate::{
    color::{ColorCorrection, FromColor},
    driver::{Driver, DriverAsync},
};

use super::{ClockedLed, ClockedLedAsync, ClockedWriter, ClockedWriterAsync};

/// Driver for clocked LEDs using a hardware SPI peripheral.
///
/// - Separate GPIO pins for data and clock
/// - A dedicated hardware SPI perhipheral for data transmission
///   - Higher data rates than bit-banging
///   - More efficient CPU usage
///   - Better timing precision
/// - Parameters defined by a ClockedLed implementation
///
/// ## Usage
///
/// ```rust
/// use embedded_hal::spi::SpiBus;
/// use blinksy::{driver::ClockedSpiDriver, drivers::apa102::Apa102Led};
///
/// fn setup_leds<S>(spi: S) -> ClockedSpiDriver<Apa102Led, S>
/// where
///     S: SpiBus<u8>,
/// {
///     // Create a new APA102 driver using SPI
///     ClockedSpiDriver::<Apa102Led, _>::new(spi)
/// }
/// ```
///
/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClockedLed<Word=u8>)
/// * `Spi` - The SPI interface type
#[derive(Debug)]
pub struct ClockedSpiDriver<Led, Spi> {
    /// Marker for the LED protocol type
    led: PhantomData<Led>,

    /// SPI interface for data transmission
    writer: Spi,
}

impl<Led, Spi> ClockedSpiDriver<Led, Spi> {
    /// Creates a new SPI-based clocked LED driver.
    ///
    /// # Arguments
    ///
    /// * `spi` - The SPI interface to use for data transmission
    ///
    /// # Returns
    ///
    /// A new ClockedSpiDriver instance
    pub fn new(spi: Spi) -> Self {
        Self {
            led: PhantomData,
            writer: spi,
        }
    }
}

impl<Led, Spi> Driver for ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
    type Error = Spi::Error;
    type Color = Led::Color;

    /// Writes a sequence of colors to the LED chain using SPI.
    ///
    /// Delegates to the Led::clocked_write method to handle the protocol-specific details.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
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

impl<Led, Spi> DriverAsync for ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLedAsync<Word = u8>,
    Spi: SpiBusAsync<u8>,
{
    type Error = Spi::Error;
    type Color = Led::Color;

    /// Writes a sequence of colors to the LED chain using SPI, asynchronously.
    ///
    /// Delegates to the Led::clocked_write method to handle the protocol-specific details.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    /// * `correction` - Color correction factors
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
    async fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        Led::clocked_write_async(&mut self.writer, pixels, brightness, correction).await
    }
}

/// Implementation of ClockedWriter for SPI interfaces.
///
/// This allows any type implementing the SpiBus trait to be used
/// as a writer for clocked LED protocols.
impl<Spi> ClockedWriter for Spi
where
    Spi: SpiBus<u8>,
{
    type Error = Spi::Error;
    type Word = u8;

    /// Writes a slice of bytes using the SPI interface.
    ///
    /// # Arguments
    ///
    /// * `words` - Slice of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
    fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error> {
        self.write(words)
    }
}

/// Implementation of ClockedWriterAsync for SPI interfaces.
///
/// This allows any type implementing the SpiBus trait to be used
/// as a writer for clocked LED protocols.
impl<Spi> ClockedWriterAsync for Spi
where
    Spi: SpiBusAsync<u8>,
{
    type Error = Spi::Error;
    type Word = u8;

    /// Writes a slice of bytes using the SPI interface.
    ///
    /// # Arguments
    ///
    /// * `words` - Slice of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
    async fn write(&mut self, words: &[Self::Word]) -> Result<(), Self::Error> {
        self.write(words).await
    }
}
