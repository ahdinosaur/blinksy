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

use crate::pattern::Pattern as PatternTrait;

pub struct Control<const NUM_PIXELS: usize, Layout, Pattern, Writer>
where
    Pattern: PatternTrait<NUM_PIXELS, Layout = Layout>,
    Writer: FnMut([Pattern::Color; NUM_PIXELS]),
{
    pattern: Pattern,
    writer: Writer,
}

impl<const NUM_PIXELS: usize, Layout, Pattern, Writer> Control<NUM_PIXELS, Layout, Pattern, Writer>
where
    Pattern: PatternTrait<NUM_PIXELS, Layout = Layout>,
    Writer: FnMut([Pattern::Color; NUM_PIXELS]),
{
    pub fn new(pattern: Pattern, writer: Writer) -> Self {
        Self { pattern, writer }
    }

    pub fn tick(&mut self, time_in_ms: u64) {
        let pixels = self.pattern.tick(time_in_ms);
        (self.writer)(pixels);
    }
}

pub struct ControlBuilder<const NUM_PIXELS: usize, Layout, Pattern, Writer> {
    pub layout: Layout,
    pub pattern: Pattern,
    pub writer: Writer,
}

impl ControlBuilder<0, (), (), ()> {
    pub fn new() -> Self {
        ControlBuilder {
            layout: (),
            pattern: (),
            writer: (),
        }
    }
}

// TODO: take in layout as constant

impl<Layout, Pattern, Writer> ControlBuilder<0, Layout, Pattern, Writer> {
    pub fn with_num_pixels<const NUM_PIXELS: usize>(
        self,
    ) -> ControlBuilder<NUM_PIXELS, Layout, Pattern, Writer> {
        ControlBuilder {
            layout: self.layout,
            pattern: self.pattern,
            writer: self.writer,
        }
    }
}

impl<const NUM_PIXELS: usize, Pattern, Writer> ControlBuilder<NUM_PIXELS, (), Pattern, Writer> {
    pub fn with_layout<Layout>(
        self,
        layout: Layout,
    ) -> ControlBuilder<NUM_PIXELS, Layout, Pattern, Writer> {
        ControlBuilder {
            layout,
            pattern: self.pattern,
            writer: self.writer,
        }
    }
}

impl<const NUM_PIXELS: usize, Layout, Writer> ControlBuilder<NUM_PIXELS, Layout, (), Writer>
where
    Layout: Clone,
{
    pub fn with_pattern<Pattern>(
        self,
        params: Pattern::Params,
    ) -> ControlBuilder<NUM_PIXELS, Layout, Pattern, Writer>
    where
        Pattern: PatternTrait<NUM_PIXELS, Layout = Layout>,
    {
        let pattern = Pattern::new(params, self.layout.clone());
        ControlBuilder {
            layout: self.layout,
            pattern,
            writer: self.writer,
        }
    }
}

impl<const NUM_PIXELS: usize, Layout, Pattern> ControlBuilder<NUM_PIXELS, Layout, Pattern, ()> {
    pub fn with_writer<Writer>(
        self,
        writer: Writer,
    ) -> ControlBuilder<NUM_PIXELS, Layout, Pattern, Writer>
    where
        Pattern: PatternTrait<NUM_PIXELS>,
        Writer: FnMut([Pattern::Color; NUM_PIXELS]),
    {
        ControlBuilder {
            layout: self.layout,
            pattern: self.pattern,
            writer,
        }
    }
}

impl<const NUM_PIXELS: usize, Layout, Pattern, Writer>
    ControlBuilder<NUM_PIXELS, Layout, Pattern, Writer>
where
    Pattern: PatternTrait<NUM_PIXELS, Layout = Layout>,
    Writer: FnMut([Pattern::Color; NUM_PIXELS]),
{
    pub fn build(self) -> Control<NUM_PIXELS, Layout, Pattern, Writer> {
        Control::new(self.pattern, self.writer)
    }
}
