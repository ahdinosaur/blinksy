use num_traits::ToBytes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitOrder {
    MostSignificantBit,
    LeastSignificantBit,
}

fn byte_to_bits(byte: &u8, order: BitOrder) -> [bool; 8] {
    match order {
        BitOrder::MostSignificantBit => core::array::from_fn(|i| ((byte >> (7 - i)) & 1) != 0),
        BitOrder::LeastSignificantBit => core::array::from_fn(|i| ((byte >> i) & 1) != 0),
    }
}

pub fn bits_of<Word>(word: &Word, order: BitOrder) -> impl Iterator<Item = bool>
where
    Word: ToBytes,
    Word::Bytes: IntoIterator<Item = u8>,
{
    let bytes = match order {
        BitOrder::MostSignificantBit => word.to_be_bytes(),
        BitOrder::LeastSignificantBit => word.to_le_bytes(),
    };

    bytes
        .into_iter()
        .flat_map(move |byte| byte_to_bits(&byte, order))
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;

    #[test]
    fn test_u8_msb() {
        let bits: Vec<bool, 8> = bits_of(0b1010_0001u8, BitOrder::MostSignificantBit).collect();

        assert_eq!(
            bits,
            Vec::<bool, 8>::from_array([true, false, true, false, false, false, false, true])
        );
    }

    #[test]
    fn test_u8_lsb() {
        let bits: Vec<bool, 8> = bits_of(0b1010_0001u8, BitOrder::LeastSignificantBit).collect();

        assert_eq!(
            bits,
            Vec::<bool, 8>::from_array([true, false, false, false, false, true, false, true])
        );
    }

    #[test]
    fn test_u16_msb() {
        let bits: Vec<bool, 16> = bits_of(0x0180u16, BitOrder::MostSignificantBit).collect();

        // 0x0180 (MSB-first across the 16-bit word): 0000_0001 1000_0000
        assert_eq!(
            bits,
            Vec::<bool, 16>::from_array([
                false, false, false, false, false, false, false, true, // 0x01
                true, false, false, false, false, false, false, false, // 0x80
            ])
        );
    }

    #[test]
    fn test_u16_lsb() {
        let bits: Vec<bool, 16> = bits_of(0x0180u16, BitOrder::LeastSignificantBit).collect();

        // 0x0180 (LSB-first across the 16-bit word): bytes reversed, and bits within byte LSB-first.
        // Little-endian byte order in memory is [0x80, 0x01]
        assert_eq!(
            bits,
            Vec::<bool, 16>::from_array([
                false, false, false, false, false, false, true, false, // 0x80 bits reversed
                true, false, false, false, false, false, false, false, // 0x01 bits reversed
            ])
        );
    }
}
