---
name: clean-architecture-solid
description: >-
  Standards and workflows for applying Clean Architecture and SOLID principles across Svelte 5,
  TypeScript state modules, and Rust Tauri services. Use this skill whenever designing new feature
  subsystems, decomposing complex business logic, or structuring multi-layer architectures.
---

# Clean Architecture & SOLID Principles Skill

Enforces modularity, high cohesion, low coupling, and clear boundaries across frontend UI, state machines, IPC adapters, and Rust domain logic.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        CLEAN ARCHITECTURE LAYERS                       │
│                                                                        │
│  [ Presentation Layer ]  ──> Svelte 5 Views, Atomic UI Components      │
│  [ State & Store Layer ]  ──> Svelte 5 Runes ($state, .svelte.ts)       │
│  [ Adapter / IPC Layer ]  ──> Typed Tauri IPC Client Wrappers           │
│  [ Domain & Backend ]    ──> Rust Tauri Commands, Services & Core Crate│
└────────────────────────────────────────────────────────────────────────┘
```

---

## 🏛️ The 5 SOLID Principles for Tauri & Svelte 5

### 1. Single Responsibility Principle (SRP)
* A component or module should have one, and only one, reason to change.
* **Views** render layout and delegate actions.
* **Prims** (`src/lib/components/ui/`) only handle visual presentation.
* **State modules** (`*.svelte.ts`) manage reactivity and state transitions.
* **IPC modules** handle communication with the Rust backend.

### 2. Open / Closed Principle (OCP)
* Entities should be open for extension, but closed for modification.
* Use Svelte 5 **Snippets** (`{#snippet ...}`) and variant maps instead of adding infinite boolean flags (`isSpecial1`, `isSpecial2`) to components.

### 3. Liskov Substitution Principle (LSP)
* Subcomponents or replacement components must adhere to the same typed property contracts (`Props` interface) without unexpected side effects.

### 4. Interface Segregation Principle (ISP)
* Keep component and function interfaces focused. Do not force components to accept large composite objects if they only require 1 or 2 fields.

### 5. Dependency Inversion Principle (DIP)
* High-level modules should not depend directly on low-level implementation details; both should depend on abstractions (TypeScript interfaces / Rust traits).

---

## 📚 Deep References
- [SOLID Guidelines & Architectural Patterns](./references/solid_svelte5_guidelines.md)
