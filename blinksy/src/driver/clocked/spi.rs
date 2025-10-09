use core::marker::PhantomData;
use embedded_hal::spi::SpiBus;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiBus as SpiBusAsync;
use heapless::Vec;

#[cfg(feature = "async")]
use crate::driver::DriverAsync;
use crate::{
    color::{ColorCorrection, FromColor},
    driver::{ClockedDriver, Driver},
};

#[cfg(feature = "async")]
use super::ClockedWriterAsync;
use super::{ClockedLed, ClockedWriter};

/// Writer for clocked LEDs using a hardware SPI peripheral.
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
///     ClockedDriver::default()
///         .with_led::<Apa102Led>()
///         .with_writer(spi)
/// }
/// ```
///
/// # Type Parameters
///
/// * `Spi` - The SPI interface type
///
/// This allows any type implementing the SpiBus trait to be used
/// as a writer for clocked LED protocols.
impl<Spi> ClockedWriter for Spi
where
    Spi: SpiBus<u8>,
{
    type Error = Spi::Error;
    type Word = u8;

    /// Writes an iterator of bytes using the SPI interface.
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
    fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Self::Word]>,
    {
        self.write(words.as_ref())
    }
}

/// Implementation of ClockedWriterAsync for SPI interfaces.
///
/// This allows any type implementing the SpiBus trait to be used
/// as a writer for clocked LED protocols.
#[cfg(feature = "async")]
impl<Spi> ClockedWriterAsync for Spi
where
    Spi: SpiBusAsync<u8>,
{
    type Error = Spi::Error;
    type Word = u8;

    /// Writes an iterator of bytes using the SPI interface.
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of bytes to write
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
    async fn write<Words>(&mut self, words: Words) -> Result<(), Self::Error>
    where
        Words: AsRef<[Self::Word]>,
    {
        self.write(words.as_ref()).await
    }
}
