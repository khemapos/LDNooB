<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import type { ActiveTab } from "$lib/types";
import Icon from "../ui/Icon.svelte";

interface Props {
  activeTab: ActiveTab;
  onTabChange: (tab: ActiveTab) => void;
  onOpenSettings?: () => void;
}

let { activeTab = $bindable("profiles"), onTabChange, onOpenSettings }: Props = $props();

const tabs: { id: ActiveTab; label: string; icon: any; badge?: number }[] = $derived([
  {
    id: "profiles",
    label: "Emulators",
    icon: "cube",
    badge: emulatorsStore.instances.length,
  },
  { id: "accounts", label: "Accounts", icon: "users" },
]);
</script>

<div class="w-full h-12 border-b border-border-default flex items-center justify-between px-4 select-none shrink-0 bg-bg-panel font-sans">
  <!-- Left Side: Main Navigation Tabs matching D:\ldremote -->
  <div class="flex items-center gap-1.5">
    {#each tabs as tab}
      {@const isActive = activeTab === tab.id}
      <button
        type="button"
        onclick={() => onTabChange(tab.id)}
        class="flex items-center gap-2 px-3.5 py-1.5 rounded-xl font-bold tracking-wide uppercase transition-all duration-200 cursor-pointer {isActive
          ? 'text-[#00b578] bg-[#00b578]/10'
          : 'text-text-muted hover:text-text-hover hover:bg-bg-card/30'}"
      >
        <Icon name={tab.icon} size={14} />
        <span>{tab.label}</span>
        {#if tab.badge !== undefined && tab.badge > 0}
          <span
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono font-bold transition-colors duration-150 {isActive
              ? 'bg-[#00b578]/20 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
          >
            {tab.badge}
          </span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Right Side: Telemetry Pill & Settings Button matching D:\ldremote -->
  <div class="flex items-center gap-2.5">
    <!-- Live Running Metrics Pill -->
    <div
      class="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-bg-card/40 border border-border-default text-xs font-mono select-none"
    >
      <span class="flex items-center gap-1.5 text-[#00b578] font-bold">
        <span class="w-2 h-2 rounded-full bg-[#00b578] animate-pulse"></span>
        {emulatorsStore.runningCount} Running
      </span>
      <span class="text-text-muted opacity-60">•</span>
      <span class="text-text-muted">
        {emulatorsStore.runningCount * 2048} MB RAM
      </span>
    </div>

    <!-- Application Settings Button matching D:\ldremote lines 1388-1415 -->
    <button
      type="button"
      onclick={() => onOpenSettings?.()}
      class="flex flex-col items-center justify-center gap-0.5 px-3 py-1 rounded-xl cursor-pointer transition-all duration-200 select-none bg-bg-card/50 hover:bg-bg-card-hover text-text-muted hover:text-text-hover border border-border-default hover:border-border-hover shadow-[0_2px_6px_rgba(0,0,0,0.05)] hover:shadow-[0_4px_12px_rgba(0,0,0,0.1)] active:shadow-[0_1px_3px_rgba(0,0,0,0.05)] h-[38px] min-w-[56px] group active:scale-95"
      title="Application Settings"
    >
      <Icon
        name="settings"
        size={15}
        class="text-text-muted group-hover:text-text-hover transition-transform group-hover:rotate-45 duration-300"
      />
      <span class="text-[9px] font-semibold tracking-wide leading-none">
        Settings
      </span>
    </button>
  </div>
</div>
