use crate::Layout1d;

pub trait Pattern1d<Layout: Layout1d> {
    type Params;
    type Color;

    fn new(params: Self::Params) -> Self;
    fn tick(&self, time_in_ms: u64) -> [Self::Color; Layout::NUM_PIXELS];
}
