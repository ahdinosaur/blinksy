##
# Examples
##

desktop-1d-rainbow:
  cargo run --release --example 1d-rainbow

desktop-2d-rainbow:
  cargo run --release --example 2d-rainbow

desktop-2d-noise:
  cargo run --release --example 2d-noise

desktop-3d-cube-noise:
  cargo run --release --example 3d-cube-noise

gledopto-ws2812-strip:
  cd esp && cargo run --release -p gledopto --example ws2812-strip --features gl_c_016wl_d

gledopto-apa102-grid:
  cd esp && cargo run --release -p gledopto --example apa102-grid --features gl_c_016wl_d

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

# Bump version for a specific crate (supports semver keywords or explicit version)
version crate bump:
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
    CURRENT_VERSION=$(grep '^version = ' "$CRATE_TOML" | sed -E 's/version = "(.*)"/\1/')
    if [ -z "$CURRENT_VERSION" ]; then
        echo "Failed to extract current version from $CRATE_TOML" >&2
        exit 1
    fi

    # Parse current version
    MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
    MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
    PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)

    # Calculate new version based on bump type
    NEW_VERSION=""
    if [[ "{{bump}}" == "patch" ]]; then
        NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
    elif [[ "{{bump}}" == "minor" ]]; then
        NEW_VERSION="$MAJOR.$((MINOR + 1)).0"
    elif [[ "{{bump}}" == "major" ]]; then
        NEW_VERSION="$((MAJOR + 1)).0.0"
    else
        # Assume bump is a specific version
        NEW_VERSION="{{bump}}"
    fi

    echo "Updating $CRATE_TOML: $CURRENT_VERSION → $NEW_VERSION" >&2
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CRATE_TOML"
    echo $NEW_VERSION

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

# Complete release flow for a crate
release crate bump:
    #!/usr/bin/env bash
    set -euo pipefail

    NEW_VERSION=$(just version {{crate}} {{bump}})
    git add .
    git commit -m "Bump {{crate}} to version $NEW_VERSION"
    just tag {{crate}}
    echo "Now push with: git push && git push --tags"
