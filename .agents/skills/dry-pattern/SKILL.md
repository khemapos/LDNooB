---
name: dry-pattern
description: >-
  Systematic guide and standards for enforcing the DRY (Don't Repeat Yourself) principle,
  Single Source of Truth (SSOT), and modular abstraction across Svelte 5, Tailwind v4,
  TypeScript types, and Tauri IPC. Use this skill whenever designing components, refactoring
  code, eliminating duplicate logic, or structuring shared utilities and state.
---

# DRY (Don't Repeat Yourself) Architecture Skill

The DRY principle states that **every piece of knowledge or logic must have a single, unambiguous, authoritative representation within a system**.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           DRY THREE-TIER AUDIT                          │
│                                                                         │
│  [1. Visual / UI Markup]   ──> [Reusable Svelte 5 UI Primitives]       │
│  [2. State & Data Flow]    ──> [Single Source of Truth & $derived]      │
│  [3. Types & Contracts]    ──> [Shared TypeScript Interfaces & Models]  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 🎯 The 4 Pillars of DRY in this Codebase

### 1. UI Markup & Visual Components (Svelte 5)
* **Never duplicate raw HTML tags with repeated classes**: If a styled button, input, card, badge, or modal is used more than once, use or create a primitive in `src/lib/components/ui/`.
* **Use Snippets for Local Repetition**: For repetitive loops or list items within a single component, encapsulate using Svelte 5 snippets:
  ```svelte
  {#snippet actionItem(icon, label, action)}
    <button type="button" onclick={action} class="flex items-center gap-2 p-2 rounded-lg hover:bg-white/5">
      <Icon name={icon} />
      <span>{label}</span>
    </button>
  {/snippet}

  {@render actionItem('editor', 'Open File', handleOpen)}
  {@render actionItem('terminal', 'New Terminal', handleTerm)}
  ```

### 2. State Management (Svelte 5 Runes)
* **Never maintain synchronized duplicate state**:
  - ❌ **Anti-Pattern**:
    ```ts
    let firstName = $state('');
    let lastName = $state('');
    let fullName = $state(''); // ⚠️ Duplicate state manually synced via effects!
    ```
  - ✅ **DRY Pattern**:
    ```ts
    let firstName = $state('');
    let lastName = $state('');
    let fullName = $derived(`${firstName} ${lastName}`.trim()); // Authoritative derived value
    ```

### 3. Type Definitions & Data Contracts (TypeScript)
* **Centralize Shared Interfaces**: Export interfaces from their primary module (e.g. `export interface TabItem` in `src/lib/types/` or component header) rather than redefining duplicate shapes across multiple files.
* **Derive Types with Utility Types**: Use `Pick<T, K>`, `Omit<T, K>`, or `Partial<T>` instead of copying interface properties.

### 4. IPC & Command Invocations (Tauri v2)
* Avoid hardcoding raw string command names or copy-pasting IPC try-catch blocks across multiple components. Create typed client wrapper helpers in `src/lib/ipc/` when commands are called from multiple views.

---

## ⚖️ The "Rule of Three" Balancing Heuristic
* **Duplication is far cheaper than the wrong abstraction**.
* **1st occurrence**: Implement directly.
* **2nd occurrence**: Note the similarity, but don't rush into premature abstraction if requirements might diverge.
* **3rd occurrence**: Extract into a shared component, utility function, or snippet.

---

## 📚 Deep References
- [DRY Rules & Anti-Patterns Guide](./references/dry_rules_and_antipatterns.md)
