//! # SK6812 LED Driver
//!
//! This module provides driver support for SK6812 LEDs, which use a
//! single-wire, timing-sensitive protocol similar to [`super::ws2812`].
//!
//! # Drivers
//!
//! - [`Sk6812Delay`]: Uses bit-banged GPIO
//! - [`blinksy-esp::Sk6812Rmt`]: On ESP devices, uses RMT peripheral
//!
//! ## Key Features
//!
//! - Single-wire protocol (data only, no clock)
//! - 32-bit color (8 bits per channel)
//! - Timing-sensitive protocol
//!
//! ## Protocol Details
//!
//! The SK6812 protocol uses precise timing of pulses on a single data line:
//!
//! - A '0' bit is represented by a short high pulse (~300ns) followed by a long low pulse (~900ns)
//! - A '1' bit is represented by a long high pulse (~600ns) followed by a long low pulse (~600ns)
//! - After sending all bits, a reset pulse of at least 80µs is required
//!
//! (References: [Datasheet](https://cdn-shop.adafruit.com/product-files/2757/p2757_SK6812RGBW_REV01.pdf))
//!
//! Each LED receives 32 bits (RGBW) and then passes subsequent data to the next LED in the chain.
//!
//! [`blinksy-esp::Sk6812Rmt`]: https://docs.rs/blinksy-esp/0.10/blinksy_esp/type.Sk6812Rmt.html

use fugit::NanosDurationU32 as Nanoseconds;

use crate::{
    color::LedChannels,
    driver::{ClocklessDelayDriver, ClocklessLed},
};

/// LED implementation for SK6812 protocol.
///
/// This type implements the ClocklessLed trait with the specifics of the SK6812 protocol,
/// including timing requirements and color channel ordering.
pub struct Sk6812Led;

impl Sk6812Led {
    // TODO make generic across all clockless leds
    pub const fn frame_buffer_size(pixel_count: usize) -> usize {
        pixel_count * 3
    }
}

impl ClocklessLed for Sk6812Led {
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

/// SK6812 driver using GPIO bit-banging with delay timing.
///
/// # Type Parameters
///
/// * `Pin` - The data pin type
/// * `Delay` - The delay implementation type
///
/// Note: This will not work unless your delay timer is able to handle microsecond
/// precision, which most microcontrollers cannot do.
pub type Sk6812Delay<Pin, Delay> = ClocklessDelayDriver<Sk6812Led, Pin, Delay>;
