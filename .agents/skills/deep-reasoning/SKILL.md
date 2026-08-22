---
name: deep-reasoning
description: >-
  Systematic cognitive framework for critical thinking, first-principles problem deconstruction,
  deep architectural analysis, trade-off evaluation, and pre-mortem risk assessment.
  Use this skill whenever tackling complex features, non-trivial refactorings, elusive bugs,
  concurrency/IPC challenges, or designing systems with high architectural impact.
---

# Deep Reasoning & Critical Thinking Skill

This skill guides agents through a rigorous, multi-stage cognitive process to prevent premature convergence, identify blind spots, and deliver robust, optimal technical implementations.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    CRITICAL THINKING COGNITIVE LOOP                     │
│                                                                         │
│  [1. Deconstruct] ──> [2. Diverge] ──> [3. Trade-offs] ──> [4. Pre-Mortem]
│                                                                  │      │
│  [6. Reflect & Record] <───────── [5. Execute & Verify] <────────┘      │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 🧠 The 6-Phase Deep Thinking Protocol

### Phase 1: Problem Deconstruction & First-Principles Analysis
Before writing any code or proposing solutions:
1. **Strip Down to Fundamentals**: What is the irreducible core problem we are trying to solve?
2. **Challenge Hidden Assumptions**:
   - What are we assuming about the environment, data flow, or user behavior that might not be true?
   - Is this constraint real, or is it an artifact of past decisions?
3. **Define Invariants & Success Metrics**: What non-negotiable properties must hold true before and after execution (e.g., type safety, memory stability, zero custom CSS, UI responsiveness)?

### Phase 2: Divergent Solution Exploration
Avoid settling on the first obvious idea. Formulate at least **2–3 distinct architectural paths**:
- **Option A (Direct / Minimalist)**: Lowest complexity, immediate payoff.
- **Option B (Idiomatic / Architecture-First)**: Long-term modularity, extensibility, strict separation of concerns.
- **Option C (Alternative / Out-of-the-Box)**: Exploring unconventional paradigms (e.g., reactive event bus vs explicit props vs Rust worker thread).

### Phase 3: Second-Order Effects & Trade-Off Matrix
Evaluate the options across critical dimensions:
| Dimension | Key Questions |
| :--- | :--- |
| **Complexity & Cognition** | Does this add cognitive load for future developers or agents? |
| **Performance & Memory** | What is the runtime overhead in Webview/Rust? Are there memory leaks or redundant re-renders? |
| **Resilience & Failure Recovery** | How does this fail? Is failure graceful or catastrophic? |
| **Maintainability** | Does this adhere to existing project rules (Pure Svelte 5 runes, Tailwind v4, Bun)? |

### Phase 4: Pre-Mortem & Boundary Stress Testing
Assume the implementation was deployed and failed catastrophically. Work backwards:
1. **Edge Cases**:
   - Empty, null, extreme, or malformed inputs.
   - Rapid user interactions (debouncing, race conditions, double-clicks).
2. **IPC & Serialization Boundaries**:
   - What happens if the Tauri backend errors out or takes too long?
   - Are types 100% aligned between Rust `serde` structs and TypeScript interfaces?
3. **Reactivity Gotchas**:
   - Are Svelte 5 Runes states (`$state`, `$derived`, `$props`) properly tracked without unwanted side effects or infinite loops?

### Phase 5: Disciplined Implementation & Verification
1. **Execute in verified increments**: Implement foundational types and logic before UI wiring.
2. **Zero-Assumption Testing**: Validate using automated diagnostics (`bun run check`, `cargo check`, `bun run build`).
3. **Verify Edge Behavior**: Explicitly test error states, loading spinners, and network/IPC fallbacks.

### Phase 6: Retrospective & Palace Recording
1. **Capture Insights**: What non-obvious lessons or edge cases were discovered?
2. **Update MemPalace**:
   - If a significant architectural choice was made, record an ADR in `.mempalace/wings/decisions/`.
   - Log the milestone in `.mempalace/wings/diary/journal.md` (`bun run palace:log "<milestone>"`).

---

## 📚 Deep Reference Guides
- [Cognitive Checklists](./references/cognitive_checklists.md)
- [Trade-off Matrix Template](./references/tradeoff_matrices.md)
