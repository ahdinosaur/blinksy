use blinksy::{
    drivers::Graphics,
    layout1d,
    patterns::{Rainbow, RainbowParams},
    ControlBuilder,
};
use std::time::Instant;

fn main() -> ! {
    layout1d!(Layout, 30);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(Graphics::new_1d::<Layout>())
        .build();

    let now = Instant::now();
    loop {
        let elapsed_in_ms: u64 = now.elapsed().as_millis().try_into().unwrap();
        control.tick(elapsed_in_ms).unwrap();
    }
}
