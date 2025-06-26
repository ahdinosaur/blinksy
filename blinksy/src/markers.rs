//! # Type Markers
//!
//! Dimension markers are type-level markers to represent dimensionality.
//!
//! - [`Dim1d`]: Marker for one-dimensional layouts
//! - [`Dim2d`]: Marker for two-dimensional layouts
//!
//! Function markers are type-level markers to represent execution type.
//!
//! - [`Blocking`]: Marker for blocking execution
//! - [`Async`]: Marker for async execution

/// Marker type for one-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 1D.
pub struct Dim1d;

/// Marker type for two-dimensional space.
///
/// Used as a type parameter to indicate patterns and controls that operate in 2D.
pub struct Dim2d;

/// Marker type for blocking execution.
///
/// Used as a type parameter to indicate execution will be blocking.
pub struct Blocking;

/// Marker type for async execution.
///
/// Used as a type parameter to indicate execution will be async.
pub struct Async;
