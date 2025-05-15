//! # Clockless LED Driver Abstractions
//!
//! This module provides abstractions for driving "clockless" LED protocols, such as
//! WS2812 (NeoPixel), SK6812, and similar. These protocols use a single data line
//! with precise timing to encode bits.
//!
//! ## Clockless Protocols
//!
//! Clockless protocols encode data bits using precise pulse timings on a single data line:
//!
//! - Each bit is represented by a high pulse followed by a low pulse
//! - The duration of the high and low pulses determines whether it's a '0' or '1' bit
//! - After all bits are sent, a longer reset period is required
//!
//! ## Traits
//!
//! - [`ClocklessLed`]: Trait defining the timing parameters for a clockless LED chipset
//!
//! ## Drivers
//!
//! - [`ClocklessDelayDriver`]: Driver using GPIO bit-banging with a delay timer
//!
//! ## Example
//!
//! ```rust
//! use blinksy::{
//!     color::{LedChannels, RgbChannels},
//!     driver::ClocklessLed,
//!     time::Nanoseconds,
//! };
//!
//! // Define a new LED chipset with specific timing requirements
//! struct MyLed;
//!
//! impl ClocklessLed for MyLed {
//!     // High pulse duration for '0' bit
//!     const T_0H: Nanoseconds = Nanoseconds::nanos(350);
//!     // Low pulse duration for '0' bit
//!     const T_0L: Nanoseconds = Nanoseconds::nanos(800);
//!     // High pulse duration for '1' bit
//!     const T_1H: Nanoseconds = Nanoseconds::nanos(700);
//!     // Low pulse duration for '1' bit
//!     const T_1L: Nanoseconds = Nanoseconds::nanos(600);
//!     // Reset period
//!     const T_RESET: Nanoseconds = Nanoseconds::micros(50);
//!     // Color channel ordering
//!     const LED_CHANNELS: LedChannels = LedChannels::Rgb(RgbChannels::RGB);
//! }
//! ```

use crate::color::LedChannels;
use crate::time::Nanoseconds;

mod delay;

pub use self::delay::*;

/// Trait that defines the timing parameters and protocol specifics for a clockless LED chipset.
///
/// Implementors of this trait specify the exact timing requirements for a specific
/// LED chipset that uses a clockless protocol.
///
/// # Example
///
/// ```rust
/// use fugit::NanosDurationU32 as Nanoseconds;
/// use blinksy::{color::{LedChannels, RgbChannels}, driver::ClocklessLed};
///
/// struct WS2811Led;
///
/// impl ClocklessLed for WS2811Led {
///     const T_0H: Nanoseconds = Nanoseconds::nanos(250);
///     const T_0L: Nanoseconds = Nanoseconds::nanos(1000);
///     const T_1H: Nanoseconds = Nanoseconds::nanos(600);
///     const T_1L: Nanoseconds = Nanoseconds::nanos(650);
///     const T_RESET: Nanoseconds = Nanoseconds::micros(50);
///     const LED_CHANNELS: LedChannels = LedChannels::Rgb(RgbChannels::RGB);
/// }
/// ```
pub trait ClocklessLed {
    /// Duration of high signal for transmitting a '0' bit.
    const T_0H: Nanoseconds;

    /// Duration of low signal for transmitting a '0' bit.
    const T_0L: Nanoseconds;

    /// Duration of high signal for transmitting a '1' bit.
    const T_1H: Nanoseconds;

    /// Duration of low signal for transmitting a '1' bit.
    const T_1L: Nanoseconds;

    /// Duration of the reset period at the end of a transmission.
    ///
    /// This low signal period marks the end of a data frame and allows the LEDs
    /// to latch the received data and update their output.
    const T_RESET: Nanoseconds;

    /// Specification of the color channel order and format.
    ///
    /// Different LED chipsets may expect data in different channel orders (e.g., RGB, GRB, RGBW).
    const LED_CHANNELS: LedChannels;

    /// Calculates the total cycle time for a bit transmission.
    ///
    /// Returns the maximum of (T_0H + T_0L) and (T_1H + T_1L) to ensure
    /// timing is correct regardless of bit value.
    fn t_cycle() -> Nanoseconds {
        (Self::T_0H + Self::T_0L).max(Self::T_1H + Self::T_1L)
    }
}
