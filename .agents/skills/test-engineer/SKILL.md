---
name: test-engineer
description: >-
  Standardized test-driven workflows, unit testing, and automated quality assurance across Svelte 5
  components, TypeScript utilities, and Rust Tauri backend services using Bun test and Cargo test.
  Use this skill whenever authoring tests, setting up regression guards, or debugging edge cases.
---

# Test Engineer & Automated Verification Skill

Robust automated tests ensure that refactorings, new features, and design updates can be delivered with zero regressions and absolute confidence.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        AUTOMATED TESTING PYRAMID                       │
│                                                                        │
│  [ E2E / Integration ]  ──> Tauri Webview & Rust IPC End-to-End Tests  │
│  [ Component Tests ]    ──> Svelte 5 Component Render & Interaction    │
│  [ Unit Tests ]         ──> Pure TypeScript Functions & Rust Lib Tests │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 🧪 Testing Commands & Tooling

* **TypeScript / Frontend Unit Tests**: Run using Bun's native runner:
  ```bash
  bun test
  ```
* **Type & Template Diagnostics**:
  ```bash
  bun run check
  ```
* **Rust Backend Tests**:
  ```bash
  cargo test --manifest-path src-tauri/Cargo.toml
  ```

---

## ✍️ Writing Tests in Svelte 5 & TypeScript

1. **Pure Unit Tests (`*.test.ts`)**:
   ```ts
   import { describe, it, expect } from 'bun:test';

   describe('Sanitize Path Utility', () => {
     it('should trim leading and trailing slashes correctly', () => {
       const result = sanitizePath('/my/path/');
       expect(result).toBe('my/path');
     });
   });
   ```

2. **Rust Backend Tests (`src-tauri/src/lib.rs`)**:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_greet_command() {
           let greeting = greet("Tester");
           assert_eq!(greeting, "Hello, Tester! You've been greeted from Rust!");
       }
   }
   ```

---

## 📚 Deep References
- [Testing Standards & Test Doubles Guide](./references/testing_standards.md)
