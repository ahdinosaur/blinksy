use core::{
    iter::{once, Once},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Add, Mul},
};

use defmt::info;
pub use glam::Vec2;
use num_traits::FromPrimitive;

#[derive(Debug, Clone)]
pub struct Layout1d;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct GridStepIterator<Item, Scalar> {
    start: Item,
    row_step: Item,
    col_step: Item,
    row_pixel_count: usize,
    col_pixel_count: usize,
    serpentine: bool,
    row_index: usize,
    col_index: usize,
    scalar: PhantomData<Scalar>,
}

impl<Item, Scalar> GridStepIterator<Item, Scalar> {
    pub fn new(
        start: Item,
        row_step: Item,
        col_step: Item,
        row_pixel_count: usize,
        col_pixel_count: usize,
        serpentine: bool,
    ) -> Self {
        Self {
            start,
            row_step,
            col_step,
            row_pixel_count,
            col_pixel_count,
            serpentine,
            row_index: 0,
            col_index: 0,
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
        if self.row_index >= self.row_pixel_count {
            return None;
        }
        let row_index = Scalar::from_usize(self.row_index)?;

        // If serpentine, reverse the column order on every other row.
        let col_index = if self.serpentine && (self.row_index % 2 == 1) {
            self.col_pixel_count - 1 - self.col_index
        } else {
            self.col_index
        };
        let col_index = Scalar::from_usize(col_index)?;

        let point = self.start + row_index * self.row_step + col_index * self.col_step;

        self.col_index += 1;
        if self.col_index >= self.col_pixel_count {
            self.col_index = 0;
            self.row_index += 1;
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
            } => {
                let row_step = (start - row_end) / row_pixel_count as f32;
                let col_step = (start - col_end) / col_pixel_count as f32;
                GridStepIterator::new(
                    start,
                    row_step,
                    col_step,
                    row_pixel_count,
                    col_pixel_count,
                    serpentine,
                )
                .into()
            }
            Shape2d::Arc {
                center,
                radius,
                angle_in_radians,
                pixel_count,
            } => todo!(),
        }
    }
}

/*
#[derive(Debug)]
pub struct Layout2dPointsIterator<const NUM_SHAPES: usize> {
    layout: Layout2d<NUM_SHAPES>,
    index: usize,
}

impl<const NUM_SHAPES: usize> Layout2dPointsIterator<NUM_SHAPES> {
    pub const fn new(layout: Layout2d<NUM_SHAPES>) -> Self {
        Self { layout, index: 0 }
    }
}


impl<const NUM_SHAPES: usize> Iterator for Layout2dPointsIterator<NUM_SHAPES> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.index >= NUM_SHAPES) {
            return None;
        }
        let next = self.layout.0[self.index]
    }
}
*/

#[derive(Debug, Clone)]
pub struct Layout2d<const NUM_SHAPES: usize>([Shape2d; NUM_SHAPES]);

impl<const NUM_SHAPES: usize> Layout2d<NUM_SHAPES> {
    pub fn points(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().flat_map(|s| s.points())
    }

    pub fn map_points<T, const NUM_PIXELS: usize>(
        &self,
        mapper: impl Fn(Vec2) -> T,
    ) -> [T; NUM_PIXELS]
    where
        T: Sized,
    {
        if self.pixel_count() != NUM_PIXELS {
            info!("run-time Layout2d::pixel_count() != compile-time NUM_PIXELS");
            panic!("run-time Layout2d::pixel_count() != compile-time NUM_PIXELS");
        }

        let mut array: [MaybeUninit<T>; NUM_PIXELS] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (index, point) in self.points().enumerate() {
            if index >= NUM_PIXELS {
                info!("overflow while mapping points");
                panic!("overflow while mapping points");
            }
            array[index].write(mapper(point));
        }

        // SAFETY: We have initialized exactly NUM_PIXELS elements.
        unsafe { array_assume_init(array) }
    }
}

unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY: The caller must guarantee that each element is initialized.
    core::ptr::read(&array as *const _ as *const [T; N])
}

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

#[derive(Debug, Clone)]
pub enum Shape3d {}

#[derive(Debug, Clone)]
pub struct Layout3d<const NUM_SHAPES: usize>([Shape3d; NUM_SHAPES]);
