#!/bin/bash

# Web TOTP Start Script for Linux/macOS
# Author: Steven

echo "========================================"
echo "  Web TOTP v2.0 - 2FA Management Tool"
echo "  Security Level: 9.0/10"
echo "  Author: Steven"
echo "========================================"
echo ""

# Check if binary exists
if [ ! -f "./target/release/web-totp" ]; then
    echo "Error: Binary not found!"
    echo "Please run ./build.sh first"
    exit 1
fi

echo "Starting server..."
echo "Server will be available at: http://127.0.0.1:18007"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Set log level
export RUST_LOG=info

# Run the server
./target/release/web-totp

