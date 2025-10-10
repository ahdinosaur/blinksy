use core::iter::{once, Once};

use crate::layout::ArcStepIterator;

use super::iterators::{GridStepIterator, LineStepIterator};

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

    /// A circular or elliptical arc in 2D.
    ///
    /// Parametric form:
    ///
    /// ```text
    /// point(theta) = center + cos(theta) * axis_u + sin(theta) * axis_v
    /// ```
    ///
    /// Angle and direction:
    ///
    /// - Theta = 0 lies along `axis_u` (i.e. `center + axis_u`)
    ///   - As theta increases, the point moves towards `axis_v` (counter-clockwise in the XY plane).
    /// - The arc is traced for theta in `[start_angle_in_radians, end_angle_in_radians]`.
    ///   - If `end` < `start`, the arc goes clockwise.
    /// - Positive angles are counter-clockwise.
    /// - To make a full ellipse, set end = start + [`TAU`].
    ///
    /// How to choose `axis_u` / `axis_v`:
    ///
    /// - Axis-aligned circle with with radius `r`:
    ///   - `axis_u = (r, 0)`
    ///   - `axis_v = (0, r)`
    /// - Axis-aligned ellipse with radii `(rx, ry)`:
    ///   - `axis_u = (rx, 0)`
    ///   - `axis_v = (0, ry)`
    /// - Rotated by `phi`:
    ///   - `axis_u = ( rx * cos(phi),  rx * sin(phi))`
    ///   - `axis_v = ( -ry * sin(phi), ry * cos(phi))`
    ///
    /// Notes:
    ///
    /// - `axis_u` and `axis_v` must not both be zero.
    /// - `axis_u` and `axis_v` need not be unit length of perpendicular.
    /// - The points returned by `shape::points()` of a `Shape2d::Arc`:
    ///   - Will have uniform density if a circular arc
    ///   - Will **not** have uniform density if an elliptical arc, as the points correspond to `theta`.
    ///
    /// [`TAU`]: https://doc.rust-lang.org/core/f32/consts/constant.TAU.html
    Arc {
        /// Center of the ellipse
        center: Vec2,
        /// Cosine-axis vector
        axis_u: Vec2,
        /// Sine-axis vector
        axis_v: Vec2,
        /// Start angle in radians
        start_angle_in_radians: f32,
        /// End angle in radians
        end_angle_in_radians: f32,
        /// Number of LEDs
        pixel_count: usize,
    },
}

/// Iterator over points in a 2D shape.
#[derive(Debug)]
pub enum Shape2dPointsIterator {
    /// Iterator for a single point
    Point(Once<Vec2>),
    /// Iterator for points along a line
    Line(LineStepIterator<Vec2, f32>),
    /// Iterator for points in a grid
    Grid(GridStepIterator<Vec2, f32>),
    /// Iterator for points along an arc
    Arc(ArcStepIterator<Vec2>),
}

impl Iterator for Shape2dPointsIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Shape2dPointsIterator::Point(iter) => iter.next(),
            Shape2dPointsIterator::Line(iter) => iter.next(),
            Shape2dPointsIterator::Grid(iter) => iter.next(),
            Shape2dPointsIterator::Arc(iter) => iter.next(),
        }
    }
}

impl From<Once<Vec2>> for Shape2dPointsIterator {
    fn from(value: Once<Vec2>) -> Self {
        Shape2dPointsIterator::Point(value)
    }
}

impl From<LineStepIterator<Vec2, f32>> for Shape2dPointsIterator {
    fn from(value: LineStepIterator<Vec2, f32>) -> Self {
        Shape2dPointsIterator::Line(value)
    }
}

impl From<GridStepIterator<Vec2, f32>> for Shape2dPointsIterator {
    fn from(value: GridStepIterator<Vec2, f32>) -> Self {
        Shape2dPointsIterator::Grid(value)
    }
}

impl From<ArcStepIterator<Vec2>> for Shape2dPointsIterator {
    fn from(value: ArcStepIterator<Vec2>) -> Self {
        Shape2dPointsIterator::Arc(value)
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
                let step = (end - start) / ((pixel_count - 1) as f32).max(1.);
                LineStepIterator::new(start, step, pixel_count).into()
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
                center,
                axis_u,
                axis_v,
                start_angle_in_radians,
                end_angle_in_radians,
                pixel_count,
            } => ArcStepIterator::new(
                center,
                axis_u,
                axis_v,
                start_angle_in_radians,
                end_angle_in_radians,
                pixel_count,
            )
            .into(),
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
/// - `(-1.0, -1.0)` as the left bottom
/// - `(1.0, -1.0)` as the right bottom
/// - `(-1.0, 1.0)` as the left top
/// - `(1.0, 1.0)` as the right top
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
/// - `#[$attr]` - Optional attributes to apply to the struct (e.g., `#[derive(Debug)]`)
/// - `$vis` - Optional visibility modifier (e.g., `pub`)
/// - `$name` - The name of the layout type to create
/// - `[$($shape:expr),*]` - A list of Shape2d instances defining the layout
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
