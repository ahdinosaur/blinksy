/// Trait for converting from another color type
pub trait FromColor<Color>: Sized {
    /// Converts from the source color type
    fn from_color(color: Color) -> Self;
}

/// Trait for converting to another color type
pub trait IntoColor<Color>: Sized {
    /// Converts into the target color type
    fn into_color(self) -> Color;
}

impl<T, U> IntoColor<U> for T
where
    U: FromColor<T>,
{
    #[inline]
    fn into_color(self) -> U {
        U::from_color(self)
    }
}

impl<T> FromColor<T> for T {
    #[inline]
    fn from_color(color: T) -> T {
        color
    }
}
