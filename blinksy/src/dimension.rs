//! # Dimension Type Markers
//!
//! Dimension markers are type-level markers to represent dimensionality.
//!
//! - [`Dim1d`]: Marker for one-dimensional layouts
//! - [`Dim2d`]: Marker for two-dimensional layouts
//! - [`Dim3d`]: Marker for three-dimensional layouts

use crate::layout::{Layout1d, Layout2d, Layout3d};

/// Marker type for one-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 1D.
pub struct Dim1d;

/// Marker type for two-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 2D.
pub struct Dim2d;

/// Marker type for three-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 3D.
pub struct Dim3d;

/// Trait for associating layout types with dimension markers.
///
/// This trait creates the relationship between a layout type and its dimensionality,
/// which helps enforce correct combinations at compile time.
pub trait LayoutForDim<Dim> {}

/// All types implementing Layout1d are compatible with Dim1d.
impl<T> LayoutForDim<Dim1d> for T where T: Layout1d {}

/// All types implementing Layout2d are compatible with Dim2d.
impl<T> LayoutForDim<Dim2d> for T where T: Layout2d {}

/// All types implementing Layout2d are compatible with Dim2d.
impl<T> LayoutForDim<Dim3d> for T where T: Layout3d {}
