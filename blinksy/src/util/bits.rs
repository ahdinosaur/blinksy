pub enum BitOrder {
    MostSignificantBit,
    LeastSignificantBit,
}

pub fn u8_to_bits(byte: &u8, order: BitOrder) -> [bool; 8] {
    let bit_positions = match order {
        BitOrder::MostSignificantBit => [128, 64, 32, 16, 8, 4, 2, 1],
        BitOrder::LeastSignificantBit => [1, 2, 4, 8, 16, 32, 64, 128],
    };
    #[allow(clippy::match_like_matches_macro)]
    bit_positions.map(|bit_position| match byte & bit_position {
        0 => false,
        _ => true,
    })
}
