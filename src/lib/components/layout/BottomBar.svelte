<script lang="ts">
import { windowStore } from "$lib/stores/window.svelte";

interface Props {
  status?: string;
}

let { status = "Ready" }: Props = $props();
</script>

<footer
  aria-label="Application Status Bar"
  class="h-6.5 w-full bg-bg-panel border-t border-border-default select-none flex items-center justify-between px-3 z-30 relative shrink-0 text-[11px] font-mono text-text-muted transition-colors duration-150 shadow-xs"
>
  <!-- Left: Core Status & Workspace -->
  <div class="flex items-center gap-3">
    <!-- Active Status Indicator -->
    <div class="flex items-center gap-1.5 hover:text-text-hover transition-colors">
      <span class="w-1.5 h-1.5 rounded-full bg-[#00b578] animate-pulse"></span>
      <span class="font-medium text-text-default">{status}</span>
    </div>

    <!-- Divider -->
    <div class="h-3 w-px bg-border-default"></div>

    <!-- App / Environment Label -->
    <div class="flex items-center gap-1 text-text-muted">
      <span>Tauri v2</span>
      <span>•</span>
      <span>Svelte 5</span>
    </div>
  </div>

  <!-- Center: Context / Spacer -->
  <div class="flex-1"></div>

  <!-- Right: Dimensions & System Telemetry -->
  <div class="flex items-center gap-3">
    <!-- Window Resolution Indicator -->
    <div
      class="flex items-center gap-1 text-text-muted hover:text-text-hover transition-colors"
      title="Window Resolution"
    >
      <span>{windowStore.width}</span>
      <span class="text-text-muted">×</span>
      <span>{windowStore.height}</span>
    </div>

    <!-- Divider -->
    <div class="h-3 w-px bg-border-default"></div>

    <!-- UTF-8 / Standard Encoding -->
    <span class="text-text-muted">UTF-8</span>
  </div>
</footer>
