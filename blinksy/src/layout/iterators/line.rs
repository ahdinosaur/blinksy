use core::{
    marker::PhantomData,
    ops::{Add, Mul},
};
use num_traits::FromPrimitive;

/// Iterator that produces values by stepping from a start point.
///
/// Used for generating points along lines and other linear patterns.
#[derive(Debug)]
pub struct LineStepIterator<Item, Scalar> {
    start: Item,
    step: Item,
    index: usize,
    length: usize,
    scalar: PhantomData<Scalar>,
}

impl<Item, Scalar> LineStepIterator<Item, Scalar> {
    /// Creates a new step iterator.
    ///
    /// # Arguments
    ///
    /// - `start` - The starting item
    /// - `step` - The step between items
    /// - `length` - The number of items to generate
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

impl<Item, Scalar> Iterator for LineStepIterator<Item, Scalar>
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
