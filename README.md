# Blinksy

Blinksy is a **Rust** **no-std**, **no-alloc** LED control library designed for 1D, 2D, and 3D (audio-reactive) LED setups, inspired by [FastLED](https://fastled.io/) and [WLED](https://kno.wled.ge/).

- Specify your LED layout in 1D, 2D, or 3D. Choose a visual pattern (effect). The pattern will compute a color for each LED given the position in 1D, 2D, or 3D space.

## Features

- **No-std, No-alloc:** Designed to run on embedded targets.
- **Layout Abstraction:** Define 1D, 2D, or 3D LED positions with shapes (grids, lines, arcs, points, etc).
- **Multi‑Chipset Support:**
  - **APA102**
  - **WS2812B**
  - [Make an issue](https://github.com/ahdinosaur/blinksy/issues) if you want help to support a new chipset!
- **Pattern (Effect) Library:**
  - **Rainbow**: Gradual, colorful gradient transition across your layout.
  - **Noise**: Dynamic noise‑based visuals using noise functions (Perlin, Simplex, OpenSimplex, etc).
  - [Make an issue](https://github.com/ahdinosaur/blinksy/issues) if you want help to port a pattern from FastLED / WLED to Rust!
- **Board Support Packages**:
  - **Gledopto**: A great LED controller available on AliExpress: [Gledopto GL-C-016WL-D](https://www.aliexpress.com/item/1005008707989546.html)
  - (TODO) [**QuinLED**](https://quinled.info/): The best DIY and pre-assembled LED controller boards
- (TODO) **Audio-Reactive**: Easily integrate audio reactivity into visual patterns.
- (TODO) **Desktop Simulation:** Run a simulation of a layout and pattern on your computer to experiment with ideas.

## Getting Started

Add Blinksy to your `Cargo.toml` (adjust the version accordingly):

```toml
[dependencies]
blinksy = "0.1"
```

## Usage Examples

### 2D APA102 Grid with Noise Pattern

https://github.com/user-attachments/assets/1c1cf3a2-f65c-4152-b444-29834ac749ee

```rust
```

### 1D WS2812 Strip with Rainbow Pattern

https://github.com/user-attachments/assets/703fe31d-e7ca-4e08-ae2b-7829c0d4d52e

```rust
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details and join the discussion on how to make Blinksy even better.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rimu by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
