use fugit::NanosDurationU32 as Nanoseconds;

mod delay;

pub use self::delay::*;

pub trait ClocklessLed {
    const T_0H: Nanoseconds;
    const T_0L: Nanoseconds;
    const T_1H: Nanoseconds;
    const T_1L: Nanoseconds;
    const T_RESET: Nanoseconds;

    // TODO Update so can handle RGBW too.
    type Word: Copy + Default;
    type ColorBytes: AsRef<[Self::Word]> + Copy;
    fn reorder_color_bytes(bytes: Self::ColorBytes) -> Self::ColorBytes;

    fn t_cycle() -> Nanoseconds {
        (Self::T_0H + Self::T_0L).max(Self::T_1H + Self::T_1L)
    }
}
