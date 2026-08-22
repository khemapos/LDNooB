# LDNooB

<div align="center">

[![Tauri v2](https://img.shields.io/badge/Tauri-v2.0-blue.svg?style=flat-square&logo=tauri)](https://tauri.app)
[![Svelte 5](https://img.shields.io/badge/Svelte-5.x-orange.svg?style=flat-square&logo=svelte)](https://svelte.dev)
[![Tailwind CSS v4](https://img.shields.io/badge/Tailwind_CSS-v4.3-38bdf8.svg?style=flat-square&logo=tailwindcss)](https://tailwindcss.com)
[![Bun](https://img.shields.io/badge/Bun-v1.3+-fbf0df.svg?style=flat-square&logo=bun)](https://bun.sh)
[![Rust](https://img.shields.io/badge/Rust-1.85+-dea584.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)

**A high-performance, modern native desktop application built with Tauri v2, Svelte 5 (Runes), Tailwind CSS v4, and Rust.**

[Getting Started](#-getting-started) • [Features](#-key-features) • [Tech Stack](#-tech-stack) • [Project Structure](#-project-structure)

</div>

---

## ✨ Key Features

- 🪟 **Frameless Window Bar**: Custom titanium glassmorphic titlebar with smooth native drag regions, double-click maximize, minimize, and close controls.
- 🌓 **Obsidian Dark & Light Modes**: Luxury dark color scheme (`#08090d` canvas, hairline borders, cyan/blue gradient accents) with reactive Svelte 5 `ThemeStore`, localStorage persistence, and system preference auto-detection.
- ⚡ **Svelte 5 Runes**: Built using modern Svelte 5 state management (`$state`, `$derived`, `$props`, `$effect`).
- 🎨 **Tailwind CSS v4**: Zero-config `@tailwindcss/vite` integration with native `@variant dark` support.
- 📊 **Status & Bottom Bar**: Minimalist desktop status bar displaying live connectivity, environment badges, and resolution telemetry.
- 🦀 **Robust Tauri v2 IPC**: Type-safe Rust backend commands for window operations, system integrations, and lightweight memory footprint.
- 🚀 **Fast Tooling with Bun**: Instant package installation, linting, and rapid development workflow.

---

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
| :--- | :--- | :--- |
| **Backend / Core** | [Tauri v2](https://tauri.app) + [Rust](https://www.rust-lang.org) | Native OS integration, security ACL, and window lifecycle |
| **Frontend Framework** | [Svelte 5](https://svelte.dev) + [SvelteKit](https://kit.svelte.dev) | Reactive UI components with Svelte 5 Runes |
| **Styling** | [Tailwind CSS v4](https://tailwindcss.com) | Modern utility-first CSS styling |
| **Bundler & Tooling** | [Vite 6](https://vite.dev) + [Bun](https://bun.sh) | Blazing-fast development server and package management |

---

## 📁 Project Structure

```text
ldnoob/
├── .agents/                 # AI pair programming skills and cognitive workflows
├── .mempalace/              # Architectural decisions (ADRs) and session journals
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   ├── layout/      # WindowBar, BottomBar, Navigation
│   │   │   └── ui/          # Reusable SVG Icons and UI widgets
│   │   └── stores/
│   │       ├── theme.svelte.ts   # Reactive Svelte 5 Theme store
│   │       └── window.svelte.ts  # Window dimensions and control store
│   ├── routes/
│   │   ├── +layout.svelte   # Root application layout with WindowBar and BottomBar
│   │   ├── +layout.ts       # Single-Page App (SPA) static adapter config
│   │   └── +page.svelte     # Main workspace canvas
│   ├── app.css              # Tailwind CSS v4 theme variables and dark mode variants
│   └── app.html             # Base HTML shell
├── src-tauri/
│   ├── capabilities/        # Tauri v2 security capabilities and ACL permissions
│   ├── src/
│   │   ├── lib.rs           # Rust backend handlers and window commands
│   │   └── main.rs          # Desktop executable entrypoint
│   ├── Cargo.toml           # Rust dependencies and compiler configuration
│   └── tauri.conf.json      # Window settings (1280×720, frameless), bundle config
├── package.json             # NPM / Bun scripts and frontend dependencies
├── vite.config.js           # Vite configuration with Tailwind CSS v4 plugin
└── tsconfig.json            # TypeScript configuration
```

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed on your system:
- [Bun](https://bun.sh) (v1.2+)
- [Rust](https://www.rust-lang.org/tools/install) (1.80+)
- [C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or MinGW GCC toolchain

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/khemapos/LDNooB.git
   cd LDNooB
   ```

2. **Install frontend dependencies:**
   ```bash
   bun install
   ```

3. **Run in development mode:**
   ```bash
   bun run tauri dev
   ```

4. **Build release desktop binary:**
   ```bash
   bun run tauri build
   ```

---

## 📜 Available Scripts

| Command | Action |
| :--- | :--- |
| `bun run dev` | Starts the Vite development server on `http://localhost:1420` |
| `bun run build` | Builds the static SvelteKit frontend bundle to `build/` |
| `bun run preview` | Previews the production frontend build locally |
| `bun run lint` | Runs ultra-fast static analysis with **Oxlint** & **Biome** |
| `bun run format` | Auto-formats the entire codebase with **Biome** (< 30ms) |
| `bun run lint:oxlint` | Runs pure Rust **Oxlint** rules across the codebase |
| `bun run lint:biome` | Runs **Biome** linter and import organizer |
| `bun run check` | Runs SvelteKit type checks and Svelte diagnostics (`svelte-check`) |
| `bun run tauri dev` | Starts the full Tauri v2 desktop application with live reload |
| `bun run tauri build` | Compiles the production desktop installer and executable |

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
