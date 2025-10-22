# Contributing to Web TOTP

First off, thank you for considering contributing to Web TOTP! 🎉

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, browser version)
- Logs (if applicable)

### Suggesting Enhancements

Feature requests are welcome! Please:
- Describe the feature clearly
- Explain why it would be useful
- Provide examples if possible

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Code Style

**Rust**:
- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add comments for complex logic

**JavaScript**:
- Use clear variable names
- Add JSDoc comments for functions
- Follow existing code style

**CSS**:
- Use existing CSS variables
- Group related styles together
- Add comments for complex selectors

### Testing

Before submitting:
- Test all functionality manually
- Ensure no compilation errors
- Check for runtime errors in browser console
- Verify security features work correctly

### Documentation

- Update README.md if adding features
- Add comments to complex code
- Update API documentation if changing endpoints

---

## Development Setup

```bash
# Clone repository
git clone https://github.com/steven/web-totp.git
cd web-totp

# Build
cargo build

# Run with debug logging
$env:RUST_LOG="debug"
cargo run

# Access
http://localhost:18007
```

---

## Code of Conduct

- Be respectful and constructive
- Help others learn
- Follow best security practices
- Test before submitting

---

**Author**: Steven
**License**: MIT

Thank you for contributing! 🙏

