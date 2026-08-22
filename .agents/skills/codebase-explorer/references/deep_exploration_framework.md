# Deep Codebase Exploration Framework

This framework provides specialized heuristics, investigative protocols, and critical reasoning models for performing high-fidelity source code audits in full-stack desktop architectures (Tauri v2 + Svelte 5 + Rust).

---

## 🧠 Cognitive Mental Models for Code Auditing

### 1. Invariant Hunting (State & Contract Integrity)
- **Question**: What must *always* be true for this component/subsystem to remain in a valid state?
- **Failure Mode**: State desynchronization between frontend runes (`$state`) and backend system singletons.
- **Probe**: Trigger rapid concurrent interactions, window resizes, unmounts, and IPC cancellations. Observe if invalid intermediate states can be rendered.

### 2. Failure Mode & Effects Analysis (FMEA / Pre-Mortem)
- **Question**: If this subsystem breaks in production, what will be the exact failure chain?
- **Probing Points**:
  - Unhandled IPC errors in async promises.
  - OS-level locks (e.g. `(os error 32)` when accessing files in `target/debug` or configuration files).
  - Webview reload clearing in-memory state while Rust daemon continues running.

### 3. Abstraction Weight vs Utility Ratio
- **Question**: Is this abstraction pulling its own weight, or is it premature indirection?
- **Rule**: If an abstraction adds 3 layers of indirection but only has 1 concrete consumer with no anticipated polymorphism, recommend collapsing it (DRY vs YAGNI balance).

---

## 🔍 Forensic Investigation Vectors

### Vector A: Reactive State & Svelte 5 Rune Hygiene
- [ ] Are `$state` runes placed in shared `.svelte.ts` modules for global singletons?
- [ ] Are `$derived` calculations pure and free of side effects?
- [ ] Are `$effect` blocks strictly reserved for DOM synchronization or external integration (never for cascading state mutations)?
- [ ] Do component unmounts clean up subscriptions, event listeners, and timers?

### Vector B: Tauri v2 IPC Security & Trust Boundaries
- [ ] Does `src-tauri/capabilities/*.json` follow the Principle of Least Privilege?
- [ ] Are command arguments validated defensively on the Rust backend before executing system commands or file operations?
- [ ] Are Rust command errors returned as typed, actionable `Result<T, AppError>` instead of panicking (`.unwrap()`) or leaking raw stack traces?

### Vector C: Desktop UX & Concurrency Ergonomics
- [ ] Are long-running commands spawned asynchronously to prevent freezing the 60fps Webview UI thread?
- [ ] Do frameless window drag regions (`data-tauri-drag-region`) preserve `event.stopPropagation()` on all interactive controls?
- [ ] Is keyboard navigation accessible with roving tab index, ARIA roles, and clear focus rings?

### Vector D: Build Pipeline & Toolchain Determinism
- [ ] Does `bun run check` produce zero errors and zero warnings?
- [ ] Does the Rust linker build cleanly without export ordinal overflows under MinGW (`crate-type = ["rlib"]`)?
- [ ] Is `.gitignore` hardened against committing binaries, node_modules, and cache files?
