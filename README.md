<div align="center">
  <img
    src="https://i.imgur.com/0FQeTbC.gif"
    alt="Blinksy simulation of 2D grid with noise pattern"
    width="320px"
    height="313px"
  />
</div>

<h1 align="center">
  <a href="https://github.com/ahdinosaur/blinksy">
    Blinksy
  </a>
  🟥🟩🟦
</h1>

<div align="center">
  A <strong>Rust</strong> <em>no-std</em> <em>no-alloc</em> LED control library.
</div>

<div align="center">
  For <strong>1D</strong>, <strong>2D</strong>, and soon <strong>3D</strong> layouts.
</div>

<div align="center">
  Inspired by
  <a href="https://fastled.io/">FastLED</a>
  and
  <a href="https://kno.wled.ge/">WLED</a>.
</div>

<br />

<div align="center">

[![crates.io version](https://img.shields.io/crates/v/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy)
[![ci status](https://img.shields.io/github/checks-status/ahdinosaur/blinksy/main?style=flat-square)](https://github.com/ahdinosaur/blinksy/actions/workflows/ci.yml?query=branch%3Amain)
[![chat](https://img.shields.io/matrix/blinksy:matrix.org?style=flat-square&label=chat)](https://matrix.to/#/#blinksy:matrix.org)

</div>

### How Blinksy works

- Define your LED [`layout`][layout] in 1D, 2D, or 3D space
- Create your visual [`pattern`][pattern] (effect), or choose from our built-in [`patterns`][patterns] library
  - The pattern will compute colors for each LED based on its position
- Setup a [`driver`][driver] to send each frame of colors to your LEDs, using our built-in [`drivers`][drivers] library.

[layout]: https://docs.rs/blinksy/0.2/blinksy/layout/index.html
[pattern]: https://docs.rs/blinksy/0.2/blinksy/pattern/index.html
[patterns]: https://docs.rs/blinksy/0.2/blinksy/patterns/index.html
[driver]: https://docs.rs/blinksy/0.2/blinksy/driver/index.html
[drivers]: https://docs.rs/blinksy/0.2/blinksy/drivers/index.html

## Features

- **No-std, no-alloc**: Designed for embedded targets.
- **Spatial in 1D, 2D, or 3D**: Map out the shape of your LEDs in space.
- **Full color support**: Supports modern and classic color spaces.
- **Global settings**: Control overall brightness and color correction.
- **Desktop simulation**: Simulate your LEDs on your desktop to play with ideas.
- **RGB+W support**: Supports RGB + White color channels

### Multi‑Chipset Support

- **[APA102][apa102]**
- **[WS2812B][ws2812]**

If you want help to support a new chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[apa102]: https://docs.rs/blinksy/0.2/blinksy/drivers/apa102/index.html
[ws2812]: https://docs.rs/blinksy/0.2/blinksy/drivers/ws2812/index.html

### Pattern (Effect) Library:

- **[Rainbow][rainbow]**
- **[Noise][noise]**

If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[rainbow]: https://docs.rs/blinksy/0.2/blinksy/patterns/rainbow/index.html
[noise]: https://docs.rs/blinksy/0.2/blinksy/patterns/noise/index.html

### Board Support Packages

- **[Gledopto][gledopto]**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
- (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards

If you want help to support a new target, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[gledopto]: https://docs.rs/gledopto/0.2/gledopto

## Modules

- [`blinksy`](./blinksy) : [![crates.io version](https://img.shields.io/crates/v/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy) [![download](https://img.shields.io/crates/d/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy)
- [`blinksy-desktop`](./blinksy-desktop) : [![crates.io version](https://img.shields.io/crates/v/blinksy-desktop.svg?style=flat-square)](https://crates.io/crates/blinksy-desktop) [![download](https://img.shields.io/crates/d/blinksy-desktop.svg?style=flat-square)](https://crates.io/crates/blinksy-desktop) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-desktop)
- [`blinksy-esp`](./esp/blinksy-esp) : [![crates.io version](https://img.shields.io/crates/v/blinksy-esp.svg?style=flat-square)](https://crates.io/crates/blinksy-esp) [![download](https://img.shields.io/crates/d/blinksy-esp.svg?style=flat-square)](https://crates.io/crates/blinksy-esp) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-esp)
- [`gledopto`](./esp/gledopto) : [![crates.io version](https://img.shields.io/crates/v/gledopto.svg?style=flat-square)](https://crates.io/crates/gledopto) [![download](https://img.shields.io/crates/d/gledopto.svg?style=flat-square)](https://crates.io/crates/gledopto) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/gledopto)

## Examples

For all examples, see:

- [Desktop examples in `./blinksy-desktop/examples`](./blinksy-desktop/examples)
- [Embedded (with Gledopto) examples in `./esp/gledopto/examples`](./esp/gledopto/examples)

### Desktop Simulation: 2D Grid with Noise Pattern

https://github.com/user-attachments/assets/22f388d0-189e-44bd-acbf-186a142b956d

<details>
<summary>
    Click to see code
</summary>

```rust
use blinksy::{
    layout::{Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

fn main() {
    layout2d!(
        Layout,
        [Shape2d::Grid {
            start: Vec2::new(-1., -1.),
            row_end: Vec2::new(-1., 1.),
            col_end: Vec2::new(1., -1.),
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
        .with_driver(Desktop::new_2d::<Layout>())
        .build();

    loop {
        if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
            break;
        }

        sleep(Duration::from_millis(16));
    }
}
```

</details>

### Embedded Gledopto: 2D APA102 Grid with Noise Pattern

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

</details>

### Embedded Gledopto: 1D WS2812 Strip with Rainbow Pattern

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

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

If you want to help, the best thing to do is use Blinksy for your own LED project, and share about your adventures.

## License

Blinksy is licensed under the [**European Union Public License (EUPL)**](./LICENSE).

You are free to use, modify, and share Blinksy freely. Whether for personal projects, art installations, or commercial products.

Only once you start distributing something based on changes to Blinksy, you must share any improvements back with the community by releasing your source code.

Unlike more viral copyleft licenses, you will not be required to release the source code for your entire project, only changes to Blinksy.
