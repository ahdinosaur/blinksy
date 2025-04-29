use blinksy::{
    layout1d,
    patterns::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_desktop::{
    drivers::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

fn main() {
    layout1d!(Layout, 30);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(Desktop::new_1d::<Layout>())
        .build();

    loop {
        if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
            break;
        }

        sleep(Duration::from_millis(16));
    }
}
