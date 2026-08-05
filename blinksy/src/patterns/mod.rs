//! # Pattern Implementations
//!
//! This is the library of built-in patterns.
//!
//! - [`rainbow`]: A basic scrolling rainbow.
//! - [`noise`]: A flow through random noise functions.
//! - [`composite_pattern!`]: A macro allowing you to combine multiple patterns, switchable at runtime.
//!
//! If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!
//!
//! [`composite_pattern!`]: crate::composite_pattern!

pub mod noise;
pub mod rainbow;
