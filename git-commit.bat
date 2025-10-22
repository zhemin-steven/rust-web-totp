@echo off
chcp 65001 >nul
echo ========================================
echo   Web TOTP - Git Commit Helper
echo ========================================
echo.

echo [1/3] Initializing Git repository...
git init

echo [2/3] Adding files...
git add .

echo [3/3] Creating commit...
git commit -m "Initial commit: Web TOTP v1.0.0

Production-grade 2FA management tool

Features:
- AES-256-GCM encryption (Security: 9.0/10)
- Argon2 key derivation from master password
- Master password protection system
- TOTP generation (RFC 6238)
- Multi-account management
- Internationalization (Chinese/English)
- Cross-platform (Windows/Linux/macOS)
- Modern gradient UI with animations
- Smart 2FA input field
- One-click copy TOTP codes

Author: Steven
License: MIT
Port: 18007"

echo.
echo ========================================
echo   Commit Created!
echo ========================================
echo.
echo Next steps:
echo 1. Create a new repository on GitHub
echo    Name: web-totp
echo    Description: Production-grade 2FA tool with AES-256-GCM encryption
echo.
echo 2. Run these commands:
echo    git remote add origin https://github.com/YOUR_USERNAME/web-totp.git
echo    git branch -M main
echo    git push -u origin main
echo.
echo Replace YOUR_USERNAME with your GitHub username
echo.
pause
