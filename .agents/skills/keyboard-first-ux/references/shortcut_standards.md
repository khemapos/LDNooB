# Keyboard Shortcut Standards & Focus Trapping Reference

Detailed guidelines for building accessible, keyboard-first desktop applications in Tauri and Svelte 5.

---

## 🎯 Best Practices for Key Handlers

1. **Avoid Interfering with Text Inputs**:
   * If a single-letter shortcut (e.g. `f` for search) is registered, make sure it is ignored when the active element is an `<input>`, `<textarea>`, or `contenteditable`.
   ```ts
   const target = event.target as HTMLElement;
   if (target?.tagName === 'INPUT' || target?.tagName === 'TEXTAREA' || target?.isContentEditable) {
     return;
   }
   ```

2. **Roving Tabindex for Lists and Tabs**:
   * For horizontal tabs or vertical command palettes, use Arrow keys (`ArrowLeft`/`ArrowRight` or `ArrowUp`/`ArrowDown`) to move focus seamlessly across items.

3. **Visual Keyboard Hints**:
   * Display subtle visual badges (e.g. `<kbd class="px-1.5 py-0.5 text-[10px] bg-white/10 rounded">⌘K</kbd>`) next to actions to teach users keyboard shortcuts organically.
