//! # Fake desktop button input
//! This module provides an emulated button input system like that of `gledopto::button::FunctionButton`.

use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant},
};

use button_driver::{Button, ButtonConfig, Mode, PinWrapper};

pub type ButtonState = Arc<AtomicBool>;

/// An emulated button input which maps a keypress to `button-driver`.
///
/// The idea is, if you want to use a button in your embedded application,
/// you can use this to preview its behaviour on the desktop.
///
/// Internally it keeps an `Arc<AtomicBool>` so state can be injected by the desktop driver.
/// See the `1d-button` example.
///
pub struct DesktopButton {
    button: Button<DesktopInput, Instant, Duration>,
}

impl Default for DesktopButton {
    fn default() -> Self {
        Self::new(
            ButtonConfig::<Duration>::default().debounce,
            ButtonConfig::<Duration>::default().release,
            ButtonConfig::<Duration>::default().hold,
        )
    }
}

impl DesktopButton {
    /// Constructor with explicit configuration times
    pub fn new(debounce: Duration, release: Duration, hold: Duration) -> Self {
        let input = DesktopInput::default();
        let config = ButtonConfig::<Duration> {
            debounce,
            release,
            hold,
            mode: Mode::PullDown,
        };
        let button = Button::new(input, config);
        Self { button }
    }

    /// Returns a clone of the internal `Arc<AtomicBool>` so that it can be shared with other threads.
    pub fn clone_state(&self) -> ButtonState {
        self.button.pin.0.clone()
    }
}

impl Deref for DesktopButton {
    type Target = Button<DesktopInput, Instant, Duration>;

    fn deref(&self) -> &Self::Target {
        &self.button
    }
}

impl DerefMut for DesktopButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.button
    }
}

pub struct DesktopInput(ButtonState);
impl Default for DesktopInput {
    fn default() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
}

impl PinWrapper for DesktopInput {
    fn is_high(&mut self) -> bool {
        self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn is_low(&mut self) -> bool {
        !self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
}
