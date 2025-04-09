use crate::dimension::LayoutForDim;

pub trait Pattern<Dim, Layout>
where
    Layout: LayoutForDim<Dim>,
{
    type Params;
    type Color;

    fn new(params: Self::Params) -> Self;
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color>;
}
