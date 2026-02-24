#!/bin/bash
# Build script for PULS-G3
# Produces a release binary compatible with Debian 10+, Ubuntu 20+, and up to bleeding edge
#
# Requirements:
#   - Rust toolchain (rustup, cargo)
#   - GTK3 development libraries: sudo apt install libgtk-3-dev
#   - pkg-config: sudo apt install pkg-config
#
# The binary dynamically links to system GTK3/GLib
# but everything else is statically linked via LTO
# This means the binary works on any Linux distro that has GTK3 >= 3.22 installed

set -euo pipefail

TARGET="x86_64-unknown-linux-gnu"
BINARY_NAME="puls-g3"

echo "=== PULS-G3 Release Build ==="
echo "Target: $TARGET"
echo ""

rustup target add "$TARGET" 2>/dev/null || true

if ! pkg-config --exists gtk+-3.0 2>/dev/null; then
    echo "ERROR: GTK3 development libraries not found."
    echo "Install with: sudo apt install libgtk-3-dev pkg-config"
    exit 1
fi

GTK_VERSION=$(pkg-config --modversion gtk+-3.0)
echo "GTK3 version: $GTK_VERSION"

echo ""
echo "Building release binary..."
RUSTFLAGS="-C target-cpu=x86-64" cargo build --release --target "$TARGET"

BINARY="target/$TARGET/release/$BINARY_NAME"

if [ ! -f "$BINARY" ]; then
    echo "ERROR: Binary not found at $BINARY"
    exit 1
fi

strip "$BINARY" 2>/dev/null || true

SIZE=$(du -h "$BINARY" | cut -f1)
echo ""
echo "=== Build Complete ==="
echo "Binary: $BINARY"
echo "Size: $SIZE"
echo ""

echo "Dynamic dependencies:"
ldd "$BINARY" 2>/dev/null | grep -E "(gtk|glib|gdk|cairo|pango|gio)" | sed 's/^/  /'
echo ""

GLIBC_VERSIONS=$(objdump -T "$BINARY" 2>/dev/null | grep GLIBC_ | sed 's/.*GLIBC_//' | sort -V | tail -1)
echo "Max GLIBC requirement: $GLIBC_VERSIONS"
echo ""
echo "This binary should work on any Linux system with:"
echo "  - GTK3 >= 3.22 (Debian 10+, Ubuntu 18.04+)"
echo "  - glibc >= $GLIBC_VERSIONS"
echo ""
echo "To install on target system:"
echo "  sudo apt install libgtk-3-0  # Ubuntu/Debian"
echo "  sudo cp $BINARY /usr/local/bin/$BINARY_NAME"
