//! # Time Library
//!
//! Types to represent time are provided by the [`fugit`] crate:
//!
//! - [`Megahertz`]: For specifying clock rates in MHz
//! - [`Nanoseconds`]: For specifying timing durations in nanoseconds
//!
//! [`fugit`]: https://docs.rs/fugit

/// Frequency in megahertz (MHz), backed by `u32`.
pub use fugit::MegahertzU32 as Megahertz;

/// Duration in microseconds (μs), backed by `u32`.
///
/// Used as an animation time source.
pub use fugit::MicrosDurationU32 as AppDuration;

/// Duration in nanoseconds (ns), backed by `u32`.
///
/// Used in protocol timing.
pub use fugit::NanosDurationU32 as ProtocolDuration;

/// Duration in seconds (s), backed by `f32`.
///
/// Used in animation timing.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AnimationDuration(pub f32);

impl AnimationDuration {
    /// A duration of zero seconds.
    pub const ZERO: Self = Self(0.0);

    /// Creates a duration from microseconds.
    #[inline]
    pub const fn from_micros(val: u32) -> Self {
        Self(val as f32 / 1_000_000.0)
    }

    /// Creates a duration from milliseconds.
    #[inline]
    pub const fn from_millis(val: u64) -> Self {
        Self(val as f32 / 1_000.0)
    }

    /// Creates a duration from seconds.
    #[inline]
    pub const fn from_secs(val: u64) -> Self {
        Self(val as f32)
    }

    /// Creates a duration from minutes.
    #[inline]
    pub const fn from_minutes(val: u64) -> Self {
        Self(val as f32 * 60.0)
    }

    /// Creates a duration from hours.
    #[inline]
    pub const fn from_hours(val: u64) -> Self {
        Self(val as f32 * 3600.0)
    }
}

pub trait TimeSource {
    fn elapsed(&mut self) -> AppDuration;
}

impl<F> TimeSource for F
where
    F: FnMut() -> AppDuration,
{
    fn elapsed(&mut self) -> AppDuration {
        self()
    }
}
