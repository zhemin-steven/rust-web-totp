#!/bin/bash

# WebTOTP Build Script (Bash/Shell)
# Author: steven
# Description: Build both frontend and backend

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
TARGET="all"
RELEASE=false
CLEAN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --release)
            RELEASE=true
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Helper functions
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

echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║              WebTOTP Build Script                          ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Print build info
print_info "Build Target: $TARGET"
if [ "$RELEASE" = true ]; then
    print_info "Mode: Release"
else
    print_info "Mode: Debug"
fi
if [ "$CLEAN" = true ]; then
    print_info "Clean: Yes"
else
    print_info "Clean: No"
fi
echo ""

# Clean if requested
if [ "$CLEAN" = true ]; then
    print_warning "Cleaning build artifacts..."
    
    if [[ "$TARGET" == "all" || "$TARGET" == "frontend" ]]; then
        print_info "Cleaning frontend..."
        rm -rf frontend/dist
        rm -rf frontend/node_modules
        print_success "Frontend cleaned"
    fi
    
    if [[ "$TARGET" == "all" || "$TARGET" == "backend" ]]; then
        print_info "Cleaning backend..."
        rm -rf target
        print_success "Backend cleaned"
    fi
    
    echo ""
fi

# Build Frontend
if [[ "$TARGET" == "all" || "$TARGET" == "frontend" ]]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${YELLOW}Building Frontend...${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Check if Node.js is installed
    if ! command -v node &> /dev/null; then
        print_error "Node.js is not installed!"
        print_info "Please install Node.js from https://nodejs.org/"
        exit 1
    fi
    
    print_info "Node.js version: $(node --version)"
    print_info "npm version: $(npm --version)"
    echo ""
    
    # Install dependencies
    print_info "Installing dependencies..."
    cd frontend
    
    if [ ! -d "node_modules" ]; then
        npm install
        print_success "Dependencies installed"
    else
        print_success "Dependencies already installed"
    fi
    
    echo ""
    
    # Build frontend
    print_info "Building frontend..."
    npm run build
    
    print_success "Frontend build completed"
    echo ""
    
    # Check dist directory
    if [ -d "dist" ]; then
        DIST_SIZE=$(du -sh dist | cut -f1)
        print_success "Frontend dist created (Size: $DIST_SIZE)"
    fi
    
    cd ..
    echo ""
fi

# Build Backend
if [[ "$TARGET" == "all" || "$TARGET" == "backend" ]]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${YELLOW}Building Backend...${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is not installed!"
        print_info "Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    print_info "Rust version: $(rustc --version)"
    print_info "Cargo version: $(cargo --version)"
    echo ""
    
    # Build backend
    print_info "Building backend..."
    if [ "$RELEASE" = true ]; then
        print_info "Building in release mode..."
        cargo build --release
        BINARY="target/release/webtotp"
    else
        print_info "Building in debug mode..."
        cargo build
        BINARY="target/debug/webtotp"
    fi
    
    print_success "Backend build completed"
    echo ""
    
    # Check binary
    if [ -f "$BINARY" ]; then
        BINARY_SIZE=$(du -h "$BINARY" | cut -f1)
        print_success "Binary created: $BINARY (Size: $BINARY_SIZE)"
    fi
    
    echo ""
fi

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${YELLOW}Build Summary${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [[ "$TARGET" == "all" || "$TARGET" == "frontend" ]]; then
    if [ -d "frontend/dist" ]; then
        print_success "Frontend: ✓ Built successfully"
        print_info "Location: frontend/dist/"
    fi
fi

if [[ "$TARGET" == "all" || "$TARGET" == "backend" ]]; then
    if [ "$RELEASE" = true ]; then
        if [ -f "target/release/webtotp" ]; then
            print_success "Backend: ✓ Built successfully (Release)"
            print_info "Location: target/release/webtotp"
        fi
    else
        if [ -f "target/debug/webtotp" ]; then
            print_success "Backend: ✓ Built successfully (Debug)"
            print_info "Location: target/debug/webtotp"
        fi
    fi
fi

echo ""
print_success "Build completed successfully!"
echo ""

# Next steps
echo -e "${CYAN}Next steps:${NC}"
if [[ "$TARGET" == "all" || "$TARGET" == "backend" ]]; then
    echo "  1. Set master key: export WEBTOTP_MASTER_KEY='your-key'"
    echo "  2. Start server: ./run.sh"
    echo "  3. Open browser: http://localhost:8080"
fi

echo ""

