use super::{
    ColorComponent, ColorCorrection, LedChannels, LedColor, LedRgb, LedRgbw, LinearRgb, LinearRgbw,
};

/// Trait for types that can be converted to output colors
///
/// This trait defines methods to convert a color type to various
/// output formats suitable for LED hardware.
pub trait OutputColor: Sized {
    /// Converts this color to a linear RGB representation
    fn to_linear_rgb(self) -> LinearRgb;

    /// Converts this color to a linear RGBW representation
    fn to_linear_rgbw(self) -> LinearRgbw;

    /// Converts this color to an RGB format suitable for direct LED output
    ///
    /// # Arguments
    ///
    /// * `brightness` - Overall brightness scaling factor
    /// * `gamma` - Additional gamma correction factor
    /// * `correction` - Color correction factors
    fn to_led_rgb<C: ColorComponent>(
        self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> LedRgb<C> {
        self.to_linear_rgb()
            .to_led_rgb(brightness, gamma, correction)
    }

    /// Converts this color to an RGBW format suitable for direct LED output
    ///
    /// # Arguments
    ///
    /// * `brightness` - Overall brightness scaling factor (0.0 to 1.0)
    /// * `gamma` - Additional gamma correction factor
    /// * `correction` - Color correction factors
    fn to_led_rgbw<C: ColorComponent>(
        self,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> LedRgbw<C> {
        self.to_linear_rgbw()
            .to_led_rgbw(brightness, gamma, correction)
    }

    /// Converts this color to channels suitable for the specified LED format
    ///
    /// # Arguments
    ///
    /// * `channels` - The channel format specification
    /// * `brightness` - Overall brightness scaling factor (0.0 to 1.0)
    /// * `gamma` - Additional gamma correction factor
    /// * `correction` - Color correction factors
    fn to_led<C: ColorComponent + Copy>(
        self,
        channels: LedChannels,
        brightness: f32,
        gamma: f32,
        correction: ColorCorrection,
    ) -> LedColor<C> {
        match channels {
            LedChannels::Rgb(rgb_order) => {
                let rgb = self.to_led_rgb(brightness, gamma, correction);
                LedColor::Rgb(rgb_order.reorder(rgb.as_array()))
            }
            LedChannels::Rgbw(rgbw_order) => {
                let rgbw = self.to_led_rgbw(brightness, gamma, correction);
                LedColor::Rgbw(rgbw_order.reorder(rgbw.as_array()))
            }
        }
    }
}
