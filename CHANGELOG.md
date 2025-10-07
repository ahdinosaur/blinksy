# Changelog

## UNRELEASED

Migration guide (0.10 -> UNRELEASED)

- `ControlBuilder::with_layout` generic type signature changes from `<Layout`> to `<Layout, const PIXEL_COUNT: usize>`
  - If your layout is `Layout`, then change `.with_layout<Layout>()` to `.with_layout::<Layout, { Layout::PIXEL_COUNT }>()`

```diff
-  .with_layout::<Layout>()
+  .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
```

- `Driver::write` now expects a `const PIXEL_COUNT: usize` generic constant as the first type argument.

Breaking changes:

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
