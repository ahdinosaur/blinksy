use blinksy::{
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

layout2d!(
    PanelLayout,
    [Shape2d::Grid {
        start: Vec2::new(-1., -1.),
        horizontal_end: Vec2::new(1., -1.),
        vertical_end: Vec2::new(-1., 1.),
        horizontal_pixel_count: 16,
        vertical_pixel_count: 16,
        serpentine: true,
    }]
);

fn main() {
    Desktop::new_2d::<PanelLayout>().start(|driver| {
        let mut control = ControlBuilder::new_2d()
            .with_layout::<PanelLayout, { PanelLayout::PIXEL_COUNT }>()
            .with_pattern::<Rainbow>(RainbowParams {
                ..Default::default()
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ PanelLayout::PIXEL_COUNT }>()
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
