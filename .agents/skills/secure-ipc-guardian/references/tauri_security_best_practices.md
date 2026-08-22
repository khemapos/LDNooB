# Tauri Security Best Practices & Audit Guide

Security hardening reference for desktop Tauri v2 applications.

---

## 🛡️ Security Checkpoints

### 1. Capability Scoping
* All Tauri plugins (`fs`, `dialog`, `shell`, `notification`, `http`) require explicit permission grants in `src-tauri/capabilities/`.
* Scope filesystem reads and writes to specific folders (e.g. `$APPDATA`, `$DOWNLOAD`, `$DESKTOP`) rather than granting root filesystem access.

### 2. State & Concurrency Safety in Rust
* Store shared application state using thread-safe wrappers (`Arc<Mutex<T>>` or `Arc<RwLock<T>>` / `tauri::State`).
* Ensure state locks are held for minimal duration to avoid deadlocks between IPC calls.

---

## 📋 Security Pre-Flight Checklist
- [ ] Are all permissions in `src-tauri/capabilities/default.json` strictly necessary?
- [ ] Does any Rust command accept user paths without path canonicalization check?
- [ ] Are all Tauri command outputs safely serialized without exposing raw credentials or memory pointers?
