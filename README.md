# blinksy

**_Work in Progress_**

Rust `no_std` 1D, 2D, or 3D audio-reactive LED control library.

## Supported chips

- [x] APA102
  - [x] SPI
  - [x] GPIO using delay ("Bit banging")
- [ ] WS2812B
  - [x] GPIO using delay (Not tested)
  - [ ] SPI
  - [x] RMT (Specific to ESP32)
- [ ] SK6812
- [ ] WS2811
- [ ] WS2813
- [ ] WS2814
- [ ] WS2815

## TODO

- Refactors
  - Refactor Clocked Driver so easy to construct for either SPI for GPIO
  - Refactor chipsets so each chip has multiple supported options, in a consistent way.
- Add layout traits
  - Layout1d
  - Layout2d
  - Layout3d
- Add pattern traits
  - Pattern1d
  - Pattern2d
  - Pattern3d
- Add layout and patterns simulator
- Add support for audio input
  - https://github.com/decaday/embedded-audio
  - https://github.com/rustaudio/cpal
- Add support for beat detection
  - https://github.com/phip1611/beat-detector
