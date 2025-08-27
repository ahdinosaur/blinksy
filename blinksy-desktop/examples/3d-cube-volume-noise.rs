use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    util::map_range,
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{iter, thread::sleep, time::Duration};

struct VolumeCubeLayout;

impl Layout3d for VolumeCubeLayout {
    const PIXEL_COUNT: usize = 5 * 5 * 5;

    fn shapes() -> impl Iterator<Item = Shape3d> {
        let mut index: usize = 0;

        fn map(n: usize) -> f32 {
            map_range(n as f32, 0., 4., -1., 1.)
        }

        iter::from_fn(move || {
            if index >= 5 * 5 * 5 {
                return None;
            }

            let x = map(index % 5);
            let z = map(index / 5 % 5);
            let y = map(index / 5 / 5);

            index += 1;

            Some(Shape3d::Point(Vec3::new(x, y, z)))
        })
    }
}

fn main() {
    let mut control = ControlBuilder::new_3d()
        .with_layout::<VolumeCubeLayout>()
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            time_scalar: 0.25 / 1e3,
            position_scalar: 0.25,
        })
        .with_driver(Desktop::new_3d::<VolumeCubeLayout>())
        .build();

    loop {
        if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
            break;
        }

        sleep(Duration::from_millis(16));
    }
}
