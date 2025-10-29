#!/bin/bash

# Web TOTP Installation Script for Linux/macOS
# Author: Steven

set -e

echo "========================================"
echo "  Web TOTP v2.0 Installation"
echo "  Author: Steven"
echo "========================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo -e "${GREEN}✓${NC} Rust installed"
else
    echo -e "${GREEN}✓${NC} Rust already installed"
fi

echo ""

# Build project
echo "Building Web TOTP..."
./build.sh

echo ""

# Make scripts executable
chmod +x build.sh
chmod +x start.sh
chmod +x target/release/web-totp

echo -e "${GREEN}✓${NC} Installation complete!"
echo ""

# Create data directory
mkdir -p backups

echo "========================================"
echo "  Installation Complete!"
echo "========================================"
echo ""
echo "To start the server:"
echo "  ./start.sh"
echo ""
echo "Or run directly:"
echo "  ./target/release/web-totp"
echo ""
echo "Then visit: http://127.0.0.1:18007"
echo ""
echo "First time use:"
echo "  1. Set master password (unlock page)"
echo "  2. Login with admin/admin"
echo "  3. Change password immediately"
echo ""
echo "⚠️  IMPORTANT: Backup your master password!"
echo ""

