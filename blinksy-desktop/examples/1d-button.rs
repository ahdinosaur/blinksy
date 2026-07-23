use blinksy::{
    color::Okhsv, layout::Layout1d, layout1d, markers::Dim1d, pattern::Pattern, ControlBuilder,
};
use blinksy_desktop::{
    button::DesktopButton,
    driver::KeyCode,
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

layout1d!(StripLayout, 30);

pub struct FlatParams {
    color: Okhsv,
}

impl Default for FlatParams {
    fn default() -> Self {
        Self {
            color: Okhsv::new(0., 1.0, 1.0),
        }
    }
}

pub struct Flat(FlatParams);

impl<Layout> Pattern<Dim1d, Layout> for Flat
where
    Layout: Layout1d,
{
    type Params = FlatParams;
    type Color = Okhsv;

    fn new(params: Self::Params) -> Self {
        Self(params)
    }

    fn tick(&self, _time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        Layout::points().map(|_x| self.0.color)
    }

    fn set_params(&mut self, params: Self::Params) {
        self.0 = params;
    }
}

fn main() {
    // Press the space bar to change the color of the strip.
    // This example only cares about single clicks, so we set the release and hold times really short.
    let mut button = DesktopButton::new(
        Duration::from_micros(900),
        Duration::from_millis(1),
        Duration::from_millis(1),
    );
    Desktop::new_1d::<StripLayout>()
        .with_button(KeyCode::Space, &button)
        .start(move |driver| {
            let mut control = ControlBuilder::new_1d()
                .with_layout::<StripLayout, { StripLayout::PIXEL_COUNT }>()
                .with_pattern::<Flat>(FlatParams::default())
                .with_driver(driver)
                .with_frame_buffer_size::<{ StripLayout::PIXEL_COUNT }>()
                .build();

            loop {
                button.tick();
                // Note that `button.held_time()` only reports after the button is released,
                // so if the user holds the button down for a long time, it won't report that until they let go.
                // If you want to detect and action long presses sooner, you can use `button.current_holding_time()`, but note that
                // that reports repeatedly while the button is being held.
                //
                // The `button-driver` crate is capable of reporting on double and triple clicks, but we have deliberately tuned the
                // driver timing on this example to map them into singles.
                if button.is_clicked() || button.held_time().is_some() {
                    println!("Button activated! Changing color...");
                    let new_color = Okhsv::new(rand::random(), 1.0, 1.0);
                    control.set_pattern_params(FlatParams { color: new_color });
                }
                button.reset();

                if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                    break;
                }

                sleep(Duration::from_millis(16));
            }
        });
}
