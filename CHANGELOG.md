# Changelog

## UNRELEASED

Migration guide (0.10 -> UNRELEASED)

- `ControlBuilder::with_layout` generic type signature changes from `<Layout`> to `<Layout, const PIXEL_COUNT: usize>`
  - If your layout is `Layout`, then change `.with_layout<Layout>()` to `.with_layout::<Layout, { Layout::PIXEL_COUNT }>()`

```diff
-  .with_layout::<Layout>()
+  .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
```

- `ControlBuilder` now has a `with_frame_buffer_size` to provide a `FRAME_BUFFER_SIZE` constant.
  - So for example, to build a frame buffer to drive Ws2812 LEDs:

```diff
+  .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
```

- `Driver::write` now expects a `const PIXEL_COUNT: usize` generic constant as the first type argument.
- Built-in LED drivers have been refactored:
  - There is a generic driver for each type: `ClocklessDriver` and `ClockedDriver`.
  - You construct the generic driver by combining an Led with a Writer, both of that type.
  - All drivers and all writers are made through the builder pattern.
  - So for example: the clockless RMT driver for a WS2812 LED:

```rust
let driver = ClocklessDriver::default()
    .with_led::<Ws2812>()
    .with_writer(ClocklessRmt::default()
        .with_led::<Ws2812>()
        .with_rmt_buffer_size::<{ rmt_buffer_size::<Ws2812>(Layout::PIXEL_COUNT) }>()
        .with_channel(/* rmt channel */)
        .with_pin(/* rmt pin */)
        .build()
    );
```

  - Another example: the clocked SPI driver for an APA102 LED:

```rust
let driver = ClockedDriver::default()
    .with_led::<Apa102>()
    .with_writer(/* spi bus */)
```

- Clockless and clocked writers are also constructed through the builder pattern.
- Move LED definitions to `blinksy::leds` module, remove `Led` suffix from structs.
- `blinksy-esp` RMT driver expects `RMT_BUFFER_SIZE`.
  - Each memory block on the RMT driver has a size of 64, with a max of 8 memory blocks.
  - If async, you should use `64`.
  - If blocking and not too many LEDs, you should use `rmt_buffer_size::<Led>(Layout::PIXEL_COUNT)`.
  - If blocking and too many LEDs, you should use `64`.

Breaking changes:

- [#90](https://github.com/ahdinosaur/blinksy/pull/90): Re-architect to pre-calculate a buffer for each frame
- [#82](https://github.com/ahdinosaur/blinksy/pull/82): Use pixels buffer
  - Write all colors from `Pattern` iterator to pixel buffer, then write pixel buffer to LEDs with `Driver`.
- [#87](https://github.com/ahdinosaur/blinksy/pull/87): Refactor clocked LED drivers


## 0.10

Yee haw `blinksy` now supports async!

Migration guide (0.9 -> 0.10)

- You can now add the `async` feature and use async drivers.
  - For `gledopto`, add the `embassy` feature and use the `embassy_main` entry.
  - See examples for async usage.
- For projects using `gledopto` macros: no known breaking changes.
- For projects using `blinksy-esp`: `create_rmt_buffer!` macro no longer needs $num_leds.
- For projects using `blinksy` internals, changes:
  - Renamed `dimensions` module to `markers`
    - `dimension::Dim1d` -> `markers::Dim1d`
    - `dimension::Dim2d` -> `markers::Dim2d`
  - Move `dimension::LayoutForDim` to `layout::LayoutForDim`
  - Un-pub-ify internal functions within drivers

Breaking changes

- [#54](https://github.com/ahdinosaur/blinksy/pull/54): Add async drivers

Bug fixes:

- [#73](https://github.com/ahdinosaur/blinksy/pull/73): Fix APA102 color correction
- [#74](https://github.com/ahdinosaur/blinksy/pull/74): Remove extranaeous docstring
- [#75](https://github.com/ahdinosaur/blinksy/pull/75): Fix color temperature

## 0.9

Migration guide (0.8 -> 0.9)

- [`blinksy-desktop::driver::Desktop`](https://docs.rs/blinksy-desktop/0.8.0/blinksy_desktop/driver/struct.Desktop.html) has a new API:

```rust
fn main() {
    Desktop::new_1d::<Layout>().start(|driver| {
        let mut control = ControlBuilder::new_1d()
            .with_layout::<Layout>()
            .with_pattern::<Pattern>(PatternParams::default())
            .with_driver(driver)
            .build();

        loop {
            // ...
        }
    });
}
```

Breaking changes:

- [#72](https://github.com/ahdinosaur/blinksy/pull/72): Fix desktop simulator on macOS

## 0.8

Migration guide (0.7 -> 0.8)

- `rust-version` has been increased to 1.88, to follow `esp-hal`.
- You should upgrade to `espflash@4`, which will also require adding bootloader metadata via a macro.
  - See `gledopto` library and examples for `bootloader!()` on how to easily do this.
  - See [`esp-hal-1.0.0-rc.0` release](https://github.com/esp-rs/esp-hal/releases/tag/esp-hal-v1.0.0-rc.0) for "Special migration note" about this.

Breaking changes:

- [#66](https://github.com/ahdinosaur/blinksy/pull/66): Upgrade to esp-hal-1.0.0-rc.0

## 0.7

Migration guide (0.6 -> 0.7)

- No known breaking changes.

Feature additions:

- [#59](https://github.com/ahdinosaur/blinksy/pull/59): Add example of 3D volumetric cube
- [#65](https://github.com/ahdinosaur/blinksy/pull/65): Add support for GL-C-017WL-D
- [#63](https://github.com/ahdinosaur/blinksy/pull/63): Add arc shape

Bug fixes:

- [#61](https://github.com/ahdinosaur/blinksy/pull/61): Fix points step of Shape::Line

Visual improvements:

- [#64](https://github.com/ahdinosaur/blinksy/pull/64): Change rainbow pattern so is not just over x, but over all available dimensions

## 0.6

Woo hoo `blinksy` now supports 3D!

Migration guide (0.5 -> 0.6):

- No known breaking changes.

Feature additions:

- [#57](https://github.com/ahdinosaur/blinksy/pull/57): Add support for 3D layouts and animations
- [#58](https://github.com/ahdinosaur/blinksy/pull/58): Add 3D to desktop simulator

Minor changes:

- [#56](https://github.com/ahdinosaur/blinksy/pull/56): Refactor layout code into separate files

Thanks [@ahdinosaur](https://github.com/ahdinosaur) for their contributions.

## 0.5

Migration guide (0.4 -> 0.5):

- No known breaking changes.

Changes:

- [#50](https://github.com/ahdinosaur/blinksy/pull/50): Use `bluurryy/noise-functions` instead of `Razaekel/noise-rs`
- [#45](https://github.com/ahdinosaur/blinksy/pull/45): Add support for SK6812

Thanks [@not-jan](https://github.com/not-jan), [@nazo6](https://github.com/nazo6), and [@ahdinosaur](https://github.com/ahdinosaur) for their contributions.
