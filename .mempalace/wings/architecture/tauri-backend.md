# Room: Tauri v2 & Rust IPC

## Summary
The backend layer is built using **Tauri v2** with **Rust 2021 edition**.

## Architecture & Conventions
- **Composition Root**: `src-tauri/src/lib.rs` declares backend modules, re-exports the stable library surface, and registers IPC commands.
- **Emulator Feature**: `src-tauri/src/emulator/` separates IPC commands, discovery/path validation, process execution, telemetry, domain models, and emulator workflows.
- **System Feature**: `src-tauri/src/system/` owns system metrics, diagnostics, greeting commands, and their response models.
- **Commands**: Thin command adapters are registered in `src-tauri/src/lib.rs` inside `tauri::generate_handler![...]`; existing command names remain stable for the frontend.
- **IPC Protocol**: Frontend communicates via `@tauri-apps/api/core` with `invoke('command_name', { args })`.
- **Capabilities & Permissions**: Tauri v2 enforces explicit capability permissions defined under `src-tauri/capabilities/`.
- **Filesystem Safety**: User-selected LDPlayer directories and executable candidates are canonicalized and verified before native process execution.
- **Configuration**: Main config is in `src-tauri/tauri.conf.json`, specifying default window dimensions (1024x720, min 700x500, centered, frameless), security CSP, and bundle assets.

## Key Files
- `src-tauri/src/main.rs`: Application entry point.
- `src-tauri/src/lib.rs`: Plugin initialization, module composition, public re-exports, and IPC handler registration.
- `src-tauri/src/emulator/commands.rs`: Tauri IPC adapters for LDPlayer operations.
- `src-tauri/src/emulator/service.rs`: Emulator listing and lifecycle workflows.
- `src-tauri/src/emulator/discovery.rs`: Installation discovery and canonical path validation.
- `src-tauri/src/emulator/process.rs`: Structured LDPlayer console execution without shell interpolation.
- `src-tauri/src/emulator/telemetry.rs`: RAM and disk telemetry helpers.
- `src-tauri/src/system/`: System-level commands and response models.
- `src-tauri/Cargo.toml`: Rust crates (`tauri`, `tauri-plugin-opener`, `serde`, `serde_json`).
- `src-tauri/tauri.conf.json`: Tauri configuration.
