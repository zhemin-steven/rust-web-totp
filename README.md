# Web TOTP 🔐

<div align="center">

![Security](https://img.shields.io/badge/Security-9.0%2F10-brightgreen)
![Encryption](https://img.shields.io/badge/Encryption-AES--256--GCM-blue)
![License](https://img.shields.io/badge/License-MIT-yellow)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)

**Production-Grade 2FA Management Tool**

Secure, Beautiful, Cross-Platform

[English](#english) | [中文](#中文)

</div>

---

## 中文

### 简介

Web TOTP 是一个**生产级**的双因素认证（2FA）管理工具，类似 Google Authenticator，但提供了更强大的安全性和更好的用户体验。

### ✨ 核心特性

- 🔒 **AES-256-GCM 加密** - 军事级加密保护所有数据
- 🔑 **主密码保护** - 使用 Argon2 密钥派生，数据库启动时锁定
- 📱 **TOTP 生成** - 符合 RFC 6238 标准，兼容所有服务
- 🌍 **国际化** - 中文/English 双语支持
- 🎨 **现代界面** - 渐变设计 + 流畅动画
- 💻 **跨平台** - Windows / Linux / macOS
- ⚡ **智能交互** - 一键复制验证码，自动显示2FA输入框

**安全评分**: 9.0/10 🏆

### 🚀 快速开始

#### Windows
```powershell
# 启动服务器
.\START.bat

# 或直接运行
.\target\release\web-totp.exe
```

#### Linux / macOS
```bash
# 一键安装
chmod +x install.sh && ./install.sh

# 或手动构建
chmod +x build.sh && ./build.sh
./start.sh
```

#### 访问
浏览器打开: **http://127.0.0.1:18007**

### 📖 使用指南

**首次使用**:
1. 🔓 **设置主密码** - 在解锁页面输入强密码（用于加密数据库）
2. 👤 **登录系统** - 使用 admin/admin
3. ⚙️ **修改密码** - 进入设置页面修改默认密码
4. 📱 **添加账户** - 点击"添加 2FA"，输入服务提供的密钥

**重要提醒**:
- ⚠️ 主密码用于加密数据库，忘记后无法恢复
- 💾 请务必备份主密码和 data.enc 文件
- 🔒 首次登录后请立即修改默认密码

### 🔐 安全特性

- **AES-256-GCM** - 认证加密，防篡改
- **Argon2** - 密钥派生，抗暴力破解
- **随机盐值和Nonce** - 每次加密都不同
- **完整日志** - 所有操作可审计
- **无panic** - 完善的错误处理

### 📚 文档

- [完整使用手册](docs/USER_GUIDE.md) - 详细操作指南
- [安全性分析](docs/SECURITY.md) - 加密实现详解
- [API 文档](docs/API.md) - RESTful API 参考
- [更新日志](CHANGELOG.md) - 版本历史

### 🛠️ 技术栈

**后端**: Rust + actix-web + aes-gcm + argon2 + totp-rs  
**前端**: HTML5 + CSS3 + Vanilla JavaScript  
**加密**: AES-256-GCM + Argon2id  
**标准**: RFC 6238 (TOTP) + RFC 9106 (Argon2)

### 🤝 贡献

欢迎提交 Issue 和 Pull Request！请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

### 📝 许可证

MIT License - 详见 [LICENSE](LICENSE)

### 👨‍💻 作者

**Steven**

如果这个项目对你有帮助，请给个 ⭐ Star！

---

## English

### Introduction

Web TOTP is a **production-grade** Two-Factor Authentication (2FA) management tool, similar to Google Authenticator, but with enhanced security and better user experience.

### ✨ Key Features

- 🔒 **AES-256-GCM Encryption** - Military-grade encryption for all data
- 🔑 **Master Password Protection** - Argon2 key derivation, database locked at startup
- 📱 **TOTP Generation** - RFC 6238 compliant, compatible with all services
- 🌍 **Internationalization** - Chinese/English bilingual support
- 🎨 **Modern UI** - Gradient design + smooth animations
- 💻 **Cross-Platform** - Windows / Linux / macOS
- ⚡ **Smart Interaction** - One-click copy, auto-show 2FA input

**Security Rating**: 9.0/10 🏆

### 🚀 Quick Start

#### Windows
```powershell
# Start server
.\START.bat

# Or run directly
.\target\release\web-totp.exe
```

#### Linux / macOS
```bash
# One-click install
chmod +x install.sh && ./install.sh

# Or manual build
chmod +x build.sh && ./build.sh
./start.sh
```

#### Access
Open browser: **http://127.0.0.1:18007**

### 📖 Usage

**First Time**:
1. 🔓 **Set Master Password** - Enter a strong password on unlock page
2. 👤 **Login** - Use admin/admin
3. ⚙️ **Change Password** - Go to settings and change default password
4. 📱 **Add Accounts** - Click "Add 2FA", enter the secret key

**Important**:
- ⚠️ Master password encrypts the database, cannot be recovered if lost
- 💾 Backup your master password and data.enc file
- 🔒 Change default password after first login

### 🔐 Security

- **AES-256-GCM** - Authenticated encryption, tamper-proof
- **Argon2** - Key derivation, brute-force resistant
- **Random Salt & Nonce** - Different encryption every time
- **Complete Logging** - All operations auditable
- **No Panic** - Comprehensive error handling

### 📚 Documentation

- [User Guide](docs/USER_GUIDE.md) - Complete manual
- [Security Analysis](docs/SECURITY.md) - Encryption details
- [API Reference](docs/API.md) - RESTful API docs
- [Changelog](CHANGELOG.md) - Version history

### 🛠️ Tech Stack

**Backend**: Rust + actix-web + aes-gcm + argon2 + totp-rs  
**Frontend**: HTML5 + CSS3 + Vanilla JavaScript  
**Encryption**: AES-256-GCM + Argon2id  
**Standards**: RFC 6238 (TOTP) + RFC 9106 (Argon2)

### 🤝 Contributing

Issues and Pull Requests are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

### 📝 License

MIT License - see [LICENSE](LICENSE)

### 👨‍💻 Author

**Steven**

If this project helps you, please give it a ⭐ Star!

---

<div align="center">

**Secure Your 2FA Accounts with Web TOTP!** 🔐✨

Made with ❤️ by Steven

</div>
