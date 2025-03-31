pub trait Pattern<const NUM_PIXELS: usize> {
    type Params;
    type Layout;
    type Color;

    fn new(params: Self::Params, layout: Self::Layout) -> Self;
    fn tick(&self, time_in_ms: u64) -> [Self::Color; NUM_PIXELS];
}
