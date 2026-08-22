<script lang="ts">
import { onMount } from "svelte";
import { themeStore } from "$lib/stores/theme.svelte";
import { windowStore } from "$lib/stores/window.svelte";
import Icon from "../ui/Icon.svelte";

interface Props {
  title?: string;
}

let { title = "LDNooB" }: Props = $props();

onMount(() => {
  windowStore.init();
});

function handleMinimize(event?: MouseEvent) {
  event?.stopPropagation();
  windowStore.minimize();
}

function handleToggleMaximize(event?: MouseEvent) {
  event?.stopPropagation();
  windowStore.toggleMaximize();
}

function handleClose(event?: MouseEvent) {
  event?.stopPropagation();
  windowStore.close();
}

function toggleTheme(event?: MouseEvent) {
  event?.stopPropagation();
  themeStore.toggle();
}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<header
  data-tauri-drag-region
  aria-label="Desktop Window Title Bar"
  ondblclick={handleToggleMaximize}
  class="h-10 w-full bg-white/90 dark:bg-[#0c0d13]/95 border-b border-slate-200/80 dark:border-white/[0.06] backdrop-blur-2xl select-none flex items-center justify-between px-3 z-40 relative shrink-0 transition-colors duration-150 shadow-[0_1px_2px_rgba(0,0,0,0.02)] dark:shadow-[0_1px_0_rgba(255,255,255,0.03)]"
>
  <!-- Left: Branding & Status -->
  <div data-tauri-drag-region class="flex items-center gap-2.5 shrink-0">
    <!-- App Logo Icon -->
    <div
      class="w-6 h-6 rounded-lg bg-gradient-to-tr from-cyan-400 via-blue-500 to-indigo-600 flex items-center justify-center shadow-[0_0_12px_rgba(6,182,212,0.3)] ring-1 ring-black/5 dark:ring-white/20"
    >
      <Icon name="cube" size={13} class="text-white drop-shadow" />
    </div>

    <!-- App Name -->
    <div class="flex items-center gap-2">
      <span
        class="text-xs font-bold tracking-tight text-slate-850 dark:text-slate-100 font-mono"
      >
        {title}
      </span>
    </div>
  </div>

  <!-- Center: Clean Drag Region -->
  <div data-tauri-drag-region class="flex-1 h-full"></div>

  <!-- Right: Actions & Window Controls -->
  <div class="flex items-center gap-1.5 shrink-0">
    <!-- Theme Switcher Button -->
    <button
      type="button"
      title={themeStore.current === "dark"
        ? "Switch to Light Mode"
        : "Switch to Dark Mode"}
      onclick={toggleTheme}
      class="w-7 h-7 rounded-md flex items-center justify-center text-slate-500 dark:text-slate-400 hover:text-amber-500 dark:hover:text-amber-300 hover:bg-slate-100 dark:hover:bg-white/[0.06] active:bg-slate-200 dark:active:bg-white/[0.10] transition-colors cursor-pointer"
    >
      <Icon name={themeStore.current === "dark" ? "sun" : "moon"} size={13} />
    </button>

    <!-- Divider -->
    <div class="h-4 w-px bg-slate-200 dark:bg-white/[0.08] mx-0.5"></div>

    <!-- Window Buttons Group -->
    <div class="flex items-center gap-0.5">
      <!-- Minimize -->
      <button
        type="button"
        title="Minimize"
        onclick={handleMinimize}
        class="w-8 h-7 rounded-md flex items-center justify-center text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-white/[0.06] active:bg-slate-200 dark:active:bg-white/[0.10] transition-colors cursor-pointer"
      >
        <Icon name="minimize" size={12} />
      </button>

      <!-- Maximize / Restore -->
      <button
        type="button"
        title={windowStore.isMaximized ? "Restore" : "Maximize"}
        onclick={handleToggleMaximize}
        class="w-8 h-7 rounded-md flex items-center justify-center text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-white/[0.06] active:bg-slate-200 dark:active:bg-white/[0.10] transition-colors cursor-pointer"
      >
        <Icon
          name={windowStore.isMaximized ? "restore" : "maximize"}
          size={12}
        />
      </button>

      <!-- Close -->
      <button
        type="button"
        title="Close"
        onclick={handleClose}
        class="w-8 h-7 rounded-md flex items-center justify-center text-slate-500 dark:text-slate-400 hover:text-white hover:bg-rose-600 active:bg-rose-700 transition-colors cursor-pointer"
      >
        <Icon name="close" size={12} />
      </button>
    </div>
  </div>
</header>
