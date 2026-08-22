# ADR-002: Component-Driven Svelte 5 Runes

## Status
Accepted

## Context
Svelte 5 introduced the Runes system (`$state`, `$props`, `$derived`, `$bindable`, `$effect`), moving away from legacy `let` declarations and `$$props` / `export let`.

## Decision
All components in `src/lib/components/` must use standard Svelte 5 Runes syntax:
- Explicit typed interfaces for props using `$props()`.
- Two-way bindings with `$bindable()`.
- Component compositions using Snippets (`{@render snippet()}`).
- No legacy Svelte 3/4 stores or syntax when building new components.

## Consequences
- Full TypeScript type safety for component inputs and outputs.
- High performance and fine-grained reactivity in desktop webviews.
