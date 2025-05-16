//! # Pattern Interface
//!
//! A pattern, most similar to [a WLED effect], generates colors for LEDs based on time and
//! position.
//!
//! A [`Pattern`] receives:
//!
//! - The layout of the LEDs (through its type parameters)
//! - Configuration parameters during initialization
//! - The current time during each update cycle
//!
//! And produces:
//!
//! - A sequence of colors for each LED in the layout
//!
//! For the library of built-in patterns, see [patterns](crate::patterns).
//!
//! [a WLED effect]: https://kno.wled.ge/features/effects/

use crate::dimension::LayoutForDim;

/// Trait for creating visual effects on LED layouts.
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
/// use blinksy::{color::Okhsv, dimension::Dim1d, layout::Layout1d, pattern::Pattern};
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
///     type Color = Okhsv;
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
///             Okhsv::new(hue, 1.0, 1.0)
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
    type Color;

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
