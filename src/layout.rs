#[derive(Debug)]
pub struct Layout1d;

#[derive(Debug)]
pub enum Shape2d {}

#[derive(Debug)]
pub struct Layout2d<const NUM_SHAPES: usize>([Shape2d; NUM_SHAPES]);

#[derive(Debug)]
pub enum Shape3d {}

#[derive(Debug)]
pub struct Layout3d<const NUM_SHAPES: usize>([Shape3d; NUM_SHAPES]);
