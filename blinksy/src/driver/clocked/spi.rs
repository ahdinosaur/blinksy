//! # Clocked SPI-based LED Driver
//!
//! This module provides an implementation of the LedDriver trait for clocked LEDs
//! using hardware SPI for data transmission. This allows for efficient, high-speed
//! LED updates using dedicated SPI peripherals.
//!
//! ## Benefits of SPI-based implementation
//!
//! - Higher data rates than bit-banging
//! - More efficient CPU usage
//! - Better timing precision
//!
//! ## Usage
//!
//! ```rust
//! use embedded_hal::spi::SpiBus;
//! use blinksy::{driver::ClockedSpiDriver, drivers::Apa102Led};
//!
//! fn setup_leds<S>(spi: S) -> ClockedSpiDriver<Apa102Led, S>
//! where
//!     S: SpiBus<u8>,
//! {
//!     // Create a new APA102 driver using SPI
//!     ClockedSpiDriver::<Apa102Led, _>::new(spi)
//! }
//! ```

use core::marker::PhantomData;
use embedded_hal::spi::SpiBus;

use super::{ClockedLed, ClockedWriter};
use crate::{
    color::{ColorCorrection, OutputColor},
    driver::LedDriver,
};

/// Driver for clocked LEDs using a hardware SPI peripheral.
///
/// # Type Parameters
///
/// * `Led` - The LED protocol implementation (must implement ClockedLed<Word=u8>)
/// * `Spi` - The SPI interface type
#[derive(Debug)]
pub struct ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
    /// Marker for the LED protocol type
    led: PhantomData<Led>,

    /// SPI interface for data transmission
    writer: Spi,
}

impl<Led, Spi> ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
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

impl<Led, Spi> LedDriver for ClockedSpiDriver<Led, Spi>
where
    Led: ClockedLed<Word = u8>,
    Spi: SpiBus<u8>,
{
    type Error = <Spi as ClockedWriter>::Error;

    /// Writes a sequence of colors to the LED chain using SPI.
    ///
    /// Delegates to the Led::clocked_write method to handle the protocol-specific details.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Iterator over colors
    /// * `brightness` - Global brightness scaling factor (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// Ok(()) on success or an error if SPI transmission fails
    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        C: OutputColor,
    {
        Led::clocked_write(&mut self.writer, pixels, brightness, gamma, correction)
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
