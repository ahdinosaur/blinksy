#[allow(unused_imports)]
use num_traits::float::FloatCore;

pub(crate) fn map_f32_to_u8_range(value: f32, max: u8) -> u8 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * (max as f32)).round() as u8
}
