#![warn(missing_docs)]
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

/// Calculates the buffer size required for encoding one frame into bits that are sent to SPI
///
/// # Usage
///
/// ```
/// clockless_spi_buffer_size::<Ws2812, SpiDma<Async>, _>(Layout::PIXEL_COUNT, SPI_FREQ_HZ)
/// ```
///
/// # Type Arguments
///
/// - `Led` - The LED protocol implementation (must implement ClocklessLed)
/// - `Spi` - The SPI driver you're using (must implement SpiBus)
/// - `Word` - The word type of the Spi driver (should be able to be inferred from Spi)
///
/// # Arguments
///
/// - `pixel_count` - Number of pixels
/// - `freq_hz` - Clock frequency (in Hz) that SPI is running at
///
/// # Returns
///
/// Buffer size is in units of the SPI word size
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

/// Calculates the size required for encoding one bit into the SPI buffer.
///
/// # Usage
///
/// ```
/// clockless_spi_pulse_size::<Ws2812, SpiDma<Async>, _>(SPI_FREQ_HZ)
/// ```
///
/// # Type Arguments
///
/// - `Led` - The LED protocol implementation (must implement ClocklessLed)
/// - `Spi` - The SPI driver you're using (must implement SpiBus)
/// - `Word` - The word type of the Spi driver (should be able to be inferred from Spi)
///
/// # Arguments
///
/// - `freq_hz` - Clock frequency (in Hz) that SPI is running at
///
/// # Returns
///
/// Size is in units of the SPI word size
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

/// Writer for clockless LEDs using [SPI](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface)
///
/// This works for any SPI driver that implements the [`embedded_hal::spi::SpiBus`] or [`embedded_hal_async::spi::SpiBus`] traits.
///
/// # How this works
///
/// Given an SPI that you have set to a particular frequency we figure out how to send the data out to best match the timings required by the LED.
/// In general, it's not possible to *exactly* match the timings. However, there is a timing tolerance (in the datasheets) that allows for a certain
/// amount of error. With [`clockless_spi_ideal_frequency_hz`] you can figure out the ideal frequency to run your SPI at given a certain
/// acceptable timing tolerance. A good starting point is the tolerance from the datasheet. For example in the case of WS2812B that is 150 ns.
/// The higher the tolerance that you and your hardware allows, the smaller number of bits in the SPI buffer that each bit can be encoded in and the
/// less memory and processing you will use.
///
/// # Usage
///
/// ```
/// const SPI_FREQ_HZ: u32 = clockless_spi_ideal_frequency_hz::<Ws2812>(150);
///
/// // Create your spi driver (platform dependent) that implements the embedded-hal SpiBus trait.
/// // Set its frequency to SPI_FREQ_HZ and configure the MOSI (master out slave in) pin.
/// // The other SPI pins are not needed.
/// //
/// // let spi = ...
///
/// let writer = ClocklessSpiBuilder::default()
///    .with_spi(spi)
///    .with_freq_hz(SPI_FREQ_HZ)
///    // Note that Spi should be the type of spi above
///    .with_buffer_size::<{ clockless_spi_buffer_size::<Ws2812, Spi, _>(Layout::PIXEL_COUNT, SPI_FREQ_HZ) }>()
///    .with_pulse_size::<{ clockless_spi_pulse_size::<Ws2812, Spi, _>(SPI_FREQ_HZ) }>()
///    .build();
///
/// let driver = ClocklessDriver::default()
///     .with_led::<Ws2812>()
///     .with_writer(writer);
/// ```
pub struct ClocklessSpi<const BUFFER_SIZE: usize, const PULSE_SIZE: usize, Led, Spi, SpiWord>
where
    Led: ClocklessLed,
    Spi: SpiBus<SpiWord>,
    SpiWord: Copy + 'static,
{
    spi: Spi,
    pub timing: Timing<Led>,
    pulses: Pulses<PULSE_SIZE>,
    spi_word: PhantomData<SpiWord>,
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
    spi_word: PhantomData<SpiWord>,
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
            spi_word: PhantomData,
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
            spi_word: PhantomData,
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
