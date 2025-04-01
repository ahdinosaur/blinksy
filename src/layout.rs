use core::iter::once;

use glam::Vec2;

#[derive(Debug)]
pub struct Layout1d;

#[derive(Debug)]
pub enum Shape2d {
    Point(Vec2),
    Line {
        start: Vec2,
        end: Vec2,
        pixel_count: usize,
    },
    // Note: Expects leds to be wired along rows.
    Grid {
        start: Vec2,
        row_end: Vec2,
        col_end: Vec2,
        row_pixel_count: usize,
        col_pixel_count: usize,
        /// Are rows of leds wired zig-zag or not
        serpentine: bool,
    },
    Arc {
        center: Vec2,
        radius: f32,
        angle_in_radians: f32,
        pixel_count: usize,
    },
}

impl Shape2d {
    fn points(&self) -> impl Iterator<Item = &Vec2> {
        match self {
            Shape2d::Point(point) => once(point),
            Shape2d::Line {
                start,
                end,
                pixel_count,
            } => {
                let spacing = (start - end) / *pixel_count as f32;
                (0..pixel_count).map(|index| start + index * spacing)
            }
            Shape2d::Grid {
                start,
                row_end,
                col_end,
                row_pixel_count,
                col_pixel_count,
                serpentine,
            } => todo!(),
            Shape2d::Arc {
                center,
                radius,
                angle_in_radians,
                pixel_count,
            } => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Layout2d<const NUM_SHAPES: usize>([Shape2d; NUM_SHAPES]);

#[derive(Debug)]
pub enum Shape3d {}

#[derive(Debug)]
pub struct Layout3d<const NUM_SHAPES: usize>([Shape3d; NUM_SHAPES]);
