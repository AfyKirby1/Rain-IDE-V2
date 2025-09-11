# RAIN.CHAT v2 - Professional Desktop AI IDE

![RAIN.CHAT v2 Logo](RAIN.png)

## 🌟 Overview

RAIN.CHAT v2 is a cutting-edge, professional desktop IDE specifically designed for AI development. Built with Rust (Tauri) and React, it combines native performance with modern web technologies to deliver an exceptional development experience.

## ✨ Features

### 🚀 **Core IDE Features**
- **📝 Advanced File Editor**: Multi-tab editor with syntax highlighting and language detection
- **📁 Smart File Explorer**: Hierarchical file tree with real-time updates
- **🎯 Cursor-Style Scratch Files**: Create untitled files instantly (Ctrl+N) - no prompts, just start coding!
- **💾 Intelligent Save System**: Save-as functionality for new files, auto-save for existing files
- **⌨️ Keyboard Shortcuts**: Ctrl+N (new file), Ctrl+S (save), and more

### 🤖 **AI-Powered Development**
- **🧠 Multi-Model Support**: HuggingFace Transformers + GGUF models
- **💬 AI Chat Integration**: Built-in chat panel for AI assistance
- **🔍 Code Analysis**: AI-powered code suggestions and debugging
- **📊 Model Management**: Easy model discovery, loading, and switching

### ⚡ **Performance & Architecture**
- **🚀 Native Performance**: Built with Tauri for optimal speed and resource usage
- **🎨 Modern UI**: Beautiful, responsive interface using Material-UI
- **🗃️ Smart Database**: SQLite-based data persistence for projects and settings
- **📦 48+ Backend Features**: Comprehensive API with terminal, git, debugging, and more

### 🔧 **Developer Experience**
- **📁 Project Management**: Advanced project organization and workspace management
- **🌐 Git Integration**: Built-in version control capabilities (ready for integration)
- **🖥️ Terminal Integration**: Embedded terminal for seamless development workflow (ready for integration)
- **🐛 Debugger Support**: Advanced debugging capabilities (ready for integration)
- **🔧 Multi-Language Support**: LSP integration for various programming languages

## 🚀 Quick Start

### Prerequisites

- **Node.js** (v18+)
- **Rust** (latest stable)
- **Windows** (primary platform)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rain-chat-v2
   ```

2. **Install frontend dependencies**
   ```bash
   npm install
   ```

3. **Install Rust dependencies** (automatic via Cargo)

4. **Launch the development environment**
   ```bash
   npm run tauri:dev
   ```

### Building for Production

```bash
npm run tauri:build
```

## 🏗️ Architecture

### Backend (Rust - Tauri)
- **Framework**: Tauri v2.0
- **Runtime**: Tokio async runtime
- **Database**: SQLite with SQLx
- **AI**: Optional Candle ML framework + GGUF support
- **LSP**: Tower-LSP for language server integration

### Frontend (React + TypeScript)
- **Framework**: React 18 with TypeScript
- **UI Library**: Material-UI (MUI) v5
- **State Management**: Zustand
- **Routing**: React Router v6
- **Editor**: Monaco Editor
- **Terminal**: xterm.js

## 📦 Key Dependencies

### New in Latest Update
- **tauri-plugin-dialog**: Native file dialogs (v2.4)
- **Enhanced Performance**: Optimized model loading and startup

### Core Dependencies
- **Tauri**: Cross-platform app framework
- **React**: Modern frontend framework  
- **SQLite**: Lightweight database
- **Monaco Editor**: VS Code editor engine
- **Material-UI**: React component library

See [SBOM.md](SBOM.md) for complete dependency list.

## 🔧 Development

### Project Structure
```
rain-chat-v2/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── stores/            # State management
│   └── types/             # TypeScript definitions
├── src-tauri/             # Rust backend
│   ├── src/               # Rust source code
│   └── Cargo.toml         # Rust dependencies
├── models/                # AI model storage
└── README.md              # This file
```

### Key Commands
- `npm run dev` - Start frontend dev server
- `npm run tauri:dev` - Start full app in development
- `npm run tauri:build` - Build production app
- `npm run lint` - Run ESLint
- `npm run type-check` - TypeScript checking

## 🛠️ Recent Updates

### 🎉 **Major Feature Release - January 9, 2025**

#### 🚀 **New IDE Features**
- ✅ **Advanced File Editor**: Multi-tab editor with real-time editing and syntax highlighting
- ✅ **Cursor-Style Scratch Files**: Create untitled files instantly (Ctrl+N) - no prompts required!
- ✅ **Smart File Explorer**: Click files to open them in the editor with full project navigation
- ✅ **Intelligent Save System**: Save-as for new files, auto-save for existing files
- ✅ **Keyboard Shortcuts**: Ctrl+N (new file), Ctrl+S (save), Ctrl+B (toggle explorer)

#### 🔧 **Technical Improvements**
- ✅ **48+ Backend Features Discovered**: Found comprehensive backend API ready for integration
- ✅ **State Management**: Robust Zustand-based editor state management
- ✅ **Error Handling**: Comprehensive error handling and user feedback
- ✅ **Performance Optimization**: Reduced compilation warnings from 119 to 23

#### 🐛 **Bug Fixes**
- ✅ Fixed file opening and editing functionality
- ✅ Resolved undefined variable errors in editor
- ✅ Fixed project opening and file tree display
- ✅ Eliminated duplicate state management issues

### 📊 **Development Statistics**
- **Backend Functions**: 48+ fully implemented and ready for frontend integration
- **Compilation Warnings**: Reduced from 119 to 23 (81% improvement)
- **Features Working**: File editing, project management, AI chat, model loading
- **Documentation**: Comprehensive docs for removed features and future implementation

## 🔐 Security

This project maintains a comprehensive Software Bill of Materials (SBOM) for security transparency. See [SBOM.md](SBOM.md) for detailed dependency tracking.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

**PROPRIETARY SOFTWARE** - All rights reserved.

This software is proprietary and confidential. No reproduction, modification, or 
distribution is permitted without explicit written permission from AfyKirby1.

See [LICENSE](LICENSE) for full terms and restrictions.

## 🆘 Support

For issues, questions, or contributions:
- Check existing issues first
- Provide detailed bug reports
- Include system information and logs

---

**RAIN.CHAT v2** - Empowering developers with AI-assisted coding in a native, high-performance IDE.