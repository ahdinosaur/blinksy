//! # Pattern Interface
//!
//! This module defines the [`Pattern`] trait, which is the core abstraction for
//! visual effects in Blinksy. Patterns generate colors for LEDs based on time and position.
//!
//! A pattern takes:
//!
//! - Configuration parameters during initialization
//! - The current time during each update cycle
//! - Information about the layout it's operating on (through its type parameters)
//!
//! It produces:
//!
//! - A sequence of colors for each LED in the layout
//!
//! Patterns are generic over both the dimension they operate in and the specific layout
//! type, allowing compile-time enforcement of dimensional compatibility.

use crate::{color::OutputColor, dimension::LayoutForDim};

/// Trait for creating visual patterns on LED layouts.
///
/// Patterns generate colors for each LED in a layout based on time and position.
/// They are generic over both the dimension they operate in and the specific layout type.
///
/// # Type Parameters
///
/// * `Dim` - The dimension marker (Dim1d or Dim2d)
/// * `Layout` - The specific layout type
///
/// # Associated Types
///
/// * `Params` - Configuration parameters for the pattern
/// * `Color` - The color type produced by the pattern
///
/// # Example
///
/// ```rust
/// use blinksy::{color::Hsi, dimension::Dim1d, layout::Layout1d, pattern::Pattern};
///
/// struct RainbowParams {
///     speed: f32,
///     scale: f32,
/// }
///
/// struct Rainbow {
///     params: RainbowParams
/// }
///
/// impl<Layout> Pattern<Dim1d, Layout> for Rainbow
/// where
///     Layout: Layout1d,
/// {
///     type Params = RainbowParams;
///     type Color = Hsi;
///
///     fn new(params: Self::Params) -> Self {
///         Self { params }
///     }
///
///     fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
///         let offset = (time_in_ms as f32 * self.params.speed);
///         let step = 0.5 * self.params.scale;
///
///         Layout::points().map(move |x| {
///             let hue = x * step + offset;
///             Hsi::new(hue, 1.0, 1.0)
///         })
///     }
/// }
/// ```
pub trait Pattern<Dim, Layout>
where
    Layout: LayoutForDim<Dim>,
{
    /// The configuration parameters type for this pattern.
    type Params;

    /// The color type produced by this pattern.
    type Color: OutputColor;

    /// Creates a new pattern instance with the specified parameters.
    fn new(params: Self::Params) -> Self;

    /// Generates colors for all LEDs in the layout at the given time.
    ///
    /// # Arguments
    ///
    /// * `time_in_ms` - The current time in milliseconds
    ///
    /// # Returns
    ///
    /// An iterator yielding one color per LED in the layout
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color>;
}
