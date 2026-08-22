---
name: performance-profiler
description: >-
  Techniques, performance targets, and audit workflows for keeping Tauri v2 + Svelte 5 desktop apps
  blazingly fast (60+ FPS rendering, <200ms cold startup, <50MB RAM, zero UI thread blocking).
  Use this skill when optimizing bundles, auditing memory usage, or writing high-throughput IPC.
---

# Performance Profiling & Desktop Optimization Skill

High-performance desktop apps should feel instantaneous, lightweight on battery and memory, and silky-smooth during user interaction.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                        PERFORMANCE TARGET BUDGETS                        │
│                                                                          │
│  [ Cold Startup Time ]   ──> < 200 ms to interactive DOM                 │
│  [ Idle RAM Usage ]      ──> < 50 MB total process footprint             │
│  [ Frame Rate ]          ──> Steady 60 - 120 FPS during scrolling/drag   │
│  [ Rust IPC Latency ]    ──> < 5 ms for local command round-trip         │
│  [ Production Bundle ]   ──> < 10 MB total binary size                   │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## ⚡ Core Rules for Ultra-Fast Tauri Apps

### 1. Zero Blocking on the Frontend UI Thread
* Never run heavy synchronous computations, large file hashing, or blocking loops in the JavaScript runtime.
* Delegate computationally heavy tasks to **Rust asynchronous threads** via Tauri commands:
  ```rust
  #[tauri::command]
  pub async fn process_large_dataset(data: Vec<u8>) -> Result<ProcessedData, String> {
      tokio::task::spawn_blocking(move || {
          // Compute in thread pool without freezing UI
          heavy_compute(&data)
      }).await.map_err(|e| e.to_string())
  }
  ```

### 2. Svelte 5 Fine-Grained Reactivity
* `$state()` and `$derived()` in Svelte 5 use fine-grained signals. They only re-render the exact DOM node that changed, rather than entire component subtrees.
* Avoid creating new object literals inside render loops that force unneeded diffing.

### 3. CSS Hardware Acceleration
* Use `transform` and `opacity` for animations (`transition-transform`, `transition-opacity`) to leverage GPU compositing. Avoid animating `width`, `height`, `margin`, or `padding` directly.

---

## 📚 Deep References
- [Desktop Performance Checklist & Profiling Guide](./references/desktop_perf_checklist.md)
