use crate::layout::{Layout1d, Layout2d};

pub trait Pattern1d<Layout: Layout1d> {
    type Params;
    type Color;

    fn new(params: Self::Params) -> Self;
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color>;
}

pub trait Pattern2d<Layout: Layout2d> {
    type Params;
    type Color;

    fn new(params: Self::Params) -> Self;
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color>;
}
