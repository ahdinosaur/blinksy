use core::ops::{Add, Mul};

#[allow(unused_imports)]
use num_traits::Float;

/// Iterator for points along an arc (circular or elliptical) in 2D or 3D.
///
/// Parametric form:
///
/// ```text
/// point(theta) = origin + cos(theta) * cos_scalar + sin(theta) * sin_scalar
/// ```
///
/// - 2D circle with radius r:
///   - `cos_scalar = (r, 0)`
///   - `sin_scalar = (0, r)`
/// - 2D ellipse:
///   - `cos_scalar = axis_u`
///   - `sin_scalar = axis_v`
/// - 3D circle in plane spanned by orthonormal {u, v}:
///   - `cos_scalar = r * u`
///   - `sin_scalar = r * v`
/// - 3D ellipse:
///   - `cos_scalar = axis_u`
///   - `sin_scalar = axis_v`
#[derive(Debug, Clone)]
pub struct ArcStepIterator<Item> {
    origin: Item,
    cos_scalar: Item,
    sin_scalar: Item,
    start_angle_in_radians: f32,
    end_angle_in_radians: f32,
    index: usize,
    length: usize,
}

impl<Item> ArcStepIterator<Item> {
    /// Create a new arc iterator.
    ///
    /// - `origin`: center of the arc
    /// - `cos_scalar`, `sin_scalar`: basis vectors used in the parametric form
    /// - `start_angle_in_radians`: start angle (radians)
    /// - `end_angle_in_radians`: end angle (radians)
    /// - `length`: number of samples; 0 yields no points
    pub const fn new(
        origin: Item,
        cos_scalar: Item,
        sin_scalar: Item,
        start_angle_in_radians: f32,
        end_angle_in_radians: f32,
        length: usize,
    ) -> Self {
        Self {
            origin,
            cos_scalar,
            sin_scalar,
            start_angle_in_radians,
            end_angle_in_radians,
            index: 0,
            length,
        }
    }
}

impl<Item> Iterator for ArcStepIterator<Item>
where
    Item: Add<Output = Item> + Copy,
    f32: Mul<Item, Output = Item> + Mul<f32, Output = f32>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None;
        }

        let denom = if self.length > 1 {
            (self.length - 1) as f32
        } else {
            1.0
        };

        let t: f32 = (self.index as f32) / denom;
        let sweep: f32 = self.end_angle_in_radians - self.start_angle_in_radians;
        let theta: f32 = self.start_angle_in_radians + t * sweep;

        let point = self.origin + theta.cos() * self.cos_scalar + theta.sin() * self.sin_scalar;

        self.index += 1;

        Some(point)
    }
}
