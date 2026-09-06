# Sieve

<div align="center">
  <h3>Sieve by BanguDevClub</h3>
  <p><strong>Ultra-fast, cross-platform desktop application for inspecting, filtering, and querying massive CSV files with DuckDB.</strong></p>
  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
    <img src="https://img.shields.io/badge/Tauri-v2.0-24C8D8.svg?logo=tauri" alt="Tauri">
    <img src="https://img.shields.io/badge/DuckDB-In--Process-FFF000.svg?logo=duckdb" alt="DuckDB">
    <img src="https://img.shields.io/badge/Svelte-5.0-FF3E00.svg?logo=svelte" alt="Svelte">
    <img src="https://img.shields.io/badge/Rust-1.85+-DEA584.svg?logo=rust" alt="Rust">
  </p>
</div>

---

## ⚡ Overview

**Sieve** is built by **BanguDevClub** to solve the universal pain of opening and exploring multi-gigabyte or multi-million-row CSV files without waiting minutes for spreadsheet software to freeze or crash.

By pairing **Tauri v2**'s lightweight desktop shell with **DuckDB**'s vectorized columnar engine and an ultra-responsive **Svelte 5** interface, Sieve streams gigabytes of data out-of-core with a tiny memory footprint.

---

## ✨ Features

- 🚀 **Lightning Fast DuckDB Out-of-Core Engine**: Queries multi-GB CSVs in milliseconds without loading the full dataset into RAM.
- ⏱️ **Estimate & Progress Reporting**: Real-time progress modal displaying sampling phase, estimated records, elapsed timer, and live cancellation.
- 🎯 **Advanced Filter Builder**:
  - Exact Match (`=`)
  - Fuzzy Search / Contains (`LIKE`)
  - In List (`IN (...)`)
  - Numeric ranges (`>`, `<`, `>=`, `<=`)
  - Multi-condition chaining with `AND` or `OR` logic.
- 📊 **Frozen Sticky Header & Resizable Columns**: Column headers stay pinned while scrolling; drag separator handles to adjust column widths fluidly.
- 💻 **Custom DuckDB SQL Script Console**: Run arbitrary analytical queries (`GROUP BY`, `COUNT(*)`, `HAVING`, window functions) directly against the CSV with execution timer badges.
- 📄 **Enforced Pagination Limits**: Caps viewing to 100 rows per page and up to 50 columns horizontally to prevent DOM slowdowns.
- 🎨 **6 Modern Design System Themes**:
  1. `Light`
  2. `Dark`
  3. `Catppuccin Latte`
  4. `Catppuccin Frappé`
  5. `Catppuccin Macchiato`
  6. `Catppuccin Mocha`
- 🎛️ **Custom Themed Inputs & Selects**: Accessible, fully customized inputs with search, clear buttons, and theme-adaptive styling (no default browser widgets).
- 🔤 **Nerd Fonts & Font Awesome Icons**: Clean vector iconography throughout the entire application—zero emojis.
- 📦 **Cross-Platform Distributions**:
  - **Linux GNU**: Standalone binary, `.deb`, `.rpm`, `.AppImage`
  - **Linux MUSL**: Standalone static binary
  - **Windows**: Standalone `.exe` and NSIS installer
  - **macOS**: Standalone binary, `.app` bundle, and `.dmg` installer

---

## 🏗️ Project Structure

```
sieve/
├── frontend/               # Svelte 5 + Vite + TypeScript interface
│   ├── src/
│   │   ├── lib/components/ # Reusable UI, table, filter, SQL, and modal components
│   │   ├── lib/icons/      # Font Awesome and Nerd Font SVG icons
│   │   └── app.css         # 6-theme design system and CSS variables
│   └── package.json
├── backend/                # Tauri v2 + Rust backend
│   ├── src/
│   │   ├── duckdb_engine.rs# DuckDB query planner, out-of-core streaming, progress
│   │   ├── commands.rs     # Tauri IPC commands
│   │   ├── state.rs        # App state and worker pool
│   │   └── main.rs         # Tauri application entry point
│   ├── Cargo.toml
│   └── tauri.conf.json
├── gitignore/              # Modular gitignore templates
├── .github/workflows/      # GitHub Actions CI matrix for Linux, Windows, macOS
├── Dockerfile              # Multi-target build image based on rust:latest
├── docker-compose.yml      # Local Docker build runner
└── dist-build/             # Final generated release packages
```

---

## 🚀 Getting Started

### Prerequisites
- Node.js (v20+ or v22+)
- Rust (1.85+)
- Linux packages (if compiling locally on Linux): `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

### Development
```bash
# Install frontend dependencies
cd frontend && npm install && cd ..

# Run frontend in development mode
npm run dev

# Type check frontend and backend
npm run check

# Preview frontend build
npm run preview
```

---

## 🐳 Docker Local Build (`docker compose up`)

You can compile and generate packages for **Linux GNU**, **Linux MUSL**, and **Windows** locally in a clean, reproducible container using Docker:

```bash
docker compose up --build
```

All compiled binaries and packages are placed into the `./dist-build/` folder:
- `dist-build/gnu/`
  - `sieve` (Standalone Linux executable)
  - `sieve_0.1.0_amd64.deb` (Debian/Ubuntu)
  - `sieve-0.1.0-1.x86_64.rpm` (Fedora/RHEL)
  - `Sieve-x86_64.AppImage` (Portable AppImage)
- `dist-build/musl/`
  - `sieve` (Statically linked MUSL executable)
- `dist-build/windows/`
  - `sieve.exe` (Standalone Windows binary)
  - `sieve-setup.exe` (Windows NSIS Installer)
- `dist-build/macos/`
  - macOS bundle tree and distribution scripts

---

## 🌐 GitHub Actions Multi-OS Matrix

Automated CI/CD runs natively on GitHub-hosted virtual environments without Docker:
- **Linux GNU / MUSL**: `ubuntu-latest`
- **Windows**: `windows-latest`
- **macOS (Intel AMD64)**: `macos-13`
- **macOS (Apple Silicon ARM64)**: `macos-14`

Artifacts are packaged and uploaded automatically on every release or push.

---

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.  
Maintained with passion by **BanguDevClub**.
