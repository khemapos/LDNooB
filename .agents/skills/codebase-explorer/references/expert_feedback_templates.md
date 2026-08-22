# Expert Architectural Feedback & Suggestion Templates

When presenting codebase exploration findings, use this structured, senior-staff format to communicate actionable insight with maximum clarity, authority, and empathy.

---

## 📋 Comprehensive Codebase Review Template

```markdown
# 🏛️ Architectural Audit & Source Code Deep Dive

## 1. Executive Summary
Brief high-level assessment of the audited subsystem or codebase.
- **Overall Architecture Health**: [🟢 Robust / 🟡 Moderate / 🔴 At Risk]
- **Key Strengths**: [List 2-3 notable architectural successes]
- **Primary Bottlenecks**: [List 1-2 urgent focus areas]

---

## 2. Findings & Risk Matrix

| Finding | Component / File | Severity | Impact | Recommendation |
| :--- | :--- | :---: | :--- | :--- |
| **State Desync on Resize** | [`window.svelte.ts`](file:///...) | 🟠 P1 | Window metrics drift out of sync | Add debounce + scale listener |
| **Unprotected Drag Region** | [`WindowBar.svelte`](file:///...) | 🟡 P2 | Clicks occasionally treated as drags | Add `stopPropagation` on controls |

---

## 3. First-Principles Deep Dive

### Finding A: [Title of Finding]
- **Location**: [`src/...`](file:///...)
- **Mechanism**: Explain *why* the issue occurs at the mechanical / runtime level.
- **Consequences**: What fails under edge-case conditions (high load, fast clicks, network drop, process termination).

```typescript
// Problematic pattern
...
```

- **Root-Cause Architectural Critique**:
Explain why the existing pattern violates Clean Architecture, SOLID, or performance targets.

---

## 4. Proposed Architectural Evolution & Code Blueprint

### High-Conviction Solution
Step-by-step remediation plan with drop-in code diffs.

```diff
- old_fragile_code()
+ new_robust_architecture()
```

---

## 5. Trade-Offs & Strategic Alternatives

| Approach | Advantages | Disadvantages | Recommendation |
| :--- | :--- | :--- | :--- |
| **Approach 1 (Proposed)** | Type-safe, Zero UI blocking | Requires small IPC schema update | **Recommended** |
| **Approach 2 (Status Quo)** | Zero changes now | Accumulates technical debt | Avoid |

---

## 6. Actionable Implementation Roadmap
1. [ ] **Step 1 (Immediate)**: Apply P0/P1 fixes.
2. [ ] **Step 2 (Structural)**: Refactor shared stores into Svelte 5 modules.
3. [ ] **Step 3 (Validation)**: Verify with `bun run check` and automated unit tests.
```
