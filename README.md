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

[![GitHub Repo stars](https://img.shields.io/github/stars/ahdinosaur/blinksy?style=flat-square)](https://github.com/ahdinosaur/blinksy)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/ahdinosaur?style=flat-square)](https://github.com/sponsors/ahdinosaur)
[![Chat](https://img.shields.io/matrix/blinksy:matrix.org?style=flat-square&label=chat)](https://matrix.to/#/#blinksy:matrix.org)
[![License](https://img.shields.io/github/license/ahdinosaur/blinksy?style=flat-square)](#license)

</div>

### How Blinksy works

- Define your LED [`layout`][layout] in 1D, 2D, or 3D space
- Create your visual [`pattern`][pattern] (effect), or choose from our built-in [`patterns`][patterns] library
  - The pattern will compute colors for each LED based on its position
- Setup a [`driver`][driver] to send each frame of colors to your LEDs, using our built-in [`drivers`][drivers] library.

[layout]: https://docs.rs/blinksy/0.4/blinksy/layout/index.html
[pattern]: https://docs.rs/blinksy/0.4/blinksy/pattern/index.html
[patterns]: https://docs.rs/blinksy/0.4/blinksy/patterns/index.html
[driver]: https://docs.rs/blinksy/0.4/blinksy/driver/index.html
[drivers]: https://docs.rs/blinksy/0.4/blinksy/drivers/index.html

## Features

- **No-std, no-alloc**: Designed for embedded targets.
- **Spatial in 1D, 2D, or 3D**: Map out the shape of your LEDs in space.
- **Async support**: Supports blocking or asynchronous execution.
- **Full color support**: Supports modern and classic color spaces.
- **Global settings**: Control overall brightness and color correction.
- **Desktop simulation**: Simulate your LEDs on your desktop to play with ideas.
- **RGB+W support**: Supports RGB + White color channels

### Multi‑Chipset Support

- [clockless][clockless]: One-wire (only data, no clock)
  - **[WS2812B][ws2812]**: Affordable RGB LED, aka NeoPixel
- [clocked][clocked]: Two-wire (data and clock)
  - **[APA102][apa102]**: High-FPS RGB LED, aka DotStar

If you want help to support a new chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[clockless]: https://docs.rs/blinksy/0.4/blinksy/driver/clockless/index.html
[ws2812]: https://docs.rs/blinksy/0.4/blinksy/drivers/ws2812/index.html
[clocked]: https://docs.rs/blinksy/0.4/blinksy/driver/clocked/index.html
[apa102]: https://docs.rs/blinksy/0.4/blinksy/drivers/apa102/index.html

### Pattern (Effect) Library:

- **[Rainbow][rainbow]**: A basic scrolling rainbow
- **[Noise][noise]**: A flow through random noise functions.

If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[rainbow]: https://docs.rs/blinksy/0.4/blinksy/patterns/rainbow/index.html
[noise]: https://docs.rs/blinksy/0.4/blinksy/patterns/noise/index.html

### Board Support Packages

- **[Gledopto][gledopto]**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
- (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards

If you want help to support a new target, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[gledopto]: https://docs.rs/gledopto/0.4/gledopto

## Modules

[![CI status](https://img.shields.io/github/actions/workflow/status/ahdinosaur/blinksy/ci.yml?branch=main&style=flat-square)](https://github.com/ahdinosaur/blinksy/actions/workflows/ci.yml?query=branch%3Amain)

- [`blinksy`](./blinksy) : [![Crates.io version](https://img.shields.io/crates/v/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/blinksy?style=flat-square&label=total%20downloads)](https://crates.io/crates/blinksy) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/blinksy?style=flat-square)](https://crates.io/crates/blinksy)
- [`blinksy-desktop`](./blinksy-desktop) : [![Crates.io version](https://img.shields.io/crates/v/blinksy-desktop.svg?style=flat-square)](https://crates.io/crates/blinksy-desktop) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-desktop) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/blinksy-desktop?style=flat-square&label=total%20downloads)](https://crates.io/crates/blinksy-desktop) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/blinksy-desktop?style=flat-square)](https://crates.io/crates/blinksy-desktop)
- [`blinksy-esp`](./esp/blinksy-esp) : [![Crates.io version](https://img.shields.io/crates/v/blinksy-esp.svg?style=flat-square)](https://crates.io/crates/blinksy-esp) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-esp) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/blinksy-esp?style=flat-square&label=total%20downloads)](https://crates.io/crates/blinksy-esp) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/blinksy-esp?style=flat-square)](https://crates.io/crates/blinksy-esp)
- [`gledopto`](./esp/gledopto) : [![Crates.io version](https://img.shields.io/crates/v/gledopto.svg?style=flat-square)](https://crates.io/crates/gledopto) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/gledopto) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/gledopto?style=flat-square&label=total%20downloads)](https://crates.io/crates/gledopto) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/gledopto?style=flat-square)](https://crates.io/crates/gledopto)

## Quick Start

To quickstart a project, see [`blinksy-quickstart-gledopto`][blinksy-quickstart-gledopto]

To start using the library, see [control][control].

[blinksy-quickstart-gledopto]: https://github.com/ahdinosaur/blinksy-quickstart-gledopto
[control]: https://docs.rs/blinksy/0.4/blinksy/control/index.html

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

### Embedded Gledopto: 1D WS2812 Strip with Rainbow Pattern

https://github.com/user-attachments/assets/703fe31d-e7ca-4e08-ae2b-7829c0d4d52e

<details>
<summary>
    Click to see code (Blocking)
</summary>

```rust
#![no_std]
#![no_main]

use blinksy::{
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
        .with_pattern::<Rainbow>(RainbowParams::default())
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

<details>
<summary>
    Click to see code (Async)
</summary>

```rust
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{board, elapsed, init_embassy, main_embassy, ws2812_async};

#[main_embassy]
async fn main(_spawner: Spawner) {
    let p = board!();

    init_embassy!(p);

    layout1d!(Layout, 60 * 5);

    let mut control = ControlBuilder::new_1d_async()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams::default())
        .with_driver(ws2812_async!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
```

</details>

### Embedded Gledopto: 2D APA102 Grid with Noise Pattern

https://github.com/user-attachments/assets/1c1cf3a2-f65c-4152-b444-29834ac749ee

<details>
<summary>
    Click to see code (Blocking)
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
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
        .with_driver(apa102!(p))
        .build();

    control.set_brightness(0.02);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
```

</details>

<details>
<summary>
    Click to see code (Async)
</summary>

```rust
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use blinksy::{
    layout::{Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{apa102_async, board, elapsed, main_embassy};

#[main_embassy]
async fn main(_spawner: Spawner) {
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
    let mut control = ControlBuilder::new_2d_async()
        .with_layout::<Layout>()
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
        .with_driver(apa102_async!(p))
        .build();

    control.set_brightness(0.02);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

If you want to help, the best thing to do is use Blinksy for your own LED project, and share about your adventures.

## License

Blinksy is licensed under the [**European Union Public License (EUPL)**](./LICENSE).

You are free to use, modify, and share Blinksy freely. Whether for personal projects, art installations, or commercial products.

Only once you start distributing something based on changes to Blinksy, you must share any improvements back with the community by releasing your source code.

Unlike more viral copyleft licenses, you will not be required to release the source code for your entire project, only changes to Blinksy.
