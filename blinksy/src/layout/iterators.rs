use core::{
    marker::PhantomData,
    ops::{Add, Mul},
};
use num_traits::FromPrimitive;

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
