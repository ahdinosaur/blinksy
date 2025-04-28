use blinksy::{
    layout1d,
    patterns::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_desktop::{drivers::Desktop, time::elapsed_in_ms};

fn main() -> ! {
    layout1d!(Layout, 30);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(Desktop::new_1d::<Layout>())
        .build();

    loop {
        control.tick(elapsed_in_ms()).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
