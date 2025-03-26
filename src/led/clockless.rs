use fugit::NanosDurationU32 as Nanoseconds;

// Examples
// - WS2812B: https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf

pub trait LedClockless {
    const T_0H: Nanoseconds;
    const T_0L: Nanoseconds;
    const T_1H: Nanoseconds;
    const T_1L: Nanoseconds;
    const T_RESET: Nanoseconds;

    const OUTPUT_COUNT: usize;

    fn t_cycle() -> Nanoseconds {
        (Self::T_0H + Self::T_0L).max(Self::T_1H + Self::T_1L)
    }
}
