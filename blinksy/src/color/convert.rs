pub trait FromColor<Color>: Sized {
    fn from_color(color: Color) -> Self;
}

pub trait IntoColor<Color>: Sized {
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
