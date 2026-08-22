# ADR-003: Feature-Modular Rust Backend

## Status
Accepted

## Context
The Tauri backend grew into a single `src-tauri/src/lib.rs` containing IPC registration, domain models, LDPlayer discovery, filesystem traversal, process execution, telemetry, parsing, workflows, and tests. The file compiled successfully but had multiple independent reasons to change and made security-sensitive behavior difficult to review in isolation.

## Decision
Organize the Rust backend by feature and responsibility:

- Keep `lib.rs` as the composition root and stable public surface.
- Place LDPlayer IPC adapters, services, discovery, process execution, telemetry, and models in `src-tauri/src/emulator/`.
- Place system commands and models in `src-tauri/src/system/`.
- Preserve all existing Tauri IPC command names and frontend payload contracts.
- Canonicalize LDPlayer filesystem paths before native executable use.
- Keep command adapters thin and test pure helpers and services within their owning modules.

## Consequences
- Native responsibilities can be reviewed and tested independently.
- Security-sensitive process and path handling have explicit boundaries.
- `lib.rs` remains small and changes mainly when application composition changes.
- Additional modules add minor navigation overhead, but avoid the complexity of a separate Rust workspace crate at the current project size.
