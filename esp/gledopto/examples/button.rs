#![no_std]
#![no_main]

use defmt::info;

use gledopto::{board, function_button, main};

#[main]
fn main() -> ! {
    let p = board!();

    info!("Init");

    let mut button = function_button!(p);

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
}
