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
  { key: "index", label: "No./ID", visible: true, canHide: true, width: 65, align: "center" },
  { key: "name", label: "Instance Name", visible: true, canHide: false, width: 170 },
  { key: "status", label: "Status", visible: true, canHide: true, width: 110, align: "center" },
  { key: "resolution", label: "Resolution & DPI", visible: true, canHide: true, width: 150 },
  { key: "model", label: "Device Model", visible: true, canHide: true, width: 150 },
  { key: "proxy", label: "Proxy / IP", visible: true, canHide: true, width: 150 },
  { key: "metrics", label: "CPU / RAM", visible: true, canHide: true, width: 130 },
  { key: "pid", label: "PID", visible: true, canHide: true, width: 75, align: "center" },
  { key: "actions", label: "Actions", visible: true, canHide: false, width: 175, align: "right" },
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
// Segmented Switcher Sliding Pill State
let tabRefs: Record<string, HTMLButtonElement | null> = $state({});
let indicatorStyle = $state({ left: 2, width: 0 });

$effect(() => {
  const currentKey = emulatorsStore.filterStatus;
  const targetEl = tabRefs[currentKey];
  if (targetEl) {
    indicatorStyle = {
      left: targetEl.offsetLeft,
      width: targetEl.offsetWidth,
    };
  }
});
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden font-sans select-none">
  <!-- Top Command Toolbar (Modern & Premium) -->
  <div
    class="flex flex-wrap items-center justify-between gap-3 p-2.5 px-3.5 bg-bg-panel/95 backdrop-blur-md border border-border-default rounded-2xl shadow-xs shrink-0"
  >
    <!-- Left: Primary & Batch Action Group -->
    <div class="flex items-center gap-2">
      <!-- Primary New Instance Button (Emerald Gradient) -->
      <button
        type="button"
        onclick={() => (showAddModal = true)}
        class="inline-flex items-center justify-center gap-1.5 h-8.5 px-4 rounded-xl text-xs font-bold text-white bg-gradient-to-b from-[#00c985] to-[#00b578] hover:from-[#00d78e] hover:to-[#00c07f] active:scale-[0.98] border border-[#00b578] shadow-[0_2px_10px_rgba(0,181,120,0.25),inset_0_1px_0_rgba(255,255,255,0.2)] transition-all cursor-pointer"
      >
        <Icon name="plus" size={13} />
        <span>New LDPlayer</span>
      </button>

      <div class="h-4.5 w-px bg-border-default/80 mx-0.5"></div>

      <!-- Batch Actions Cluster -->
      <div class="flex items-center gap-1.5">
        <!-- Batch Start -->
        <button
          type="button"
          disabled={emulatorsStore.selectedIndices.length === 0}
          onclick={() => emulatorsStore.batchLaunch()}
          class="inline-flex items-center gap-1.5 h-8.5 px-3 rounded-xl text-xs font-semibold bg-bg-card hover:bg-bg-card-hover border border-border-default hover:border-[#00b578]/40 text-text-default hover:text-[#00b578] disabled:opacity-40 disabled:pointer-events-none transition-all cursor-pointer active:scale-95 shadow-xs"
          title="Launch selected emulators"
        >
          <Icon name="play" size={12} class="text-[#00b578]" />
          <span>Start ({emulatorsStore.selectedIndices.length})</span>
        </button>

        <!-- Batch Stop -->
        <button
          type="button"
          disabled={emulatorsStore.selectedIndices.length === 0}
          onclick={() => emulatorsStore.batchQuit()}
          class="inline-flex items-center gap-1.5 h-8.5 px-3 rounded-xl text-xs font-semibold bg-bg-card hover:bg-bg-card-hover border border-border-default hover:border-[#ff4d4f]/40 text-text-default hover:text-[#ff4d4f] disabled:opacity-40 disabled:pointer-events-none transition-all cursor-pointer active:scale-95 shadow-xs"
          title="Close selected emulators"
        >
          <Icon name="stop" size={12} class="text-[#ff4d4f]" />
          <span>Close ({emulatorsStore.selectedIndices.length})</span>
        </button>

        <!-- Batch Rename -->
        <button
          type="button"
          disabled={emulatorsStore.selectedIndices.length === 0}
          onclick={() => (showBulkRenameModal = true)}
          class="inline-flex items-center gap-1.5 h-8.5 px-3 rounded-xl text-xs font-semibold bg-bg-card hover:bg-bg-card-hover border border-border-default hover:border-border-hover text-text-muted hover:text-text-hover disabled:opacity-40 disabled:pointer-events-none transition-all cursor-pointer active:scale-95 shadow-xs"
          title="Bulk rename selected emulators"
        >
          <Icon name="edit" size={12} />
          <span>Rename</span>
        </button>
      </div>
      <!-- Window Arrangement & Visibility Controls (matching D:\ldremote) -->
      <div class="flex items-center gap-2">
        <!-- Auto / Cols / Arrange Multi-segment Pill -->
        <div
          class="flex items-center h-8.5 rounded-xl border border-border-default bg-bg-app overflow-hidden shadow-xs"
        >
          <!-- Auto Arrange Checkbox -->
          <label
            class="flex items-center gap-1.5 px-3 h-full border-r border-border-default/60 hover:bg-bg-card/50 cursor-pointer select-none"
          >
            <input
              type="checkbox"
              bind:checked={emulatorsStore.autoArrange}
              class="custom-checkbox shrink-0 cursor-pointer"
            />
            <span
              class="text-[10px] font-extrabold uppercase tracking-wider text-text-muted"
            >
              Auto
            </span>
          </label>

          <!-- Cols Input -->
          <div class="flex items-center pl-2.5 pr-1 h-full">
            <span
              class="select-none text-[10px] font-extrabold uppercase tracking-wider text-text-muted mr-1"
            >
              Cols
            </span>
            <input
              type="number"
              min="1"
              max="20"
              bind:value={emulatorsStore.arrangeCols}
              class="w-7 bg-transparent border-none outline-none text-center text-xs font-mono font-bold text-text-hover p-0 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
          </div>

          <div class="h-4 border-r border-border-default/60"></div>

          <!-- Arrange Button -->
          <button
            type="button"
            onclick={() => emulatorsStore.sortWindows()}
            title="Arrange emulator windows"
            class="flex items-center gap-1.5 px-3 h-full bg-bg-card/40 hover:bg-bg-card hover:text-text-hover text-text-default text-xs font-bold transition-colors cursor-pointer border-none"
          >
            <Icon name="grid" size={13} class="text-[#00b578]" />
            <span>Arrange</span>
          </button>
        </div>

        <!-- Green Eye Button (Toggle Visibility) -->
        <button
          type="button"
          onclick={() => emulatorsStore.toggleVisibility()}
          title={emulatorsStore.isEmulatorsHidden
            ? "Show/Restore Running Emulators"
            : "Hide Running Emulators"}
          class="w-8.5 h-8.5 rounded-xl border flex items-center justify-center transition-all cursor-pointer active:scale-95 shadow-xs {emulatorsStore.isEmulatorsHidden
            ? 'border-[#ff4d4f]/50 bg-[#ff4d4f]/15 text-[#ff4d4f]'
            : 'border-[#00b578]/40 hover:border-[#00b578] bg-[#00b578]/10 hover:bg-[#00b578]/20 text-[#00b578]'}"
        >
          <Icon
            name={emulatorsStore.isEmulatorsHidden ? "eyeOff" : "eye"}
            size={15}
          />
        </button>
      </div>
    </div>

    <!-- Right: Segmented Status Filter, Search, & Refresh -->
    <div class="flex items-center gap-2.5">
      <!-- Status Segmented Switcher with Smooth Animated Pill -->
      <div
        class="relative flex items-center p-0.5 bg-bg-app border border-border-default rounded-xl text-xs font-semibold shadow-inner"
      >
        <!-- Smooth Animated Sliding Pill Background -->
        {#if indicatorStyle.width > 0}
          <div
            class="absolute top-0.5 bottom-0.5 rounded-lg bg-bg-card border border-border-default shadow-xs transition-all duration-200 cubic-bezier(0.16,1,0.3,1) pointer-events-none z-0"
            style="left: {indicatorStyle.left}px; width: {indicatorStyle.width}px;"
          ></div>
        {/if}

        <button
          type="button"
          bind:this={tabRefs["all"]}
          onclick={() => (emulatorsStore.filterStatus = "all")}
          class="relative z-10 flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-colors duration-150 cursor-pointer {emulatorsStore.filterStatus ===
          'all'
            ? 'text-text-hover font-bold'
            : 'text-text-muted hover:text-text-hover'}"
        >
          <span>All</span>
          <span
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono transition-colors duration-150 {emulatorsStore.filterStatus ===
            'all'
              ? 'bg-bg-app text-[#00b578]'
              : 'bg-bg-card text-text-muted'}"
          >
            {emulatorsStore.instances.length}
          </span>
        </button>

        <button
          type="button"
          bind:this={tabRefs["running"]}
          onclick={() => (emulatorsStore.filterStatus = "running")}
          class="relative z-10 flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-colors duration-150 cursor-pointer {emulatorsStore.filterStatus ===
          'running'
            ? 'text-text-hover font-bold'
            : 'text-text-muted hover:text-text-hover'}"
        >
          <span class="w-1.5 h-1.5 rounded-full bg-[#00b578] animate-pulse"></span>
          <span>Running</span>
          <span
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono transition-colors duration-150 {emulatorsStore.filterStatus ===
            'running'
              ? 'bg-bg-app text-[#00b578]'
              : 'bg-bg-card text-text-muted'}"
          >
            {emulatorsStore.runningCount}
          </span>
        </button>

        <button
          type="button"
          bind:this={tabRefs["stopped"]}
          onclick={() => (emulatorsStore.filterStatus = "stopped")}
          class="relative z-10 flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-colors duration-150 cursor-pointer {emulatorsStore.filterStatus ===
          'stopped'
            ? 'text-text-hover font-bold'
            : 'text-text-muted hover:text-text-hover'}"
        >
          <span class="w-1.5 h-1.5 rounded-full bg-zinc-500"></span>
          <span>Stopped</span>
          <span
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono transition-colors duration-150 {emulatorsStore.filterStatus ===
            'stopped'
              ? 'bg-bg-app text-text-default'
              : 'bg-bg-card text-text-muted'}"
          >
            {emulatorsStore.stoppedCount}
          </span>
        </button>
      </div>

      <!-- Search Input -->
      <div class="relative flex items-center h-8.5 w-48 group">
        <input
          type="text"
          placeholder="Search emulators..."
          bind:value={emulatorsStore.searchQuery}
          class="w-full h-8.5 pl-8 pr-7 text-xs font-medium rounded-xl border border-border-default hover:border-border-hover focus:border-[#00b578] bg-bg-app text-text-default placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-[#00b578]/20 transition-all duration-150"
        />
        <span class="absolute left-2.5 text-text-muted pointer-events-none group-focus-within:text-[#00b578]">
          <Icon name="search" size={13} />
        </span>
        {#if emulatorsStore.searchQuery}
          <button
            type="button"
            onclick={() => (emulatorsStore.searchQuery = "")}
            class="absolute right-2 text-text-muted hover:text-text-hover p-0.5 rounded-md cursor-pointer flex items-center justify-center transition-colors"
          >
            <Icon name="close" size={11} />
          </button>
        {/if}
      </div>

      <!-- Refresh Fleet Button -->
      <button
        type="button"
        title="Refresh Fleet Telemetry"
        onclick={() => emulatorsStore.refresh()}
        class="w-8.5 h-8.5 rounded-xl border border-border-default hover:border-border-hover bg-bg-app hover:bg-bg-card flex items-center justify-center text-text-muted hover:text-text-hover transition-all cursor-pointer active:scale-95 shadow-xs"
      >
        <Icon
          name="refresh"
          size={13}
          class={emulatorsStore.isLoading ? "animate-spin text-[#00b578]" : ""}
        />
      </button>
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
      {#snippet renderCell(colKey: string, item: Emulator)}
        {#if colKey === "index"}
          <span class="font-mono font-bold text-text-muted">
            #{item.index}
          </span>
        {:else if colKey === "name"}
          <div class="flex items-center gap-2 overflow-hidden">
            <span
              class="w-2 h-2 rounded-full shrink-0 {item.is_running
                ? 'bg-[#00b578] shadow-[0_0_8px_rgba(0,181,120,0.6)] animate-pulse'
                : 'bg-text-muted/40'}"
            ></span>
            <span class="truncate font-bold text-text-hover">{item.name}</span>
          </div>
        {:else if colKey === "status"}
          <div class="flex items-center justify-center">
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
          </div>
        {:else if colKey === "resolution"}
          <span class="font-mono text-[11px] text-text-muted">
            {item.width} × {item.height} ({item.dpi} DPI)
          </span>
        {:else if colKey === "model"}
          <span class="text-text-default text-xs truncate">
            {item.brand ? `${item.brand} ` : ""}{item.model || "Samsung Galaxy S22"}
          </span>
        {:else if colKey === "proxy"}
          <span class="font-mono text-[11px] text-text-muted truncate">
            {item.proxy || "Direct (No Proxy)"}
          </span>
        {:else if colKey === "metrics"}
          <span class="font-mono text-[11px] text-text-muted">
            2 Cores • 2048 MB
          </span>
        {:else if colKey === "pid"}
          <span class="font-mono text-text-muted block text-center">
            {item.pid > 0 ? item.pid : "-"}
          </span>
        {:else if colKey === "actions"}
          <div class="flex items-center justify-end gap-1.5">
            {#if item.is_running}
              <button
                type="button"
                title="Open ADB Terminal"
                onclick={(e) => {
                  e.stopPropagation();
                  openAdb(item.index);
                }}
                class="p-1.5 rounded-lg text-text-muted hover:text-[#00b578] hover:bg-bg-card transition-colors cursor-pointer"
              >
                <Icon name="terminal" size={13} />
              </button>
              <button
                type="button"
                title="Stop Emulator Instance"
                onclick={(e) => {
                  e.stopPropagation();
                  emulatorsStore.quit(item.index);
                }}
                class="px-2.5 py-1 text-[11px] font-bold rounded-lg bg-[#ff4d4f]/15 text-[#ff4d4f] border border-[#ff4d4f]/30 hover:bg-[#ff4d4f]/25 transition-all cursor-pointer active:scale-95"
              >
                Stop
              </button>
            {:else}
              <button
                type="button"
                title="Start Emulator Instance"
                onclick={(e) => {
                  e.stopPropagation();
                  emulatorsStore.launch(item.index);
                }}
                class="px-2.5 py-1 text-[11px] font-bold rounded-lg bg-[#00b578]/15 text-[#00b578] border border-[#00b578]/30 hover:bg-[#00b578]/25 transition-all cursor-pointer active:scale-95"
              >
                Start
              </button>
            {/if}

            <!-- Modify Settings Gear -->
            <button
              type="button"
              title="Modify Instance Settings"
              onclick={(e) => {
                e.stopPropagation();
                openModify(item);
              }}
              class="p-1.5 rounded-lg text-text-muted hover:text-text-hover hover:bg-bg-card transition-colors cursor-pointer"
            >
              <Icon name="settings" size={13} />
            </button>

            <!-- Delete -->
            <button
              type="button"
              title="Delete Instance"
              onclick={(e) => {
                e.stopPropagation();
                promptDelete(item.index);
              }}
              class="p-1.5 rounded-lg text-text-muted hover:text-[#ff4d4f] hover:bg-[#ff4d4f]/10 transition-colors cursor-pointer"
            >
              <Icon name="trash" size={13} />
            </button>
          </div>
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
