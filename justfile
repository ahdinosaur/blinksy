test-core:
  cargo test

build-esp:
  cd esp && cargo build

gledopto-apa102-grid:
  cd esp && cargo run -p gledopto --example apa102-grid

gledopto-ws2812-strip:
  cd esp && cargo run -p gledopto --example ws2812-strip
