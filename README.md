# blinksy

**_Work in Progress_**

Rust `no_std` 1D, 2D, or 3D audio-reactive LED control library.

## Supported chips

- [x] Apa102
  - SPI
  - [x] Bit banging
- [ ] WS2812
  - [ ] Delay
  - [ ] SPI
  - [ ] RMT (specific to ESP32)

## TODO

- Add layout traits
  - Layout1d
  - Layout2d
  - Layout3d
- Add pattern traits
  - Pattern1d
  - Pattern2d
  - Pattern3d
- Add driver trait
  - `smart-leds-trait` is good, but not sufficient.
  - Use inspiration from https://github.com/hannobraun/stepper
  - Should be independent of LED type AND method
  - Should have two traits:
    - Led
    - Driver
  - So e.g. ESP RMT + APA102
    - or generic embedded-hal delay + WS2811
    - etc
- Add layout and patterns simulator
- Add support for built-in i2s microphone
  - https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32/esp_hal/i2s/master/index.html
  - https://github.com/decaday/embedded-audio/blob/master/embedded-audio/src/stream/cpal_stream.rs
- Add support for beat detection
  - https://github.com/phip1611/beat-detector
