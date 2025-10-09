//! # WS2812 LED Driver
//!
//! This module provides driver support for WS2812 (NeoPixel) LEDs, which use a
//! single-wire, timing-sensitive protocol. WS2812 LEDs are widely used due to their
//! simplicity and low cost.
//!
//! # Driver
//!
//! - [`ClocklessDriver`](crate::driver::ClocklessDriver)
//!
//! ## Key Features
//!
//! - Single-wire [clockless protocol](crate::driver::clockless) (data only, no clock)
//! - 24-bit color (8 bits per channel)
//! - Timing-sensitive protocol
//! - Fixed update rate: 30μs per pixel
//!
//! ## Protocol Details
//!
//! The WS2812 protocol uses precise timing of pulses on a single data line:
//!
//! - A '0' bit is represented by a short high pulse (~400ns) followed by a long low pulse (~850ns)
//! - A '1' bit is represented by a long high pulse (~800ns) followed by a short low pulse (~450ns)
//! - After sending all bits, a reset pulse of at least 50µs is required
//!
//! (References: [Datasheet](https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf))
//!
//! Each LED receives 24 bits (RGB) and then passes subsequent data to the next LED in the chain.
//!
//! [`blinksy-esp::ClocklessRmt`]: https://docs.rs/blinksy-esp/0.10/blinksy_esp/type.ClocklessRmt.html

use fugit::NanosDurationU32 as Nanoseconds;

use crate::{
    color::{LedChannels, RgbChannels},
    driver::ClocklessLed,
};

/// LED implementation for WS2812 protocol.
///
/// This type implements the ClocklessLed trait with the specifics of the WS2812 protocol,
/// including timing requirements and color channel ordering.
pub struct Ws2812;

impl Ws2812 {
    pub const fn frame_buffer_size(pixel_count: usize) -> usize {
        super::clockless_frame_buffer_size(pixel_count)
    }
}

impl ClocklessLed for Ws2812 {
    type Word = u8;

    /// Duration of high signal for '0' bit (~400ns)
    const T_0H: Nanoseconds = Nanoseconds::nanos(400);

    /// Duration of low signal for '0' bit (~850ns)
    const T_0L: Nanoseconds = Nanoseconds::nanos(850);

    /// Duration of high signal for '1' bit (~800ns)
    const T_1H: Nanoseconds = Nanoseconds::nanos(800);

    /// Duration of low signal for '1' bit (~450ns)
    const T_1L: Nanoseconds = Nanoseconds::nanos(450);

    /// Reset period (>50µs) - signals the end of a data stream
    const T_RESET: Nanoseconds = Nanoseconds::micros(50);

    /// LED channel specification - WS2812 uses GRB ordering
    const LED_CHANNELS: LedChannels = LedChannels::Rgb(RgbChannels::GRB);
}
