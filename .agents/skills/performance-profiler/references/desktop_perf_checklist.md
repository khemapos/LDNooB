# Desktop Performance Checklist & Profiling Guide

Audit checklist for validating desktop application responsiveness, memory footprint, and binary optimization.

---

## 📋 Pre-Release Performance Audit Checklist

### Frontend Optimization
- [ ] Are dynamic list renderings keyed properly (`{#each items as item (item.id)}`)?
- [ ] Are unused libraries and bulky icons stripped from production bundles?
- [ ] Are static assets (SVGs, fonts) served locally from `static/` without external network requests?
- [ ] Does `bun run build` generate gzipped client bundles under 50KB total?

### Backend & IPC Optimization
- [ ] Are all filesystem I/O operations in Rust asynchronous (`tokio::fs`)?
- [ ] Are IPC payloads minimal (only transmit required fields, avoid passing multi-megabyte JSON blobs when binary streams can be used)?
- [ ] Is Rust release profile compiled with LTO (Link-Time Optimization) and size stripping in `Cargo.toml`:
  ```toml
  [profile.release]
  panic = "abort"
  codegen-units = 1
  lto = true
  opt-level = "z"
  strip = true
  ```
