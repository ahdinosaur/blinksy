use core::{
    iter::{once, Once},
    marker::PhantomData,
    ops::{Add, Mul},
};

pub use glam::Vec2;
use num_traits::FromPrimitive;

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

#[derive(Debug)]
pub enum Shape2dPointsIterator {
    Point(Once<Vec2>),
    Line(StepIterator<Vec2, f32>),
}

impl From<Once<Vec2>> for Shape2dPointsIterator {
    fn from(value: Once<Vec2>) -> Self {
        Shape2dPointsIterator::Point(value)
    }
}

#[derive(Debug)]
pub struct StepIterator<Item, Scalar> {
    start: Item,
    step: Item,
    index: usize,
    length: usize,
    scalar: PhantomData<Scalar>,
}

impl<Item, Scalar> StepIterator<Item, Scalar> {
    pub fn new(start: Item, step: Item, length: usize) -> Self {
        Self {
            start,
            step,
            index: 0,
            length,
            scalar: PhantomData,
        }
    }
}

impl<Item, Scalar> Iterator for StepIterator<Item, Scalar>
where
    Item: Add<Output = Item> + Copy,
    Scalar: FromPrimitive + Mul<Item, Output = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None;
        }
        let index = Scalar::from_usize(self.index)?;
        let next = self.start + index * self.step;
        self.index += 1;
        Some(next)
    }
}

impl From<StepIterator<Vec2, f32>> for Shape2dPointsIterator {
    fn from(value: StepIterator<Vec2, f32>) -> Self {
        Shape2dPointsIterator::Line(value)
    }
}

impl Shape2d {
    pub const fn pixel_count(&self) -> usize {
        match *self {
            Shape2d::Point(_) => 1,
            Shape2d::Line { pixel_count, .. } => pixel_count,
            Shape2d::Grid {
                row_pixel_count,
                col_pixel_count,
                ..
            } => row_pixel_count * col_pixel_count,
            Shape2d::Arc { pixel_count, .. } => pixel_count,
        }
    }

    pub fn points(&self) -> Shape2dPointsIterator {
        match *self {
            Shape2d::Point(point) => once(point).into(),
            Shape2d::Line {
                start,
                end,
                pixel_count,
            } => {
                let step = (start - end) / pixel_count as f32;
                StepIterator::new(start, step, pixel_count).into()
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

impl<const NUM_SHAPES: usize> Layout2d<NUM_SHAPES> {
    pub const fn new(shapes: [Shape2d; NUM_SHAPES]) -> Self {
        Self(shapes)
    }

    pub const fn pixel_count(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < NUM_SHAPES {
            count += self.0[i].pixel_count();
            i += 1;
        }
        count
    }
}

#[derive(Debug)]
pub enum Shape3d {}

#[derive(Debug)]
pub struct Layout3d<const NUM_SHAPES: usize>([Shape3d; NUM_SHAPES]);
