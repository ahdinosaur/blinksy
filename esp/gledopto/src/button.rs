use button_driver::{Button, ButtonConfig, InstantProvider, Mode};
use core::ops::{Deref, DerefMut, Sub};
use esp_hal::{
    gpio::{GpioPin, Input, InputConfig, Pull},
    time::{Duration, Instant},
};

pub struct FunctionButton<'a>(Button<Input<'a>, ButtonInstant, Duration>);

impl FunctionButton<'_> {
    pub fn new(pin: GpioPin<0>) -> Self {
        let input = Input::new(pin, InputConfig::default().with_pull(Pull::Up));
        let button_config = ButtonConfig::<Duration> {
            mode: Mode::PullUp,
            debounce: Duration::from_micros(900),
            release: Duration::from_millis(150),
            hold: Duration::from_millis(500),
        };
        let button = Button::new(input, button_config);
        Self(button)
    }
}

impl<'a> Deref for FunctionButton<'a> {
    type Target = Button<Input<'a>, ButtonInstant, Duration>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FunctionButton<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ButtonInstant(Instant);

impl Sub for ButtonInstant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl InstantProvider<Duration> for ButtonInstant {
    fn now() -> Self {
        Self(Instant::now())
    }
}
