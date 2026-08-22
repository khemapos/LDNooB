# ADR-001: Pure Tailwind CSS v4 & Zero Custom CSS

## Status
Accepted

## Context
Traditional CSS and scoped `<style>` blocks in Svelte can lead to style fragmentation and maintenance overhead across desktop UI components. Tailwind CSS v4 provides a zero-config, lightning-fast CSS engine integrated directly into Vite.

## Decision
All styling must be expressed through Tailwind CSS v4 utility classes. No `<style>` blocks or ad-hoc custom CSS classes should be introduced in `.svelte` components.

## Consequences
- Fast, consistent styling and easy theming.
- Components are fully self-contained and easily portable.
