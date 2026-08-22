# Testing Standards & Test Doubles Guide

Standardized conventions for automated testing in this repository.

---

## 🎯 Test Structure Convention (AAA Pattern)

Every test should follow the **Arrange, Act, Assert** pattern:
1. **Arrange**: Set up test fixtures, state objects, or mocks.
2. **Act**: Execute the function or trigger the user interaction.
3. **Assert**: Verify the expected output or state change.

---

## 📋 Quality Assurance Pre-Commit Checklist
- [ ] Do all tests pass via `bun test`?
- [ ] Do all Rust unit tests pass via `cargo test`?
- [ ] Does `bun run check` report 0 errors and 0 warnings?
- [ ] Have edge cases (empty inputs, null values, network timeouts, invalid paths) been covered?
