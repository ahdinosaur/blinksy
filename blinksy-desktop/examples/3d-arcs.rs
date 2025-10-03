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
use core::f32::consts::PI;
use std::{thread::sleep, time::Duration};

layout3d!(
    /// Five half-arches stepping through z ∈ [-1, 1]
    TunnelLayout,
    [
        // Perfect semi-circle (apex at y = 0)
        Shape3d::Arc {
            center: Vec3::new(0., -1., -1.),
            axis_u: Vec3::new(1., 0., 0.),
            axis_v: Vec3::new(0., 1., 0.), // v_radius = 1.0 (circle)
            start_angle_in_radians: 0.0,
            end_angle_in_radians: PI,
            pixel_count: 60,
        },
        Shape3d::Arc {
            center: Vec3::new(0., -1., -0.5),
            axis_u: Vec3::new(1., 0., 0.),
            axis_v: Vec3::new(0., 1.25, 0.), // v_radius = 1.25
            start_angle_in_radians: 0.0,
            end_angle_in_radians: PI,
            pixel_count: 68,
        },
        Shape3d::Arc {
            center: Vec3::new(0., -1., 0.),
            axis_u: Vec3::new(1., 0., 0.),
            axis_v: Vec3::new(0., 1.5, 0.), // v_radius = 1.5
            start_angle_in_radians: 0.0,
            end_angle_in_radians: PI,
            pixel_count: 76,
        },
        Shape3d::Arc {
            center: Vec3::new(0., -1., 0.5),
            axis_u: Vec3::new(1., 0., 0.),
            axis_v: Vec3::new(0., 1.75, 0.), // v_radius = 1.75
            start_angle_in_radians: 0.0,
            end_angle_in_radians: PI,
            pixel_count: 84,
        },
        // Tall elliptical arch, apex reaches y = +1
        Shape3d::Arc {
            center: Vec3::new(0., -1., 1.),
            axis_u: Vec3::new(1., 0., 0.),
            axis_v: Vec3::new(0., 2., 0.), // v_radius = 2.0 (ellipse)
            start_angle_in_radians: 0.0,
            end_angle_in_radians: PI,
            pixel_count: 93,
        },
    ]
);

fn main() {
    Desktop::new_3d::<TunnelLayout>().start(|driver| {
        let mut control = ControlBuilder::new_3d()
            .with_layout::<TunnelLayout, { TunnelLayout::PIXEL_COUNT }>()
            .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
                ..Default::default()
            })
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
