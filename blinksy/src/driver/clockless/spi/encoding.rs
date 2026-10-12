use crate::driver::ClocklessLed;
use crate::util::bits::{copy_bits_msb, BitSlice, BitSliceMut, Word};
use core::marker::PhantomData;

/// Represents the timing (in number of SPI bits) to encode zero, one, and reset LED signals
///
/// These timings are equivalent to those in [`ClocklessLed`] but converted to number of SPI bits for
/// a particular SPI frequency.
pub struct ClocklessSpiTiming<Led: ClocklessLed> {
    clock_period_ns: u32,
    /// Number of SPI bits for the zero signal high
    pub t_0h: u32,
    /// Number of SPI bits for the zero signal low
    pub t_0l: u32,
    /// Number of SPI bits for the one signal high
    pub t_1h: u32,
    /// Number of SPI bits for the one signal low
    pub t_1l: u32,
    /// Number of SPI bits for the reset signal
    pub t_reset: u32,
    led: PhantomData<Led>,
}

impl<Led: ClocklessLed> ClocklessSpiTiming<Led> {
    pub(crate) const fn new(clock_period_ns: u32) -> Self {
        Self {
            clock_period_ns,
            t_0h: Led::T_0H.to_nanos() / clock_period_ns,
            t_0l: Led::T_0L.to_nanos() / clock_period_ns,
            t_1h: Led::T_1H.to_nanos() / clock_period_ns,
            t_1l: Led::T_1L.to_nanos() / clock_period_ns,
            t_reset: Led::T_RESET.to_nanos() / clock_period_ns,
            led: PhantomData,
        }
    }

    const fn t0(&self) -> u32 {
        self.t_0h + self.t_0l
    }

    const fn t1(&self) -> u32 {
        self.t_1h + self.t_1l
    }

    /// Returns total number of SPI bits needed to send a zero or a one signal
    pub const fn duty_cycle_bits(&self) -> u32 {
        let t0 = self.t0();
        let t1 = self.t1();
        // We can't yet use max in const function
        if t0 > t1 {
            t0
        } else {
            t1
        }
    }

    /// Returns the maximum error in nanoseconds that this encoding
    /// has compared to the ideal timings in [`ClocklessLed`]
    pub const fn max_error_ns(&self) -> u32 {
        let error_0h_ns = Led::T_0H.to_nanos() - self.t_0h * self.clock_period_ns;
        let error_0l_ns = Led::T_0L.to_nanos() - self.t_0l * self.clock_period_ns;
        let error_1h_ns = Led::T_1H.to_nanos() - self.t_1h * self.clock_period_ns;
        let error_1l_ns = Led::T_1L.to_nanos() - self.t_1l * self.clock_period_ns;

        let mut max_error_ns = 0;
        if error_0h_ns > max_error_ns {
            max_error_ns = error_0h_ns
        };
        if error_0l_ns > max_error_ns {
            max_error_ns = error_0l_ns
        };
        if error_1h_ns > max_error_ns {
            max_error_ns = error_1h_ns
        };
        if error_1l_ns > max_error_ns {
            max_error_ns = error_1l_ns
        };
        max_error_ns
    }
}

pub(crate) struct PulseCode<W, const N: usize> {
    buffer: [W; N],
    len: usize,
}

impl<W: Word, const N: usize> PulseCode<W, N> {
    fn new(high: usize, low: usize) -> Self {
        let mut buffer = [W::ZERO; N];
        let high_bits = BitSlice {
            buffer: &[!W::ZERO; N],
            offset: 0,
            length: high,
        };
        copy_bits_msb(
            &high_bits,
            &mut BitSliceMut {
                buffer: &mut buffer,
                offset: 0,
                length: high_bits.length,
            },
        );
        Self {
            buffer,
            len: high + low,
        }
    }

    pub(crate) fn bits<'a>(&'a self) -> BitSlice<'a, W> {
        BitSlice {
            buffer: &self.buffer,
            offset: 0,
            length: self.len,
        }
    }
}

pub(crate) struct Pulses<W, const N: usize> {
    zero: PulseCode<W, N>,
    one: PulseCode<W, N>,
}

impl<W: Word, const N: usize> Pulses<W, N> {
    pub(crate) fn new<Led: ClocklessLed>(timing: &ClocklessSpiTiming<Led>) -> Self {
        Self {
            zero: PulseCode::new(timing.t_0h as usize, timing.t_0l as usize),
            one: PulseCode::new(timing.t_1h as usize, timing.t_1l as usize),
        }
    }

    pub(crate) fn get(&self, value: bool) -> &PulseCode<W, N> {
        if value {
            &self.one
        } else {
            &self.zero
        }
    }
}

pub(crate) const fn freq_hz_to_duration_ns(freq_hz: u32) -> u32 {
    1_000_000_000 / freq_hz
}

pub(crate) const fn duration_ns_to_freq_hz(duration_ns: u32) -> u32 {
    1_000_000_000 / duration_ns
}
