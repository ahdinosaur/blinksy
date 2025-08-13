use core::iter::{once, Once};

use super::iterators::{GridStepIterator, StepIterator};

pub use glam::Vec2;

/// Enumeration of two-dimensional shape primitives.
///
/// Each variant represents a different type of 2D arrangement of LEDs.
#[derive(Debug, Clone)]
pub enum Shape2d {
    /// A single point at the specified location.
    Point(Vec2),

    /// A line of LEDs from `start` to `end` with `pixel_count` LEDs.
    Line {
        /// Starting point of the line
        start: Vec2,
        /// Ending point of the line
        end: Vec2,
        /// Number of LEDs along the line
        pixel_count: usize,
    },

    /// A grid of LEDs defined by three corners and dimensions.
    Grid {
        /// Starting point (origin) of the grid
        start: Vec2,
        /// Ending point for first horizontal row (defines the horizontal axis)
        horizontal_end: Vec2,
        /// Ending point for first vertical column (defines the vertical axis)
        vertical_end: Vec2,
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
        center: Vec2,
        /// Radius of the arc
        radius: f32,
        /// Angular span of the arc in radians
        angle_in_radians: f32,
        /// Number of LEDs along the arc
        pixel_count: usize,
    },
}

/// Iterator over points in a 2D shape.
#[derive(Debug)]
pub enum Shape2dPointsIterator {
    /// Iterator for a single point
    Point(Once<Vec2>),
    /// Iterator for points along a line
    Line(StepIterator<Vec2, f32>),
    /// Iterator for points in a grid
    Grid(GridStepIterator<Vec2, f32>),
}

impl Iterator for Shape2dPointsIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Shape2dPointsIterator::Point(iter) => iter.next(),
            Shape2dPointsIterator::Line(iter) => iter.next(),
            Shape2dPointsIterator::Grid(iter) => iter.next(),
        }
    }
}

impl From<Once<Vec2>> for Shape2dPointsIterator {
    fn from(value: Once<Vec2>) -> Self {
        Shape2dPointsIterator::Point(value)
    }
}

impl From<StepIterator<Vec2, f32>> for Shape2dPointsIterator {
    fn from(value: StepIterator<Vec2, f32>) -> Self {
        Shape2dPointsIterator::Line(value)
    }
}

impl From<GridStepIterator<Vec2, f32>> for Shape2dPointsIterator {
    fn from(value: GridStepIterator<Vec2, f32>) -> Self {
        Shape2dPointsIterator::Grid(value)
    }
}

impl Shape2d {
    /// Returns the total number of pixels (LEDs) in this shape.
    pub const fn pixel_count(&self) -> usize {
        match *self {
            Shape2d::Point(_) => 1,
            Shape2d::Line { pixel_count, .. } => pixel_count,
            Shape2d::Grid {
                horizontal_pixel_count,
                vertical_pixel_count,
                ..
            } => horizontal_pixel_count * vertical_pixel_count,
            Shape2d::Arc { pixel_count, .. } => pixel_count,
        }
    }

    /// Returns an iterator over all points (LED positions) in this shape.
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
            Shape2d::Arc {
                center: _,
                radius: _,
                angle_in_radians: _,
                pixel_count: _,
            } => todo!(),
        }
    }
}

/// Trait for two-dimensional LED layouts.
///
/// Implementors of this trait represent a 2D arrangement of LEDs using one or more shapes.
///
/// Use [`layout2d!`](crate::layout2d) to define a type that implements [`Layout2d`].
///
/// For our 2D space, we can think of:
///
/// - `(-1.0, -1.0)` as the bottom left
/// - `(1.0, -1.0)` as the bottom right
/// - `(-1.0, 1.0)` as the top left
/// - `(1.0, 1.0)` as the top right
pub trait Layout2d {
    /// The total number of LEDs in this layout.
    const PIXEL_COUNT: usize;

    /// Returns an iterator over the shapes that make up this layout.
    fn shapes() -> impl Iterator<Item = Shape2d>;

    /// Returns an iterator over all points (LED positions) in this layout.
    fn points() -> impl Iterator<Item = Vec2> {
        Self::shapes().flat_map(|s| s.points())
    }
}

/// Creates a two-dimensional LED layout from a collection of shapes.
///
/// # Arguments
///
/// * `#[$attr]` - Optional attributes to apply to the struct (e.g., `#[derive(Debug)]`)
/// * `$vis` - Optional visibility modifier (e.g., `pub`)
/// * `$name` - The name of the layout type to create
/// * `[$($shape:expr),*]` - A list of Shape2d instances defining the layout
///
/// # Output
///
/// Macro output will be a type definition that implements [`Layout2d`].
///
/// # Example
///
/// ```rust
/// use blinksy::{layout2d, layout::Shape2d, layout::Vec2};
///
/// layout2d!(
///     Layout,
///     [Shape2d::Grid {
///         start: Vec2::new(-1., -1.),
///         horizontal_end: Vec2::new(1., -1.),
///         vertical_end: Vec2::new(-1., 1.),
///         horizontal_pixel_count: 16,
///         vertical_pixel_count: 16,
///         serpentine: true,
///     }]
/// );
/// ```
#[macro_export]
macro_rules! layout2d {
    ($(#[$attr:meta])* $vis:vis $name:ident, [$($shape:expr),* $(,)?]) => {
        $(#[$attr])*
        $vis struct $name;

        impl $crate::layout::Layout2d for $name {
            const PIXEL_COUNT: usize = 0 $(+ $shape.pixel_count())*;

            fn shapes() -> impl Iterator<Item = $crate::layout::Shape2d> {
                [$($shape),*].into_iter()
            }
        }
    };
}
