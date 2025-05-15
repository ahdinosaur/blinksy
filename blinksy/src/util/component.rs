/// Blinksy represents values, such as an LED color component, either as:
///
/// - a normalized `f32` between 0.0 and 1.0 (inclusive)
/// - an integer between that type's min and max
///
/// This trait enables conversions between these two cases.
pub trait Component: Copy {
    /// Converts the component value to a normalized f32 in range [0.0, 1.0].
    fn to_normalized_f32(self) -> f32;

    /// Creates a component value from a normalized f32 in range [0.0, 1.0].
    fn from_normalized_f32(value: f32) -> Self;
}

macro_rules! impl_component_for_uint {
    ($T:ident) => {
        impl Component for $T {
            fn to_normalized_f32(self) -> f32 {
                self as f32 / ($T::MAX as f32)
            }

            fn from_normalized_f32(value: f32) -> Self {
                (value * ($T::MAX as f32)) as $T
            }
        }
    };
}

impl_component_for_uint!(u8);
impl_component_for_uint!(u16);
impl_component_for_uint!(u32);

impl Component for f32 {
    fn to_normalized_f32(self) -> f32 {
        self.clamp(0., 1.)
    }

    fn from_normalized_f32(value: f32) -> f32 {
        value
    }
}
