use core::marker::PhantomData;

use embedded_hal::spi::SpiBus;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiBus as SpiBusAsync;

use crate::driver::ClocklessLed;
use crate::driver::ClocklessSpi;
#[cfg(feature = "async")]
use crate::driver::ClocklessSpiAsync;
use crate::markers::{Async, Blocking};
use crate::util::bits::Word;

/// Builder for [`ClocklessSpi`] and [`ClocklessSpiAsync`]
pub struct ClocklessSpiBuilder<
    const BUFFER_SIZE: usize,
    const PULSE_SIZE: usize,
    Spi,
    Led,
    SpiWord,
    Freq,
    Exec,
> {
    spi: Spi,
    freq_hz: Freq,
    led: PhantomData<Led>,
    spi_word: PhantomData<SpiWord>,
    exec: PhantomData<Exec>,
}

impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Spi, Led, SpiWord, Freq>
    ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, Freq, Blocking>
{
    pub fn with_async(
        self,
    ) -> ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, Freq, Async> {
        ClocklessSpiBuilder {
            spi: self.spi,
            freq_hz: self.freq_hz,
            led: PhantomData,
            spi_word: PhantomData,
            exec: PhantomData,
        }
    }
}

impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Spi, Led, SpiWord, Dm>
    ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, (), Dm>
{
    pub fn with_freq_hz(
        self,
        freq_hz: u32,
    ) -> ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, u32, Dm> {
        ClocklessSpiBuilder {
            spi: self.spi,
            freq_hz,
            led: PhantomData,
            spi_word: PhantomData,
            exec: PhantomData,
        }
    }
}

impl<const BUFFER_SIZE: usize, Spi, Led, SpiWord, Freq, Dm>
    ClocklessSpiBuilder<BUFFER_SIZE, 0, Spi, Led, SpiWord, Freq, Dm>
{
    pub fn with_pulse_size<const PULSE_SIZE: usize>(
        self,
    ) -> ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, Freq, Dm> {
        ClocklessSpiBuilder {
            spi: self.spi,
            freq_hz: self.freq_hz,
            led: PhantomData,
            spi_word: PhantomData,
            exec: PhantomData,
        }
    }
}

impl<const N: usize, const PULSE_SIZE: usize, Spi, Led, SpiWord, Freq, Dm>
    ClocklessSpiBuilder<N, PULSE_SIZE, Spi, Led, SpiWord, Freq, Dm>
{
    pub fn with_buffer_size<const BUFFER_SIZE: usize>(
        self,
    ) -> ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, Freq, Dm> {
        ClocklessSpiBuilder {
            spi: self.spi,
            freq_hz: self.freq_hz,
            led: PhantomData,
            spi_word: PhantomData,
            exec: PhantomData,
        }
    }
}

impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, SpiWord, Freq, Dm>
    ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, (), Led, SpiWord, Freq, Dm>
{
    pub fn with_spi<Spi>(
        self,
        spi: Spi,
    ) -> ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, Freq, Dm> {
        ClocklessSpiBuilder {
            spi,
            freq_hz: self.freq_hz,
            led: PhantomData,
            spi_word: PhantomData,
            exec: PhantomData,
        }
    }
}

impl<Led, SpiWord> Default for ClocklessSpiBuilder<0, 0, (), Led, SpiWord, (), Blocking> {
    fn default() -> Self {
        Self {
            spi: (),
            freq_hz: (),
            led: PhantomData,
            spi_word: PhantomData,
            exec: PhantomData,
        }
    }
}

impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Spi, Led, SpiWord>
    ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, u32, Blocking>
where
    Spi: SpiBus<SpiWord>,
    SpiWord: Word + 'static,
    Led: ClocklessLed,
{
    pub fn build(self) -> ClocklessSpi<BUFFER_SIZE, PULSE_SIZE, Led, Spi, SpiWord> {
        assert!(BUFFER_SIZE > 0, "set buffer_size before calling build");
        assert!(PULSE_SIZE > 0, "set pulse_size before calling build");
        ClocklessSpi::new(self.spi, self.freq_hz)
    }
}

#[cfg(feature = "async")]
impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Spi, Led, SpiWord>
    ClocklessSpiBuilder<BUFFER_SIZE, PULSE_SIZE, Spi, Led, SpiWord, u32, Async>
where
    Spi: SpiBusAsync<SpiWord>,
    SpiWord: Word + 'static,
    Led: ClocklessLed,
{
    pub fn build(self) -> ClocklessSpiAsync<BUFFER_SIZE, PULSE_SIZE, Led, Spi, SpiWord> {
        assert!(BUFFER_SIZE > 0, "set buffer_size before calling build");
        assert!(PULSE_SIZE > 0, "set pulse_size before calling build");
        ClocklessSpiAsync::new(self.spi, self.freq_hz)
    }
}
