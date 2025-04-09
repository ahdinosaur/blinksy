# `gledopto`

Rust `no_std` [embedded](https://github.com/rust-embedded/awesome-embedded-rust) board support crate for Gledopto ESP32 Digital LED controllers.

## Supported Boards

Currently this library only supports one board:

- [x] [Gledopto GL-C-016WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_1), `gl_c_016wl_d`

Select the board by using its respective feature.

## Features

- [ ] 1D, 2D, or 3D LED control using [`blinksy`](https://github.com/ahdinosaur/blinksy)
  - Supported LED chips [here](https://github.ahdinosaur/blinksy#supported-led-chips)
- [x] Built-in "Function" button
- [ ] Alternative "IO33" button
- [ ] Built-in microphone

## Getting started

### Pre-requisites

- Install Rust with `rustup`
- Install ESP components

```shell
cargo install espup
espup install
```

- Install `espflash`

```shell
cargo install espflash
```

- On Linux, add user to `dialout` group

```shell
sudo adduser $USER dialout
```

### Run An Example

Source the ESP environment variables

```shell
. $HOME/export-esp.sh
```

(See also: https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables )

```shell
cargo run --example dev
```

## Resources

- Rust on ESP book: https://docs.esp-rs.org/book
- ESP no-std book: https://docs.esp-rs.org/no_std-training
- ESP no-std examples: https://github.com/esp-rs/no_std-training
- Gledopto GL-C-016WL-D page: https://www.gledopto.eu/gledopto-esp32-wled-uart_1
- Gledopto GL-C-016WL-D user instructions: https://www.gledopto.eu/mediafiles/anleitungen/7002-gl-c-016wl-d-eng.pdf
- smart-leds: https://github.com/smart-leds-rs/smart-leds
- esp-hal-smartled: https://github.com/DaveRichmond/esp-hal-smartled
- FastLED: https://github.com/FastLED/FastLED

## TODO

- Add support for built-in i2s microphone
  - https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32/esp_hal/i2s/master/index.html
  - https://github.com/decaday/embedded-audio/blob/master/embedded-audio/src/stream/cpal_stream.rs
- Add support for beat detection
  - https://github.com/phip1611/beat-detector
