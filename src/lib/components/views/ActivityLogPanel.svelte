<script lang="ts">
  import Icon from "../ui/Icon.svelte";
  import { logsStore } from "$lib/stores/logs.svelte";

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
  class="w-full bg-white/90 dark:bg-[#0c0d13]/95 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl rounded-2xl overflow-hidden flex flex-col transition-all duration-200 shadow-sm {isCollapsed
    ? 'h-9 shrink-0'
    : 'h-48 shrink-0'}"
>
  <!-- Header Bar -->
  <div
    class="h-9 px-3.5 flex items-center justify-between border-b border-slate-200/80 dark:border-white/[0.06] select-none bg-slate-50/50 dark:bg-white/[0.02]"
  >
    <div class="flex items-center gap-2">
      <Icon name="terminal" size={13} class="text-cyan-500" />
      <span class="text-xs font-bold text-slate-800 dark:text-slate-200 uppercase tracking-wider font-mono">
        Activity Log
      </span>
      <span class="text-[10px] font-mono text-slate-400">
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
              ? 'bg-slate-200 dark:bg-white/10 text-slate-900 dark:text-white'
              : 'text-slate-400 hover:text-slate-600 dark:hover:text-slate-300'}"
          >
            All
          </button>
          <button
            type="button"
            onclick={() => (selectedLevel = "adb")}
            class="px-2 py-0.5 rounded cursor-pointer {selectedLevel === 'adb'
              ? 'bg-cyan-500/20 text-cyan-700 dark:text-cyan-300'
              : 'text-slate-400 hover:text-slate-600 dark:hover:text-slate-300'}"
          >
            ADB
          </button>
          <button
            type="button"
            onclick={() => (selectedLevel = "error")}
            class="px-2 py-0.5 rounded cursor-pointer {selectedLevel === 'error'
              ? 'bg-rose-500/20 text-rose-700 dark:text-rose-300'
              : 'text-slate-400 hover:text-slate-600 dark:hover:text-slate-300'}"
          >
            Error
          </button>
        </div>

        <button
          type="button"
          title="Clear Logs"
          onclick={() => logsStore.clear()}
          class="p-1 rounded text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-white/[0.06] transition-colors cursor-pointer"
        >
          <Icon name="trash" size={12} />
        </button>
      {/if}

      <button
        type="button"
        title={isCollapsed ? "Expand Log Panel" : "Collapse Log Panel"}
        onclick={() => (isCollapsed = !isCollapsed)}
        class="p-1 rounded text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-white/[0.06] transition-colors cursor-pointer"
      >
        <Icon name={isCollapsed ? "chevronUp" : "chevronDown"} size={13} />
      </button>
    </div>
  </div>

  <!-- Log Entries List -->
  {#if !isCollapsed}
    <div
      class="flex-1 p-3 overflow-y-auto font-mono text-[11px] space-y-1 select-text bg-[#07080d]"
    >
      {#if filteredLogs.length === 0}
        <div class="py-6 text-center text-slate-600">
          No log events recorded
        </div>
      {:else}
        {#each filteredLogs as log (log.id)}
          <div class="flex items-start gap-2 leading-relaxed">
            <span class="text-slate-500 shrink-0">{log.timestamp}</span>
            <span
              class="px-1.5 py-0.2 rounded text-[9px] font-bold uppercase shrink-0 {log.level ===
              'success'
                ? 'bg-emerald-500/20 text-emerald-400'
                : log.level === 'error'
                  ? 'bg-rose-500/20 text-rose-400'
                  : log.level === 'warn'
                    ? 'bg-amber-500/20 text-amber-400'
                    : log.level === 'adb'
                      ? 'bg-purple-500/20 text-purple-400'
                      : 'bg-cyan-500/20 text-cyan-400'}"
            >
              {log.category}
            </span>
            <span
              class="break-all {log.level === 'error'
                ? 'text-rose-300'
                : log.level === 'success'
                  ? 'text-emerald-300'
                  : log.level === 'adb'
                    ? 'text-purple-300'
                    : 'text-slate-300'}"
            >
              {log.message}
            </span>
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>
