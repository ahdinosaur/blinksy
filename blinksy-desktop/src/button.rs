//! # Fake desktop button input
//! This module provides an emulated button input system like that of `gledopto::button::FunctionButton`.

use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant},
};

use button_driver::{Button, ButtonConfig, InstantProvider, Mode, PinWrapper};

pub type ButtonState = Arc<AtomicBool>;

/// An emulated button input which maps a keypress to `button-driver`.
///
/// The idea is, if you want to use a button in your embedded application,
/// you can use this to preview its behaviour on the desktop.
///
/// Internally it keeps an `Arc<AtomicBool>` so state can be injected by the desktop driver.
/// See the `1d-button` example.
///
pub struct DesktopButton<D = Duration, I = Instant>
where
    I: InstantProvider<D>,
{
    button: Button<DesktopInput, I, D>,
}

impl Default for DesktopButton<Duration, Instant> {
    fn default() -> Self {
        let def = ButtonConfig::<Duration>::default();
        Self::new(def.debounce, def.release, def.hold)
    }
}

#[cfg(feature = "async")]
impl Default for DesktopButton<embassy_time::Duration, embassy_time::Instant> {
    fn default() -> Self {
        let def = ButtonConfig::<Duration>::default();
        DesktopButton::<embassy_time::Duration, embassy_time::Instant>::new(
            embassy_time::Duration::from_millis(def.debounce.as_millis() as u64),
            embassy_time::Duration::from_millis(def.release.as_millis() as u64),
            embassy_time::Duration::from_millis(def.hold.as_millis() as u64),
        )
    }
}

impl<D, I> DesktopButton<D, I>
where
    D: Clone + Ord,
    I: InstantProvider<D> + PartialEq,
{
    /// Constructor with explicit configuration times.
    ///
    /// Note that it is sometimes necessary to address this as `DesktopButton::<Duration, Instant>::new(...)` or `DesktopButton::<embassy_time::Duration, embassy_time::Instant>::new(...)` to avoid type inference issues,
    /// but see also the `new_std` and `new_embassy` convenience methods.
    pub fn new(debounce: D, release: D, hold: D) -> Self {
        let input = DesktopInput::default();
        let config = ButtonConfig::<D> {
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

impl DesktopButton<Duration, Instant> {
    /// Syntactic sugar for `DesktopButton::new` with std time types.
    pub fn new_std(debounce: Duration, release: Duration, hold: Duration) -> Self {
        Self::new(debounce, release, hold)
    }
}

#[cfg(feature = "async")]
impl DesktopButton<embassy_time::Duration, embassy_time::Instant> {
    /// Syntactic sugar for `DesktopButton::new` with embassy time types.
    pub fn new_embassy(
        debounce: embassy_time::Duration,
        release: embassy_time::Duration,
        hold: embassy_time::Duration,
    ) -> Self {
        Self::new(debounce, release, hold)
    }
}

impl<D, I> Deref for DesktopButton<D, I>
where
    I: InstantProvider<D>,
{
    type Target = Button<DesktopInput, I, D>;

    fn deref(&self) -> &Self::Target {
        &self.button
    }
}

impl<D, I> DerefMut for DesktopButton<D, I>
where
    I: InstantProvider<D>,
{
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
