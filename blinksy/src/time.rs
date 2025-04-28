//! # Timing Utilities
//!
//! This module provides time-related types for precise timing control in LED animations.
//! It re-exports types from the `fugit` crate for convenient time handling in a no-std environment.
//!
//! The primary types are:
//!
//! - [`Megahertz`]: For specifying clock frequencies in MHz
//! - [`Nanoseconds`]: For precise timing durations in nanoseconds
//!
//! These types are used throughout the library for:
//!
//! - Configuring driver timing parameters
//! - Controlling animation speed and transitions
//! - Managing timing-sensitive LED protocols

/// Represents a frequency in megahertz (MHz).
///
/// Used for specifying clock speeds for drivers and timing calculations.
pub use fugit::MegahertzU32 as Megahertz;

/// Represents a duration in nanoseconds.
///
/// Used for precise timing control in LED driver protocols and animations.
pub use fugit::NanosDurationU32 as Nanoseconds;
