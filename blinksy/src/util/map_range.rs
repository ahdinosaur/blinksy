use num_traits::float::FloatCore;

/// Maps a value from one range to another.
///
/// # Arguments
/// * `value` - The input value to map.
/// * `in_min` - The lower bound of the input range.
/// * `in_max` - The upper bound of the input range.
/// * `out_min` - The lower bound of the output range.
/// * `out_max` - The upper bound of the output range.
///
/// # Example
/// ```
/// let x = map_range(2.5, 0.0, 5.0, -1.0, 1.0);
/// assert!((x - 0.0).abs() < 1e-6);
/// ```
pub fn map_range<N>(value: N, in_min: N, in_max: N, out_min: N, out_max: N) -> N
where
    N: FloatCore,
{
    (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
