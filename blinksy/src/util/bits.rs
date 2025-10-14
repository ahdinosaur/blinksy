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
}
