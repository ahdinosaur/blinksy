use fugit::NanosDurationU32 as Nanoseconds;

use crate::driver::{ClocklessDelayDriver, ClocklessLed, ColorChannels, RgbChannels};

pub struct Ws2812Led;

// WS2812B docs:
// - https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf
impl ClocklessLed for Ws2812Led {
    const T_0H: Nanoseconds = Nanoseconds::nanos(400);
    const T_0L: Nanoseconds = Nanoseconds::nanos(850);
    const T_1H: Nanoseconds = Nanoseconds::nanos(800);
    const T_1L: Nanoseconds = Nanoseconds::nanos(450);
    const T_RESET: Nanoseconds = Nanoseconds::micros(50);

    const COLOR_CHANNELS: ColorChannels = ColorChannels::Rgb(RgbChannels::GRB);
}

pub type Ws2812Delay<Pin, Delay> = ClocklessDelayDriver<Ws2812Led, Pin, Delay>;
