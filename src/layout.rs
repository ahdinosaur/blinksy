#[derive(Debug)]
pub struct Layout1d;

#[derive(Debug)]
pub struct Point2d {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub enum Shape2d {
    Point(Point2d),
    Line {
        start: Point2d,
        end: Point2d,
    },
    // Note: Expects leds to be wired along rows.
    Grid {
        start: Point2d,
        row_end: Point2d,
        col_end: Point2d,
        row_pixel_spacing: f32,
        col_pixel_spacing: f32,
        /// Are rows of leds wired zig-zag or not
        serpentine: bool,
    },
    Arc {
        center: Point2d,
        radius: f32,
        angle_in_radians: f32,
        pixel_spacing: f32,
    },
}

#[derive(Debug)]
pub struct Layout2d<const NUM_SHAPES: usize>([Shape2d; NUM_SHAPES]);

#[derive(Debug)]
pub enum Shape3d {}

#[derive(Debug)]
pub struct Layout3d<const NUM_SHAPES: usize>([Shape3d; NUM_SHAPES]);
