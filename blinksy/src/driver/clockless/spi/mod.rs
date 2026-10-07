use core::marker::PhantomData;

use bitvec::prelude::*;
use bitvec::view::BitView;
use bitvec::view::BitViewSized;
use embedded_hal::spi::SpiBus;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiBus as SpiBusAsync;

use crate::driver::t_cycle;
#[cfg(feature = "async")]
use crate::driver::ClocklessWriterAsync;
use crate::driver::{ClocklessLed, ClocklessWriter};

mod encoding;
use encoding::{duration_ns_to_freq_hz, freq_hz_to_duration_ns, Pulses, Timing};
mod builder;
pub use builder::ClocklessSpiBuilder;

pub const fn clockless_spi_buffer_size<Led: ClocklessLed, Spi, Word>(
    pixel_count: usize,
    freq_hz: u32,
) -> usize
where
    Spi: SpiBus<Word>,
    Word: Copy + 'static,
{
    let clock_period_ns = freq_hz_to_duration_ns(freq_hz);
    let timing = Timing::<Led>::new(clock_period_ns);

    // TODO: Check that resulting timings are within spec for the LED and error if not
    let spi_word_bits = size_of::<Word>() * 8;
    let total_bits = spi_word_bits
        * pixel_count
        * Led::LED_CHANNELS.channel_count()
        * timing.duty_cycle_bits() as usize
        + timing.t_reset as usize;
    total_bits.div_ceil(spi_word_bits)
}

pub const fn clockless_spi_pulse_size<Led: ClocklessLed, Spi, Word>(freq_hz: u32) -> usize
where
    Spi: SpiBus<Word>,
    Word: Copy + 'static,
{
    let clock_period_ns = freq_hz_to_duration_ns(freq_hz);
    let timing = Timing::<Led>::new(clock_period_ns);
    let spi_word_bits = size_of::<Word>() * 8;

    let total_bits = timing.duty_cycle_bits() as usize;
    total_bits.div_ceil(spi_word_bits)
}

// Brute-force an "ideal" clock frequency to run the SPI bus at
// Lower frequencies are better because they mean that we need fewer spi bits to
// encode a single LED bit which means less processing and memory usage. However
// lower frequencies increase the timing errors.
// Each LED has a tolerance for timing variations. We take advantage of this to pick
// the lowest clock frequency that gives us errors within our chosen target tolerance.
pub const fn clockless_spi_ideal_frequency_hz<Led: ClocklessLed>(target_tolerance_ns: u32) -> u32 {
    // There's going to be some smart ways of doing this but for the time being
    // let's just do the simplest possible thing and explore a whole range of timings
    // and see what works best.
    let t_cycle_ns = t_cycle::<Led>().to_nanos();
    let mut max_clock_period_ns = 0;
    let mut clock_period_ns = 1;
    loop {
        let error_ns = Timing::<Led>::new(clock_period_ns).max_error_ns();
        if error_ns < target_tolerance_ns && clock_period_ns > max_clock_period_ns {
            max_clock_period_ns = clock_period_ns;
        }
        clock_period_ns += 1;
        if clock_period_ns >= t_cycle_ns {
            break;
        }
    }
    duration_ns_to_freq_hz(max_clock_period_ns)
}

fn encode_spi_buffer<
    Led: ClocklessLed,
    const FRAME_BUFFER_SIZE: usize,
    const SPI_BUFFER_SIZE: usize,
    const N: usize,
    SpiWord,
>(
    frame: heapless::Vec<Led::Word, FRAME_BUFFER_SIZE>,
    buffer: &mut BitArray<[SpiWord; SPI_BUFFER_SIZE], Msb0>,
    pulses: &Pulses<N>,
) where
    [SpiWord; SPI_BUFFER_SIZE]: BitViewSized,
    Led::Word: BitView,
{
    let mut dest = buffer.as_mut_bitslice();
    for v in frame {
        for bit in v.view_bits::<Msb0>() {
            let pattern = pulses.get(*bit);
            dest[..pattern.len()].clone_from_bitslice(pattern.bits());
            dest = &mut dest[pattern.len()..]
        }
    }
}

pub struct ClocklessSpi<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    Spi: SpiBus<SpiWord>,
    SpiWord: Copy + 'static,
{
    spi: Spi,
    pub timing: Timing<Led>,
    pulses: Pulses<PULSE_SIZE>,
    _spi_word: PhantomData<SpiWord>,
}

#[cfg(feature = "async")]
pub struct ClocklessSpiAsync<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    Spi: SpiBusAsync<SpiWord>,
    SpiWord: Copy + 'static,
{
    spi: Spi,
    pub timing: Timing<Led>,
    pulses: Pulses<PULSE_SIZE>,
    _spi_word: PhantomData<SpiWord>,
}

impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord>
    ClocklessSpi<BUFFER_SIZE, PULSE_SIZE, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    Spi: SpiBus<SpiWord>,
    SpiWord: Copy + 'static,
{
    pub fn new(spi: Spi, freq_hz: u32) -> Self {
        let timing = Timing::new(freq_hz_to_duration_ns(freq_hz));
        Self {
            spi,
            pulses: Pulses::new(&timing),
            timing,
            _spi_word: PhantomData,
        }
    }
}

#[cfg(feature = "async")]
impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord>
    ClocklessSpiAsync<BUFFER_SIZE, PULSE_SIZE, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    SpiWord: Copy + 'static,
    Spi: SpiBusAsync<SpiWord>,
{
    pub fn new(spi: Spi, freq_hz: u32) -> Self {
        let timing = Timing::new(freq_hz_to_duration_ns(freq_hz));
        Self {
            spi,
            pulses: Pulses::new::<Led>(&timing),
            timing,
            _spi_word: PhantomData,
        }
    }
}

impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord> ClocklessWriter<Led>
    for ClocklessSpi<BUFFER_SIZE, PULSE_SIZE, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    Led::Word: BitView,
    Spi: SpiBus<SpiWord>,
    SpiWord: Copy + 'static,
    [SpiWord; BUFFER_SIZE]: BitViewSized,
{
    type Error = Spi::Error;

    fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: heapless::Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        let mut buffer = BitArray::<[SpiWord; BUFFER_SIZE], Msb0>::ZERO;
        encode_spi_buffer::<Led, _, _, _, _>(frame, &mut buffer, &self.pulses);
        self.spi.write(&buffer.into_inner())
    }
}

#[cfg(feature = "async")]
impl<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord> ClocklessWriterAsync<Led>
    for ClocklessSpiAsync<BUFFER_SIZE, PULSE_SIZE, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    Led::Word: BitView,
    Spi: SpiBusAsync<SpiWord>,
    SpiWord: Copy + 'static,
    [SpiWord; BUFFER_SIZE]: BitViewSized,
{
    type Error = Spi::Error;

    async fn write<const FRAME_BUFFER_SIZE: usize>(
        &mut self,
        frame: heapless::Vec<Led::Word, FRAME_BUFFER_SIZE>,
    ) -> Result<(), Self::Error> {
        let mut buffer = BitArray::<[SpiWord; BUFFER_SIZE], Msb0>::ZERO;
        encode_spi_buffer::<Led, _, _, _, _>(frame, &mut buffer, &self.pulses);
        self.spi.write(&buffer.into_inner()).await
    }
}
