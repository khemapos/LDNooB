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

<div class="w-full h-9 border-b border-border-default flex items-center justify-between px-3 select-none shrink-0 bg-bg-panel font-sans">
  <!-- Left Side: Compact Navigation Tabs -->
  <div class="flex items-center gap-1">
    {#each tabs as tab}
      {@const isActive = activeTab === tab.id}
      <button
        type="button"
        onclick={() => onTabChange(tab.id)}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-[11px] font-bold tracking-wide uppercase transition-all duration-150 cursor-pointer {isActive
          ? 'text-[#00b578] bg-[#00b578]/10'
          : 'text-text-muted hover:text-text-hover hover:bg-bg-card/40'}"
      >
        <Icon name={tab.icon} size={13} />
        <span>{tab.label}</span>
        {#if tab.badge !== undefined && tab.badge > 0}
          <span
            class="px-1.5 py-0.2 rounded-full text-[9.5px] font-mono font-bold transition-colors duration-150 {isActive
              ? 'bg-[#00b578]/20 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
          >
            {tab.badge}
          </span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Right Side: Telemetry Pill & Compact Settings Button -->
  <div class="flex items-center gap-2">
    <!-- Live Running Metrics Pill -->
    <div
      class="flex items-center gap-1.5 px-2.5 py-0.5 rounded-lg bg-bg-card/40 border border-border-default text-[10.5px] font-mono select-none"
    >
      <span class="flex items-center gap-1.5 text-[#00b578] font-bold">
        <span class="w-1.5 h-1.5 rounded-full bg-[#00b578] animate-pulse"></span>
        {emulatorsStore.runningCount} Running
      </span>
      <span class="text-text-muted opacity-50">•</span>
      <span class="text-text-muted">
        {emulatorsStore.runningCount * 2048} MB RAM
      </span>
    </div>

    <!-- Compact Application Settings Button -->
    <button
      type="button"
      onclick={() => onOpenSettings?.()}
      class="flex items-center gap-1.5 px-2.5 h-6.5 rounded-lg cursor-pointer transition-all duration-150 select-none bg-bg-card/50 hover:bg-bg-card-hover text-text-muted hover:text-text-hover border border-border-default hover:border-border-hover text-[10.5px] font-bold shadow-2xs active:scale-95 group"
      title="Application Settings"
    >
      <Icon
        name="settings"
        size={12}
        class="text-text-muted group-hover:text-text-hover transition-transform group-hover:rotate-45 duration-300"
      />
      <span class="leading-none">Settings</span>
    </button>
  </div>
</div>
