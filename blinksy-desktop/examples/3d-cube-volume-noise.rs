use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
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
            assert!(n <= 4, "Input must be between 0 and 4 inclusive.");
            // Map 0..=5 to 0.0..=1.0
            let normalized = n as f32 / 4.0;
            // Map 0.0..=1.0 to -1.0..=1.0
            normalized * 2.0 - 1.0
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
            ..Default::default()
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
