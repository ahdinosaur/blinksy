//! # LED Driver Implementations
//!
//! - [`apa102`]: APA102 (DotStar) LEDs
//! - [`ws2812`]: WS2812 (NeoPixel) LEDs
//! - [`sk6812`]: SK6812 LEDs
//!
//! If you want help to support a new chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

mod apa102;
mod sk6812;
mod ws2812;

pub use apa102::*;
pub use sk6812::*;
pub use ws2812::*;

pub const fn clockless_frame_buffer_size(pixel_count: usize) -> usize {
    pixel_count * 3
}
