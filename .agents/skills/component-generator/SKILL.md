---
name: component-generator
description: >-
  Standardized workflow and template for generating new Svelte 5 components with pure Tailwind CSS v4.
  Use this skill whenever scaffolding new UI components, cards, forms, or views in the project.
---

# Component Generator Skill

All new components must adhere to the **Svelte 5 Runes** and **Zero Custom CSS** architectural standards.

## Component Standards
1. **Location**:
   - Primitive UI elements: `src/lib/components/ui/<ComponentName>.svelte`
   - Feature blocks & composite sections: `src/lib/components/<ComponentName>.svelte`
2. **Reactivity**:
   - Use `$props()` with an explicit TypeScript interface `Props`.
   - Use `$bindable()` for two-way bindings.
   - Use `$state()` for local component state.
   - Use `$derived()` for computed states.
   - Use Snippets (`{@render snippet()}`) for slot-like projection.
3. **Styling**:
   - 100% Tailwind CSS v4 utility classes.
   - No `<style>` tags or custom CSS.

## Standard Component Template
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    subtitle?: string;
    variant?: 'primary' | 'secondary';
    children?: Snippet;
    class?: string;
  }

  let {
    title,
    subtitle,
    variant = 'primary',
    children,
    class: className = ''
  }: Props = $props();
</script>

<div class="rounded-2xl border border-slate-800 bg-slate-900/60 p-6 backdrop-blur-xl transition-all duration-200 {className}">
  <h3 class="text-base font-semibold text-slate-100">{title}</h3>
  {#if subtitle}
    <p class="text-xs text-slate-400 mt-1">{subtitle}</p>
  {/if}
  {#if children}
    <div class="mt-4">
      {@render children()}
    </div>
  {/if}
</div>
```
