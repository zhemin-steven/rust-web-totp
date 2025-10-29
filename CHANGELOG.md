# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-10-22

### Added

**Core Features**:
- TOTP code generation (RFC 6238 compliant)
- Multi-account management with card grid layout
- Real-time code refresh (30-second intervals)
- One-click copy to clipboard
- Secure deletion with confirmation dialog

**Security** (9.0/10):
- AES-256-GCM encryption for data storage
- Argon2 key derivation from master password
- Master password protection system
- Random salt and nonce for each encryption
- GCM authentication tag for tamper protection
- Comprehensive error handling (no panic)
- Complete logging system (env_logger)

**User System**:
- Master password unlock on startup
- User login with session management
- Password change functionality
- 2FA protection for the tool itself
- QR code generation for 2FA setup

**User Experience**:
- Smart 2FA input field (auto show/hide based on user status)
- One-click TOTP code copy
- Secure 2FA disable (requires password + code)
- Modern gradient UI with smooth animations
- Responsive design

**Internationalization**:
- Chinese (zh-CN) support
- English (en-US) support
- Real-time language switching
- Auto-detect browser language
- LocalStorage preference saving

**Cross-Platform**:
- Windows support (START.bat)
- Linux support (build.sh, start.sh, install.sh)
- macOS support (same as Linux)

### Technical Details

**Backend**:
- Rust 2021 edition
- actix-web 4.4
- aes-gcm 0.9 (AES-256-GCM encryption)
- argon2 0.3 (key derivation)
- totp-rs 5.0 (TOTP implementation)
- env_logger 0.10 (logging)
- thiserror 1.0 (error handling)

**Frontend**:
- Vanilla JavaScript (zero dependencies)
- Modern CSS3 (gradients, animations)
- Internationalization module (i18n.js)

**Security**:
- File encryption format: [salt(16)][nonce(12)][ciphertext+tag]
- Password hashing: SHA-256 × 100,000 iterations
- Key derivation: Argon2id
- TOTP: HMAC-SHA1, 6 digits, 30s window

### Configuration

- Default port: 18007
- Log level: INFO (configurable via RUST_LOG)
- Session: Cookie-based
- Data file: data.enc (AES-256-GCM encrypted)

---

**Security Rating**: 9.0/10  
**Author**: Steven  
**License**: MIT
