---
name: tauri-dev
description: >-
  Workflows and standards for developing, configuring, and building Tauri v2 desktop applications.
  Use this skill when adding or modifying Rust Tauri commands, managing security capabilities,
  configuring desktop window properties, or building desktop release packages.
---

# Tauri v2 Development Skill

## Overview
This project uses Tauri v2 with a SvelteKit SPA frontend and Rust backend.

## 1. Adding New Rust Commands
1. Define the command in `src-tauri/src/lib.rs` (or a dedicated Rust module):
   ```rust
   #[tauri::command]
   fn my_command(param: String) -> Result<String, String> {
       Ok(format!("Processed: {}", param))
   }
   ```
2. Register the command in `tauri::generate_handler!`:
   ```rust
   .invoke_handler(tauri::generate_handler![greet, my_command])
   ```
3. Call from Svelte 5 components:
   ```svelte
   <script lang="ts">
     import { invoke } from '@tauri-apps/api/core';

     async function callCommand() {
       const res = await invoke('my_command', { param: 'value' });
     }
   </script>
   ```

## 2. Capabilities & Permissions
In Tauri v2, permissions are explicitly scoped. Check `src-tauri/capabilities/` or `tauri.conf.json` when adding new plugins or API accesses.

## 3. Running & Building with Bun
- Dev mode: `bun run tauri dev`
- Production installer build: `bun run tauri build`
