use core::{marker::PhantomData, slice::from_ref};
use embedded_hal::spi::SpiBus;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiBus as SpiBusAsync;

#[cfg(feature = "async")]
use crate::driver::DriverAsync;
use crate::{
    color::{ColorCorrection, FromColor},
    driver::{ClockedDriver, Driver},
};

#[cfg(feature = "async")]
use super::ClockedWriterAsync;
use super::{ClockedLed, ClockedWriter};

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
pub struct ClockedSpiDriver<Led, Spi>(ClockedDriver<Led, Spi>);

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
        Self(ClockedDriver {
            led: PhantomData,
            writer: spi,
        })
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
    /// Orchestrates start, per-LED frames, and end frame by generating frames
    /// from the `Led` implementation and sending them via SPI.
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
    fn write<const PIXEL_COUNT: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        self.0
            .write::<PIXEL_COUNT, _, _>(pixels, brightness, correction)
    }
}

#[cfg(feature = "async")]
impl<Led, Spi> DriverAsync for ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBusAsync<u8>,
{
    type Error = Spi::Error;
    type Color = Led::Color;

    /// Writes a sequence of colors to the LED chain using SPI, asynchronously.
    ///
    /// Orchestrates start, per-LED frames, and end frame by generating frames
    /// from the `Led` implementation and sending them via SPI.
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
    async fn write<const PIXEL_COUNT: usize, I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        // Start frame
        ClockedWriterAsync::write(&mut self.0.writer, Led::start()).await?;

        // Per-LED frames
        let mut pixel_count = 0;
        for color in pixels.into_iter() {
            let color = Led::Color::from_color(color);
            ClockedWriterAsync::write(&mut self.0.writer, Led::led(color, brightness, correction))
                .await?;
            pixel_count += 1;
        }

        // End frame
        ClockedWriterAsync::write(&mut self.0.writer, Led::end(pixel_count)).await?;

        Ok(())
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
        Words: IntoIterator<Item = Self::Word>,
    {
        for w in words {
            self.write(from_ref(&w))?;
        }
        Ok(())
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
        Words: IntoIterator<Item = Self::Word>,
    {
        for w in words {
            self.write(from_ref(&w)).await?;
        }
        Ok(())
    }
}
