use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;
use palette::{FromColor, LinSrgb, Srgb};

use crate::led::clocked::{ClockedDriver, ClockedLed, ClockedWriter};
use crate::time::Megahertz;
use crate::util::map_f32_to_u8_range;
use crate::{ClockedDelayWriter, LedDriver, RgbOrder};

// Apa102 docs:
// - https://hackaday.com/2014/12/09/digging-into-the-apa102-serial-led-protocol/
// - https://www.pololu.com/product/2554

#[derive(Debug)]
pub struct Apa102;

impl ClockedLed for Apa102 {
    type Word = u8;
    type Color = Srgb;

    fn start<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
    ) -> Result<(), Writer::Error> {
        writer.write(&[0x00, 0x00, 0x00, 0x00])
    }

    fn color<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        writer: &mut Writer,
        color: Self::Color,
        brightness: f32,
    ) -> Result<(), Writer::Error> {
        let color: LinSrgb<f32> = Srgb::from_color(color).into_linear();
        let color: LinSrgb<u16> = color.into_format();
        let brightness: u8 = map_f32_to_u8_range(brightness, 255);
        let ((red, green, blue), brightness) =
            five_bit_bitshift(color.red, color.green, color.blue, brightness);
        let led_frame = RgbOrder::BGR.reorder(red, green, blue);
        writer.write(&[0b11100000 | brightness & 0b00011111]);
        writer.write(&led_frame)?;
        Ok(())
    }

    fn reset<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
        _writer: &mut Writer,
    ) -> Result<(), Writer::Error> {
        Ok(())
    }
    fn end<Writer: ClockedWriter<Word = Self::Word>>(
        &self,
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

/// Implements the core APA102HD “bitshift” routine. It takes 16‑bit color channels and an 8‑bit global
/// brightness value, then “steals” brightness bits from the color channels into a 5‑bit driver brightness.
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

    // Step 1: Initialize brightness
    const K5_INITIAL: u8 = 0b00010000; // starting value: 16
    let mut v5: u8 = K5_INITIAL;

    // Step 2: Boost brightness by swapping power with the driver brightness.
    brightness_bitshifter8(&mut v5, &mut brightness, 4);

    // Step 3: Boost brightness of the color channels by swapping power with the
    // driver brightness.
    let mut max_component = max3(r16, g16, b16);
    let shifts = brightness_bitshifter16(&mut v5, &mut max_component, 4, 2);
    if shifts > 0 {
        r16 <<= shifts;
        g16 <<= shifts;
        b16 <<= shifts;
    }

    // Step 4: scale by final brightness factor.
    if brightness != 0xff {
        r16 = scale16_by_8(r16, brightness);
        g16 = scale16_by_8(g16, brightness);
        b16 = scale16_by_8(b16, brightness)
    };

    // Brighten hardware brightness by turning on low order bits.
    //
    // Since v5 is a power of two, subtracting one will invert the leading bit
    // and invert all the bits below it.
    // Example: 0b00010000 -1 = 0b00001111
    // So 0b00010000 | 0b00001111 = 0b00011111
    let v5 = if v5 > 1 { v5 | (v5 - 1) } else { v5 };

    // Step 5: Convert back to 8-bit and output.
    ((map16_to_8(r16), map16_to_8(g16), map16_to_8(b16)), v5)
}

/// Steals brightness from `brightness_src` and adds it to `brightness_dst`
/// without changing the product of the two. Returns the number of shifts performed.
///
/// # Parameters
/// - `brightness_src`: Source brightness (typically the global brightness value).
/// - `brightness_dst`: Destination brightness (driver brightness value).
/// - `max_shifts`: Maximum number of shifts to attempt.
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
        // Check if the next shift on `curr` would overflow the 8-bit value.
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
/// - `brightness_src`: Source brightness (global brightness value).
/// - `brightness_dst`: Destination brightness (16-bit color channel value).
/// - `max_shifts`: Maximum number of shifts to attempt.
/// - `steps`: The number of bits to shift on the destination per iteration (default is 2).
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
    // Initialize the overflow mask to check for potential overflow.
    let mut overflow_mask: u16 = 0b1000_0000_0000_0000;
    // For each extra step beyond 1, shift the mask right and set its MSB.
    for _ in 1..steps {
        overflow_mask >>= 1;
        overflow_mask |= 0b1000_0000_0000_0000;
    }
    // Underflow mask to check the lowest bit.
    let underflow_mask: u8 = 0x1;
    let mut curr = *brightness_dst;
    let mut shifts = 0;
    for _ in 0..max_shifts {
        // If the source's lowest bit is set, we cannot shift further.
        if src & underflow_mask != 0 {
            break;
        }
        // If the destination would overflow on the next shift, stop.
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

// Convert an 8-bit value to 16-bit by replicating the value (x << 8) | x.
#[inline]
fn map8_to_16(x: u8) -> u16 {
    (x as u16) * 257
}

// Multiply a 16-bit value by an 8-bit scale (with an extra factor of 1) and shift right by 8.
#[inline]
fn scale16_by_8(val: u16, scale: u8) -> u16 {
    (((val as u32) * ((scale as u32) + 1)) >> 8) as u16
}

// Clamp a 16-bit value down to 8-bit.
#[inline]
fn map16_to_8(x: u16) -> u8 {
    if x > 255 {
        255
    } else {
        x as u8
    }
}

// Return the maximum of three 16-bit values.
#[inline]
fn max3(a: u16, b: u16, c: u16) -> u16 {
    a.max(b).max(c)
}
#[derive(Debug)]
pub struct Apa102Delay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    driver: ClockedDriver<Apa102, ClockedDelayWriter<Data, Clock, Delay>>,
}

impl<Data, Clock, Delay> Apa102Delay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    pub fn new(data: Data, clock: Clock, delay: Delay, data_rate: Megahertz) -> Self {
        let led = Apa102;
        let writer = ClockedDelayWriter::new(data, clock, delay, data_rate);
        let driver = ClockedDriver::new(led, writer);
        Self { driver }
    }
}

impl<Data, Clock, Delay> LedDriver for Apa102Delay<Data, Clock, Delay>
where
    Data: OutputPin,
    Clock: OutputPin,
    Delay: DelayNs,
{
    type Error =
        <ClockedDriver<Apa102, ClockedDelayWriter<Data, Clock, Delay>> as LedDriver>::Error;
    type Color =
        <ClockedDriver<Apa102, ClockedDelayWriter<Data, Clock, Delay>> as LedDriver>::Color;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        self.driver.write(pixels, brightness)
    }
}

#[derive(Debug)]
pub struct Apa102Spi<Spi>
where
    Spi: SpiBus,
{
    driver: ClockedDriver<Apa102, Spi>,
}

impl<Spi> Apa102Spi<Spi>
where
    Spi: SpiBus,
{
    pub fn new(spi: Spi) -> Self {
        let led = Apa102;
        let driver = ClockedDriver::new(led, spi);
        Self { driver }
    }
}

impl<Spi> LedDriver for Apa102Spi<Spi>
where
    Spi: SpiBus,
{
    type Error = <ClockedDriver<Apa102, Spi> as LedDriver>::Error;
    type Color = <ClockedDriver<Apa102, Spi> as LedDriver>::Color;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        self.driver.write(pixels, brightness)
    }
}
