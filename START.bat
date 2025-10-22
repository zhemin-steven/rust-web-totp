@echo off
echo ========================================
echo   Web TOTP v2.0 - 2FA Management Tool
echo   Security Level: 9.0/10
echo ========================================
echo.
echo Starting server...
echo Server will be available at: http://127.0.0.1:18007
echo.
echo Press Ctrl+C to stop the server
echo.

set RUST_LOG=info
target\release\web-totp.exe

pause

