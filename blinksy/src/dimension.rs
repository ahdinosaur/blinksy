//! # Dimension Type Markers
//!
//! This module provides type-level markers for representing the dimensionality of LED layouts.
//! It allows the compiler to enforce correct combinations of layouts and patterns based on
//! their dimensional compatibility.
//!
//! The core types are:
//!
//! - [`Dim1d`]: Marker for one-dimensional layouts
//! - [`Dim2d`]: Marker for two-dimensional layouts
//!
//! These are used as type parameters in various parts of the API to ensure
//! dimensionally compatible combinations.

use crate::layout::{Layout1d, Layout2d};

/// Marker type for one-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 1D.
pub struct Dim1d;

/// Marker type for two-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 2D.
pub struct Dim2d;

/// Trait for associating layout types with dimension markers.
///
/// This trait creates the relationship between a layout type and its dimensionality,
/// which helps enforce correct combinations at compile time.
pub trait LayoutForDim<Dim> {}

/// All types implementing Layout1d are compatible with Dim1d.
impl<T> LayoutForDim<Dim1d> for T where T: Layout1d {}

/// All types implementing Layout2d are compatible with Dim2d.
impl<T> LayoutForDim<Dim2d> for T where T: Layout2d {}
