---
name: secure-ipc-guardian
description: >-
  Standards, threat models, and capability hardening rules for Tauri v2 desktop applications.
  Use this skill whenever adding new IPC commands, managing filesystem access, handling shell execution,
  or configuring security capabilities in src-tauri/capabilities/.
---

# Secure IPC Guardian Skill

Tauri applications bridge web frontends with native OS capabilities. A strict least-privilege security model prevents remote code execution, arbitrary file writes, and privilege escalation.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         TAURI V2 DEFENSE-IN-DEPTH                        │
│                                                                          │
│  [ CSP Protection ]       ──> No inline scripts, strict local sources    │
│  [ Scoped Capabilities ]  ──> Explicit permissions in capabilities/*.json│
│  [ Rust Input Validation] ──> Path canonicalization & sanitization       │
│  [ Least Privilege IPC ]  ──> Frontend only asks for intent, not direct  │
│                               system execution                           │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 🔒 Security Invariants for Tauri v2

### 1. Principle of Least Privilege in Capabilities
* Never grant blanket permissions (`"core:default"`) when only specific APIs are needed.
* In `src-tauri/capabilities/default.json`, only list explicit window and command permissions:
  ```json
  {
    "identifier": "default-capability",
    "description": "Default window controls permissions",
    "windows": ["main"],
    "permissions": [
      "core:window:allow-minimize",
      "core:window:allow-toggle-maximize",
      "core:window:allow-close",
      "core:window:allow-start-dragging",
      "core:window:allow-is-maximized"
    ]
  }
  ```

### 2. Rust Input Validation & Path Traversal Prevention
* Never trust file paths passed from the frontend. Always canonicalize paths and verify they reside within allowed directories:
  ```rust
  use std::path::{Path, PathBuf};

  pub fn sanitize_path(base: &Path, user_path: &str) -> Result<PathBuf, String> {
      let candidate = base.join(user_path).canonicalize().map_err(|e| e.to_string())?;
      if !candidate.starts_with(base) {
          return Err("Access Denied: Path traversal detected".into());
      }
      Ok(candidate)
  }
  ```

### 3. Avoid Shell Injection
* Never pass un-sanitized user strings into OS shell commands. Use structured arguments (`std::process::Command::new("app").args(["--flag", user_arg])`).

---

## 📚 Deep References
- [Tauri Security Best Practices & Audit Guide](./references/tauri_security_best_practices.md)
