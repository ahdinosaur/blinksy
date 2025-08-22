//! # Time Library
//!
//! Types to represent time are provided by the [`fugit`] crate:
//!
//! - [`Megahertz`]: For specifying clock rates in MHz
//! - [`Nanoseconds`]: For specifying timing durations in nanoseconds
//!
//! [`fugit`]: https://docs.rs/fugit

/// Frequency in hertz (Hz), backed by `u32`.
pub use fugit::HertzU32;

/// Frequency in hertz (Hz), backed by `u64`.
pub use fugit::HertzU64;

/// Frequency in kilohertz (KHz), backed by `u32`.
pub use fugit::KilohertzU32;

/// Frequency in kilohertz (KHz), backed by `u64`.
pub use fugit::KilohertzU64;

/// Frequency in megahertz (MHz), backed by `u32`.
pub use fugit::MegahertzU32;

/// Frequency in megahertz (MHz), backed by `u64`.
pub use fugit::MegahertzU64;

/// Duration in nanoseconds (ns), backed by `u32`.
pub use fugit::NanosDurationU32 as NanosecondsU32;

/// Duration in nanoseconds (ns), backed by `u64`.
pub use fugit::NanosDurationU64 as NanosecondsU64;

/// Duration in microseconds (μs), backed by `u32`.
pub use fugit::MicrosDurationU32 as MicrosecondsU32;

/// Duration in microseconds (μs), backed by `u64`.
pub use fugit::MicrosDurationU64 as MicrosecondsU64;

/// Duration in milliseconds (ms), backed by `u32`.
pub use fugit::MillisDurationU32 as MillisecondsU32;

/// Duration in milliseconds (ms), backed by `u64`.
pub use fugit::MillisDurationU64 as MillisecondsU64;

/// Duration in seconds (s), backed by `u32`.
pub use fugit::SecsDurationU32 as SecondsU32;

/// Duration in seconds (s), backed by `u64`.
pub use fugit::SecsDurationU64 as SecondsU64;

/// Duration in seconds (s), backed by `u32`.
pub use fugit::MinutesDurationU32 as MinutesU32;

/// Duration in seconds (s), backed by `u64`.
pub use fugit::MinutesDurationU64 as MinutesU64;

/// Duration in seconds (s), backed by `u32`.
pub use fugit::HoursDurationU32 as HoursU32;

/// Duration in seconds (s), backed by `u64`.
pub use fugit::HoursDurationU64 as HoursU64;

pub trait TimeSource {
    fn now(&mut self) -> MillisecondsU64;
}

impl<F> TimeSource for F
where
    F: FnMut() -> MillisecondsU64,
{
    fn now(&mut self) -> MillisecondsU64 {
        self()
    }
}
