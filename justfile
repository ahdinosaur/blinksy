##
# Examples
##

desktop-1d-rainbow:
  cargo run --release --example 1d-rainbow

desktop-2d-rainbow:
  cargo run --release --example 2d-rainbow

desktop-2d-noise:
  cargo run --release --example 2d-noise

desktop-3d-cube-face-noise:
  cargo run --release --example 3d-cube-face-noise

desktop-3d-cube-volume-rainbow:
  cargo run --release --example 3d-cube-volume-rainbow

desktop-3d-cube-volume-noise:
  cargo run --release --example 3d-cube-volume-noise

gledopto-ws2812-strip:
  cd esp && cargo run --release -p gledopto --example ws2812-strip --features gl_c_016wl_d

gledopto-apa102-grid:
  cd esp && cargo run --release -p gledopto --example apa102-grid --features gl_c_016wl_d

gledopto-ws2812-face-cube:
  cd esp && cargo run --release -p gledopto --example ws2812-face-cube --features gl_c_016wl_d

gledopto-ws2812-volume-cube:
  cd esp && cargo run --release -p gledopto --example ws2812-volume-cube --features gl_c_016wl_d

##
# Testing
##

test-core:
  cargo test

check-esp:
  cd esp && cargo check -F esp32 -F gl_c_016wl_d

test-esp: check-esp
  cd esp && cargo test --doc -F esp32 -F gl_c_016wl_d

doc-esp:
  cd esp && cargo doc -F esp32 -F gl_c_016wl_d --open

##
# Releasing
##

# List all crates in the project
crates:
    @echo "Root workspace crates:"
    @find . -name "Cargo.toml" -not -path "./Cargo.toml" -not -path "./esp/*" -not -path "./target/*" | sort
    @echo "\nESP workspace crates:"
    @find ./esp -name "Cargo.toml" -not -path "./esp/Cargo.toml" -not -path "*/target/*" | sort

# Create a tag for a crate release
tag crate:
    #!/usr/bin/env bash
    set -euo pipefail

    CRATE_TOML=$(find . \
        -path "./{{crate}}/Cargo.toml" \
        -o -path "./esp/{{crate}}/Cargo.toml" \
        | head -n1)
    if [ -z "$CRATE_TOML" ]; then
        echo "Crate {{crate}} not found!" >&2
        exit 1
    fi

    # Extract current version
    VERSION=$(grep '^version = ' "$CRATE_TOML" | sed -E 's/version = "(.*)"/\1/')
    if [ -z "$VERSION" ]; then
        echo "Failed to extract current version from $CRATE_TOML" >&2
        exit 1
    fi

    echo "Creating tag {{crate}}/v$VERSION" >&2
    git tag {{crate}}/v$VERSION
