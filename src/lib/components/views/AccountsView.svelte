<script lang="ts">
import { accountsStore } from "$lib/stores/accounts.svelte";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { logsStore } from "$lib/stores/logs.svelte";
import { proxiesStore } from "$lib/stores/proxies.svelte";
import type { FacebookAccount } from "$lib/types";
import BaseModal from "../common/BaseModal.svelte";
import BaseTable, { type ColumnConfig } from "../common/BaseTable.svelte";
import CustomButton from "../common/CustomButton.svelte";
import ProxyInjectModal from "../modals/ProxyInjectModal.svelte";
import Icon from "../ui/Icon.svelte";

let showImportModal = $state(false);
let showProxyModal = $state(false);
let importInput = $state("");
let accountSearchQuery = $state("");

let columns = $state<ColumnConfig[]>([
  { key: "index", label: "Index", visible: true, canHide: true, width: 60, align: "center" },
  { key: "hostEmulator", label: "Host Emulator", visible: true, canHide: true, width: 140 },
  { key: "uid", label: "UID / Name", visible: true, canHide: false, width: 150 },
  { key: "profileName", label: "Profile Name", visible: true, canHide: true, width: 140 },
  { key: "password", label: "Password", visible: true, canHide: true, width: 120 },
  { key: "twoFA", label: "2FA Key", visible: true, canHide: true, width: 120 },
  { key: "proxy", label: "Proxy", visible: true, canHide: true, width: 140 },
  { key: "status", label: "FB Status", visible: true, canHide: true, width: 110, align: "center" },
  { key: "actions", label: "Actions", visible: true, canHide: false, width: 80, align: "right" },
]);

let filteredAccounts = $derived(
  accountsStore.accounts.filter((acc) => {
    const q = accountSearchQuery.toLowerCase().trim();
    if (!q) return true;
    return (
      acc.uid.toLowerCase().includes(q) ||
      acc.username?.toLowerCase().includes(q) ||
      acc.proxy?.toLowerCase().includes(q)
    );
  })
);

function handleImport() {
  if (!importInput.trim()) return;
  accountsStore.batchImport(importInput.split("\n"));
  importInput = "";
  showImportModal = false;
}
let threads = $state(3);
let isRunningTasks = $state(false);
let isStoppingTasks = $state(false);

async function handleStartTasks() {
  if (isRunningTasks) return;
  isRunningTasks = true;
  isStoppingTasks = false;
  const targetAccounts =
    accountsStore.selectedUids.length > 0
      ? accountsStore.accounts.filter((a) => accountsStore.selectedUids.includes(a.uid))
      : accountsStore.accounts;

  if (targetAccounts.length === 0) {
    logsStore.warn("Workflow", "No accounts selected or available to run");
    isRunningTasks = false;
    return;
  }

  logsStore.info(
    "Workflow",
    `Starting workflow pool for ${targetAccounts.length} account(s) with ${threads} thread(s)`
  );

  // Run parallel workers based on thread count
  const queue = [...targetAccounts];
  const workerCount = Math.min(threads, queue.length);

  const workers = Array.from({ length: workerCount }, async () => {
    while (queue.length > 0 && isRunningTasks && !isStoppingTasks) {
      const acc = queue.shift();
      if (!acc) break;
      try {
        acc.status = "logged_in";
        logsStore.info("Account", `Processing account ${acc.uid}...`);
        await new Promise((r) => setTimeout(r, 1200));
      } catch (e) {
        acc.status = "error";
        logsStore.error("Account", `Account ${acc.uid} failed: ${e}`);
      }
    }
  });

  try {
    await Promise.all(workers);
    logsStore.success("Workflow", "Workflow execution completed");
  } finally {
    isRunningTasks = false;
    isStoppingTasks = false;
  }
}

function handleStopTasks() {
  isStoppingTasks = true;
  isRunningTasks = false;
  logsStore.warn("Workflow", "Stopping active task execution...");
}
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden font-sans select-none">
  <!-- Accounts Command Toolbar (100% Parity with D:\ldremote) -->
  <div
    class="flex flex-wrap items-center justify-between gap-3 p-2.5 px-3.5 bg-bg-panel/95 backdrop-blur-md border border-border-default rounded-2xl shadow-xs shrink-0"
  >
    <!-- Left: Search, Auto/Cols/Arrange, & Visibility Controls -->
    <div class="flex items-center gap-2.5">
      <!-- Search Input -->
      <div class="relative flex items-center h-8.5 w-48 group">
        <input
          type="text"
          placeholder="Search account..."
          bind:value={accountSearchQuery}
          class="w-full h-8.5 pl-8 pr-7 text-xs font-medium rounded-xl border border-border-default hover:border-border-hover focus:border-[#00b578] bg-bg-app text-text-default placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-[#00b578]/20 transition-all duration-150"
        />
        <span
          class="absolute left-2.5 text-text-muted pointer-events-none group-focus-within:text-[#00b578]"
        >
          <Icon name="search" size={13} />
        </span>
        {#if accountSearchQuery}
          <button
            type="button"
            aria-label="Clear Search"
            onclick={() => (accountSearchQuery = "")}
            class="absolute right-2 text-text-muted hover:text-text-hover p-0.5 rounded-md cursor-pointer flex items-center justify-center transition-colors"
          >
            <Icon name="close" size={11} />
          </button>
        {/if}
      </div>

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

    <!-- Right: Stop All, Stop Tasks, Threads / Start, & Refresh -->
    <div class="flex items-center gap-2">
      <!-- Stop All Emulators Button -->
      <button
        type="button"
        onclick={() => emulatorsStore.quitAll()}
        disabled={emulatorsStore.runningCount === 0}
        class="inline-flex items-center gap-1.5 h-8.5 px-3 rounded-xl text-xs font-bold bg-bg-card hover:bg-[#ff4d4f]/10 border border-border-default hover:border-[#ff4d4f]/40 text-text-default hover:text-[#ff4d4f] disabled:opacity-40 disabled:pointer-events-none transition-all cursor-pointer active:scale-95 shadow-xs"
        title="Stop all running emulators"
      >
        <Icon name="stop" size={13} class="text-[#ff4d4f]" />
        <span>Stop All</span>
      </button>

      <!-- Stop Tasks Button -->
      <button
        type="button"
        onclick={handleStopTasks}
        disabled={!isRunningTasks}
        class="inline-flex items-center gap-1.5 h-8.5 px-3 rounded-xl text-xs font-bold bg-bg-card hover:bg-[#ff4d4f]/10 border border-border-default hover:border-[#ff4d4f]/40 text-text-default hover:text-[#ff4d4f] disabled:opacity-40 disabled:pointer-events-none transition-all cursor-pointer active:scale-95 shadow-xs"
        title="Stop running background tasks"
      >
        <Icon name="close" size={13} class="text-[#ff4d4f]" />
        <span>Stop Tasks</span>
      </button>

      <!-- Threads & Start Tasks Multi-segment Pill -->
      <div
        class="flex items-center h-8.5 rounded-xl border border-border-default bg-bg-app overflow-hidden shadow-xs"
      >
        <!-- Threads Input -->
        <div class="flex items-center pl-3 pr-2 h-full">
          <span
            class="select-none text-[10px] font-extrabold uppercase tracking-wider text-text-muted mr-1.5"
          >
            Threads
          </span>
          <input
            type="number"
            min="1"
            max="50"
            bind:value={threads}
            class="w-7 bg-transparent border-none outline-none text-center text-xs font-mono font-bold text-text-hover p-0 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
          />
        </div>

        <div class="h-4 border-r border-border-default/60"></div>

        <!-- Start Tasks Button -->
        <button
          type="button"
          disabled={isRunningTasks}
          onclick={handleStartTasks}
          title="Start automation workflow pool"
          class="flex items-center gap-1.5 px-3.5 h-full bg-bg-card/40 hover:bg-bg-card hover:text-[#00b578] text-text-default text-xs font-bold transition-colors cursor-pointer border-none disabled:opacity-50"
        >
          <Icon name="play" size={12} class="text-[#00b578]" />
          <span>{isRunningTasks ? "Running..." : "Start"}</span>
        </button>
      </div>

      <!-- Refresh Button -->
      <button
        type="button"
        title="Refresh accounts & fleet"
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

  <!-- Accounts Table -->
  <div class="flex-1 overflow-hidden flex flex-col min-h-0">
    <BaseTable
      bind:columns
      items={filteredAccounts}
      bind:selectedKeys={accountsStore.selectedUids}
      itemKey="uid"
    >
      {#snippet renderCell(colKey: string, item: FacebookAccount, index: number)}
        {#if colKey === "index"}
          <span class="font-mono font-bold text-text-muted text-center block">
            {index + 1}
          </span>
        {:else if colKey === "hostEmulator"}
          {#if item.emuIndex >= 0}
            <span
              class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-bg-card text-[#00b578] font-mono text-[11px] border border-border-default"
            >
              #{item.emuIndex}
            </span>
          {:else}
            <span class="text-text-muted italic text-[11px]">Unassigned</span>
          {/if}
        {:else if colKey === "uid"}
          <span class="font-mono font-bold text-text-hover truncate block">
            {item.uid}
          </span>
        {:else if colKey === "profileName"}
          <span class="text-text-default text-xs truncate block">
            {item.username || "-"}
          </span>
        {:else if colKey === "password"}
          <span class="font-mono text-xs text-text-muted">
            ••••••••
          </span>
        {:else if colKey === "twoFA"}
          <span class="font-mono text-xs text-text-muted">
            {item.twoFA ? "••••••••" : "-"}
          </span>
        {:else if colKey === "proxy"}
          <span class="font-mono text-xs text-text-muted truncate block">
            {item.proxy || "Direct"}
          </span>
        {:else if colKey === "status"}
          <div class="flex items-center justify-center">
            <span
              class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider bg-[#1877f2]/10 text-[#1877f2] border border-[#1877f2]/20"
            >
              {item.status}
            </span>
          </div>
        {:else if colKey === "actions"}
          <div class="flex items-center justify-end">
            <button
              type="button"
              title="Delete Account"
              onclick={(e) => {
                e.stopPropagation();
                accountsStore.removeAccount(item.uid);
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
<BaseModal
  bind:open={showImportModal}
  title="Batch Import Facebook Accounts"
  subtitle="Format: UID|Password|2FA|Cookie (one account per line)"
  icon="users"
>
  <div class="space-y-3 font-sans">
    <textarea
      rows="6"
      placeholder="1000847291029|MyPassword123|JBSWY3DPEHPK3PXP|sb=123..."
      bind:value={importInput}
      class="w-full p-3 text-xs rounded-xl bg-bg-card border border-border-default text-text-default placeholder:text-text-muted focus:outline-none focus:border-[#1877f2] font-mono shadow-inner resize-none"
    ></textarea>
  </div>

  {#snippet footer()}
    <CustomButton
      variant="secondary"
      size="md"
      onclick={() => (showImportModal = false)}
    >
      Cancel
    </CustomButton>

    <CustomButton
      variant="blue"
      size="md"
      disabled={!importInput.trim()}
      onclick={handleImport}
    >
      Import Accounts
    </CustomButton>
  {/snippet}
</BaseModal>

<ProxyInjectModal bind:open={showProxyModal} />
