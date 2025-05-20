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
//! - [`Hsv`] - HSV color space
//! - [`Oklab`] - Perceptually uniform LAB space
//! - [`Okhsl`] - Perceptual HSL color space based on Oklab
//! - [`Okhsv`] - Perceptual HSV color space based on Oklab
//! - [`Xyz`] - CIE XYZ color space
//! - [`Lms`] - LMS cone response space
//!
//! ## Conversion Traits
//!
//! - [`FromColor`] - Convert from a color type
//! - [`IntoColor`] - Convert to a color type
//!
//! ## LED Output Modifiers
//!
//! - [`ColorCorrection`] - Correction factors for LED output
//!
//! ## LED Output
//!
//! - [`LedColor`] - Output-ready color data for LED hardware
//!   - [`LedRgb`]
//!   - [`LedRgbw`]
//! - [`LedChannels`] - Color channel formats for different LED chipsets
//!   - [`RgbChannels`]
//!   - [`RgbwChannels`]

mod convert;
mod correction;
mod gamma_srgb;
mod hsv;
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
pub use self::hsv::*;
pub use self::led::*;
pub use self::linear_srgb::*;
pub use self::lms::*;
pub use self::okhsl::*;
pub use self::okhsv::*;
pub use self::oklab::*;
pub use self::srgb::*;
pub use self::xyz::*;
