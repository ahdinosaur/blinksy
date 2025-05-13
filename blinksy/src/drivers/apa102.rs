//! # APA102 LED Driver
//!
//! This module provides driver support for APA102 (DotStar) LEDs, which use a
//! clocked SPI-like protocol. APA102 LEDs offer high refresh rates and precise
//! brightness control.
//!
//! The module implements two driver types:
//!
//! - [`Apa102Delay`]: Uses bit-banged GPIO with precise timing
//! - [`Apa102Spi`]: Uses a hardware SPI interface for improved performance
//!
//! ## Key Features
//!
//! - 8-bit global brightness control (0-255)
//! - 8-bit per-channel color resolution
//! - Supports high update rates
//! - No strict timing requirements (unlike WS2812)
//!
//! ## Protocol Details
//!
//! The APA102 protocol consists of:
//!
//! 1. Start frame: 32 bits of zeros
//! 2. LED frames: Each LED gets 32 bits (8-bit brightness, 8-bit blue, 8-bit green, 8-bit red)
//! 3. End frame: (n/2) bits of zeros where n is the number of LEDs
//!
//! (References: [Hackaday](https://hackaday.com/2014/12/09/digging-into-the-apa102-serial-led-protocol/), [Pololu](https://www.pololu.com/product/2554))
//!
//! This implementation includes the "High Definition" color handling from FastLED, which
//! optimizes the use of the 5-bit brightness and 8-bit per-channel values.

use crate::color::{gamma_encode, ColorComponent, ColorCorrection, OutputColor};
use crate::{
    color::RgbChannels,
    driver::clocked::{ClockedDelayDriver, ClockedLed, ClockedSpiDriver, ClockedWriter},
};

/// APA102 driver using GPIO bit-banging with delay timing.
///
/// # Type Parameters
///
/// * `Data` - The data pin type
/// * `Clock` - The clock pin type
/// * `Delay` - The delay implementation type
pub type Apa102Delay<Data, Clock, Delay> = ClockedDelayDriver<Apa102Led, Data, Clock, Delay>;

/// APA102 driver using hardware SPI.
///
/// # Type Parameters
///
/// * `Spi` - The SPI interface type
pub type Apa102Spi<Spi> = ClockedSpiDriver<Apa102Led, Spi>;

/// LED implementation for APA102 protocol.
///
/// This type implements the ClockedLed trait with the specifics of the APA102 protocol.
/// It handles start/end frames and the color frame format with 5-bit brightness control.
#[derive(Debug)]
pub struct Apa102Led;

impl ClockedLed for Apa102Led {
    type Word = u8;

    /// Writes the APA102 start frame (32 bits of zeros).
    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
    ) -> Result<(), Writer::Error> {
        writer.write(&[0x00, 0x00, 0x00, 0x00])
    }

    /// Writes a color frame for one LED, including the 5-bit global brightness.
    ///
    /// Uses the "High Definition" color handling algorithm from FastLED to optimize
    /// the use of the 5-bit brightness and 8-bit per-channel color values.
    fn color<Writer: ClockedWriter<Word = Self::Word>, C: OutputColor>(
        writer: &mut Writer,
        color: C,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> Result<(), Writer::Error> {
        let linear = color.to_linear_rgb();
        let (red, green, blue) = (linear.red, linear.green, linear.blue);

        // First color correct the linear RGB
        let red = red * correction.red;
        let green = green * correction.red;
        let blue = blue * correction.red;

        // Then, adjust additional gamma
        let red = gamma_encode(red, gamma);
        let green = gamma_encode(green, gamma);
        let blue = gamma_encode(blue, gamma);

        // Then, convert to u16's
        let (red_u16, green_u16, blue_u16) = (
            ColorComponent::from_normalized_f32(red),
            ColorComponent::from_normalized_f32(green),
            ColorComponent::from_normalized_f32(blue),
        );

        let brightness: u8 = ColorComponent::from_normalized_f32(brightness);

        let ((red_u8, green_u8, blue_u8), brightness) =
            five_bit_bitshift(red_u16, green_u16, blue_u16, brightness);

        let led_frame = RgbChannels::BGR.reorder([red_u8, green_u8, blue_u8]);

        writer.write(&[0b11100000 | (brightness & 0b00011111)])?;
        writer.write(&led_frame)?;

        Ok(())
    }

    /// No special reset needed for APA102.
    fn reset<Writer: ClockedWriter<Word = Self::Word>>(
        _writer: &mut Writer,
    ) -> Result<(), Writer::Error> {
        Ok(())
    }

    /// Writes the APA102 end frame.
    ///
    /// The end frame needs to be at least (n-1)/16 + 1 bytes of zeros, where n is
    /// the number of LEDs in the chain.
    fn end<Writer: ClockedWriter<Word = Self::Word>>(
        writer: &mut Writer,
        length: usize,
    ) -> Result<(), Writer::Error> {
        let num_bytes = (length - 1).div_ceil(16);
        for _ in 0..num_bytes {
            writer.write(&[0x00])?
        }
        Ok(())
    }
}

/// Implements the core APA102HD "bitshift" routine. It takes 16‑bit color channels and an 8‑bit global
/// brightness value, then "steals" brightness bits from the color channels into a 5‑bit driver brightness.
///
/// Returns a new (r, g, b) tuple in 8‑bit space and the adjusted 5‑bit brightness.
///
/// Source: https://github.com/FastLED/FastLED/blob/57f2dc1/src/fl/five_bit_hd_gamma.cpp#L73-L128
fn five_bit_bitshift(
    mut r16: u16,
    mut g16: u16,
    mut b16: u16,
    mut brightness: u8,
) -> ((u8, u8, u8), u8) {
    if brightness == 0 {
        return ((0, 0, 0), 0);
    }

    if r16 == 0 && g16 == 0 && b16 == 0 {
        let out_power = if brightness <= 31 { brightness } else { 31 };
        return ((0, 0, 0), out_power);
    }

    static K5_INITIAL: u8 = 0b00010000;
    let mut v5: u8 = K5_INITIAL;

    brightness_bitshifter8(&mut v5, &mut brightness, 4);

    let mut max_component = max3(r16, g16, b16);
    let shifts = brightness_bitshifter16(&mut v5, &mut max_component, 4, 2);

    if shifts > 0 {
        r16 <<= shifts;
        g16 <<= shifts;
        b16 <<= shifts;
    }

    if brightness != 0xff {
        r16 = scale16_by_8(r16, brightness);
        g16 = scale16_by_8(g16, brightness);
        b16 = scale16_by_8(b16, brightness);
    };

    let v5 = if v5 > 1 { v5 | (v5 - 1) } else { v5 };

    ((map16_to_8(r16), map16_to_8(g16), map16_to_8(b16)), v5)
}

/// Steals brightness from `brightness_src` and adds it to `brightness_dst`
/// without changing the product of the two. Returns the number of shifts performed.
///
/// # Parameters
///
/// * `brightness_src`: Source brightness (typically the global brightness value).
/// * `brightness_dst`: Destination brightness (driver brightness value).
/// * `max_shifts`: Maximum number of shifts to attempt.
///
/// Source: https://github.com/FastLED/FastLED/blob/57f2dc1/src/lib8tion/brightness_bitshifter.h#L14-L39
fn brightness_bitshifter8(brightness_src: &mut u8, brightness_dst: &mut u8, max_shifts: u8) -> u8 {
    let mut src = *brightness_src;
    if *brightness_dst == 0 || src == 0 {
        return 0;
    }

    let mut curr = *brightness_dst;
    let mut shifts = 0;

    for _ in 0..max_shifts {
        if src <= 1 {
            break;
        }
        if curr & 0b1000_0000 != 0 {
            break;
        }
        curr <<= 1;
        src >>= 1;
        shifts += 1;
    }

    *brightness_dst = curr;
    *brightness_src = src;
    shifts
}

/// Steals brightness from `brightness_src` and adds it to `brightness_dst` for 16-bit
/// color channels. Returns the number of shifts performed.
///
/// # Parameters
///
/// * `brightness_src`: Source brightness (global brightness value).
/// * `brightness_dst`: Destination brightness (16-bit color channel value).
/// * `max_shifts`: Maximum number of shifts to attempt.
/// * `steps`: The number of bits to shift on the destination per iteration (default is 2).
///
/// Source: https://github.com/FastLED/FastLED/blob/57f2dc1/src/lib8tion/brightness_bitshifter.h#L41-L75
fn brightness_bitshifter16(
    brightness_src: &mut u8,
    brightness_dst: &mut u16,
    max_shifts: u8,
    steps: u8,
) -> u8 {
    let mut src = *brightness_src;
    if *brightness_dst == 0 || src == 0 {
        return 0;
    }

    let mut overflow_mask: u16 = 0b1000_0000_0000_0000;
    for _ in 1..steps {
        overflow_mask >>= 1;
        overflow_mask |= 0b1000_0000_0000_0000;
    }

    let underflow_mask: u8 = 0x1;
    let mut curr = *brightness_dst;
    let mut shifts = 0;

    for _ in 0..max_shifts {
        if src & underflow_mask != 0 {
            break;
        }
        if curr & overflow_mask != 0 {
            break;
        }
        curr <<= steps;
        src >>= 1;
        shifts += 1;
    }

    *brightness_dst = curr;
    *brightness_src = src;
    shifts
}

/// Scales a 16-bit value by an 8-bit value, treating the 8-bit value as a fraction from 0-255/256.
#[inline]
fn scale16_by_8(val: u16, scale: u8) -> u16 {
    ((val as u32 * (scale as u32 + 1)) >> 8) as u16
}

/// Maps a 16-bit value to an 8-bit value, with rounding.
#[inline]
fn map16_to_8(x: u16) -> u8 {
    if x == 0 {
        return 0;
    }
    if x >= 0xff00 {
        return 0xff;
    }
    ((x + 128) >> 8) as u8
}

/// Returns the maximum of three values.
#[inline]
fn max3(a: u16, b: u16, c: u16) -> u16 {
    a.max(b).max(c)
}
