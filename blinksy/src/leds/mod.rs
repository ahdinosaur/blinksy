//! # LED Chipsets
//!
//! - [`Apa102`]: APA102 (DotStar) LEDs
//! - [`Ws2812`]: WS2812 (NeoPixel) LEDs
//! - [`Sk6812`]: SK6812 LEDs
//! - [`Lpd8806`]: LPD8806 LEDs
//!
//! If you want help to support a new chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

mod apa102;
mod sk6812;
mod ws2812;
mod lpd8806;

pub use apa102::Apa102;
pub use sk6812::Sk6812;
pub use ws2812::Ws2812;
pub use lpd8806::{Lpd8806, Lpd8806Brg};

use crate::driver::ClocklessLed;

pub const fn clockless_frame_buffer_size<Led: ClocklessLed>(pixel_count: usize) -> usize {
    pixel_count * Led::LED_CHANNELS.channel_count()
}
