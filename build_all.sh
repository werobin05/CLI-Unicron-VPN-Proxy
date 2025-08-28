#!/bin/bash
set -e

PROJECT_NAME="cli_unicron" 
DIST_DIR="dist"
mkdir -p "$DIST_DIR"

echo "=== Building macOS ARM (M1/M2) ==="
cargo build --release
cp "target/release/$PROJECT_NAME" "$DIST_DIR/$PROJECT_NAME-macos-arm"
echo "Done: $DIST_DIR/$PROJECT_NAME-macos-arm"

echo "=== Building macOS Intel (x86_64) ==="
cargo build --release --target x86_64-apple-darwin
cp "target/x86_64-apple-darwin/release/$PROJECT_NAME" "$DIST_DIR/$PROJECT_NAME-macos-intel"
echo "Done: $DIST_DIR/$PROJECT_NAME-macos-intel"

if command -v cross >/dev/null 2>&1; then
    echo "=== Building Linux x86_64 ==="
    cross build --release --target x86_64-unknown-linux-gnu
    cp "target/x86_64-unknown-linux-gnu/release/$PROJECT_NAME" "$DIST_DIR/$PROJECT_NAME-linux"
    echo "Done: $DIST_DIR/$PROJECT_NAME-linux"

    echo "=== Building Windows x86_64 ==="
    cross build --release --target x86_64-pc-windows-gnu
    cp "target/x86_64-pc-windows-gnu/release/$PROJECT_NAME.exe" "$DIST_DIR/$PROJECT_NAME-windows.exe"
    echo "Done: $DIST_DIR/$PROJECT_NAME-windows.exe"
else
    echo "cross not found. Skipping Linux and Windows builds."
fi

echo "=== All builds completed. Binaries in $DIST_DIR ==="
