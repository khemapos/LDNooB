<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import type { Emulator } from "$lib/types";
import BaseTable, { type ColumnConfig } from "../common/BaseTable.svelte";
import ConfirmDialog from "../common/ConfirmDialog.svelte";
import CustomButton from "../common/CustomButton.svelte";
import CustomInput from "../common/CustomInput.svelte";
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
  { key: "index", label: "No./ID", visible: true, canHide: true, width: 65, align: "center" },
  { key: "name", label: "Instance Name", visible: true, canHide: false, width: 170 },
  { key: "status", label: "Status", visible: true, canHide: true, width: 110, align: "center" },
  { key: "resolution", label: "Resolution & DPI", visible: true, canHide: true, width: 150 },
  { key: "model", label: "Device Model", visible: true, canHide: true, width: 150 },
  { key: "proxy", label: "Proxy / IP", visible: true, canHide: true, width: 150 },
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

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden font-sans">
  <!-- Top Command Toolbar -->
  <div
    class="flex flex-wrap items-center justify-between gap-3 p-3 bg-bg-panel border border-border-default rounded-2xl shadow-xs"
  >
    <!-- Left: Batch Operations -->
    <div class="flex items-center gap-2">
      <!-- Primary New Instance Button (Brand Green) -->
      <CustomButton
        variant="primary"
        size="md"
        onclick={() => (showAddModal = true)}
      >
        <Icon name="plus" size={14} />
        <span>New LDPlayer</span>
      </CustomButton>

      <div class="h-4 w-px bg-border-default mx-1"></div>

      <!-- Batch Actions -->
      <CustomButton
        variant="secondary"
        size="sm"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => emulatorsStore.batchLaunch()}
        class="text-[#00b578] hover:text-[#00b578]"
      >
        <Icon name="play" size={12} />
        <span>Start ({emulatorsStore.selectedIndices.length})</span>
      </CustomButton>

      <CustomButton
        variant="secondary"
        size="sm"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => emulatorsStore.batchQuit()}
        class="text-[#ff4d4f] hover:text-[#ff4d4f]"
      >
        <Icon name="stop" size={12} />
        <span>Close ({emulatorsStore.selectedIndices.length})</span>
      </CustomButton>

      <CustomButton
        variant="secondary"
        size="sm"
        disabled={emulatorsStore.selectedIndices.length === 0}
        onclick={() => (showBulkRenameModal = true)}
      >
        <Icon name="edit" size={12} />
        <span>Rename</span>
      </CustomButton>

      <CustomButton
        variant="secondary"
        size="sm"
        onclick={() => emulatorsStore.sortWindows()}
      >
        <Icon name="sort" size={12} />
        <span>Arrange Windows</span>
      </CustomButton>
    </div>

    <!-- Right: Filter & Search Controls -->
    <div class="flex items-center gap-2.5">
      <!-- Status Segmented Control -->
      <div
        class="flex items-center p-0.5 bg-bg-app border border-border-default rounded-xl text-xs font-semibold"
      >
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "all")}
          class="px-2.5 py-1 rounded-lg transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'all'
            ? 'bg-bg-card text-[#00b578] shadow-xs'
            : 'text-text-muted hover:text-text-hover'}"
        >
          All ({emulatorsStore.instances.length})
        </button>
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "running")}
          class="px-2.5 py-1 rounded-lg transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'running'
            ? 'bg-bg-card text-[#00b578] shadow-xs'
            : 'text-text-muted hover:text-text-hover'}"
        >
          Running ({emulatorsStore.runningCount})
        </button>
        <button
          type="button"
          onclick={() => (emulatorsStore.filterStatus = "stopped")}
          class="px-2.5 py-1 rounded-lg transition-all cursor-pointer {emulatorsStore.filterStatus ===
          'stopped'
            ? 'bg-bg-card text-text-default shadow-xs'
            : 'text-text-muted hover:text-text-hover'}"
        >
          Stopped ({emulatorsStore.stoppedCount})
        </button>
      </div>

      <!-- Search Input -->
      <div class="w-48">
        <CustomInput
          placeholder="Search emulators..."
          bind:value={emulatorsStore.searchQuery}
          icon="search"
        />
      </div>

      <!-- Refresh Button -->
      <CustomButton
        variant="icon"
        size="icon"
        title="Refresh Fleet"
        onclick={() => emulatorsStore.refresh()}
      >
        <Icon
          name="refresh"
          size={14}
          class={emulatorsStore.isLoading ? "animate-spin" : ""}
        />
      </CustomButton>
    </div>
  </div>

  <!-- Primary Fleet Table -->
  <div class="flex-1 overflow-hidden flex flex-col min-h-0">
    <BaseTable
      bind:columns
      items={emulatorsStore.filteredInstances}
      bind:selectedKeys={emulatorsStore.selectedIndices}
      itemKey="index"
    >
      {#snippet rowSnippet(item: Emulator, isSelected: boolean)}
        <!-- No./ID -->
        {#if columns.find((c) => c.key === "index")?.visible}
          <td
            class="py-2.5 px-3 text-center font-mono font-bold text-text-muted border-r border-border-default/20"
          >
            #{item.index}
          </td>
        {/if}

        <!-- Name with Live Glow Indicator -->
        {#if columns.find((c) => c.key === "name")?.visible}
          <td
            class="py-2.5 px-3 font-bold text-text-hover border-r border-border-default/20"
          >
            <div class="flex items-center gap-2">
              <span
                class="w-2 h-2 rounded-full shrink-0 {item.is_running
                  ? 'bg-[#00b578] shadow-[0_0_8px_rgba(0,181,120,0.6)] animate-pulse'
                  : 'bg-text-muted/40'}"
              ></span>
              <span class="truncate">{item.name}</span>
            </div>
          </td>
        {/if}

        <!-- Status Pill -->
        {#if columns.find((c) => c.key === "status")?.visible}
          <td
            class="py-2.5 px-3 text-center border-r border-border-default/20"
          >
            {#if item.is_running}
              <span
                class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-[#00b578]/10 text-[#00b578] border border-[#00b578]/25"
              >
                <span class="w-1.5 h-1.5 rounded-full bg-[#00b578] animate-ping"></span>
                Running
              </span>
            {:else}
              <span
                class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium bg-bg-card text-text-muted border border-border-default"
              >
                Stopped
              </span>
            {/if}
          </td>
        {/if}

        <!-- Resolution & DPI -->
        {#if columns.find((c) => c.key === "resolution")?.visible}
          <td
            class="py-2.5 px-3 font-mono text-[11px] text-text-muted border-r border-border-default/20"
          >
            {item.width} × {item.height} ({item.dpi} DPI)
          </td>
        {/if}

        <!-- Device Model -->
        {#if columns.find((c) => c.key === "model")?.visible}
          <td
            class="py-2.5 px-3 text-text-default text-xs border-r border-border-default/20"
          >
            {item.brand ? `${item.brand} ` : ""}{item.model || "Samsung Galaxy S22"}
          </td>
        {/if}

        <!-- Proxy -->
        {#if columns.find((c) => c.key === "proxy")?.visible}
          <td
            class="py-2.5 px-3 font-mono text-[11px] text-text-muted border-r border-border-default/20"
          >
            {item.proxy || "Direct (No Proxy)"}
          </td>
        {/if}

        <!-- CPU / RAM -->
        {#if columns.find((c) => c.key === "metrics")?.visible}
          <td
            class="py-2.5 px-3 font-mono text-[11px] text-text-muted border-r border-border-default/20"
          >
            2 Cores • 2048 MB
          </td>
        {/if}

        <!-- PID -->
        {#if columns.find((c) => c.key === "pid")?.visible}
          <td
            class="py-2.5 px-3 text-center font-mono text-text-muted border-r border-border-default/20"
          >
            {item.pid > 0 ? item.pid : "-"}
          </td>
        {/if}

        <!-- Action Buttons (Sticky Right) -->
        {#if columns.find((c) => c.key === "actions")?.visible}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
          <td
            class="py-2 px-3 text-right sticky right-0 z-30 shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.25)] group-hover:z-30 focus-within:z-30 {isSelected
              ? 'bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-panel))] group-hover:bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-card-hover))]'
              : 'bg-bg-panel group-hover:bg-bg-card-hover'}"
            onclick={(e) => e.stopPropagation()}
            onmousedown={(e) => e.stopPropagation()}
          >
            <div class="flex items-center justify-end gap-1.5">
              {#if item.is_running}
                <button
                  type="button"
                  title="Open ADB Terminal"
                  onclick={() => openAdb(item.index)}
                  class="p-1.5 rounded-lg text-text-muted hover:text-[#00b578] hover:bg-bg-card transition-colors cursor-pointer"
                >
                  <Icon name="terminal" size={13} />
                </button>
                <button
                  type="button"
                  title="Stop Emulator Instance"
                  onclick={() => emulatorsStore.quit(item.index)}
                  class="px-2.5 py-1 text-[11px] font-bold rounded-lg bg-[#ff4d4f]/15 text-[#ff4d4f] border border-[#ff4d4f]/30 hover:bg-[#ff4d4f]/25 transition-all cursor-pointer active:scale-95"
                >
                  Stop
                </button>
              {:else}
                <button
                  type="button"
                  title="Start Emulator Instance"
                  onclick={() => emulatorsStore.launch(item.index)}
                  class="px-2.5 py-1 text-[11px] font-bold rounded-lg bg-[#00b578]/15 text-[#00b578] border border-[#00b578]/30 hover:bg-[#00b578]/25 transition-all cursor-pointer active:scale-95"
                >
                  Start
                </button>
              {/if}

              <!-- Modify Settings Gear -->
              <button
                type="button"
                title="Modify Instance Settings"
                onclick={() => openModify(item)}
                class="p-1.5 rounded-lg text-text-muted hover:text-text-hover hover:bg-bg-card transition-colors cursor-pointer"
              >
                <Icon name="settings" size={13} />
              </button>

              <!-- Delete -->
              <button
                type="button"
                title="Delete Instance"
                onclick={() => promptDelete(item.index)}
                class="p-1.5 rounded-lg text-text-muted hover:text-[#ff4d4f] hover:bg-[#ff4d4f]/10 transition-colors cursor-pointer"
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
