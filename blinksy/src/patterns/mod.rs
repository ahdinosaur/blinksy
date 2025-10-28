//! # Pattern Implementations
//!
//! This is the library of built-in patterns.
//!
//! - [`rainbow`][]: A basic scrolling rainbow.
//! - [`noise`]: A flow through random noise functions.
//!
//! If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

#[cfg(feature = "noise")]
pub mod noise;
pub mod rainbow;
