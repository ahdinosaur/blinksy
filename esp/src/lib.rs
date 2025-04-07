#![no_std]

mod rmt;

pub mod driver {
    pub use crate::rmt::*;
}

pub mod drivers {
    use blinksy::drivers::Ws2812Led;

    use crate::rmt::ClocklessRmtDriver;

    pub type Ws2812Rmt<Tx, const BUFFER_SIZE: usize> =
        ClocklessRmtDriver<Ws2812Led, Tx, BUFFER_SIZE>;
}
