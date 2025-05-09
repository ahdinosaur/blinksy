pub trait ColorComponent {
    fn to_normalized_f32(self) -> f32;
}

macro_rules! impl_component_for_uint {
    ($T:ident) => {
        impl ColorComponent for $T {
            fn to_normalized_f32(self) -> f32 {
                self as f32 / ($T::MAX as f32)
            }
        }
    };
}

impl_component_for_uint!(u8);
impl_component_for_uint!(u16);
impl_component_for_uint!(u32);

impl ColorComponent for f32 {
    fn to_normalized_f32(self) -> f32 {
        self.clamp(0., 1.)
    }
}
