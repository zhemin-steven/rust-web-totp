#!/bin/bash

# WebTOTP Startup Script
# Author: steven
# Description: Start WebTOTP server with optional master key

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

print_info "WebTOTP Server Startup"
echo ""

# Check if master key is provided as argument
if [ $# -gt 0 ]; then
    MASTER_KEY="$1"
    print_info "Master key provided via argument"
else
    # Check if WEBTOTP_MASTER_KEY environment variable is set
    if [ -z "$WEBTOTP_MASTER_KEY" ]; then
        print_warning "No master key provided"
        print_info "Please set WEBTOTP_MASTER_KEY environment variable or provide it as argument:"
        echo "  Usage: ./run.sh 'your-master-key'"
        echo "  Or: export WEBTOTP_MASTER_KEY='your-master-key' && ./run.sh"
        echo ""
        read -p "Enter master key (or press Enter for interactive mode): " MASTER_KEY
        if [ -n "$MASTER_KEY" ]; then
            export WEBTOTP_MASTER_KEY="$MASTER_KEY"
        fi
    else
        print_success "Master key loaded from environment variable"
    fi
fi

# Set master key if provided
if [ -n "$MASTER_KEY" ]; then
    export WEBTOTP_MASTER_KEY="$MASTER_KEY"
    print_success "Master key set"
fi

echo ""

# Check if binary exists
if [ ! -f "./target/release/webtotp" ]; then
    print_warning "Binary not found, building..."
    print_info "Running: cargo build --release"
    cargo build --release
    if [ $? -ne 0 ]; then
        print_error "Build failed!"
        exit 1
    fi
    print_success "Build completed"
    echo ""
fi

# Start the server
print_info "Starting WebTOTP server..."
print_info "Access the application at: http://localhost:18007"
print_info "Press Ctrl+C to stop"
echo ""

# Run the server
./target/release/webtotp

