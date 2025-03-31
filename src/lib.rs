#![no_std]

mod layout;
mod led;
mod pattern;
pub mod patterns;
mod pixels;
pub mod time;
mod util;

pub use crate::layout::*;
pub use crate::led::*;
pub use crate::pattern::*;
pub use crate::pixels::*;

pub struct Control<Layout, Pat, Params, Color, Writer, const NUM_PIXELS: usize>
where
    Pat: Pattern<NUM_PIXELS, Params = Params, Layout = Layout, Color = Color>,
    Writer: Fn([Color; NUM_PIXELS]),
{
    pattern: Pat,
    writer: Writer,
}

impl<Layout, Pat, Params, Color, Writer, const NUM_PIXELS: usize>
    Control<Layout, Pat, Params, Color, Writer, NUM_PIXELS>
where
    Pat: Pattern<NUM_PIXELS, Params = Params, Layout = Layout, Color = Color>,
    Writer: Fn([Color; NUM_PIXELS]),
{
    pub fn new(layout: Layout, params: Params, writer: Writer) -> Self {
        let pattern = Pat::new(params, layout);
        Self { pattern, writer }
    }

    pub fn tick(&self, time_in_ms: u64) {
        let pixels = self.pattern.tick(time_in_ms);
        (self.writer)(pixels);
    }
}
