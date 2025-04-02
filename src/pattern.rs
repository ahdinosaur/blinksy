use crate::Layout;

pub trait Pattern<const NUM_SHAPES: usize, const LAYOUT: Layout<{ NUM_SHAPES }>> {
    type Params;
    type Color;

    fn new(params: Self::Params) -> Self;
    fn tick(&self, time_in_ms: u64) -> [Self::Color; LAYOUT.pixel_count()];
}
