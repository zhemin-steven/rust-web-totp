#!/bin/bash

# Web TOTP Build Script for Linux/macOS
# Author: Steven
# Version: 1.0.1

set -e

echo "========================================"
echo "  Web TOTP v2.0 Build Script"
echo "  Author: Steven"
echo "========================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust installation found"
echo ""

# Clean previous build
echo "Cleaning previous build..."
cargo clean
echo -e "${GREEN}✓${NC} Clean complete"
echo ""

# Build release version
echo "Building release version..."
echo "This may take a few minutes..."
echo ""

if cargo build --release; then
    echo ""
    echo -e "${GREEN}✓${NC} Build successful!"
    echo ""
    echo "========================================"
    echo "  Build Complete!"
    echo "========================================"
    echo ""
    echo "Executable location:"
    echo "  ./target/release/web-totp"
    echo ""
    echo "File size:"
    ls -lh ./target/release/web-totp | awk '{print "  " $5}'
    echo ""
    echo "To run:"
    echo "  ./target/release/web-totp"
    echo ""
    echo "Or use the start script:"
    echo "  ./start.sh"
    echo ""
else
    echo ""
    echo -e "${RED}✗${NC} Build failed!"
    echo "Please check the error messages above"
    exit 1
fi

