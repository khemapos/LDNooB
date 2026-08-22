# Cognitive Checklists for Critical Thinking

Use these checklists during code design, implementation, and review.

---

## 1. Problem Definition & Invariants
- [ ] Have I stated the problem in a single crisp sentence without solution bias?
- [ ] Have I identified the root cause instead of treating a symptom?
- [ ] What system invariants must never be violated under any circumstances?
- [ ] Have I confirmed all requirements against project rules (`AGENTS.md`)?

---

## 2. Architecture & Design Evaluation
- [ ] Does this design introduce unnecessary abstractions or over-engineering (YAGNI)?
- [ ] Is state localized to where it is needed, or unnecessarily globalized?
- [ ] Are dependencies explicit, decoupled, and easy to unit test?
- [ ] Is there clear separation between data fetching, business logic, and UI presentation?

---

## 3. Concurrency & Asynchronous Safety
- [ ] What happens if an async request resolves after the component has unmounted?
- [ ] Are race conditions possible if the user clicks a button multiple times in rapid succession?
- [ ] Are in-flight requests cancelled or debounced where appropriate?
- [ ] Are errors caught at boundary points with user-facing recovery mechanisms?

---

## 4. UI & Reactivity (Svelte 5 + Tailwind v4)
- [ ] Are all reactive states declared using Svelte 5 runes (`$state`, `$derived`, `$props`)?
- [ ] Is there zero custom CSS or `<style>` blocks?
- [ ] Are layout shifts (CLS) prevented with proper sizing/skeletons?
- [ ] Does the UI handle empty, loading, error, and overflowing data gracefully?

---

## 5. Native Desktop & Tauri IPC Boundary
- [ ] Is Rust serde deserialization safe against unexpected or missing frontend fields?
- [ ] Are permissions explicitly declared in Tauri v2 capabilities?
- [ ] Is memory consumption monitored for long-lived native processes?
