<script lang="ts">
import { windowStore } from "$lib/stores/window.svelte";
import Icon from "../ui/Icon.svelte";

interface Props {
  status?: string;
}

let { status = "Ready" }: Props = $props();
</script>

<footer
  aria-label="Application Status Bar"
  class="h-6.5 w-full bg-white/90 dark:bg-[#0c0d13]/95 border-t border-slate-200/80 dark:border-white/[0.06] backdrop-blur-xl select-none flex items-center justify-between px-3 z-30 relative shrink-0 text-[11px] font-mono text-slate-500 dark:text-slate-400 transition-colors duration-150 shadow-[0_-1px_2px_rgba(0,0,0,0.01)] dark:shadow-[0_-1px_0_rgba(255,255,255,0.02)]"
>
  <!-- Left: Core Status & Workspace -->
  <div class="flex items-center gap-3">
    <!-- Active Status Indicator -->
    <div class="flex items-center gap-1.5 hover:text-slate-800 dark:hover:text-slate-200 transition-colors">
      <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 dark:bg-emerald-400 animate-pulse"></span>
      <span class="font-medium text-slate-700 dark:text-slate-300">{status}</span>
    </div>

    <!-- Divider -->
    <div class="h-3 w-px bg-slate-200 dark:bg-white/[0.08]"></div>

    <!-- App / Environment Label -->
    <div class="flex items-center gap-1 text-slate-400 dark:text-slate-500">
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
      class="flex items-center gap-1 text-slate-400 dark:text-slate-500 hover:text-slate-700 dark:hover:text-slate-300 transition-colors"
      title="Window Resolution"
    >
      <span>{windowStore.width}</span>
      <span class="text-slate-300 dark:text-slate-600">×</span>
      <span>{windowStore.height}</span>
    </div>

    <!-- Divider -->
    <div class="h-3 w-px bg-slate-200 dark:bg-white/[0.08]"></div>

    <!-- UTF-8 / Standard Encoding -->
    <span class="text-slate-400 dark:text-slate-500">UTF-8</span>
  </div>
</footer>
