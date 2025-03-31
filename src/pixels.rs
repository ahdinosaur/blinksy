pub use palette::{Hsl, Hsv, IntoColor, Srgb};

pub struct Pixels<const N: usize>([Srgb; N]);

impl<const N: usize> IntoIterator for Pixels<N> {
    type Item = Srgb;
    type IntoIter = <[Srgb; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const N: usize> From<[T; N]> for Pixels<N>
where
    T: IntoColor<Srgb>,
{
    fn from(value: [T; N]) -> Self {
        Self(value.map(|t| t.into_color()))
    }
}
