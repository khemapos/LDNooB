# DRY Rules & Anti-Patterns Reference

This reference catalogs common code smells, anti-patterns, and the corresponding DRY refactorings for our Tauri + Svelte 5 + Tailwind v4 stack.

---

## 🚫 Common DRY Violations & Fixes

### 1. Inlined Window / Dialog Actions
* ❌ **Violation**: Re-writing maximize/minimize/close handlers or double-click drag regions in multiple page headers.
* ✅ **Fix**: Centralize titlebar and window lifecycle management inside `WindowBar.svelte`.

---

### 2. Copy-Pasting Tailwind Class Strings
* ❌ **Violation**:
  ```svelte
  <button class="px-4 py-2 bg-titanium-800 text-silver-100 rounded-xl border border-titanium-700 hover:bg-titanium-750 transition-all">Submit</button>
  <button class="px-4 py-2 bg-titanium-800 text-silver-100 rounded-xl border border-titanium-700 hover:bg-titanium-750 transition-all">Cancel</button>
  ```
* ✅ **Fix**:
  ```svelte
  <Button variant="secondary">Submit</Button>
  <Button variant="secondary">Cancel</Button>
  ```

---

### 3. Duplicated Vector SVGs
* ❌ **Violation**: Inlining identical `<svg viewBox="0 0 16 16">...</svg>` across 5 different components.
* ✅ **Fix**: Add the icon to `<Icon name="iconName" />` in `src/lib/components/ui/Icon.svelte`.

---

### 4. Redundant Derived State Syncing
* ❌ **Violation**: Using `$effect()` to recalculate a value whenever another state changes.
* ✅ **Fix**: Use `$derived()` or `$derived.by()` which compute lazily and automatically track dependencies without triggering extra render cycles.

---

## 📋 DRY Pre-Commit Checklist
- [ ] Are all recurring UI controls using `src/lib/components/ui/` primitives?
- [ ] Are all icons referenced via `<Icon name="..." />` instead of hardcoded SVG strings or emojis?
- [ ] Are computed values using `$derived()` instead of manual synchronization effects?
- [ ] Are shared TypeScript types imported from a single source?
- [ ] Has unnecessary premature abstraction been avoided (Rule of Three)?
