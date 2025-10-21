# WebTOTP Build Script (PowerShell)
# Author: steven
# Description: Build both frontend and backend

param(
    [string]$Target = "all",  # all, frontend, backend
    [switch]$Release = $false,
    [switch]$Clean = $false
)

# Colors
$GREEN = "Green"
$YELLOW = "Yellow"
$RED = "Red"
$CYAN = "Cyan"

function Write-Info {
    Write-Host "[INFO] $args" -ForegroundColor $CYAN
}

function Write-Success {
    Write-Host "[✓] $args" -ForegroundColor $GREEN
}

function Write-Warning {
    Write-Host "[!] $args" -ForegroundColor $YELLOW
}

function Write-Error {
    Write-Host "[✗] $args" -ForegroundColor $RED
}

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor $CYAN
Write-Host "║              WebTOTP Build Script                          ║" -ForegroundColor $CYAN
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor $CYAN
Write-Host ""

# Parse arguments
Write-Info "Build Target: $Target"
if ($Release) { Write-Info "Mode: Release" } else { Write-Info "Mode: Debug" }
if ($Clean) { Write-Info "Clean: Yes" } else { Write-Info "Clean: No" }
Write-Host ""

# Clean if requested
if ($Clean) {
    Write-Warning "Cleaning build artifacts..."
    
    if ($Target -in @("all", "frontend")) {
        Write-Info "Cleaning frontend..."
        if (Test-Path "frontend/dist") {
            Remove-Item -Path "frontend/dist" -Recurse -Force
            Write-Success "Frontend dist cleaned"
        }
        if (Test-Path "frontend/node_modules") {
            Write-Warning "Removing node_modules (this may take a while)..."
            Remove-Item -Path "frontend/node_modules" -Recurse -Force
            Write-Success "Frontend node_modules cleaned"
        }
    }
    
    if ($Target -in @("all", "backend")) {
        Write-Info "Cleaning backend..."
        if (Test-Path "target") {
            Remove-Item -Path "target" -Recurse -Force
            Write-Success "Backend target cleaned"
        }
    }
    
    Write-Host ""
}

# Build Frontend
if ($Target -in @("all", "frontend")) {
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
    Write-Host "Building Frontend..." -ForegroundColor $YELLOW
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
    Write-Host ""
    
    # Check if Node.js is installed
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        Write-Error "Node.js is not installed!"
        Write-Info "Please install Node.js from https://nodejs.org/"
        exit 1
    }
    
    Write-Info "Node.js version: $(node --version)"
    Write-Info "npm version: $(npm --version)"
    Write-Host ""
    
    # Install dependencies
    Write-Info "Installing dependencies..."
    Set-Location "frontend"
    
    if (-not (Test-Path "node_modules")) {
        npm install
        if ($LASTEXITCODE -ne 0) {
            Write-Error "npm install failed!"
            exit 1
        }
        Write-Success "Dependencies installed"
    } else {
        Write-Success "Dependencies already installed"
    }
    
    Write-Host ""
    
    # Build frontend
    Write-Info "Building frontend..."
    npm run build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Frontend build failed!"
        exit 1
    }
    
    Write-Success "Frontend build completed"
    Write-Host ""
    
    # Check dist directory
    if (Test-Path "dist") {
        $distSize = (Get-ChildItem -Path "dist" -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB
        Write-Success "Frontend dist created (Size: $([Math]::Round($distSize, 2)) MB)"
    }
    
    Set-Location ".."
    Write-Host ""
}

# Build Backend
if ($Target -in @("all", "backend")) {
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
    Write-Host "Building Backend..." -ForegroundColor $YELLOW
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
    Write-Host ""
    
    # Check if Rust is installed
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Rust/Cargo is not installed!"
        Write-Info "Please install Rust from https://rustup.rs/"
        exit 1
    }
    
    Write-Info "Rust version: $(rustc --version)"
    Write-Info "Cargo version: $(cargo --version)"
    Write-Host ""
    
    # Build backend
    Write-Info "Building backend..."
    if ($Release) {
        Write-Info "Building in release mode..."
        cargo build --release
    } else {
        Write-Info "Building in debug mode..."
        cargo build
    }
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Backend build failed!"
        exit 1
    }
    
    Write-Success "Backend build completed"
    Write-Host ""
    
    # Check binary
    if ($Release) {
        $binary = "target/release/webtotp.exe"
    } else {
        $binary = "target/debug/webtotp.exe"
    }
    
    if (Test-Path $binary) {
        $binarySize = (Get-Item $binary).Length / 1MB
        Write-Success "Binary created: $binary (Size: $([Math]::Round($binarySize, 2)) MB)"
    }
    
    Write-Host ""
}

# Summary
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host "Build Summary" -ForegroundColor $YELLOW
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

if ($Target -in @("all", "frontend")) {
    if (Test-Path "frontend/dist") {
        Write-Success "Frontend: ✓ Built successfully"
        Write-Info "Location: frontend/dist/"
    }
}

if ($Target -in @("all", "backend")) {
    if ($Release) {
        if (Test-Path "target/release/webtotp.exe") {
            Write-Success "Backend: ✓ Built successfully (Release)"
            Write-Info "Location: target/release/webtotp.exe"
        }
    } else {
        if (Test-Path "target/debug/webtotp.exe") {
            Write-Success "Backend: ✓ Built successfully (Debug)"
            Write-Info "Location: target/debug/webtotp.exe"
        }
    }
}

Write-Host ""
Write-Success "Build completed successfully!"
Write-Host ""

# Next steps
Write-Host "Next steps:" -ForegroundColor $CYAN
if ($Target -in @("all", "backend")) {
    Write-Host "  1. Set master key: `$env:WEBTOTP_MASTER_KEY='your-key'" -ForegroundColor White
    Write-Host "  2. Start server: .\run.ps1" -ForegroundColor White
    Write-Host "  3. Open browser: http://localhost:8080" -ForegroundColor White
}

Write-Host ""

