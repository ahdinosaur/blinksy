//! # Time Library
//!
//! Types to represent time are provided by the [`fugit`] crate:
//!
//! - [`Megahertz`]: For specifying clock rates in MHz
//! - [`Nanoseconds`]: For specifying timing durations in nanoseconds
//!
//! [`fugit`]: https://docs.rs/fugit

/// Represents a frequency in megahertz (MHz).
///
/// Used for specifying clock speeds for drivers and timing calculations.
pub use fugit::MegahertzU32 as Megahertz;

/// Represents a duration in nanoseconds.
///
/// Used for precise timing control in LED driver protocols and animations.
pub use fugit::NanosDurationU32 as Nanoseconds;
