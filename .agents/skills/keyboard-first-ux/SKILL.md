---
name: keyboard-first-ux
description: >-
  Standards, interaction patterns, and keybinding management for crafting elite keyboard-first
  desktop user experiences (Command Palette ⌘K/Ctrl+K, global hotkeys, focus trapping, ARIA
  roving tabindex). Use this skill when implementing desktop shortcuts, modal dialogs, or power-user workflows.
---

# Keyboard-First Desktop UX Skill

Desktop software feels truly pro and lightning-fast when users can navigate, execute commands, and control every feature entirely via the keyboard.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                       KEYBOARD INTERACTION HIERARCHY                    │
│                                                                         │
│  [ Level 1: Global Shortcuts ]     ──> ⌘K / Ctrl+K (Palette), ⌘, (Pref) │
│  [ Level 2: Contextual Navigation] ──> Tab / Shift+Tab, Arrow Keys      │
│  [ Level 3: Modal Esc & Dismiss ]  ──> Escape closes overlays/popovers  │
│  [ Level 4: Action Triggers ]      ──> Enter / Space on active items    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## ⌨️ Desktop Keyboard Conventions

| Action | macOS Shortcut | Windows / Linux Shortcut |
| :--- | :--- | :--- |
| **Command Palette** | `⌘ + K` | `Ctrl + K` |
| **New Tab / Workspace** | `⌘ + T` | `Ctrl + T` |
| **Close Tab / Overlay** | `⌘ + W` / `Esc` | `Ctrl + W` / `Esc` |
| **Switch Tabs** | `⌘ + 1..9` / `Ctrl + Tab` | `Ctrl + 1..9` / `Ctrl + Tab` |
| **Settings / Preferences**| `⌘ + ,` | `Ctrl + ,` |
| **Toggle Sidebar** | `⌘ + B` | `Ctrl + B` |

---

## 🛠️ Implementation Standards in Svelte 5

1. **Window Event Listeners with Cleanup**:
   ```svelte
   <script lang="ts">
     function handleKeydown(event: KeyboardEvent) {
       const isMac = navigator.platform.toUpperCase().includes('MAC');
       const modKey = isMac ? event.metaKey : event.ctrlKey;

       if (modKey && event.key.toLowerCase() === 'k') {
         event.preventDefault();
         togglePalette();
       } else if (event.key === 'Escape') {
         closePalette();
       }
     }
   </script>

   <svelte:window onkeydown={handleKeydown} />
   ```

2. **Focus Trapping in Modals**:
   * Modals must auto-focus the primary input or first interactive button on mount.
   * `Escape` must close any active modal overlay.

---

## 📚 Deep References
- [Desktop Shortcut Standards & Focus Trapping Reference](./references/shortcut_standards.md)
