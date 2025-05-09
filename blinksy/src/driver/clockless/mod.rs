use crate::time::Nanoseconds;

mod delay;

pub use self::delay::*;

use crate::color::ColorChannels;

pub trait ClocklessLed {
    const T_0H: Nanoseconds;
    const T_0L: Nanoseconds;
    const T_1H: Nanoseconds;
    const T_1L: Nanoseconds;
    const T_RESET: Nanoseconds;

    const COLOR_CHANNELS: ColorChannels;

    fn t_cycle() -> Nanoseconds {
        (Self::T_0H + Self::T_0L).max(Self::T_1H + Self::T_1L)
    }
}
