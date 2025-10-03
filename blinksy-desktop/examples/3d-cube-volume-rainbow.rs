use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    patterns::rainbow::{Rainbow, RainbowParams},
    util::map_range,
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{iter, thread::sleep, time::Duration};

struct CubeVolumeLayout;

impl Layout3d for CubeVolumeLayout {
    const PIXEL_COUNT: usize = 10 * 10 * 10;

    fn shapes() -> impl Iterator<Item = Shape3d> {
        let mut index: usize = 0;

        fn map(n: usize) -> f32 {
            map_range(n as f32, 0., 9., -1., 1.)
        }

        iter::from_fn(move || {
            if index >= 10 * 10 * 10 {
                return None;
            }

            let x = map(index % 10);
            let z = map(index / 10 % 10);
            let y = map(index / 10 / 10);

            index += 1;

            Some(Shape3d::Point(Vec3::new(x, y, z)))
        })
    }
}

fn main() {
    Desktop::new_3d::<CubeVolumeLayout>().start(|driver| {
        let mut control = ControlBuilder::new_3d()
            .with_layout::<CubeVolumeLayout, { CubeVolumeLayout::PIXEL_COUNT }>()
            .with_pattern::<Rainbow>(RainbowParams::default())
            .with_driver(driver)
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
