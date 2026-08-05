//! Composite patterns

/// Generates a composite [`Pattern`](crate::pattern::Pattern) that contains multiple patterns and can switch between them at runtime.
///
/// # Arguments
///
/// - `$name` - The name of the composite pattern type.
/// - `$color` - The color type used by the patterns. They must all use the same color type.
/// - `$dims` - The dimensions type used by the patterns. They must all use the same dimensions type.
/// - `$layout_class` - The layout trait that the patterns must implement.
/// - `$($pattern),+` - A list of pattern types to include in the composite.
///   - Note that types with generics must be entered using a type alias, e.g., `type Noise = Noise2d<OpenSimplex2>;`.
///   - You can use type aliases to include the same pattern with different parameters. e.g. `type Rainbow2 = Rainbow;`
///
/// # Generated Types
///
/// - `$name` - The composite pattern type. This is an enum that can hold any of the specified patterns.
///   The enum variants are named after the pattern types, e.g., `Composite::Noise1`, `Composite::Noise2`, etc.
/// - `${name}Params` - The composite parameters type. This is an enum that can hold any of the specified pattern parameter types.
/// - `${name}Iter` - (Internal) A helper iterator type for the `tick()` function. This is also an enum, one member per pattern.
///
///
/// # Example
///
/// This is how you might set up a composite pattern with three different noise patterns.
///
/// ```rust
/// # use blinksy::{color::Okhsv, layout2d, layout::{Layout2d, Shape2d, Vec2}, markers::Dim2d, pattern::{Pattern}, patterns::noise::{Noise2d, noise_fns::{OpenSimplex2, Perlin, Simplex}}};
/// layout2d!(
///     #[derive(Debug, Copy, Clone)]
///     Layout2,
///     [Shape2d::Grid {
///         start: Vec2::new(-1., -1.),
///         horizontal_end: Vec2::new(1., -1.),
///         vertical_end: Vec2::new(-1., 1.),
///         horizontal_pixel_count: 16,
///         vertical_pixel_count: 16,
///         serpentine: true,
///     }]
/// );
///
/// type Noise1 = Noise2d<OpenSimplex2>;
/// type Noise2 = Noise2d<Perlin>;
/// type Noise3 = Noise2d<Simplex>;
///
/// blinksy::composite_pattern! {
///     // NOTE! This is a macro, not a struct; arguments must appear in strict order.
///     name: MyComposite,
///     color: Okhsv,
///     dims: Dim2d,
///     layout: Layout2d,
///     patterns: [Noise1, Noise2, Noise3]
/// }
///
/// ```
///
/// In this example, the composite pattern is an enum named `Composite`, and its parameters are an enum named `CompositeParams`.
/// The variants of both are named `Noise1`, `Noise2`, and `Noise3`, corresponding to the three type names given to the macro.
///
/// ## Usage
///
/// When you want to switch pattern, call `set_params()` with the new pattern's parameters.
/// For example:
/// ```skip
/// control.set_pattern_params(
///     CompositeParams::Noise2(NoiseParams { time_scalar: 0.5, position_scalar: 0.5 })
/// );
/// ```
///
/// ## See Also
/// Refer to the `ws2812-face-button-composite.rs` example in the `gledopto` crate for a more complete demonstration.
///
#[macro_export]
macro_rules! composite_pattern {
    (
        name: $name:ident,
        color: $color:ty,
        dims: $dims:ty,
        layout: $layout_class:path,
        patterns: [$($pattern:ty),+ $(,)?] $(,)?
    ) => { blinksy::pastey::paste! {
        /// Composite pattern that can switch between multiple patterns
        pub enum $name<Layout>
        where
            Layout: $layout_class,
        {
            $($pattern($pattern),)+
            // An uninstantiable variant is necessary to make the enum generic over Layout.
            #[doc(hidden)]
            __Phantom(core::marker::PhantomData<Layout>),
        }

        impl <Dims, Layout> blinksy::pattern::Pattern<Dims, Layout> for $name<Layout>
        where
            Layout: $layout_class,
            Layout: blinksy::layout::LayoutForDim<Dims>,
        {
            type Params = [<$name Params>]<Layout>;
            type Color = $color;

            fn new(params: Self::Params) -> Self {
                Self::from(params)
            }

            fn tick(&self, _time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
                match self {
                    $(Self::$pattern(p) =>
                        [<$name Iter>]::$pattern(
                            <$pattern as blinksy::pattern::Pattern<$dims,Layout>>::tick(p, _time_in_ms)
                        ),
                    )+
                    // The phantom variant is guaranteed unreachable, as Infallible cannot be instantiated.
                    Self::__Phantom(_) => unreachable!(),
                }
            }
            fn set_params(&mut self, params: Self::Params) {
                *self = Self::from(params);
            }
        }

        impl <Layout> From<[<$name Params>]<Layout>> for $name<Layout>
        where
            Layout: $layout_class,
        {
            fn from(params: [<$name Params>]<Layout>) -> Self {
                match params {
                    $( [<$name Params>]::$pattern(p) => Self::$pattern(<$pattern as blinksy::pattern::Pattern<$dims, Layout>>::new(p)), )+
                }
            }
        }

        /// Composite parameters type
        ///
        /// NOTE: All the patterns' parameters and the `Layout` must be `Clone`.
        ///
        /// You can use
        #[derive(Clone, Debug)]
        pub enum [<$name Params>]<Layout>
        where
            Layout: $layout_class,
        {
            $($pattern(<$pattern as blinksy::pattern::Pattern<$dims, Layout>>::Params),)+
        }

        /// Composite iterator type for the `tick()` function
        #[allow(non_camel_case_types)]
        enum [<$name Iter>]<$([<P_ $pattern>],)+>
        {
            $($pattern([<P_ $pattern>]),)+
        }

        #[allow(non_camel_case_types)]
        impl <T, $([<P_ $pattern>],)+> Iterator for [<$name Iter>]<$([<P_ $pattern>],)+>
        where
            $([<P_ $pattern>]: Iterator<Item = T>,)+
        {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $(Self::$pattern(iter) => iter.next(),)+
                }
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                match self {
                    $(Self::$pattern(iter) => iter.size_hint(),)+
                }
            }
        }
    }};
}
