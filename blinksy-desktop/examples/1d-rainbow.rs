use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::time::Duration;

layout1d!(Layout, 30);

fn main() {
    let desktop = Desktop::new_1d::<Layout>();
    desktop.start(|driver| {
        let mut control = ControlBuilder::new_1d()
            .with_layout::<Layout>()
            .with_pattern::<Rainbow>(RainbowParams {
                ..Default::default()
            })
            .with_driver(driver)
            .build();

        std::thread::sleep(Duration::from_secs(2));
        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }
        }
    });
}
