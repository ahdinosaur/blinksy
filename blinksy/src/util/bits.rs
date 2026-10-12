#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitOrder {
    MostSignificantBit,
    LeastSignificantBit,
}

pub trait Word:
    Copy
    + PartialEq
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::Not<Output = Self>
    + core::ops::Shl<usize, Output = Self>
    + core::ops::Shr<usize, Output = Self>
{
    const BITS: u32;
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_word {
    ($t:ty) => {
        impl Word for $t {
            const BITS: u32 = <$t>::BITS;
            const ZERO: Self = 0 as $t;
            const ONE: Self = 1 as $t;
        }
    };
}

impl_word!(u8);
impl_word!(u16);
impl_word!(u32);
impl_word!(u64);
impl_word!(u128);

/// MSB-first bit iterator
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BitsMsb<W: Word> {
    value: W,
    mask: W,
    remaining: u32,
}

impl<W: Word> BitsMsb<W> {
    #[inline]
    pub fn new(word: W) -> Self {
        // Safe because T::BITS >= 8 for all supported primitives.
        let top = (W::BITS - 1) as usize;
        Self {
            value: word,
            mask: W::ONE << top,
            remaining: W::BITS,
        }
    }
}

impl<W: Word> Iterator for BitsMsb<W> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.remaining == 0 {
            return None;
        }
        let bit = (self.value & self.mask) != W::ZERO;
        self.mask = self.mask >> 1;
        self.remaining -= 1;
        Some(bit)
    }
}

#[inline]
pub fn word_to_bits_msb<W: Word>(word: W) -> BitsMsb<W> {
    BitsMsb::new(word)
}

/// Appends bits to a slice of words, filling each word from its most significant bit
pub struct BitWriterMsb<'a, W: Word> {
    words: &'a mut [W],
    position: usize,
}

impl<'a, W: Word> BitWriterMsb<'a, W> {
    pub fn new(words: &'a mut [W]) -> Self {
        Self { words, position: 0 }
    }

    pub fn write_bits(&mut self, slice: BitSlice<W>) {
        copy_bits_msb(
            &slice,
            &mut BitSliceMut {
                buffer: self.words,
                offset: self.position,
                length: slice.length,
            },
        );
        self.position += slice.length;
    }
}

// Represents a view of the bits in a buffer
pub struct BitSlice<'a, W: Word> {
    pub buffer: &'a [W],
    pub offset: usize,
    pub length: usize,
}

pub struct BitSliceMut<'a, W: Word> {
    pub buffer: &'a mut [W],
    pub offset: usize,
    pub length: usize,
}

/// Copies the bits from `src` to `dst`
/// Note that this is currently restricted to the source having zero offset
pub fn copy_bits_msb<W: Word>(src: &BitSlice<W>, dst: &mut BitSliceMut<W>) {
    {
        assert_eq!(src.offset, 0);
        assert_eq!(
            src.length, dst.length,
            "source and destination need to be the same size when copying"
        );
        let word_bits = W::BITS as usize;
        let dst_first_index = dst.offset / word_bits;
        let dst_shift = dst.offset % word_bits;
        for (i, &input) in src.buffer[..src.length.div_ceil(word_bits)]
            .iter()
            .enumerate()
        {
            // Number of bits to copy in this word
            let count = (src.length - i * word_bits).min(word_bits);
            // The bits that fit into the current destination word
            copy_bits_in_word(
                input,
                &mut dst.buffer[dst_first_index + i],
                0,
                dst_shift,
                count,
            );
            // If there wasn't enough room in the first word we overflow into the next
            if word_bits - dst_shift < count {
                copy_bits_in_word(
                    input,
                    &mut dst.buffer[dst_first_index + i + 1],
                    word_bits - dst_shift,
                    0,
                    count - (word_bits - dst_shift),
                );
            }
        }
    };
}

fn copy_bits_in_word<W: Word>(
    input: W,
    output: &mut W,
    input_offset: usize,
    output_offset: usize,
    length: usize,
) {
    let word_bits = W::BITS as usize;

    let output_mask = !W::ZERO << (word_bits - length) >> output_offset;
    let input_in_output_position = if output_offset > input_offset {
        input >> (output_offset - input_offset)
    } else {
        input << (input_offset - output_offset)
    };
    *output = (*output & !output_mask) | (input_in_output_position & output_mask);
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;

    #[test]
    fn test_u8_msb() {
        let bits: Vec<bool, 8> = word_to_bits_msb(0b1010_0001_u8).collect();

        assert_eq!(
            bits,
            Vec::<bool, 8>::from_array([true, false, true, false, false, false, false, true])
        );
    }

    #[test]
    fn test_u16_msb() {
        let bits: Vec<bool, 16> = word_to_bits_msb(0x0408_u16).collect();

        assert_eq!(
            bits,
            Vec::<bool, 16>::from_array([
                false, false, false, false, false, true, false, false, // 0x04
                false, false, false, false, true, false, false, false, // 0x08
            ])
        );
    }

    #[test]
    fn test_copy_bits_msb_simple() {
        let mut dst: [u8; 2] = [0b1111_111, 0b1111_1111];
        copy_bits_msb(
            &BitSlice {
                buffer: &[0b0000_0000, 0b0000_0000],
                offset: 0,
                length: 11,
            },
            &mut BitSliceMut {
                buffer: &mut dst,
                offset: 0,
                length: 11,
            },
        );

        assert_eq!(dst, [0b0000_0000, 0b0001_1111]);
    }

    #[test]
    fn test_copy_bits_msb_across_word_boundary() {
        let mut dst: [u8; 3] = [0b1111_1111, 0b1111_1111, 0b1111_1111];
        copy_bits_msb(
            &BitSlice {
                buffer: &[0b0000_0000, 0b0000_0000],
                offset: 0,
                length: 11,
            },
            &mut BitSliceMut {
                buffer: &mut dst,
                offset: 5,
                length: 11,
            },
        );

        assert_eq!(dst, [0b1111_1000, 0b0000_0000, 0b1111_1111]);
    }

    #[test]
    fn test_copy_bits_in_words_simple() {
        let mut dst: u8 = 0b1111_1111;
        copy_bits_in_word(0b0000_0000, &mut dst, 0, 0, 3);

        assert_eq!(dst, 0b0001_1111);
    }

    #[test]
    fn test_copy_bits_in_words_with_src_offset() {
        let mut dst: u8 = 0b1111_1111;
        copy_bits_in_word(0b0000_0000, &mut dst, 4, 0, 3);

        assert_eq!(dst, 0b0001_1111);
    }

    #[test]
    fn test_copy_bits_in_words_with_dst_offset() {
        let mut dst: u8 = 0b1111_1111;
        copy_bits_in_word(0b0000_0000, &mut dst, 0, 4, 3);

        assert_eq!(dst, 0b1111_0001);
    }
}
