# Architectural Trade-Off Analysis Matrix

When choosing between architectural options, structure evaluations using the following multidimensional matrix:

---

## Trade-Off Decision Matrix Template

| Evaluation Dimension | Option 1: [Approach Name] | Option 2: [Approach Name] | Option 3: [Approach Name] |
| :--- | :--- | :--- | :--- |
| **Cognitive Complexity** | Low / Med / High | Low / Med / High | Low / Med / High |
| **Runtime Performance** | Speed, memory footprint | Speed, memory footprint | Speed, memory footprint |
| **Type Safety** | End-to-end typing fidelity | End-to-end typing fidelity | End-to-end typing fidelity |
| **Error Blast Radius** | Localized vs Cascading | Localized vs Cascading | Localized vs Cascading |
| **Implementation Effort** | Immediate dev time | Immediate dev time | Immediate dev time |
| **Long-Term Debt** | Maintenance overhead | Maintenance overhead | Maintenance overhead |

---

## 4-Step Decision Heuristic
1. **Eliminate Non-Viable Options**: Remove any option violating core invariants or workspace rules (`AGENTS.md`).
2. **Weigh Reversibility**: Prefer decisions that are easy to reverse ("Two-Way Doors") over irreversible ("One-Way Doors") commitments.
3. **Optimize for the Primary Bottleneck**: Identify whether latency, maintainability, or implementation speed is the actual bottleneck.
4. **Document Rationale**: Record why the winning option was chosen in an ADR inside `.mempalace/wings/decisions/`.
