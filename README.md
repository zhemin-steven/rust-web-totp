# WebTOTP - Secure TOTP Manager

A web-based Time-based One-Time Password (TOTP) manager with enterprise-grade security features.

**Author:** steven

## Features

### 🔐 Security
- **Dynamic Master Key Management** - Master key required at startup, never stored in plaintext
- **AES-256-GCM Encryption** - All TOTP secrets encrypted with AES-256-GCM
- **bcrypt Password Hashing** - User passwords hashed with bcrypt (cost factor 12)
- **2FA Login Protection** - Independent secret key for login verification
- **Session Management** - UUID-based session tokens with in-memory storage

### 📱 TOTP Management
- Add and manage multiple TOTP entries
- Generate 6-digit TOTP codes (RFC 6238 compliant)
- QR code generation for easy setup with authenticator apps
- Support for Google Authenticator and compatible apps

### 🔑 2FA Login Protection
- Optional 2FA for login with independent secret key
- Separate from TOTP entry management
- Time-tolerant verification (±60 seconds)

## Quick Start

### Prerequisites
- Rust 1.70+ (for building from source)
- Node.js 16+ (for frontend development)

### Installation

#### Option 1: Using Environment Variable (Recommended for Production)
```bash
$env:WEBTOTP_MASTER_KEY="your-secure-master-key"
.\target\release\webtotp.exe
```

#### Option 2: Interactive Input (Recommended for Development)
```bash
.\target\release\webtotp.exe
# Follow the prompts to set or enter master key
```

#### Option 3: Using Startup Script
```bash
.\run.ps1 "your-secure-master-key"
```

### Access the Application
Open your browser and navigate to: `http://localhost:18007`

**Default Credentials:**
- Username: `admin`
- Password: `admin`

## Architecture

### Backend (Rust + Actix-web)
- **src/main.rs** - Application entry point and server setup
- **src/models.rs** - Data structures (AppState, TOTPEntry, SessionInfo)
- **src/crypto.rs** - Encryption/decryption and password hashing
- **src/config.rs** - Global configuration and master key storage
- **src/storage.rs** - Data persistence (JSON file)
- **src/auth.rs** - Authentication utilities and TOTP verification
- **src/handlers/** - HTTP request handlers
  - **auth.rs** - Login, logout, password change
  - **totp.rs** - TOTP entry management
  - **settings.rs** - 2FA configuration

### Frontend (React + Vite)
- **frontend/src/components/** - React components
  - Login, Settings, TOTP management UI
- **frontend/src/services/** - API communication
- **frontend/public/** - Static assets

### Data Storage
- **data.json** - Persistent application state
  - User credentials (password hash)
  - TOTP entries (encrypted secrets)
  - 2FA configuration (encrypted secret)
  - Master key hash (for verification)
  - Session information

## API Endpoints

### Authentication
- `POST /api/auth/login` - User login
- `POST /api/auth/logout` - User logout
- `POST /api/auth/change-password` - Change password
- `GET /api/auth/status` - Get authentication status

### TOTP Management
- `GET /api/totp/list` - List all TOTP entries
- `POST /api/totp/add` - Add new TOTP entry
- `POST /api/totp/delete` - Delete TOTP entry
- `POST /api/totp/get-code` - Get current TOTP code

### 2FA Settings
- `GET /api/settings/2fa-enabled` - Get 2FA status
- `GET /api/settings/2fa-setup` - Get 2FA setup info (QR code)
- `POST /api/settings/enable-2fa` - Enable 2FA
- `POST /api/settings/disable-2fa` - Disable 2FA
- `GET /api/settings/2fa-code` - Get current 2FA code

## Security Considerations

### Master Key
- **Minimum length:** 8 characters
- **Recommended:** 16+ characters with mixed case, numbers, and symbols
- **Storage:** Only bcrypt hash stored in data.json
- **Loss:** Cannot be recovered if forgotten

### Data Protection
| Data | Protection | Security Level |
|------|-----------|-----------------|
| Password | bcrypt hash | ⭐⭐⭐⭐⭐ |
| TOTP Secrets | AES-256-GCM | ⭐⭐⭐⭐⭐ |
| 2FA Secret | AES-256-GCM | ⭐⭐⭐⭐⭐ |
| Master Key | bcrypt hash | ⭐⭐⭐⭐⭐ |

### Best Practices
1. Use a strong, unique master key
2. Keep master key secure and backed up
3. Use HTTPS in production
4. Regularly update dependencies
5. Limit access to data.json file
6. Use environment variables for deployment

## Building from Source

### Backend
```bash
cd \workspace\webtotp
cargo build --release
```

### Frontend
```bash
cd \workspace\webtotp\frontend
npm install
npm run build
```

## Configuration

### Environment Variables
- `WEBTOTP_MASTER_KEY` - Master key for encryption (optional, prompted if not set)

### Data File
- Location: `data.json` (in application directory)
- Format: JSON
- Permissions: Should be readable/writable by application only

## Troubleshooting

### Master Key Issues
**Problem:** "Invalid master key"
- Ensure you're entering the correct master key
- Check for typos and case sensitivity

**Problem:** Master key forgotten
- Delete `data.json` and restart
- Set a new master key
- Re-add all TOTP entries

### TOTP Code Issues
**Problem:** Code doesn't match
- Verify time synchronization on device
- Check that you're using the correct TOTP entry
- Codes expire after 30 seconds

**Problem:** 2FA code rejected
- Ensure using code from "2FA Login" entry (not TOTP entries)
- Check time synchronization
- Verify 2FA is enabled

## Documentation

- **MASTER_KEY_GUIDE.md** - Detailed master key management guide
- **SECURITY_IMPROVEMENTS.md** - Security implementation details
- **FINAL_SUMMARY.md** - Project completion summary

## License

MIT

## Support

For issues or questions, please refer to the documentation files or check the application logs.

---

**Version:** 1.0.0
**Author:** steven
**Last Updated:** 2025-10-21
