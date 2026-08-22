<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import type { Emulator } from "$lib/types";
import BaseTable, { type TableColumn } from "../common/BaseTable.svelte";
import ConfirmDialog from "../common/ConfirmDialog.svelte";
import AdbTerminalModal from "../modals/AdbTerminalModal.svelte";
import AddEmulatorModal from "../modals/AddEmulatorModal.svelte";
import BulkRenameModal from "../modals/BulkRenameModal.svelte";
import ModifySettingsModal from "../modals/ModifySettingsModal.svelte";
import Icon from "../ui/Icon.svelte";

let showAddModal = $state(false);
let showBulkRenameModal = $state(false);
let showModifyModal = $state(false);
let modifyTarget = $state<Emulator | null>(null);
let showAdbModal = $state(false);
let adbTargetIndex = $state(0);

let showDeleteConfirm = $state(false);
let deleteTargetIndex = $state<number | null>(null);

const columns: TableColumn[] = [
  { key: "index", label: "Index", width: "w-16", align: "center" },
  { key: "name", label: "Instance Name", width: "w-48" },
  { key: "status", label: "Status", width: "w-28", align: "center" },
  { key: "resolution", label: "Resolution & DPI", width: "w-40" },
  { key: "model", label: "Device Model", width: "w-36" },
  { key: "pid", label: "PID", width: "w-20", align: "center" },
  { key: "actions", label: "Actions", width: "w-52", align: "right" },
];

function promptDelete(index: number) {
  deleteTargetIndex = index;
  showDeleteConfirm = true;
}

function confirmDelete() {
  if (deleteTargetIndex !== null) {
    emulatorsStore.deleteInstance(deleteTargetIndex);
    deleteTargetIndex = null;
  }
}

function openAdb(index: number) {
  adbTargetIndex = index;
  showAdbModal = true;
}

function openModify(emu: Emulator) {
  modifyTarget = emu;
  showModifyModal = true;
}
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden">
  <!-- Top Command Toolbar -->
  <div
    class="flex flex-wrap items-center justify-between gap-3 p-3 bg-white/90 dark:bg-[#0e1018]/90 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl rounded-2xl shadow-xs"
  >
    <!-- Left: Batch Operations -->
    <div class="flex items-center gap-2">
      <!-- Primary New Instance Button (Brand Green) -->
      <button
        type="button"
        onclick={() => (showAddModal = true)}
        class="flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-bold text-white bg-gradient-to-r from-[#00b578] to-[#009963] hover:from-[#00c985] hover:to-[#00a86d] transition-all shadow-[0_2px_10px_rgba(0,181,120,0.3)] cursor-pointer"
      >
        <Icon name="plus" size={14} />
        <span>New LDPlayer</span>
      </button>

      <div class="h-4 w-px bg-slate-200 dark:bg-white/[0.08] mx-1"></div>

      <!-- Batch Actions -->
      <button
        type="button"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => emulatorsStore.batchLaunch()}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-emerald-500/10 text-emerald-700 dark:text-emerald-300 border border-emerald-500/20 hover:bg-emerald-500/20 transition-colors disabled:opacity-40 cursor-pointer"
      >
        <Icon name="play" size={12} />
        <span>Start ({emulatorsStore.selectedIndices.length})</span>
      </button>

      <button
        type="button"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => emulatorsStore.batchQuit()}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-amber-500/10 text-amber-700 dark:text-amber-300 border border-amber-500/20 hover:bg-amber-500/20 transition-colors disabled:opacity-40 cursor-pointer"
      >
        <Icon name="stop" size={12} />
        <span>Close ({emulatorsStore.selectedIndices.length})</span>
      </button>

      <button
        type="button"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => (showBulkRenameModal = true)}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-100 dark:bg-white/[0.04] text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.08] transition-colors disabled:opacity-40 cursor-pointer"
      >
        <Icon name="edit" size={12} />
        <span>Rename</span>
      </button>

      <button
        type="button"
        onclick={() => emulatorsStore.sortWindows()}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-100 dark:bg-white/[0.04] text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.08] transition-colors cursor-pointer"
      >
        <Icon name="sort" size={12} />
        <span>Arrange Windows</span>
      </button>
    </div>

    <!-- Right: Filter & Search Controls -->
    <div class="flex items-center gap-2.5">
      <!-- Status Segmented Control -->
      <div
        class="flex items-center p-0.5 bg-slate-100 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] rounded-xl text-xs"
      >
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "all")}
          class="px-2.5 py-1 rounded-lg font-semibold transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'all'
            ? 'bg-white dark:bg-[#161824] text-emerald-600 dark:text-emerald-400 shadow-xs'
            : 'text-slate-500 hover:text-slate-800 dark:hover:text-slate-200'}"
        >
          All ({emulatorsStore.instances.length})
        </button>
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "running")}
          class="px-2.5 py-1 rounded-lg font-semibold transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'running'
            ? 'bg-white dark:bg-[#161824] text-emerald-600 dark:text-emerald-400 shadow-xs'
            : 'text-slate-500 hover:text-slate-800 dark:hover:text-slate-200'}"
        >
          Running ({emulatorsStore.runningCount})
        </button>
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "stopped")}
          class="px-2.5 py-1 rounded-lg font-semibold transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'stopped'
            ? 'bg-white dark:bg-[#161824] text-slate-700 dark:text-slate-300 shadow-xs'
            : 'text-slate-500 hover:text-slate-800 dark:hover:text-slate-200'}"
        >
          Stopped ({emulatorsStore.stoppedCount})
        </button>
      </div>

      <!-- Search Input -->
      <div class="relative w-44">
        <Icon
          name="search"
          size={13}
          class="absolute left-3 top-2.5 text-slate-400"
        />
        <input
          type="text"
          placeholder="Filter..."
          bind:value={emulatorsStore.searchQuery}
          class="w-full pl-8 pr-3 py-1.5 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:border-emerald-500 shadow-inner font-mono"
        />
      </div>

      <!-- Refresh Button -->
      <button
        type="button"
        title="Refresh Fleet"
        onclick={() => emulatorsStore.refresh()}
        class="p-2 rounded-xl text-slate-500 dark:text-slate-400 hover:text-emerald-600 dark:hover:text-emerald-400 hover:bg-slate-100 dark:hover:bg-white/[0.06] border border-slate-200 dark:border-white/[0.06] transition-colors cursor-pointer"
      >
        <Icon
          name="refresh"
          size={14}
          class={emulatorsStore.isLoading ? "animate-spin" : ""}
        />
      </button>
    </div>
  </div>

  <!-- Primary Fleet Table -->
  <div class="flex-1 overflow-hidden">
    <BaseTable
      {columns}
      items={emulatorsStore.filteredInstances}
      selectedKeys={emulatorsStore.selectedIndices}
      itemKey="index"
      onSelectAll={() => {
        if (
          emulatorsStore.selectedIndices.length ===
          emulatorsStore.filteredInstances.length
        ) {
          emulatorsStore.deselectAll();
        } else {
          emulatorsStore.selectAll();
        }
      }}
      onToggleSelect={(idx) => emulatorsStore.toggleSelection(idx)}
    >
      {#snippet rowSnippet(item: Emulator)}
        <!-- Index -->
        <td class="px-3 py-2 text-center font-mono font-bold text-slate-500 dark:text-slate-400">
          #{item.index}
        </td>

        <!-- Name -->
        <td class="px-3 py-2 font-semibold text-slate-900 dark:text-white">
          <div class="flex items-center gap-1.5">
            <span>{item.name}</span>
          </div>
        </td>

        <!-- Status -->
        <td class="px-3 py-2 text-center">
          {#if item.is_running}
            <span
              class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/25"
            >
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
              Running
            </span>
          {:else}
            <span
              class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium bg-slate-200 dark:bg-white/[0.06] text-slate-600 dark:text-slate-400"
            >
              Stopped
            </span>
          {/if}
        </td>

        <!-- Resolution & DPI -->
        <td class="px-3 py-2 font-mono text-[11px] text-slate-600 dark:text-slate-400">
          {item.width} × {item.height} ({item.dpi} DPI)
        </td>

        <!-- Model -->
        <td class="px-3 py-2 text-slate-700 dark:text-slate-300 text-xs">
          {item.brand} {item.model}
        </td>

        <!-- PID -->
        <td class="px-3 py-2 text-center font-mono text-slate-500 dark:text-slate-400">
          {item.pid > 0 ? item.pid : "-"}
        </td>

        <!-- Action Buttons -->
        <td class="px-3 py-2 text-right" onclick={(e) => e.stopPropagation()}>
          <div class="flex items-center justify-end gap-1.5">
            {#if item.is_running}
              <button
                type="button"
                title="Open ADB Terminal"
                onclick={() => openAdb(item.index)}
                class="p-1.5 rounded-lg text-slate-500 hover:text-cyan-500 dark:hover:text-cyan-400 hover:bg-slate-100 dark:hover:bg-white/[0.06] transition-colors cursor-pointer"
              >
                <Icon name="terminal" size={13} />
              </button>
              <button
                type="button"
                title="Stop Emulator"
                onclick={() => emulatorsStore.quit(item.index)}
                class="px-2.5 py-1 text-xs font-bold rounded-lg bg-amber-500/10 text-amber-700 dark:text-amber-400 border border-amber-500/20 hover:bg-amber-500/20 transition-colors cursor-pointer"
              >
                Stop
              </button>
            {:else}
              <button
                type="button"
                title="Start Emulator"
                onclick={() => emulatorsStore.launch(item.index)}
                class="px-2.5 py-1 text-xs font-bold rounded-lg bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 border border-emerald-500/20 hover:bg-emerald-500/20 transition-colors cursor-pointer"
              >
                Start
              </button>
            {/if}

            <!-- Modify Settings Gear -->
            <button
              type="button"
              title="Modify Instance Settings"
              onclick={() => openModify(item)}
              class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-white/[0.06] transition-colors cursor-pointer"
            >
              <Icon name="settings" size={13} />
            </button>

            <!-- Delete -->
            <button
              type="button"
              title="Delete Instance"
              onclick={() => promptDelete(item.index)}
              class="p-1.5 rounded-lg text-slate-400 hover:text-rose-600 hover:bg-rose-500/10 transition-colors cursor-pointer"
            >
              <Icon name="trash" size={13} />
            </button>
          </div>
        </td>
      {/snippet}
    </BaseTable>
  </div>
</div>

<!-- Add Modal -->
<AddEmulatorModal bind:open={showAddModal} />

<!-- Bulk Rename Modal -->
<BulkRenameModal bind:open={showBulkRenameModal} />

<!-- Modify Settings Modal -->
<ModifySettingsModal bind:open={showModifyModal} emulator={modifyTarget} />

<!-- ADB Terminal Modal -->
<AdbTerminalModal bind:open={showAdbModal} emulatorIndex={adbTargetIndex} />

<!-- Delete Confirmation -->
<ConfirmDialog
  bind:open={showDeleteConfirm}
  title="Delete Emulator Instance"
  message="Are you sure you want to permanently delete this emulator instance and all its local storage?"
  confirmText="Delete Instance"
  isDestructive={true}
  onConfirm={confirmDelete}
/>
