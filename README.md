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
  For <strong>1D</strong>, <strong>2D</strong>, and <strong>3D</strong> layouts.
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

[layout]: https://docs.rs/blinksy/0.10/blinksy/layout/index.html
[pattern]: https://docs.rs/blinksy/0.10/blinksy/pattern/index.html
[patterns]: https://docs.rs/blinksy/0.10/blinksy/patterns/index.html
[driver]: https://docs.rs/blinksy/0.10/blinksy/driver/index.html
[drivers]: https://docs.rs/blinksy/0.10/blinksy/drivers/index.html

## Features

- **No-std, no-alloc**: Designed for embedded targets.
- **Spatial in 1D, 2D, or 3D**: Map out the shape of your LEDs in space.
- **Async support**: Supports blocking or asynchronous execution.
- **Full color support**: Supports modern and classic color spaces.
- **Global settings**: Control overall brightness and color correction.
- **Desktop simulation**: Simulate your LEDs on your desktop to play with ideas.
- **RGB+W support**: Supports RGB + White color channels

### LED Support

#### [Clockless][clockless]: One-wire (only data, no clock)

- **[WS2812B][ws2812]**: Affordable RGB LED, aka NeoPixel
- **[SK6812][sk6812]**: RGBW LED

[clockless]: https://docs.rs/blinksy/0.10/blinksy/driver/clockless/index.html
[ws2812]: https://docs.rs/blinksy/0.10/blinksy/drivers/ws2812/index.html
[sk6812]: https://docs.rs/blinksy/latest/blinksy/drivers/sk6812/index.html

#### [Clocked][clocked]: Two-wire (data and clock)

- **[APA102][apa102]**: High-FPS RGB LED, aka DotStar

If you want help to support a new LED chipset, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[clocked]: https://docs.rs/blinksy/0.10/blinksy/driver/clocked/index.html
[apa102]: https://docs.rs/blinksy/0.10/blinksy/drivers/apa102/index.html

### Pattern (Effect) Library:

- **[Rainbow][rainbow]**: A basic scrolling rainbow
- **[Noise][noise]**: A flow through random noise functions

If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[rainbow]: https://docs.rs/blinksy/0.10/blinksy/patterns/rainbow/index.html
[noise]: https://docs.rs/blinksy/0.10/blinksy/patterns/noise/index.html

### Microcontroller Family Support

Clocked LED support (e.g. APA102):

|Micro|HAL|Blinksy|Recommended Driver|Backup Driver|
|---|---|---|---|---|
|ALL|[embedded-hal]|[blinksy]|[Spi][clocked-spi]|[Delay][clocked-delay]|

[clocked-spi]: https://docs.rs/blinksy/0.10.0/blinksy/driver/clocked/struct.ClockedSpiDriver.html
[clocked-delay]:https://docs.rs/blinksy/0.10.0/blinksy/driver/clocked/struct.ClockedDelayDriver.html

Clockless LED support (e.g. WS2812)

|Micro|HAL|Blinksy|Recommended Driver|Backup Driver|
|---|---|---|---|---|
|ALL|[embedded-hal]|[blinksy]|-|TODO [Spi #12][clockless-spi]|
|ESP32|[esp-hal]|[blinksy-esp]|[Rmt][rmt]|-|
|RP (2040 or 2350)|[rp-hal]|TODO|TODO [#36][rp-issue]|-|
|STM32|[stm32-hal]|TODO|TODO [#78][stm32-issue]|-|
|nRF|[nrf-hal]|TODO|TODO [#77][nrf-issue]|-|
|atsamd|[atsamd]|TODO|TODO [#67][atsamd-issue]|-|
|AVR (Arduino)|[avr-hal]|TODO|TODO [#79][avr-issue]|-|
|CH32|[ch32-hal]|TODO|TODO [#80][ch32-issue]|-|
|???|-|-|-|-|

If you want help to support a new microcontroller family, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[embedded-hal]: https://docs.rs/embedded-hal/latest/embedded_hal/
[blinksy]: https://docs.rs/blinksy/0.10/blinksy/
[clockless-spi]: https://github.com/ahdinosaur/blinksy/issues/12
[esp-hal]: https://docs.espressif.com/projects/rust/esp-hal/latest/
[blinksy-esp]: https://docs.rs/blinksy-esp/0.10/blinksy-esp/
[rmt]: https://docs.espressif.com/projects/rust/esp-hal/latest/
[rp-hal]: https://github.com/rp-rs/rp-hal/
[rp-issue]: https://github.com/ahdinosaur/blinksy/issues/36
[stm32-hal]: https://github.com/David-OConnor/stm32-hal
[stm32-issue]: https://github.com/ahdinosaur/blinksy/issues/78
[nrf-hal]: https://github.com/nrf-rs/nrf-hal
[nrf-issue]: https://github.com/ahdinosaur/blinksy/issues/77
[atsamd]: https://github.com/atsamd-rs/atsamd
[atsamd-issue]: https://github.com/ahdinosaur/blinksy/issues/67
[avr-hal]: https://github.com/Rahix/avr-hal
[avr-issue]: https://github.com/ahdinosaur/blinksy/issues/79
[ch32-hal]: https://github.com/ch32-rs/ch32-hal
[ch32-issue]: https://github.com/ahdinosaur/blinksy/issues/80

### Board Support

These are ready-to-go LED controllers with board support crates to make things even easier:

- **[Gledopto][gledopto]**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
- (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards

If you want help to support a new target, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

[gledopto]: https://docs.rs/gledopto/0.10/gledopto

## Quick Start

To quickstart a project, see:

- [`blinksy-quickstart-1d-rope`][blinksy-quickstart-1d-rope]
- [`blinksy-quickstart-3d-grid`][blinksy-quickstart-3d-grid]

To start using the library, see [control][control].

[blinksy-quickstart-1d-rope]: https://github.com/ahdinosaur/blinksy-quickstart-1d-rope
[blinksy-quickstart-3d-grid]: https://github.com/ahdinosaur/blinksy-quickstart-3d-grid
[control]: https://docs.rs/blinksy/0.10/blinksy/control/index.html

## Modules

[![CI status](https://img.shields.io/github/actions/workflow/status/ahdinosaur/blinksy/ci.yml?branch=main&style=flat-square)](https://github.com/ahdinosaur/blinksy/actions/workflows/ci.yml?query=branch%3Amain)

- [`blinksy`](./blinksy) : [![Crates.io version](https://img.shields.io/crates/v/blinksy.svg?style=flat-square)](https://crates.io/crates/blinksy) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/blinksy?style=flat-square&label=total%20downloads)](https://crates.io/crates/blinksy) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/blinksy?style=flat-square)](https://crates.io/crates/blinksy)
- [`blinksy-desktop`](./blinksy-desktop) : [![Crates.io version](https://img.shields.io/crates/v/blinksy-desktop.svg?style=flat-square)](https://crates.io/crates/blinksy-desktop) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-desktop) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/blinksy-desktop?style=flat-square&label=total%20downloads)](https://crates.io/crates/blinksy-desktop) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/blinksy-desktop?style=flat-square)](https://crates.io/crates/blinksy-desktop)
- [`blinksy-esp`](./esp/blinksy-esp) : [![Crates.io version](https://img.shields.io/crates/v/blinksy-esp.svg?style=flat-square)](https://crates.io/crates/blinksy-esp) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/blinksy-esp) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/blinksy-esp?style=flat-square&label=total%20downloads)](https://crates.io/crates/blinksy-esp) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/blinksy-esp?style=flat-square)](https://crates.io/crates/blinksy-esp)
- [`gledopto`](./esp/gledopto) : [![Crates.io version](https://img.shields.io/crates/v/gledopto.svg?style=flat-square)](https://crates.io/crates/gledopto) [![Doc.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/gledopto) [![Crates.io Downloads (total)](https://img.shields.io/crates/d/gledopto?style=flat-square&label=total%20downloads)](https://crates.io/crates/gledopto) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/gledopto?style=flat-square)](https://crates.io/crates/gledopto)

## Examples

For all examples, see:

- [Desktop examples in `./blinksy-desktop/examples`](./blinksy-desktop/examples)
- [Embedded (with Gledopto) examples in `./esp/gledopto/examples`](./esp/gledopto/examples)

### Embedded Gledopto: 3D Cube with Noise Pattern

https://github.com/user-attachments/assets/36a2c6ad-7ae6-4498-85b3-ed76d0b62264

<details>
<summary>
    Click to see code
</summary>

```rust
#![no_std]
#![no_main]

use blinksy::{
    layout::{Layout3d, Shape3d, Vec3},
    layout3d,
    leds::Ws2812,
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    ControlBuilder,
};
use gledopto::{board, bootloader, elapsed, main, ws2812};

bootloader!();

#[main]
fn main() -> ! {
    let p = board!();

    layout3d!(
        Layout,
        [
            // bottom face
            Shape3d::Grid {
                start: Vec3::new(1., -1., 1.),           // right bottom front
                horizontal_end: Vec3::new(-1., -1., 1.), // left bottom front
                vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // back face
            Shape3d::Grid {
                start: Vec3::new(-1., -1., -1.),         // left bottom back
                horizontal_end: Vec3::new(-1., 1., -1.), // left top back
                vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // right face
            Shape3d::Grid {
                start: Vec3::new(1., 1., -1.),         // right top back
                horizontal_end: Vec3::new(1., 1., 1.), // right top front
                vertical_end: Vec3::new(1., -1., -1.), // right bottom back
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // front face
            Shape3d::Grid {
                start: Vec3::new(-1., -1., 1.),         // left bottom front
                horizontal_end: Vec3::new(1., -1., 1.), // right bottom front
                vertical_end: Vec3::new(-1., 1., 1.),   // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // left face
            Shape3d::Grid {
                start: Vec3::new(-1., 1., -1.),           // left top back
                horizontal_end: Vec3::new(-1., -1., -1.), // left bottom back
                vertical_end: Vec3::new(-1., 1., 1.),     // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            },
            // top face
            Shape3d::Grid {
                start: Vec3::new(1., 1., 1.),           // right top front
                horizontal_end: Vec3::new(1., 1., -1.), // right top back
                vertical_end: Vec3::new(-1., 1., 1.),   // left top front
                horizontal_pixel_count: 16,
                vertical_pixel_count: 16,
                serpentine: true,
            }
        ]
    );

    let mut control = ControlBuilder::new_3d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Noise3d<noise_fns::Perlin>>(NoiseParams {
            ..Default::default()
        })
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}
```

</details>

### Desktop Simulation: 2D Grid with Noise Pattern

https://github.com/user-attachments/assets/22f388d0-189e-44bd-acbf-186a142b956d

<details>
<summary>
    Click to see code
</summary>

```rust
use blinksy::{
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

layout2d!(
    PanelLayout,
    [Shape2d::Grid {
        start: Vec2::new(-1., -1.),
        horizontal_end: Vec2::new(1., -1.),
        vertical_end: Vec2::new(-1., 1.),
        horizontal_pixel_count: 16,
        vertical_pixel_count: 16,
        serpentine: true,
    }]
);

fn main() {
    Desktop::new_2d::<PanelLayout>().start(|driver| {
        let mut control = ControlBuilder::new_2d()
            .with_layout::<PanelLayout, { PanelLayout::PIXEL_COUNT }>()
            .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams {
                ..Default::default()
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ PanelLayout::PIXEL_COUNT }>()
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
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
    layout::Layout1d,
    layout1d,
    leds::Ws2812,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use gledopto::{board, bootloader, elapsed, main, ws2812};

bootloader!();

#[main]
fn main() -> ! {
    let p = board!();

    layout1d!(Layout, 50);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Rainbow>(RainbowParams::default())
        .with_driver(ws2812!(p, Layout::PIXEL_COUNT, buffered))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
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
    layout::Layout1d,
    layout1d,
    leds::Ws2812,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{board, bootloader, elapsed, init_embassy, main_embassy, ws2812_async};

bootloader!();

#[main_embassy]
async fn main(_spawner: Spawner) {
    let p = board!();

    init_embassy!(p);

    layout1d!(Layout, 50);

    let mut control = ControlBuilder::new_1d_async()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Rainbow>(RainbowParams::default())
        .with_driver(ws2812_async!(p, Layout::PIXEL_COUNT, buffered))
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
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
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    leds::Apa102,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use gledopto::{apa102, board, bootloader, elapsed, main};

bootloader!();

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
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
        .with_driver(apa102!(p))
        .with_frame_buffer_size::<{ Apa102::frame_buffer_size(Layout::PIXEL_COUNT) }>()
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
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    patterns::noise::{noise_fns, Noise2d, NoiseParams},
    ControlBuilder,
};
use embassy_executor::Spawner;
use gledopto::{apa102_async, board, bootloader, elapsed, main_embassy};

bootloader!();

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
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Noise2d<noise_fns::Perlin>>(NoiseParams::default())
        .with_driver(apa102_async!(p))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
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
