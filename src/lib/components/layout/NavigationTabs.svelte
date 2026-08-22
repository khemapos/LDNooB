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
  {
    id: "profiles",
    label: "Emulators",
    icon: "cube",
    badge: emulatorsStore.instances.length,
  },
  { id: "accounts", label: "Accounts", icon: "users" },
  { id: "workflows", label: "Workflows", icon: "sparkles" },
  { id: "inspector", label: "Hierarchy Inspector", icon: "eye" },
  { id: "settings", label: "Settings", icon: "settings" },
]);

// Sliding Indicator Pill State
let tabRefs: Record<string, HTMLButtonElement | null> = $state({});
let indicatorStyle = $state({ left: 4, width: 0 });

$effect(() => {
  const el = tabRefs[activeTab];
  if (el) {
    indicatorStyle = {
      left: el.offsetLeft,
      width: el.offsetWidth,
    };
  }
});
</script>

<div class="w-full flex items-center justify-between gap-3 select-none font-sans">
  <!-- Left: Main Navigation Tabs with Smooth Sliding Pill -->
  <div
    class="relative flex items-center gap-1.5 p-1 bg-bg-panel border border-border-default rounded-xl shadow-xs"
  >
    <!-- Smooth Animated Sliding Pill Background -->
    {#if indicatorStyle.width > 0}
      <div
        class="absolute top-1 bottom-1 rounded-lg bg-bg-card border border-border-default shadow-xs transition-all duration-200 cubic-bezier(0.16,1,0.3,1) pointer-events-none z-0"
        style="left: {indicatorStyle.left}px; width: {indicatorStyle.width}px;"
      ></div>
    {/if}

    {#each tabs as tab}
      {@const isActive = activeTab === tab.id}
      <button
        type="button"
        bind:this={tabRefs[tab.id]}
        onclick={() => onTabChange(tab.id)}
        class="relative z-10 flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-colors duration-150 cursor-pointer {isActive
          ? 'text-[#00b578]'
          : 'text-text-muted hover:text-text-hover'}"
      >
        <Icon name={tab.icon} size={14} />
        <span>{tab.label}</span>
        {#if tab.badge !== undefined && tab.badge > 0}
          <span
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono font-bold transition-colors duration-150 {isActive
              ? 'bg-[#00b578]/15 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
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
      class="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-bg-panel border border-border-default text-xs font-mono"
    >
      <span class="flex items-center gap-1.5 text-[#00b578] font-bold">
        <span class="w-2 h-2 rounded-full bg-[#00b578] animate-pulse"></span>
        {emulatorsStore.runningCount} Running
      </span>
      <span class="text-text-muted">•</span>
      <span class="text-text-muted">
        {emulatorsStore.runningCount * 2048} MB RAM
      </span>
    </div>

    <!-- Theme Mode Toggle -->
    <button
      type="button"
      title="Toggle Light/Dark Theme ({themeStore.current})"
      onclick={() => themeStore.toggle()}
      class="p-2 rounded-xl text-text-muted hover:text-[#ffc107] hover:bg-bg-card border border-border-default transition-colors cursor-pointer"
    >
      <Icon name={themeStore.current === "dark" ? "sun" : "moon"} size={14} />
    </button>
  </div>
</div>
