# `gledopto`

Rust **no-std** [embedded](https://github.com/rust-embedded/awesome-embedded-rust) board support crate for Gledopto ESP32 Digital LED controllers.

Uses [Blinksy](https://github.com/ahdinosaur/blinksy): an LED control library for 1D, 2D, and 3D LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).

## Supported Boards

- [x] [Gledopto GL-C-016WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_1), `gl_c_016wl_d`
- [x] [Gledopto GL-C-017WL-D](https://www.gledopto.eu/gledopto-esp32-wled-uart_5), `gl_c_017wl_d`

Select the board by using its respective feature.

## Features

- [x] LED control using [`blinksy`](https://github.com/ahdinosaur/blinksy)
- [x] Built-in "Function" button
- [ ] Alternative "IO33" button
- [ ] Built-in microphone

## Examples

### 2D APA102 Grid with Noise Pattern

https://github.com/user-attachments/assets/1c1cf3a2-f65c-4152-b444-29834ac749ee

<details>
<summary>
    Click to see code
</summary>

```rust
#![no_std]
#![no_main]

use blinksy::{
    layout::{Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
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
            horizontal_end: Vec2::new(1., -1.),
            vertical_end: Vec2::new(-1., 1.),
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
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

</details>

### 1D WS2812 Strip with Rainbow Pattern

https://github.com/user-attachments/assets/703fe31d-e7ca-4e08-ae2b-7829c0d4d52e

<details>
<summary>
    Click to see code
</summary>

```rust
#![no_std]
#![no_main]

use blinksy::{
    layout::Layout1d,
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
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

</details>

## Getting started

To quickstart a project, see [`blinksy-quickstart-gledopto`][blinksy-quickstart-gledopto].

[blinksy-quickstart-gledopto]: https://github.com/ahdinosaur/blinksy-quickstart-gledopto

### Resources

As the Gledopto controller is an ESP32, if you want to get started here are some more resources to help:

- [The Rust on ESP Book](https://docs.esp-rs.org/book/introduction.html): An overall guide on ESP32 on Rust
- [esp-hal](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32/esp_hal/index.html): The Hardware Abstraction Layer for an ESP32 on Rust
- [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html): How to install the Xtensa target for Rust, required for ESP32
- [esp-generate](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/esp-generate.html): A template to help you kickstart your project

And in case they are helpful:

- [ESP no-std book](https://docs.esp-rs.org/no_std-training)
- [ESP no-std examples](https://github.com/esp-rs/no_std-training)
- [Gledopto GL-C-016WL-D page](https://www.gledopto.eu/gledopto-esp32-wled-uart_1)
- [Gledopto GL-C-016WL-D user instructions](https://www.gledopto.eu/mediafiles/anleitungen/7002-gl-c-016wl-d-eng.pdf)
