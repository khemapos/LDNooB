# SOLID & Clean Architecture Guidelines for Svelte 5 & Tauri

This document provides concrete code patterns demonstrating how to apply SOLID in this codebase.

---

## 🛠️ Concrete Examples

### 1. SRP: Separating State from Presentation
* ❌ **Violating SRP**: Inlining heavy network fetching, audio parsing, state caching, and 400 lines of UI inside a single `+page.svelte`.
* ✅ **Applying SRP**:
  - `src/lib/state/workspace.svelte.ts` (Class / module using `$state()` runes for state logic).
  - `src/lib/ipc/workspace.ts` (Typed Rust Tauri invocations).
  - `src/routes/+page.svelte` (Clean view consuming the state).

### 2. OCP: Snippet & Variant Architecture
* ❌ **Violating OCP**: Modifying `Button.svelte` with 15 nested `if / else` blocks for each new custom one-off design.
* ✅ **Applying OCP**: Use typed variant maps (`variant: 'primary' | 'secondary' | 'danger' | 'ghost'`) and allow custom children snippets `{@render children?.()}`.

---

## 📋 Architectural Checklist
- [ ] Are UI components free of direct raw filesystem or OS IPC logic?
- [ ] Is business logic encapsulated in `.svelte.ts` state files?
- [ ] Are modules decoupled and independently testable?
