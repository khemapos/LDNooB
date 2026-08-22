# Room: Svelte 5 & Components

## Summary
The frontend layer is built using **Svelte 5 (SvelteKit)** configured in SPA mode with `@sveltejs/adapter-static`.

## Architecture & Conventions
- **Component-First**: All UI is broken down into atomic components in `src/lib/components/` and reusable base components in `src/lib/components/ui/`.
- **Svelte 5 Runes**:
  - Use `$state()` for reactive variables.
  - Use `$props()` with TypeScript interfaces for component props.
  - Use `$derived()` for computed reactive expressions.
  - Use `$bindable()` for two-way component bindings.
  - Use Snippets (`{@render snippet()}`) instead of legacy Svelte slots.
- **Pages & Routes**:
  - `src/routes/+layout.ts`: `export const prerender = true; export const ssr = false;` (ensures pure client-side SPA build for Tauri).
  - `src/routes/+layout.svelte`: Loads `src/app.css` and renders root children snippet.
  - `src/routes/+page.svelte`: Assembles the page from modular components.
