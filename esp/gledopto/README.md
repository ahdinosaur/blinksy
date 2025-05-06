# `gledopto`

Rust **no-std** [embedded](https://github.com/rust-embedded/awesome-embedded-rust) board support crate for Gledopto ESP32 Digital LED controllers.

Uses [Blinksy](https://github.com/ahdinosaur/blinksy): an LED control library designed for 1D, 2D, and 3D (audio-reactive) LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).

## Supported Boards

Currently this library only supports one board:

- [x] [Gledopto GL-C-016WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_1), `gl_c_016wl_d`

Select the board by using its respective feature.

## Features

- [x] 1D, 2D, or 3D LED control using [`blinksy`](https://github.com/ahdinosaur/blinksy)
- [x] Built-in "Function" button
- [ ] Alternative "IO33" button
- [ ] Built-in microphone

## Examples

### 2D APA102 Grid with Noise Pattern

https://github.com/user-attachments/assets/1c1cf3a2-f65c-4152-b444-29834ac749ee

```rust
#![no_std]
#![no_main]

use blinksy::{
    layout::{Shape2d, Vec2},
    layout2d,
    patterns::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use gledopto::{apa102, board, elapsed, main};

#[main]
fn main() -> ! {
    let p = board!();

    layout2d!(
        Layout,
        [Shape2d::Grid {
            start: Vec2::new(-1., -1.),
            row_end: Vec2::new(1., -1.),
            col_end: Vec2::new(-1., 1.),
            row_pixel_count: 16,
            col_pixel_count: 16,
            serpentine: true,
        }]
    );
    let mut control = ControlBuilder::new_2d()
        .with_layout::<Layout>()
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
            ..Default::default()
        })
        .with_driver(apa102!(p))
        .build();

    control.set_brightness(0.1);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
```

### 1D WS2812 Strip with Rainbow Pattern

https://github.com/user-attachments/assets/703fe31d-e7ca-4e08-ae2b-7829c0d4d52e

```rust
#![no_std]
#![no_main]

use blinksy::{
    layout::Layout1d,
    layout1d,
    patterns::{Rainbow, RainbowParams},
    ControlBuilder,
};
use gledopto::{board, elapsed, main, ws2812};

#[main]
fn main() -> ! {
    let p = board!();

    layout1d!(Layout, 60 * 5);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
```

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

Clone this repository and go into the `./esp` workspace:

```shell
git clone git@github.com:ahdinosaur/blinksy
cd blinksy/esp
```

Run an example:

```shell
cargo run --release -p gledopto --example ws2812-strip
```

## Resources

- Rust on ESP book: https://docs.esp-rs.org/book
- ESP no-std book: https://docs.esp-rs.org/no_std-training
- ESP no-std examples: https://github.com/esp-rs/no_std-training
- Gledopto GL-C-016WL-D page: https://www.gledopto.eu/gledopto-esp32-wled-uart_1
- Gledopto GL-C-016WL-D user instructions: https://www.gledopto.eu/mediafiles/anleitungen/7002-gl-c-016wl-d-eng.pdf
