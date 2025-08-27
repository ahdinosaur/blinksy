# Changelog

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
