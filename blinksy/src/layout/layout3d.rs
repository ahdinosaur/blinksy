use core::iter::{once, Once};

use super::iterators::{GridStepIterator, StepIterator};

pub use glam::Vec3;

/// Enumeration of three-dimensional shape primitives.
///
/// Each variant represents a different type of 3D arrangement of LEDs.
#[derive(Debug, Clone)]
pub enum Shape3d {
    /// A single point at the specified location.
    Point(Vec3),

    /// A line of LEDs from `start` to `end` with `pixel_count` LEDs.
    Line {
        /// Starting point of the line
        start: Vec3,
        /// Ending point of the line
        end: Vec3,
        /// Number of LEDs along the line
        pixel_count: usize,
    },

    /// A grid of LEDs defined by three corners and dimensions.
    Grid {
        /// Starting point (origin) of the grid
        start: Vec3,
        /// Ending point for first horizontal row (defines the horizontal axis)
        horizontal_end: Vec3,
        /// Ending point for first vertical column (defines the vertical axis)
        vertical_end: Vec3,
        /// Number of LEDs along each horizontal row
        horizontal_pixel_count: usize,
        /// Number of LEDs along each vertical column
        vertical_pixel_count: usize,
        /// Whether horizontal rows of LEDs are wired in a zigzag pattern
        serpentine: bool,
    },

    /// An arc of LEDs centered at `center` with the specified `radius`.
    Arc {
        /// Center point of the arc
        center: Vec3,
        /// Radius of the arc
        radius: f32,
        /// Angular span of the arc in radians
        angle_in_radians: f32,
        /// Number of LEDs along the arc
        pixel_count: usize,
    },
}

impl Shape3d {
    /// Returns the total number of pixels (LEDs) in this shape.
    pub const fn pixel_count(&self) -> usize {
        match *self {
            Shape3d::Point(_) => 1,
            Shape3d::Line { pixel_count, .. } => pixel_count,
            Shape3d::Grid {
                horizontal_pixel_count,
                vertical_pixel_count,
                ..
            } => horizontal_pixel_count * vertical_pixel_count,
            Shape3d::Arc { pixel_count, .. } => pixel_count,
        }
    }

    /// Returns an iterator over all points (LED positions) in this shape.
    pub fn points(&self) -> Shape3dPointsIterator {
        match *self {
            Shape3d::Point(point) => once(point).into(),
            Shape3d::Line {
                start,
                end,
                pixel_count,
            } => {
                let step = (end - start) / ((pixel_count - 1) as f32).max(1.);
                StepIterator::new(start, step, pixel_count).into()
            }
            Shape3d::Grid {
                start,
                horizontal_end,
                vertical_end,
                horizontal_pixel_count,
                vertical_pixel_count,
                serpentine,
            } => {
                let horizontal_step =
                    (horizontal_end - start) / (horizontal_pixel_count as f32 - 1.).max(1.);
                let vertical_step =
                    (vertical_end - start) / (vertical_pixel_count as f32 - 1.).max(1.);
                GridStepIterator::new(
                    start,
                    vertical_step,
                    horizontal_step,
                    horizontal_pixel_count,
                    vertical_pixel_count,
                    serpentine,
                )
                .into()
            }
            Shape3d::Arc {
                center: _,
                radius: _,
                angle_in_radians: _,
                pixel_count: _,
            } => todo!(),
        }
    }
}

/// Trait for three-dimensional LED layouts.
///
/// Implementors of this trait represent a 3D arrangement of LEDs using one or more shapes.
///
/// Use [`layout3d!`](crate::layout3d) to define a type that implements [`Layout3d`].
///
/// For our 3D space, we can think of:
///
/// - `(-1.0, -1.0, -1.0)` as the left bottom back
/// - `(-1.0, -1.0, 1.0)` as the left bottom front
/// - `(1.0, -1.0, -1.0)` as the right bottom back
/// - `(1.0, -1.0, 1.0)` as the right bottom front
/// - `(-1.0, 1.0, -1.0)` as the left top back
/// - `(-1.0, 1.0, 1.0)` as the left top front
/// - `(1.0, 1.0, -1.0)` as the right top back
/// - `(1.0, 1.0, 1.0)` as the right top front
pub trait Layout3d {
    /// The total number of LEDs in this layout.
    const PIXEL_COUNT: usize;

    /// Returns an iterator over the shapes that make up this layout.
    fn shapes() -> impl Iterator<Item = Shape3d>;

    /// Returns an iterator over all points (LED positions) in this layout.
    fn points() -> impl Iterator<Item = Vec3> {
        Self::shapes().flat_map(|s| s.points())
    }
}

/// Iterator over points in a 3D shape.
#[derive(Debug)]
pub enum Shape3dPointsIterator {
    /// Iterator for a single point
    Point(Once<Vec3>),
    /// Iterator for points along a line
    Line(StepIterator<Vec3, f32>),
    /// Iterator for points in a grid
    Grid(GridStepIterator<Vec3, f32>),
}

impl Iterator for Shape3dPointsIterator {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Shape3dPointsIterator::Point(iter) => iter.next(),
            Shape3dPointsIterator::Line(iter) => iter.next(),
            Shape3dPointsIterator::Grid(iter) => iter.next(),
        }
    }
}

impl From<Once<Vec3>> for Shape3dPointsIterator {
    fn from(value: Once<Vec3>) -> Self {
        Shape3dPointsIterator::Point(value)
    }
}

impl From<StepIterator<Vec3, f32>> for Shape3dPointsIterator {
    fn from(value: StepIterator<Vec3, f32>) -> Self {
        Shape3dPointsIterator::Line(value)
    }
}

impl From<GridStepIterator<Vec3, f32>> for Shape3dPointsIterator {
    fn from(value: GridStepIterator<Vec3, f32>) -> Self {
        Shape3dPointsIterator::Grid(value)
    }
}

#[macro_export]
macro_rules! layout3d {
    ($(#[$attr:meta])* $vis:vis $name:ident, [$($shape:expr),* $(,)?]) => {
        $(#[$attr])*
        $vis struct $name;

        impl $crate::layout::Layout3d for $name {
            const PIXEL_COUNT: usize = 0 $(+ $shape.pixel_count())*;

            fn shapes() -> impl Iterator<Item = $crate::layout::Shape3d> {
                [$($shape),*].into_iter()
            }
        }
    };
}
