use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    layout3d,
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

fn main() {
    layout3d!(
        CubeFaceLayout,
        [
            // bottom face
            Shape3d::Grid {
                start: Vec3::new(1., -1., 1.),           // right bottom front
                horizontal_end: Vec3::new(-1., -1., 1.), // left bottom front
                vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // back face
            Shape3d::Grid {
                start: Vec3::new(-1., -1., -1.),         // left bottom back
                horizontal_end: Vec3::new(-1., 1., -1.), // left top back
                vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // right face
            Shape3d::Grid {
                start: Vec3::new(1., 1., -1.),         // right top back
                horizontal_end: Vec3::new(1., 1., 1.), // right top front
                vertical_end: Vec3::new(1., -1., -1.), // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // front face
            Shape3d::Grid {
                start: Vec3::new(-1., -1., 1.),         // left bottom front
                horizontal_end: Vec3::new(1., -1., 1.), // right bottom front
                vertical_end: Vec3::new(-1., 1., 1.),   // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // left face
            Shape3d::Grid {
                start: Vec3::new(-1., 1., -1.),           // left top back
                horizontal_end: Vec3::new(-1., -1., -1.), // left bottom back
                vertical_end: Vec3::new(-1., 1., 1.),     // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // top face
            Shape3d::Grid {
                start: Vec3::new(1., 1., 1.),           // right top front
                horizontal_end: Vec3::new(1., 1., -1.), // right top back
                vertical_end: Vec3::new(-1., 1., 1.),   // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            }
        ]
    );

    Desktop::new_3d::<CubeFaceLayout>().start(|driver| {
        let mut control = ControlBuilder::new_3d()
            .with_layout::<CubeFaceLayout, { CubeFaceLayout::PIXEL_COUNT }>()
            .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
                ..Default::default()
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ CubeFaceLayout::PIXEL_COUNT }>()
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
