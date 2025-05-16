# Blinksy 🟥🟩🟦

Blinksy is a **Rust** **no-std**, **no-alloc** LED control library for 1D, 2D, and soon 3D LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).

## How Blinksy works

- Define LED layouts in 1D, 2D, or soon 3D space
- Create your visual pattern (effect), or choose from our built-in library
  - The pattern will compute colors for each LED based on its position
- Drive various LED chipsets with each frame of colors

## Features

- **No-std, No-alloc:** Designed to run on embedded targets.
- **Layout Abstraction:** Define 1D, 2D, or soon 3D LED positions with shapes (grids, lines, arcs, points, etc).
- **Pattern (Effect) Library:**
  - **Rainbow**
  - **Noise**
  - [Make an issue](https://github.com/ahdinosaur/blinksy/issues) if you want help to port a pattern from FastLED / WLED to Rust!
- **Multi‑Chipset Support:**
  - **APA102**
  - **WS2812B**
  - [Make an issue](https://github.com/ahdinosaur/blinksy/issues) if you want help to support a new chipset!
- **Board Support Packages**:
  - **Gledopto**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
  - (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards
- **RGBW Support:** Supports RGB + White color channels
- **Desktop Simulation:** Run a simulation of a layout and pattern on your computer to experiment with ideas.
- (TODO) **Audio-Reactive**: Easily integrate audio reactivity into visual patterns. ([#9](https://github.com/ahdinosaur/blinksy/issues/9))
- (TODO) **Advanced LED Calibration**: Supports color correction based on LED-specific spectrometer data. ([#24](https://github.com/ahdinosaur/blinksy/issues/24))
- (TODO) **Multi-LED Solver**: Supports LEDs with color channels beyond RGB or RGBW. ([#23](https://github.com/ahdinosaur/blinksy/issues/23))

## Modules

- [`blinksy`](./blinksy) : [![crates.io version](https://img.shields.io/crates/v/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy) [![download](https://img.shields.io/crates/d/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy)
- [`blinksy-desktop`](./blinksy-desktop) : [![crates.io version](https://img.shields.io/crates/v/blinksy-desktop.svg?style=flat-square)](https://crates.io/crates/blinksy-desktop) [![download](https://img.shields.io/crates/d/blinksy-desktop.svg?style=flat-square)](https://crates.io/crates/blinksy-desktop) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-desktop)
- [`blinksy-esp`](./esp/blinksy-esp) : [![crates.io version](https://img.shields.io/crates/v/blinksy-esp.svg?style=flat-square)](https://crates.io/crates/blinksy-esp) [![download](https://img.shields.io/crates/d/blinksy-esp.svg?style=flat-square)](https://crates.io/crates/blinksy-esp) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-esp)
- [`gledopto`](./esp/gledopto) : [![crates.io version](https://img.shields.io/crates/v/gledopto.svg?style=flat-square)](https://crates.io/crates/gledopto) [![download](https://img.shields.io/crates/d/gledopto.svg?style=flat-square)](https://crates.io/crates/gledopto) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/gledopto)

## Examples

For all examples, see:

- [Desktop examples in `./blinksy-desktop/examples`](./blinksy-desktop/examples)
- [Gledopto examples in `./gledopto/examples`](./gledopto/examples)

### 2D APA102 Grid with Noise Pattern

https://github.com/user-attachments/assets/1c1cf3a2-f65c-4152-b444-29834ac749ee

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

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

If you want to help, the best thing to do is use Blinksy for your own LED project, and share about your adventures.

## License

Blinksy is licensed under the [**European Union Public License (EUPL)**](./LICENSE).

We chose the EUPL, a copyleft license which combines reciprocity and share-alike, to ensure that Blinksy remains free and open.

You are free to use, modify, and share Blinksy freely. Whether for personal projects, art installations, or commercial products.

Only once you start distributing something based on changes to Blinksy, you must share any improvements back with the community by releasing your source code.

Unlike more viral copyleft licenses, you will not be required to release the source code for your entire project, only changes to Blinksy.
