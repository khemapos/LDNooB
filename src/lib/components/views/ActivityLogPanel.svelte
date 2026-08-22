<script lang="ts">
import { logsStore } from "$lib/stores/logs.svelte";
import Icon from "../ui/Icon.svelte";

let isCollapsed = $state(false);
let selectedLevel = $state<string>("all");

let filteredLogs = $derived(
  logsStore.entries.filter((entry) => {
    if (selectedLevel === "all") return true;
    return entry.level === selectedLevel;
  })
);
</script>

<div
  class="w-full bg-bg-panel border border-border-default rounded-2xl overflow-hidden flex flex-col transition-all duration-200 shadow-xs {isCollapsed
    ? 'h-9 shrink-0'
    : 'h-48 shrink-0'}"
>
  <!-- Header Bar -->
  <div
    class="h-9 px-3.5 flex items-center justify-between border-b border-border-default select-none bg-bg-card"
  >
    <div class="flex items-center gap-2">
      <Icon name="terminal" size={13} class="text-[#00b578]" />
      <span class="text-xs font-bold text-text-default uppercase tracking-wider font-mono">
        Activity Log
      </span>
      <span class="text-[10px] font-mono text-text-muted">
        ({logsStore.entries.length})
      </span>
    </div>

    <!-- Right: Filter & Collapse -->
    <div class="flex items-center gap-2">
      {#if !isCollapsed}
        <!-- Filter Pills -->
        <div class="flex items-center gap-1 text-[10px] font-mono">
          <button
            type="button"
            onclick={() => (selectedLevel = "all")}
            class="px-2 py-0.5 rounded cursor-pointer {selectedLevel === 'all'
              ? 'bg-bg-card-hover text-text-hover'
              : 'text-text-muted hover:text-text-hover'}"
          >
            All
          </button>
          <button
            type="button"
            onclick={() => (selectedLevel = "adb")}
            class="px-2 py-0.5 rounded cursor-pointer {selectedLevel === 'adb'
              ? 'bg-[#1877f2]/20 text-[#1877f2]'
              : 'text-text-muted hover:text-text-hover'}"
          >
            ADB
          </button>
          <button
            type="button"
            onclick={() => (selectedLevel = "error")}
            class="px-2 py-0.5 rounded cursor-pointer {selectedLevel === 'error'
              ? 'bg-[#ff4d4f]/20 text-[#ff4d4f]'
              : 'text-text-muted hover:text-text-hover'}"
          >
            Error
          </button>
        </div>

        <button
          type="button"
          title="Clear Logs"
          onclick={() => logsStore.clear()}
          class="p-1 rounded text-text-muted hover:text-text-hover hover:bg-bg-card-hover transition-colors cursor-pointer"
        >
          <Icon name="trash" size={12} />
        </button>
      {/if}

      <button
        type="button"
        title={isCollapsed ? "Expand Log Panel" : "Collapse Log Panel"}
        onclick={() => (isCollapsed = !isCollapsed)}
        class="p-1 rounded text-text-muted hover:text-text-hover hover:bg-bg-card-hover transition-colors cursor-pointer"
      >
        <Icon name={isCollapsed ? "chevronUp" : "chevronDown"} size={13} />
      </button>
    </div>
  </div>

  <!-- Log Entries List -->
  {#if !isCollapsed}
    <div
      class="flex-1 p-3 overflow-y-auto font-mono text-[11px] space-y-1 select-text bg-bg-app"
    >
      {#if filteredLogs.length === 0}
        <div class="py-6 text-center text-text-muted italic">
          No log events recorded
        </div>
      {:else}
        {#each filteredLogs as log (log.id)}
          <div class="flex items-start gap-2 leading-relaxed">
            <span class="text-text-muted shrink-0">{log.timestamp}</span>
            <span
              class="px-1.5 py-0.2 rounded text-[9px] font-bold uppercase shrink-0 {log.level ===
              'success'
                ? 'bg-[#00b578]/20 text-[#00b578]'
                : log.level === 'error'
                  ? 'bg-[#ff4d4f]/20 text-[#ff4d4f]'
                  : log.level === 'warn'
                    ? 'bg-[#ffc107]/20 text-[#ffc107]'
                    : log.level === 'adb'
                      ? 'bg-purple-500/20 text-purple-400'
                      : 'bg-[#1877f2]/20 text-[#1877f2]'}"
            >
              {log.category}
            </span>
            <span
              class="break-all {log.level === 'error'
                ? 'text-[#ff4d4f]'
                : log.level === 'success'
                  ? 'text-[#00b578]'
                  : log.level === 'adb'
                    ? 'text-purple-400'
                    : 'text-text-default'}"
            >
              {log.message}
            </span>
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>
