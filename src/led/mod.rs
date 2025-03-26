use palette::IntoColor;

mod chipsets;
mod clocked;
mod clockless;

pub use chipsets::*;
pub use clocked::ClockedWriterBitBang;

pub trait LedDriver {
    type Error;
    type Color;

    fn write<C, const N: usize>(&mut self, pixels: [C; N]) -> Result<(), Self::Error>
    where
        C: IntoColor<Self::Color>;
}

#[derive(Debug)]
pub enum RgbOrder {
    RGB,
    RBG,
    GRB,
    GBR,
    BRG,
    BGR,
}
