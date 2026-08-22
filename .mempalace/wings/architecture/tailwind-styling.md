# Room: Tailwind CSS v4 Engine

## Summary
Styling is powered by **Tailwind CSS v4** via the `@tailwindcss/vite` plugin.

## Rules & Conventions
- **Zero Custom CSS**: No `<style>` tags or inline styles are used anywhere in `.svelte` or `.html` files.
- **Entrypoint**: Single stylesheet `src/app.css` containing only `@import "tailwindcss";`.
- **Vite Integration**: `@tailwindcss/vite` registered in `vite.config.js` before `sveltekit()`.
- **Design Tokens**: Rely on modern Tailwind v4 utilities, CSS variables, dark themes, and opacity modifiers (`bg-slate-900/60`, `backdrop-blur-xl`, `border-slate-800/80`).
