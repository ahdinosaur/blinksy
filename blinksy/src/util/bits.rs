#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug)]
pub struct BitsLsb<T: Word> {
    v: T,
    remaining: u32,
}

impl<T: Word> BitsLsb<T> {
    #[inline]
    pub const fn new(word: T) -> Self {
        Self {
            v: word,
            remaining: T::BITS,
        }
    }
}

impl<T: Word> Iterator for BitsLsb<T> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.remaining == 0 {
            return None;
        }
        let bit = (self.v & T::ONE) != T::ZERO;
        self.v = self.v >> 1;
        self.remaining -= 1;
        Some(bit)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.remaining as usize;
        (n, Some(n))
    }

    // O(1) skip-ahead.
    #[inline]
    fn nth(&mut self, n: usize) -> Option<bool> {
        let n = n as u32;
        if n >= self.remaining {
            self.remaining = 0;
            return None;
        }
        self.v = self.v >> (n as usize);
        self.remaining -= n;
        self.next()
    }
}

impl<T: Word> core::iter::FusedIterator for BitsLsb<T> {}
impl<T: Word> ExactSizeIterator for BitsLsb<T> {}

/// MSB-first bit iterator: yields bit BITS-1, BITS-2, ..., down to bit 0.
#[derive(Clone, Copy, Debug)]
pub struct BitsMsb<T: Word> {
    v: T,
    mask: T,
    remaining: u32,
}

impl<T: Word> BitsMsb<T> {
    #[inline]
    pub fn new(word: T) -> Self {
        // Safe because T::BITS >= 8 for all supported primitives.
        let top = (T::BITS - 1) as usize;
        Self {
            v: word,
            mask: T::ONE << top,
            remaining: T::BITS,
        }
    }
}

impl<T: Word> Iterator for BitsMsb<T> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.remaining == 0 {
            return None;
        }
        let bit = (self.v & self.mask) != T::ZERO;
        self.mask = self.mask >> 1;
        self.remaining -= 1;
        Some(bit)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.remaining as usize;
        (n, Some(n))
    }

    // O(1) skip-ahead.
    #[inline]
    fn nth(&mut self, n: usize) -> Option<bool> {
        let n = n as u32;
        if n >= self.remaining {
            self.remaining = 0;
            return None;
        }
        self.mask = self.mask >> (n as usize);
        self.remaining -= n;
        self.next()
    }
}

impl<T: Word> core::iter::FusedIterator for BitsMsb<T> {}
impl<T: Word> ExactSizeIterator for BitsMsb<T> {}

#[derive(Clone, Copy, Debug)]
pub enum Bits<T: Word> {
    Msb(BitsMsb<T>),
    Lsb(BitsLsb<T>),
}

impl<T: Word> Iterator for Bits<T> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        match self {
            Bits::Msb(it) => it.next(),
            Bits::Lsb(it) => it.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Bits::Msb(it) => it.size_hint(),
            Bits::Lsb(it) => it.size_hint(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<bool> {
        match self {
            Bits::Msb(it) => it.nth(n),
            Bits::Lsb(it) => it.nth(n),
        }
    }
}

impl<T: Word> core::iter::FusedIterator for Bits<T> {}
impl<T: Word> ExactSizeIterator for Bits<T> {}

#[inline]
pub fn bits_of<T: Word>(word: &T, order: BitOrder) -> Bits<T> {
    match order {
        BitOrder::MostSignificantBit => Bits::Msb(BitsMsb::new(*word)),
        BitOrder::LeastSignificantBit => Bits::Lsb(BitsLsb::new(*word)),
    }
}

#[inline]
pub fn bits_of_msb<T: Word>(word: T) -> BitsMsb<T> {
    BitsMsb::new(word)
}

#[inline]
pub fn bits_of_lsb<T: Word>(word: T) -> BitsLsb<T> {
    BitsLsb::new(word)
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;

    #[test]
    fn test_u8_msb() {
        let bits: Vec<bool, 8> = bits_of(&0b1010_0001_u8, BitOrder::MostSignificantBit).collect();

        assert_eq!(
            bits,
            Vec::<bool, 8>::from_array([true, false, true, false, false, false, false, true])
        );
    }

    #[test]
    fn test_u8_lsb() {
        let bits: Vec<bool, 8> = bits_of(&0b1010_0001_u8, BitOrder::LeastSignificantBit).collect();

        assert_eq!(
            bits,
            Vec::<bool, 8>::from_array([true, false, false, false, false, true, false, true])
        );
    }

    #[test]
    fn test_u16_msb() {
        let bits: Vec<bool, 16> = bits_of(&0x0408_u16, BitOrder::MostSignificantBit).collect();

        assert_eq!(
            bits,
            Vec::<bool, 16>::from_array([
                false, false, false, false, false, true, false, false, // 0x04
                false, false, false, false, true, false, false, false, // 0x08
            ])
        );
    }

    #[test]
    fn test_u16_lsb() {
        let bits: Vec<bool, 16> = bits_of(&0x0408_u16, BitOrder::LeastSignificantBit).collect();

        assert_eq!(
            bits,
            Vec::<bool, 16>::from_array([
                false, false, false, true, false, false, false, false, // 0x08
                false, false, true, false, false, false, false, false, // 0x04
            ])
        );
    }
}
