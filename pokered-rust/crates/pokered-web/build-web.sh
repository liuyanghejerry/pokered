#!/bin/bash
# Build script for pokered-web WASM deployment
# Usage: ./build-web.sh [release|debug]

set -e

# Configuration
CRATE_NAME="pokered_web"
WEB_DIR="web"
OUTPUT_DIR="pkg"
BUILD_TYPE="${1:-release}"

echo "========================================="
echo "Pokémon Red Web Build Script"
echo "========================================="
echo ""

# Check prerequisites
echo "Checking prerequisites..."

# Check wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo ""
    echo "Install wasm-pack:"
    echo "  cargo install wasm-pack"
    exit 1
fi

# Check wasm32 target
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Error: wasm32-unknown-unknown target is not installed."
    echo ""
    echo "Install the target:"
    echo "  rustup target add wasm32-unknown-unknown"
    exit 1
fi

echo "✓ All prerequisites satisfied"
echo ""

# Clean previous build
echo "Cleaning previous build..."
rm -rf "$OUTPUT_DIR"
echo "✓ Cleaned"
echo ""

# Build WASM package
echo "Building WASM package..."
echo "  Build type: $BUILD_TYPE"
echo "  Target: wasm32-unknown-unknown"
echo ""

if [ "$BUILD_TYPE" == "release" ]; then
    wasm-pack build --release --target web --out-dir "$OUTPUT_DIR" --no-typescript
else
    wasm-pack build --dev --target web --out-dir "$OUTPUT_DIR" --no-typescript
fi

echo "✓ WASM package built"
echo ""

# Copy web assets
echo "Copying web assets..."
if [ -d "$WEB_DIR" ]; then
    cp -r "$WEB_DIR"/* "$OUTPUT_DIR"/
    echo "✓ Web assets copied"
else
    echo "⚠ No web directory found, skipping"
fi
echo ""

# Print build info
echo "========================================="
echo "Build Complete!"
echo "========================================="
echo ""
echo "Output directory: $OUTPUT_DIR"
echo "Files generated:"
ls -lh "$OUTPUT_DIR" | tail -n +2
echo ""
echo "To deploy:"
echo "  1. Copy the '$OUTPUT_DIR' directory to your web server"
echo "  2. Serve it as static files"
echo "  3. Access index.html in a browser"
echo ""
echo "For local testing:"
echo "  cd $OUTPUT_DIR"
echo "  python3 -m http.server 8080"
echo "  # Then open http://localhost:8080 in your browser"
echo ""
echo "For GitHub Pages deployment:"
echo "  Copy '$OUTPUT_DIR' contents to your gh-pages branch"
echo ""