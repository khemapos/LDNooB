<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { logsStore } from "$lib/stores/logs.svelte";
import { windowStore } from "$lib/stores/window.svelte";
import Icon from "../ui/Icon.svelte";

interface Props {
  status?: string;
}

let { status = "Ready" }: Props = $props();
</script>

<footer
  aria-label="Application Status Bar"
  class="h-7 w-full bg-bg-panel border-t border-border-default select-none flex items-center justify-between px-3 z-30 relative shrink-0 text-[11px] font-mono text-text-muted transition-colors duration-150 shadow-xs"
>
  <!-- Left: Core Status & Workspace -->
  <div class="flex items-center gap-3">
    <!-- Active Status Indicator -->
    <div class="flex items-center gap-1.5 hover:text-text-hover transition-colors font-medium">
      <span class="w-1.5 h-1.5 rounded-full bg-[#00b578] animate-pulse"></span>
      <span class="text-text-default">{status}</span>
    </div>

    <!-- Divider -->
    <div class="h-3 w-px bg-border-default"></div>

    <!-- Framework Info -->
    <div class="flex items-center gap-1 text-[10px] text-text-muted">
      <span>Tauri v2</span>
      <span>•</span>
      <span>Svelte 5</span>
    </div>
  </div>

  <!-- Center: Live Total Resource & Performance Usage -->
  <div class="absolute left-1/2 -translate-x-1/2 flex items-center gap-2 select-none">
    <span
      class="w-1.5 h-1.5 rounded-full {emulatorsStore.runningCount > 0
        ? 'bg-[#00b578] animate-pulse'
        : 'bg-zinc-600'}"
    ></span>
    <span class="text-[10px] text-text-muted font-bold uppercase tracking-wider">
      Fleet Usage:
    </span>
    <span
      class="px-2 py-0.5 rounded-lg font-mono font-bold text-[10px] border bg-[#00b578]/10 border-[#00b578]/20 text-[#00b578]"
    >
      Running: {emulatorsStore.runningCount}
    </span>
    <span
      class="px-2 py-0.5 rounded-lg font-mono font-bold text-[10px] border bg-bg-card border-border-default text-text-default"
    >
      RAM: {emulatorsStore.runningCount * 2048} MB
    </span>
  </div>

  <!-- Right: Activity Log & Window Telemetry -->
  <div class="flex items-center gap-2.5">
    <!-- Activity Log Toggle Button -->
    <button
      type="button"
      onclick={() => logsStore.togglePanel()}
      class="flex items-center gap-1.5 px-2 py-0.5 rounded-md cursor-pointer transition-all duration-150 font-bold select-none text-[10.5px] border {logsStore.isPanelOpen
        ? 'bg-[#00b578]/10 text-[#00b578] border-[#00b578]/25 shadow-xs'
        : 'bg-bg-card border-border-default text-text-muted hover:text-text-hover hover:border-border-hover'}"
      title="Toggle Activity Log (Ctrl+B)"
    >
      <span class="relative flex h-1.5 w-1.5">
        {#if logsStore.isPanelOpen}
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#00b578] opacity-75"></span>
        {/if}
        <span
          class="relative inline-flex rounded-full h-1.5 w-1.5 {logsStore.isPanelOpen
            ? 'bg-[#00b578]'
            : 'bg-zinc-500'}"
        ></span>
      </span>
      <Icon name="terminal" size={12} />
      <span>Activity Log ({logsStore.entries.length})</span>
      <kbd class="px-1 py-0.2 rounded text-[9px] bg-bg-app border border-border-default font-mono">
        Ctrl+B
      </kbd>
    </button>

    <!-- Divider -->
    <div class="h-3 w-px bg-border-default"></div>

    <!-- Window Resolution Indicator -->
    <div
      class="flex items-center gap-1 text-text-muted hover:text-text-hover transition-colors font-mono text-[10px]"
      title="Window Resolution"
    >
      <span>{windowStore.width}</span>
      <span class="text-text-muted">×</span>
      <span>{windowStore.height}</span>
    </div>
  </div>
</footer>
