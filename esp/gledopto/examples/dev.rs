#![no_std]
#![no_main]

use core::array::from_fn;

use defmt::info;

use gledopto::{main, Board, Hsv, LedDriver};

#[main]
fn main() -> ! {
    let mut board = Board::new();

    info!("Init");

    const NUM_PIXELS: usize = 16;
    let mut apa102 = board.apa102(0.5);

    loop {
        let elapsed_in_ms = Board::elapsed().as_millis();
        let pixels: [Hsv; NUM_PIXELS] = from_fn(|i| {
            let hue = (i as f32 / NUM_PIXELS as f32) * 360. + (elapsed_in_ms as f32 / 4.);
            let saturation = 1.;
            let value = 1.;
            Hsv::new(hue, saturation, value)
        });
        apa102.write(pixels).unwrap();
    }

    /*
    let mut button = FunctionButton::new(peripherals.GPIO0);

    loop {
        button.tick();

        #[allow(clippy::collapsible_else_if)]
        if let Some(dur) = button.held_time() {
            info!("Total holding time {:?}", dur);

            if button.is_clicked() {
                info!("Clicked + held");
            } else if button.is_double_clicked() {
                info!("Double clicked + held");
            } else if button.holds() == 2 && button.clicks() > 0 {
                info!("Held twice with {} clicks", button.clicks());
            } else if button.holds() == 2 {
                info!("Held twice");
            }
        } else {
            if button.is_clicked() {
                info!("Click");
            } else if button.is_double_clicked() {
                info!("Double click");
            } else if button.is_triple_clicked() {
                info!("Triple click");
            } else if let Some(dur) = button.current_holding_time() {
                info!("Held for {:?}", dur);
            }
        }

        button.reset();
    }
    */
}
