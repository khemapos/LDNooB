<script lang="ts">
import type { ActiveTab } from "$lib/types";
import Icon from "../ui/Icon.svelte";

interface Props {
  activeTab: ActiveTab;
  onTabChange: (tab: ActiveTab) => void;
}

let { activeTab = $bindable("profiles"), onTabChange }: Props = $props();

const tabs: { id: ActiveTab; label: string; icon: any }[] = [
  { id: "profiles", label: "Emulators", icon: "cube" },
  { id: "accounts", label: "Accounts", icon: "users" },
  { id: "workflows", label: "Workflows", icon: "sparkles" },
  { id: "inspector", label: "Inspector", icon: "eye" },
  { id: "settings", label: "Settings", icon: "settings" },
];
</script>

<div
  class="flex items-center gap-1 p-1 bg-slate-200/70 dark:bg-white/[0.04] border border-slate-300/80 dark:border-white/[0.06] rounded-xl select-none"
>
  {#each tabs as tab}
    <button
      type="button"
      onclick={() => onTabChange(tab.id)}
      class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all duration-150 cursor-pointer {activeTab ===
      tab.id
        ? 'bg-white dark:bg-[#151824] text-cyan-600 dark:text-cyan-400 shadow-xs border border-slate-200/80 dark:border-white/[0.08]'
        : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200 hover:bg-white/40 dark:hover:bg-white/[0.03]'}"
    >
      <Icon name={tab.icon} size={14} />
      <span>{tab.label}</span>
    </button>
  {/each}
</div>
