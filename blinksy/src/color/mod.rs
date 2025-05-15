//! # Color Types and Utilities
//!
//! This module provides types and utilities for working with different color spaces,
//! color conversions, and LED color representation.
//!
//! ## Color Spaces
//!
//! - [`Srgb`] - Standard RGB color space (gamma-corrected)
//! - [`LinearSrgb`] - Linear RGB color space (no gamma correction)
//! - [`GammaSrgb`] - RGB with custom gamma correction
//! - [`Xyz`] - CIE XYZ color space
//! - [`Lms`] - LMS cone response space
//! - [`Oklab`] - Perceptually uniform LAB space
//! - [`Okhsl`] - Perceptual HSL color space based on Oklab
//! - [`Okhsv`] - Perceptual HSV color space based on Oklab
//!
//! ## LED Color Handling
//!
//! - [`LedColor`] - Output-ready color data for LED hardware
//! - [`ColorCorrection`] - Correction factors for LED output
//! - [`LedChannels`] - Color channel formats for different LED chipsets

mod convert;
mod correction;
mod gamma_srgb;
mod led;
mod linear_srgb;
mod lms;
mod okhsl;
mod okhsv;
mod oklab;
mod srgb;
mod xyz;

pub use self::convert::*;
pub use self::correction::*;
pub use self::gamma_srgb::*;
pub use self::led::*;
pub use self::linear_srgb::*;
pub use self::lms::*;
pub use self::okhsl::*;
pub use self::okhsv::*;
pub use self::oklab::*;
pub use self::srgb::*;
pub use self::xyz::*;
