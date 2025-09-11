# RAIN.CHAT v2 - Software Bill of Materials (SBOM)

Generated: January 9, 2025
Last Updated: January 9, 2025 - Added Tauri dialog plugin dependencies

## Backend (Rust - src-tauri)

Dependencies (Cargo.toml):
- tauri = 2.0
- tauri-plugin-dialog = 2.4
- sysinfo = 0.30
- tokio = 1.x (full)
- futures = 0.3
- serde = 1.x (derive)
- serde_json = 1.x
- sqlx = 0.7 (sqlite, chrono, uuid, runtime-tokio-rustls)
- tokio-util = 0.7
- walkdir = 2.3
- glob = 0.3
- candle-core = 0.3 (optional, feature: ai_candle)
- candle-nn = 0.3 (optional, feature: ai_candle)
- candle-transformers = 0.3 (optional, feature: ai_candle)
- candle-onnx = 0.3 (optional, feature: ai_onnx)
- hf-hub = 0.3
- tokenizers = 0.15
- llama-cpp-2 = 0.1 (optional via feature: gguf)
- reqwest = 0.11 (json, stream)
- sysinfo = 0.29
- which = 4.4
- git2 = 0.18
- tower-lsp = 0.20
- tracing = 0.1
- tracing-subscriber = 0.3 (env-filter)
- anyhow = 1.0
- thiserror = 1.0
- uuid = 1.x (v4, serde)
- chrono = 0.4 (serde)
- regex = 1.10
- base64 = 0.21
- sha2 = 0.10
- hex = 0.4
- config = 0.14
- dirs = 5.0
- pathdiff = 0.2
- bytes = 1.5
- flate2 = 1.0
- tar = 0.4
- winapi = 0.3 (windows)
- nix = 0.27 (unix)

## Frontend (Node/React)

Package.json dependencies:
- @emotion/react ^11.11.0
- @emotion/styled ^11.11.0
- @monaco-editor/react ^4.6.0
- @mui/icons-material ^5.14.0
- @mui/lab ^5.0.0-alpha.150
- @mui/material ^5.14.0
- @tauri-apps/api ^2.8.0
- @tauri-apps/plugin-dialog ^2.4.0
- axios ^1.6.0
- date-fns ^2.30.0
- diff ^5.1.0
- highlight.js ^11.9.0
- immer ^10.0.0
- lodash ^4.17.0
- monaco-editor ^0.44.0
- monaco-vim ^0.4.0
- react ^18.2.0
- react-dom ^18.2.0
- react-hotkeys-hook ^4.4.0
- react-markdown ^9.0.0
- react-query ^3.39.0
- react-router-dom ^6.20.0
- react-split-pane-2 ^0.1.3
- react-virtualized ^9.22.0
- rehype-highlight ^7.0.0
- remark-gfm ^4.0.0
- xterm ^5.3.0
- xterm-addon-fit ^0.8.0
- xterm-addon-web-links ^0.9.0
- zustand ^4.4.0

Dev dependencies:
- @tauri-apps/cli ^2.0.0
- @types/* (react, node, lodash, diff)
- @typescript-eslint/* ^6.x
- @vitejs/plugin-react ^4.x
- eslint ^8.45.0
- eslint-plugin-react-hooks ^4.6.0
- eslint-plugin-react-refresh ^0.4.3
- typescript ^5.0.2
- vite ^4.4.5

Assets:
- logo.svg (Custom RAIN.CHAT logo)
- vite.svg (Vite logo)

Note: For exact versions, consult Cargo.lock and package-lock.json.

