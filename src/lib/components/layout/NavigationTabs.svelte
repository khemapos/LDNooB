<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { themeStore } from "$lib/stores/theme.svelte";
import type { ActiveTab } from "$lib/types";
import Icon from "../ui/Icon.svelte";

interface Props {
  activeTab: ActiveTab;
  onTabChange: (tab: ActiveTab) => void;
}

let { activeTab = $bindable("profiles"), onTabChange }: Props = $props();

const tabs: { id: ActiveTab; label: string; icon: any; badge?: number }[] = $derived([
  { id: "profiles", label: "Emulators", icon: "cube", badge: emulatorsStore.instances.length },
  { id: "accounts", label: "Accounts", icon: "users" },
  { id: "workflows", label: "Workflows", icon: "sparkles" },
  { id: "inspector", label: "Hierarchy Inspector", icon: "eye" },
  { id: "settings", label: "Settings", icon: "settings" },
]);
</script>

<div class="w-full flex items-center justify-between gap-3 select-none">
  <!-- Left: Main Navigation Tabs -->
  <div
    class="flex items-center gap-1.5 p-1 bg-slate-200/60 dark:bg-white/[0.04] border border-slate-300/70 dark:border-white/[0.06] rounded-xl shadow-xs"
  >
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => onTabChange(tab.id)}
        class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold transition-all duration-150 cursor-pointer {activeTab ===
        tab.id
          ? 'bg-white dark:bg-[#161824] text-emerald-600 dark:text-emerald-400 shadow-xs border border-slate-200 dark:border-white/[0.08]'
          : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200 hover:bg-white/40 dark:hover:bg-white/[0.03]'}"
      >
        <Icon name={tab.icon} size={14} />
        <span>{tab.label}</span>
        {#if tab.badge !== undefined && tab.badge > 0}
          <span
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono font-bold {activeTab === tab.id
              ? 'bg-emerald-500/20 text-emerald-600 dark:text-emerald-400'
              : 'bg-slate-200 dark:bg-white/10 text-slate-500 dark:text-slate-400'}"
          >
            {tab.badge}
          </span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Right: Real-time Telemetry Pill & Quick Controls -->
  <div class="flex items-center gap-2">
    <!-- Live Running Metrics Pill -->
    <div
      class="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-slate-100 dark:bg-[#0c0e15] border border-slate-200 dark:border-white/[0.06] text-xs font-mono"
    >
      <span class="flex items-center gap-1.5 text-emerald-600 dark:text-emerald-400 font-bold">
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
        {emulatorsStore.runningCount} Running
      </span>
      <span class="text-slate-300 dark:text-slate-600">•</span>
      <span class="text-slate-500 dark:text-slate-400">
        {emulatorsStore.runningCount * 2048} MB RAM
      </span>
    </div>

    <!-- Theme Mode Toggle -->
    <button
      type="button"
      title="Toggle Light/Dark Theme"
      onclick={() => themeStore.toggle()}
      class="p-2 rounded-xl text-slate-500 dark:text-slate-400 hover:text-amber-500 dark:hover:text-amber-300 hover:bg-slate-100 dark:hover:bg-white/[0.06] border border-slate-200 dark:border-white/[0.06] transition-colors cursor-pointer"
    >
      <Icon name={themeStore.current === "dark" ? "sun" : "moon"} size={14} />
    </button>
  </div>
</div>
