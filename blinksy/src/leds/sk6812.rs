use fugit::NanosDurationU32 as Nanoseconds;

use crate::{color::LedChannels, driver::ClocklessLed};

/// # SK6812 LEDs
///
/// This type describes the SK6812 LEDs, which are similar to [`super::ws2812`] but with white: RGB + W.
///
/// # Driver
///
/// - [`ClocklessDriver`](crate::driver::ClocklessDriver)
///
/// ## Key Features
///
/// - Single-wire [clockless protocol](crate::driver::clockless) (data only, no clock)
/// - 32-bit color (8 bits per channel, 4 channels)
pub struct Sk6812;

impl Sk6812 {
    pub const fn frame_buffer_size(pixel_count: usize) -> usize {
        super::clockless_frame_buffer_size(pixel_count)
    }
}

/// ## Protocol Details
///
/// The SK6812 protocol uses precise timing of pulses on a single data line:
///
/// - A '0' bit is represented by a short high pulse (~300ns) followed by a long low pulse (~900ns)
/// - A '1' bit is represented by a long high pulse (~600ns) followed by a long low pulse (~600ns)
/// - After sending all bits, a reset pulse of at least 80µs is required
///
/// (References: [Datasheet](https://cdn-shop.adafruit.com/product-files/2757/p2757_SK6812RGBW_REV01.pdf))
///
/// Each LED receives 32 bits (RGBW) and then passes subsequent data to the next LED in the chain.
impl ClocklessLed for Sk6812 {
    type Word = u8;

    /// Duration of high signal for '0' bit (~300ns)
    const T_0H: Nanoseconds = Nanoseconds::nanos(300);

    /// Duration of low signal for '0' bit (~900ns)
    const T_0L: Nanoseconds = Nanoseconds::nanos(900);

    /// Duration of high signal for '1' bit (~600ns)
    const T_1H: Nanoseconds = Nanoseconds::nanos(600);

    /// Duration of low signal for '1' bit (~600ns)
    const T_1L: Nanoseconds = Nanoseconds::nanos(600);

    /// Reset period (>80µs) - signals the end of a data stream
    const T_RESET: Nanoseconds = Nanoseconds::micros(80);

    /// LED channel specification - SK6812 uses RGBW ordering
    const LED_CHANNELS: LedChannels = LedChannels::Rgbw(crate::color::RgbwChannels::RBGW);
}
