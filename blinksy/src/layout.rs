//! # LED Layouts
//!
//! A layout defines the physical or logical positions of the LEDs in your setup, as
//! arrangements in 1D, 2D, and 3D space.
//!
//! - For 1D, use [`layout1d!`] to define a type that implements [`Layout1d`]
//! - For 2D, use [`layout2d!`] to define a type that implements [`Layout2d`]
//!
//! The layout traits provide a `PIXEL_COUNT` constant, which is the number of LEDs, and a
//! `.points()`. method, which maps each LED pixel into a 1D, 2D, or soon 3D space between -1.0 and
//! 1.0.
//!
//! ## 1D Layouts
//!
//! For simple linear arrangements, use the [`layout1d!`] macro:
//!
//! ```rust
//! use blinksy::layout1d;
//!
//! // Define a strip with 60 LEDs
//! layout1d!(Layout, 60);
//! ```
//!
//! ## 2D Layouts
//!
//! For 2D layouts, use the [`layout2d!`] macro with one or more [`Shape2d`] definitions:
//!
//! ```rust
//! use blinksy::{layout2d, layout::Shape2d, layout::Vec2};
//!
//! // Define a 16x16 LED grid
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         horizontal_end: Vec2::new(1., -1.),
//!         vertical_end: Vec2::new(-1., 1.),
//!         horizontal_pixel_count: 16,
//!         vertical_pixel_count: 16,
//!         serpentine: true,
//!     }]
//! );
//! ```
//!
//! [`layout1d!`]: crate::layout1d!
//! [`layout2d!`]: crate::layout2d!

use core::{
    iter::{once, Once},
    marker::PhantomData,
    ops::{Add, Mul},
};
pub use glam::Vec2;
use num_traits::FromPrimitive;

/// Trait for one-dimensional LED layouts.
///
/// Implementors of this trait represent a linear arrangement of LEDs.
///
/// Use [`layout1d!`](crate::layout1d) to define a type that implements [`Layout1d`].
///
/// For our 1D space, the first LED pixel will be at -1.0 and the last LED pixel will be at 1.0.
pub trait Layout1d {
    /// The total number of LEDs in this layout.
    const PIXEL_COUNT: usize;

    /// Returns an iterator over all points (LED positions) in this layout.
    fn points() -> impl Iterator<Item = f32> {
        let spacing = if Self::PIXEL_COUNT > 1 {
            2.0 / (Self::PIXEL_COUNT as f32 - 1.0)
        } else {
            0.0
        };
        (0..Self::PIXEL_COUNT).map(move |index| -1.0 + (index as f32 * spacing))
    }
}

/// Creates a one-dimensional LED layout from a pixel count.
///
/// # Arguments
///
/// * `#[$attr]` - Optional attributes to apply to the struct (e.g., `#[derive(Debug)]`)
/// * `$vis` - Optional visibility modifier (e.g., `pub`)
/// * `$name` - The name of the layout type to create
/// * `$pixel_count` - The number of LEDs in the layout
///
/// # Output
///
/// Macro output will be a type definition that implements [`Layout1d`].
///
/// # Example
///
/// ```rust
/// use blinksy::layout1d;
///
/// // Define a strip with 60 LEDs
/// layout1d!(Layout, 60);
///
/// // Define a public strip with 60 LEDs
/// layout1d!(pub PubLayout, 60);
///
/// // Define a layout with attributes
/// layout1d!(
///     #[doc = "A strip of 60 LEDs for the main display"]
///     #[derive(Debug)]
///     pub AttrsLayout,
///     60
/// );
/// ```
#[macro_export]
macro_rules! layout1d {
    ($(#[$attr:meta])* $vis:vis $name:ident, $pixel_count:expr) => {
        $(#[$attr])*
        $vis struct $name;

        impl $crate::layout::Layout1d for $name {
            const PIXEL_COUNT: usize = $pixel_count;
        }
    };
}

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

/// Iterator that produces values by stepping from a start point.
///
/// Used for generating points along lines and other linear patterns.
#[derive(Debug)]
pub struct StepIterator<Item, Scalar> {
    start: Item,
    step: Item,
    index: usize,
    length: usize,
    scalar: PhantomData<Scalar>,
}

impl<Item, Scalar> StepIterator<Item, Scalar> {
    /// Creates a new step iterator.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting item
    /// * `step` - The step between items
    /// * `length` - The number of items to generate
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

/// Iterator for grid points with support for serpentine (zigzag) patterns.
#[derive(Debug)]
pub struct GridStepIterator<Item, Scalar> {
    start: Item,
    vertical_step: Item,
    horizontal_step: Item,
    horizontal_pixel_count: usize,
    vertical_pixel_count: usize,
    serpentine: bool,
    horizontal_index: usize,
    vertical_index: usize,
    scalar: PhantomData<Scalar>,
}

impl<Item, Scalar> GridStepIterator<Item, Scalar> {
    /// Creates a new grid iterator.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting point (origin) of the grid
    /// * `vertical_step` - The vertical step between horizontal rows
    /// * `horizontal_step` - The horizontal step between vertical columns
    /// * `horizontal_pixel_count` - Number of pixels in a horizontal row
    /// * `vertical_pixel_count` - Number of pixels in a vertical column
    /// * `serpentine` - Whether to use zigzag pattern
    pub fn new(
        start: Item,
        vertical_step: Item,
        horizontal_step: Item,
        horizontal_pixel_count: usize,
        vertical_pixel_count: usize,
        serpentine: bool,
    ) -> Self {
        Self {
            start,
            vertical_step,
            horizontal_step,
            horizontal_pixel_count,
            vertical_pixel_count,
            serpentine,
            horizontal_index: 0,
            vertical_index: 0,
            scalar: PhantomData,
        }
    }
}

impl<Item, Scalar> Iterator for GridStepIterator<Item, Scalar>
where
    Item: Add<Output = Item> + Copy,
    Scalar: FromPrimitive + Mul<Item, Output = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vertical_index >= self.vertical_pixel_count {
            return None;
        }
        let vertical_index = Scalar::from_usize(self.vertical_index)?;
        let horizontal_index = if self.serpentine && (self.vertical_index % 2 == 1) {
            self.horizontal_pixel_count - 1 - self.horizontal_index
        } else {
            self.horizontal_index
        };
        let horizontal_index = Scalar::from_usize(horizontal_index)?;
        let point = self.start
            + vertical_index * self.vertical_step
            + horizontal_index * self.horizontal_step;
        self.horizontal_index += 1;
        if self.horizontal_index >= self.horizontal_pixel_count {
            self.horizontal_index = 0;
            self.vertical_index += 1;
        }
        Some(point)
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
