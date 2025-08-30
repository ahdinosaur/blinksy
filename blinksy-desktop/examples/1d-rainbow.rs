use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError, DesktopStage},
    time::elapsed_in_ms,
};
use std::time::Duration;

fn main() {
    DesktopStage::start(move || {
        layout1d!(Layout, 30);

        let (driver, stage) = Desktop::new_1d::<Layout>();
        let mut control = ControlBuilder::new_1d()
            .with_layout::<Layout>()
            .with_pattern::<Rainbow>(RainbowParams {
                ..Default::default()
            })
            .with_driver(driver)
            .build();

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            loop {
                if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                    break;
                }
            }
        });
        stage
    });
}
