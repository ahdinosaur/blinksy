use crate::layout::{Layout1d, Layout2d};

pub struct Dim1d;
pub struct Dim2d;

pub trait LayoutForDim<Dim> {}

impl<T> LayoutForDim<Dim1d> for T where T: Layout1d {}
impl<T> LayoutForDim<Dim2d> for T where T: Layout2d {}
