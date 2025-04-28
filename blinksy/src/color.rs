//! # Color Types and Utilities
//!
//! This module provides color representations and utilities for LED patterns.
//! It re-exports types from the `palette` crate to provide a rich set of color
//! types and operations.
//!
//! The main types are:
//!
//! - [`Srgb`]: sRGB color representation
//! - [`LinSrgb`]: Linear sRGB color representation
//! - [`Hsv`]: Hue-Saturation-Value color representation
//! - [`RgbHue`]: Hue component of RGB-based colors
//!
//! The module also provides conversion traits:
//!
//! - [`FromColor`]: For converting between different color types
//! - [`IntoColor`]: For converting a color into another color space

/// sRGB color representation.
///
/// This is the standard RGB color space used for most displays and LEDs.
pub use palette::Srgb;

/// Linear sRGB color representation.
///
/// Linear color space is used for accurate color mixing and transformations.
pub use palette::LinSrgb;

/// Hue-Saturation-Value color representation.
///
/// A more intuitive way to work with colors, especially for animations and patterns.
pub use palette::Hsv;

/// RGB hue component.
///
/// Represents the hue component (color angle) in RGB-based color spaces.
pub use palette::RgbHue;

/// Conversion trait for converting between color types.
pub use palette::FromColor;

/// Conversion trait for converting colors to other color spaces.
pub use palette::IntoColor;
