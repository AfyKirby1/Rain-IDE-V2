# Build Tools for RAIN.CHAT v2

This folder contains all the build scripts and documentation for creating distributable executables of RAIN.CHAT v2.

## 📁 Contents

- **`build.ps1`** - Advanced PowerShell build script with options
- **`build.bat`** - Simple batch file for basic builds  
- **`BUILD_GUIDE.md`** - Comprehensive build documentation and troubleshooting
- **`README.md`** - This file

## 🚀 Quick Start

### PowerShell (Recommended)
```powershell
# From project root
.\build\build.ps1              # Build release version
.\build\build.ps1 -Debug       # Build debug version  
.\build\build.ps1 -Clean       # Clean and build
.\build\build.ps1 -Help        # Show help
```

### Batch File (Simple)
```cmd
# From project root
build\build.bat
```

### NPM Commands (Alternative)
```powershell
# From project root
npm run build:exe              # Build release
npm run build:exe:debug        # Build debug
npm run clean                  # Clean build artifacts
```

## 📋 Prerequisites

- **Node.js 18+** - [Download from nodejs.org](https://nodejs.org/)
- **Rust 1.70+** - [Install via rustup.rs](https://rustup.rs/)
- **Git** - [Download from git-scm.com](https://git-scm.com/)

## 🎯 Build Output

After a successful build, you'll find:

### Executable Files
- **Release**: `src-tauri/target/release/rain-chat-v2.exe`
- **Debug**: `src-tauri/target/debug/rain-chat-v2.exe`

### Distribution Files
- **MSI Installer**: `src-tauri/target/release/bundle/msi/RAIN.CHAT v2_2.0.0_x64_en-US.msi`
- **NSIS Installer**: `src-tauri/target/release/bundle/nsis/RAIN.CHAT v2_2.0.0_x64-setup.exe`

## 📖 Documentation

For detailed information, troubleshooting, and advanced configuration options, see:
- **[BUILD_GUIDE.md](./BUILD_GUIDE.md)** - Complete build documentation
- **[../README.md](../README.md)** - Main project documentation

## ⚡ Performance

- **Build Time**: ~6 minutes for release build
- **File Sizes**: 
  - Release executable: ~20-40 MB
  - MSI installer: ~6-7 MB
  - NSIS installer: ~4-5 MB

## 🛠️ Troubleshooting

Common issues and solutions are covered in [BUILD_GUIDE.md](./BUILD_GUIDE.md).

For additional help, check the main project documentation or create an issue on GitHub.

---

**Happy Building! 🚀**
