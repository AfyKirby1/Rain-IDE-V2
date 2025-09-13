# RAIN.CHAT v2 Build Guide

This guide explains how to build RAIN.CHAT v2 into a distributable Windows executable (.exe) file.

## Prerequisites

Before building, ensure you have the following installed:

- **Node.js 18+** - [Download from nodejs.org](https://nodejs.org/)
- **Rust 1.70+** - [Install via rustup.rs](https://rustup.rs/)
- **Git** - [Download from git-scm.com](https://git-scm.com/)

### Verify Installation

Run these commands to verify your setup:

```powershell
node --version    # Should show v18.x.x or higher
npm --version     # Should show 8.x.x or higher
rustc --version   # Should show 1.70+ or higher
cargo --version   # Should show 1.70+ or higher
```

## Build Methods

### Method 1: PowerShell Script (Recommended)

Use the comprehensive PowerShell build script:

```powershell
# Build release version
.\build.ps1

# Build debug version (faster, larger file)
.\build.ps1 -Debug

# Clean and build
.\build.ps1 -Clean

# Show help
.\build.ps1 -Help
```

### Method 2: Batch File (Simple)

Use the simple batch file for basic builds:

```cmd
build.bat
```

### Method 3: NPM Scripts

Use the npm scripts directly:

```powershell
# Build release version
npm run build:exe

# Build debug version
npm run build:exe:debug

# Clean build artifacts
npm run clean

# Clean everything (including node_modules)
npm run clean:all
```

### Method 4: Manual Build

Build step by step:

```powershell
# 1. Install dependencies
npm install

# 2. Build frontend
npm run build

# 3. Build Tauri app
npm run tauri:build

# For debug version:
npm run tauri:build:debug
```

## Build Output

After a successful build, you'll find:

### Executable Files
- **Release**: `src-tauri/target/release/rain-chat-v2.exe`
- **Debug**: `src-tauri/target/debug/rain-chat-v2.exe`

### Distribution Files
- **Installer**: `src-tauri/target/release/bundle/msi/`
- **Portable**: `src-tauri/target/release/bundle/nsis/`

## Build Configuration

### Tauri Configuration
The build is configured in `src-tauri/tauri.conf.json`:

- **App Name**: RAIN.CHAT v2
- **Version**: 2.0.0
- **Window Size**: 1600x900 (min: 1200x700)
- **Targets**: All platforms (Windows, macOS, Linux)

### Frontend Build
The frontend is built using Vite and configured in `vite.config.ts`.

## Troubleshooting

### Common Issues

1. **"Node.js not found"**
   - Install Node.js 18+ from [nodejs.org](https://nodejs.org/)
   - Restart your terminal after installation

2. **"Rust not found"**
   - Install Rust from [rustup.rs](https://rustup.rs/)
   - Run `rustup update` to ensure latest version

3. **"Permission denied"**
   - Run PowerShell as Administrator
   - Or use `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`

4. **Build fails with "cargo" errors**
   - Update Rust: `rustup update`
   - Clean build: `npm run clean:all` then rebuild

5. **Frontend build fails**
   - Check for TypeScript errors: `npm run type-check`
   - Fix linting issues: `npm run lint:fix`

### Performance Tips

- **Debug builds** are faster to compile but produce larger files
- **Release builds** are optimized but take longer to compile
- Use `-Clean` flag to ensure fresh builds
- Close other applications during build to free up memory

## File Sizes

Typical build sizes:
- **Debug executable**: ~50-100 MB
- **Release executable**: ~20-40 MB
- **Installer package**: ~30-60 MB

## Distribution

### For End Users
Provide the installer from `src-tauri/target/release/bundle/msi/`

### For Developers
Share the portable executable from `src-tauri/target/release/`

### Code Signing (Optional)
For production distribution, consider code signing:
- Obtain a code signing certificate
- Configure in `tauri.conf.json`
- Sign the final executable

## Advanced Configuration

### Custom Build Features
Enable optional features in `src-tauri/Cargo.toml`:

```toml
[features]
default = ["custom-protocol"]
gguf = ["llama-cpp-2"]           # GGUF model support
ai_candle = ["candle-core"]      # Candle ML backend
ai_onnx = ["ai_candle", "candle-onnx"]  # ONNX support
```

### Environment Variables
Set these for custom builds:

```powershell
$env:TAURI_PRIVATE_KEY = "your-private-key"
$env:TAURI_KEY_PASSWORD = "your-password"
```

## Support

If you encounter issues:
1. Check this guide first
2. Review build output for specific errors
3. Check [Tauri documentation](https://tauri.app/)
4. Review project issues on GitHub

---

**Happy Building! 🚀**
