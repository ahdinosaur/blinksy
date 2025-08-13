/// Trait for one-dimensional LED layouts.
///
/// Implementors of this trait represent a linear arrangement of LEDs.
///
/// Use [`layout1d!`](crate::layout1d) to define a type that implements [`Layout1d`].
///
/// For our 1D space, the first LED pixel will be at -1.0 and the last LED pixel will be at 1.0.
pub trait Layout1d {
    /// The total number of LEDs in this layout.
    const PIXEL_COUNT: usize;

    /// Returns an iterator over all points (LED positions) in this layout.
    fn points() -> impl Iterator<Item = f32> {
        let spacing = if Self::PIXEL_COUNT > 1 {
            2.0 / (Self::PIXEL_COUNT as f32 - 1.0)
        } else {
            0.0
        };
        (0..Self::PIXEL_COUNT).map(move |index| -1.0 + (index as f32 * spacing))
    }
}

/// Creates a one-dimensional LED layout from a pixel count.
///
/// # Arguments
///
/// * `#[$attr]` - Optional attributes to apply to the struct (e.g., `#[derive(Debug)]`)
/// * `$vis` - Optional visibility modifier (e.g., `pub`)
/// * `$name` - The name of the layout type to create
/// * `$pixel_count` - The number of LEDs in the layout
///
/// # Output
///
/// Macro output will be a type definition that implements [`Layout1d`].
///
/// # Example
///
/// ```rust
/// use blinksy::layout1d;
///
/// // Define a strip with 60 LEDs
/// layout1d!(Layout, 60);
///
/// // Define a public strip with 60 LEDs
/// layout1d!(pub PubLayout, 60);
///
/// // Define a layout with attributes
/// layout1d!(
///     #[doc = "A strip of 60 LEDs for the main display"]
///     #[derive(Debug)]
///     pub AttrsLayout,
///     60
/// );
/// ```
#[macro_export]
macro_rules! layout1d {
    ($(#[$attr:meta])* $vis:vis $name:ident, $pixel_count:expr) => {
        $(#[$attr])*
        $vis struct $name;

        impl $crate::layout::Layout1d for $name {
            const PIXEL_COUNT: usize = $pixel_count;
        }
    };
}
