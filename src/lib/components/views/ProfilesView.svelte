<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import type { Emulator } from "$lib/types";
import BaseTable, { type ColumnConfig } from "../common/BaseTable.svelte";
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

let columns = $state<ColumnConfig[]>([
  { key: "index", label: "No./ID", visible: true, canHide: true, width: 70, align: "center" },
  { key: "hostEmulator", label: "Engine", visible: true, canHide: true, width: 110 },
  { key: "name", label: "Name", visible: true, canHide: false, width: 170 },
  { key: "status", label: "Status", visible: true, canHide: true, width: 105, align: "center" },
  { key: "resolution", label: "Resolution & DPI", visible: true, canHide: true, width: 160 },
  { key: "model", label: "Device Model", visible: true, canHide: true, width: 150 },
  { key: "proxy", label: "Proxy", visible: true, canHide: true, width: 150 },
  { key: "metrics", label: "CPU / RAM", visible: true, canHide: true, width: 130 },
  { key: "pid", label: "PID", visible: true, canHide: true, width: 80, align: "center" },
  { key: "actions", label: "Actions", visible: true, canHide: false, width: 180, align: "right" },
]);

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
    class="flex flex-wrap items-center justify-between gap-3 p-3 bg-[#141517] border border-[#25272b] rounded-2xl shadow-xs"
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

      <div class="h-4 w-px bg-[#25272b] mx-1"></div>

      <!-- Batch Actions -->
      <button
        type="button"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => emulatorsStore.batchLaunch()}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-[#00b578]/10 text-[#00b578] border border-[#00b578]/25 hover:bg-[#00b578]/20 transition-colors disabled:opacity-40 cursor-pointer"
      >
        <Icon name="play" size={12} />
        <span>Start ({emulatorsStore.selectedIndices.length})</span>
      </button>

      <button
        type="button"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => emulatorsStore.batchQuit()}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-[#ff4d4f]/10 text-[#ff4d4f] border border-[#ff4d4f]/25 hover:bg-[#ff4d4f]/20 transition-colors disabled:opacity-40 cursor-pointer"
      >
        <Icon name="stop" size={12} />
        <span>Close ({emulatorsStore.selectedIndices.length})</span>
      </button>

      <button
        type="button"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => (showBulkRenameModal = true)}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-[#18191c] text-[#d9d9d9] border border-[#25272b] hover:bg-[#1f2125] transition-colors disabled:opacity-40 cursor-pointer"
      >
        <Icon name="edit" size={12} />
        <span>Rename</span>
      </button>

      <button
        type="button"
        onclick={() => emulatorsStore.sortWindows()}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-[#18191c] text-[#d9d9d9] border border-[#25272b] hover:bg-[#1f2125] transition-colors cursor-pointer"
      >
        <Icon name="sort" size={12} />
        <span>Arrange Windows</span>
      </button>
    </div>

    <!-- Right: Filter & Search Controls -->
    <div class="flex items-center gap-2.5">
      <!-- Status Segmented Control -->
      <div
        class="flex items-center p-0.5 bg-[#0e0f11] border border-[#25272b] rounded-xl text-xs font-sans font-semibold"
      >
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "all")}
          class="px-2.5 py-1 rounded-lg transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'all'
            ? 'bg-[#18191c] text-[#00b578] shadow-xs'
            : 'text-[#8c8c8c] hover:text-white'}"
        >
          All ({emulatorsStore.instances.length})
        </button>
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "running")}
          class="px-2.5 py-1 rounded-lg transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'running'
            ? 'bg-[#18191c] text-[#00b578] shadow-xs'
            : 'text-[#8c8c8c] hover:text-white'}"
        >
          Running ({emulatorsStore.runningCount})
        </button>
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "stopped")}
          class="px-2.5 py-1 rounded-lg transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'stopped'
            ? 'bg-[#18191c] text-[#d9d9d9] shadow-xs'
            : 'text-[#8c8c8c] hover:text-white'}"
        >
          Stopped ({emulatorsStore.stoppedCount})
        </button>
      </div>

      <!-- Search Input -->
      <div class="relative w-48">
        <Icon
          name="search"
          size={13}
          class="absolute left-3 top-2.5 text-[#8c8c8c]"
        />
        <input
          type="text"
          placeholder="Search emulators..."
          bind:value={emulatorsStore.searchQuery}
          class="w-full pl-8 pr-3 py-1.5 text-xs rounded-xl bg-[#0e0f11] border border-[#25272b] text-white placeholder-[#8c8c8c] focus:outline-none focus:border-[#00b578] shadow-inner font-sans"
        />
      </div>

      <!-- Refresh Button -->
      <button
        type="button"
        title="Refresh Fleet"
        onclick={() => emulatorsStore.refresh()}
        class="p-2 rounded-xl text-[#8c8c8c] hover:text-[#00b578] hover:bg-[#18191c] border border-[#25272b] transition-colors cursor-pointer"
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
      bind:columns
      items={emulatorsStore.filteredInstances}
      bind:selectedKeys={emulatorsStore.selectedIndices}
      itemKey="index"
    >
      {#snippet rowSnippet(item: Emulator, isSelected: boolean)}
        <!-- No./ID -->
        {#if columns.find(c => c.key === "index")?.visible}
          <td class="py-2.5 px-3 text-center font-mono font-bold text-[#8c8c8c] border-r border-[#25272b]/30">
            #{item.index}
          </td>
        {/if}

        <!-- Engine -->
        {#if columns.find(c => c.key === "hostEmulator")?.visible}
          <td class="py-2.5 px-3 font-sans text-xs text-[#d9d9d9] border-r border-[#25272b]/30">
            <span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-md bg-[#18191c] border border-[#25272b] text-[11px] font-medium text-[#8c8c8c]">
              LDPlayer 9
            </span>
          </td>
        {/if}

        <!-- Name -->
        {#if columns.find(c => c.key === "name")?.visible}
          <td class="py-2.5 px-3 font-bold text-white border-r border-[#25272b]/30">
            {item.name}
          </td>
        {/if}

        <!-- Status -->
        {#if columns.find(c => c.key === "status")?.visible}
          <td class="py-2.5 px-3 text-center border-r border-[#25272b]/30">
            {#if item.is_running}
              <span
                class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-[#00b578]/10 text-[#00b578] border border-[#00b578]/25"
              >
                <span class="w-1.5 h-1.5 rounded-full bg-[#00b578] animate-pulse"></span>
                Running
              </span>
            {:else}
              <span
                class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium bg-[#18191c] text-[#8c8c8c] border border-[#25272b]"
              >
                Stopped
              </span>
            {/if}
          </td>
        {/if}

        <!-- Resolution & DPI -->
        {#if columns.find(c => c.key === "resolution")?.visible}
          <td class="py-2.5 px-3 font-mono text-[11px] text-[#8c8c8c] border-r border-[#25272b]/30">
            {item.width} × {item.height} ({item.dpi} DPI)
          </td>
        {/if}

        <!-- Device Model -->
        {#if columns.find(c => c.key === "model")?.visible}
          <td class="py-2.5 px-3 text-[#d9d9d9] text-xs border-r border-[#25272b]/30">
            {item.brand} {item.model}
          </td>
        {/if}

        <!-- Proxy -->
        {#if columns.find(c => c.key === "proxy")?.visible}
          <td class="py-2.5 px-3 font-mono text-[11px] text-[#8c8c8c] border-r border-[#25272b]/30">
            {item.proxy || "Direct (No Proxy)"}
          </td>
        {/if}

        <!-- CPU / RAM -->
        {#if columns.find(c => c.key === "metrics")?.visible}
          <td class="py-2.5 px-3 font-mono text-[11px] text-[#8c8c8c] border-r border-[#25272b]/30">
            2 Cores • 2048 MB
          </td>
        {/if}

        <!-- PID -->
        {#if columns.find(c => c.key === "pid")?.visible}
          <td class="py-2.5 px-3 text-center font-mono text-[#8c8c8c] border-r border-[#25272b]/30">
            {item.pid > 0 ? item.pid : "-"}
          </td>
        {/if}

        <!-- Action Buttons (Sticky Right) -->
        {#if columns.find(c => c.key === "actions")?.visible}
          <td
            class="py-2.5 px-3 text-right sticky right-0 z-10 shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.3)] {isSelected
              ? 'bg-[#121c18]'
              : 'bg-[#141517] group-hover:bg-[#1f2125]'}"
            onclick={(e) => e.stopPropagation()}
          >
            <div class="flex items-center justify-end gap-1.5">
              {#if item.is_running}
                <button
                  type="button"
                  title="Open ADB Terminal"
                  onclick={() => openAdb(item.index)}
                  class="p-1.5 rounded-lg text-[#8c8c8c] hover:text-[#00b578] hover:bg-[#18191c] transition-colors cursor-pointer"
                >
                  <Icon name="terminal" size={13} />
                </button>
                <button
                  type="button"
                  title="Stop Emulator"
                  onclick={() => emulatorsStore.quit(item.index)}
                  class="px-2.5 py-1 text-xs font-bold rounded-lg bg-[#ff4d4f]/10 text-[#ff4d4f] border border-[#ff4d4f]/25 hover:bg-[#ff4d4f]/20 transition-colors cursor-pointer"
                >
                  Stop
                </button>
              {:else}
                <button
                  type="button"
                  title="Start Emulator"
                  onclick={() => emulatorsStore.launch(item.index)}
                  class="px-2.5 py-1 text-xs font-bold rounded-lg bg-[#00b578]/10 text-[#00b578] border border-[#00b578]/25 hover:bg-[#00b578]/20 transition-colors cursor-pointer"
                >
                  Start
                </button>
              {/if}

              <!-- Modify Settings Gear -->
              <button
                type="button"
                title="Modify Instance Settings"
                onclick={() => openModify(item)}
                class="p-1.5 rounded-lg text-[#8c8c8c] hover:text-white hover:bg-[#18191c] transition-colors cursor-pointer"
              >
                <Icon name="settings" size={13} />
              </button>

              <!-- Delete -->
              <button
                type="button"
                title="Delete Instance"
                onclick={() => promptDelete(item.index)}
                class="p-1.5 rounded-lg text-[#8c8c8c] hover:text-[#ff4d4f] hover:bg-[#ff4d4f]/10 transition-colors cursor-pointer"
              >
                <Icon name="trash" size={13} />
              </button>
            </div>
          </td>
        {/if}
      {/snippet}
    </BaseTable>
  </div>
</div>

<!-- Modals -->
<AddEmulatorModal bind:open={showAddModal} />
<BulkRenameModal bind:open={showBulkRenameModal} />
<ModifySettingsModal bind:open={showModifyModal} emulator={modifyTarget} />
<AdbTerminalModal bind:open={showAdbModal} emulatorIndex={adbTargetIndex} />

<ConfirmDialog
  bind:open={showDeleteConfirm}
  title="Delete Emulator Instance"
  message="Are you sure you want to permanently delete this emulator instance and all its local storage?"
  confirmText="Delete Instance"
  isDestructive={true}
  onConfirm={confirmDelete}
/>
