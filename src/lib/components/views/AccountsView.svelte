<script lang="ts">
import { accountsStore } from "$lib/stores/accounts.svelte";
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
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden font-sans select-none">
  <!-- Accounts Command Toolbar (Modern & Premium) -->
  <div
    class="flex flex-wrap items-center justify-between gap-3 p-2.5 px-3.5 bg-bg-panel/95 backdrop-blur-md border border-border-default rounded-2xl shadow-xs shrink-0"
  >
    <div class="flex items-center gap-2">
      <!-- Import Accounts Button (Brand Blue Gradient) -->
      <button
        type="button"
        onclick={() => (showImportModal = true)}
        class="inline-flex items-center justify-center gap-1.5 h-8.5 px-4 rounded-xl text-xs font-bold text-white bg-gradient-to-b from-[#2583ff] to-[#1877f2] hover:from-[#3890ff] hover:to-[#166fe5] active:scale-[0.98] border border-[#1877f2] shadow-[0_2px_10px_rgba(24,119,242,0.25),inset_0_1px_0_rgba(255,255,255,0.2)] transition-all cursor-pointer"
      >
        <Icon name="plus" size={13} />
        <span>Import Accounts</span>
      </button>

      <div class="h-4.5 w-px bg-border-default/80 mx-0.5"></div>

      <!-- Proxy Pool Action -->
      <button
        type="button"
        onclick={() => (showProxyModal = true)}
        class="inline-flex items-center gap-1.5 h-8.5 px-3 rounded-xl text-xs font-semibold bg-bg-card hover:bg-bg-card-hover border border-border-default hover:border-border-hover text-text-muted hover:text-text-hover transition-all cursor-pointer active:scale-95 shadow-xs"
      >
        <Icon name="network" size={12} class="text-[#1877f2]" />
        <span>Proxy Pool ({proxiesStore.proxies.length})</span>
      </button>
    </div>

    <!-- Right: Search & Telemetry -->
    <div class="flex items-center gap-3">
      <!-- Search Input -->
      <div class="relative flex items-center h-8.5 w-52 group">
        <input
          type="text"
          placeholder="Search accounts..."
          bind:value={accountSearchQuery}
          class="w-full h-8.5 pl-8 pr-7 text-xs font-medium rounded-xl border border-border-default hover:border-border-hover focus:border-[#1877f2] bg-bg-app text-text-default placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-[#1877f2]/20 transition-all duration-150"
        />
        <span class="absolute left-2.5 text-text-muted pointer-events-none group-focus-within:text-[#1877f2]">
          <Icon name="search" size={13} />
        </span>
        {#if accountSearchQuery}
          <button
            type="button"
            onclick={() => (accountSearchQuery = "")}
            class="absolute right-2 text-text-muted hover:text-text-hover p-0.5 rounded-md cursor-pointer flex items-center justify-center transition-colors"
          >
            <Icon name="close" size={11} />
          </button>
        {/if}
      </div>

      <div
        class="h-8.5 px-3 rounded-xl bg-bg-app border border-border-default text-xs font-mono text-text-muted flex items-center gap-1.5"
      >
        <span>Accounts:</span>
        <strong class="text-text-hover font-bold">{accountsStore.accounts.length}</strong>
      </div>
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
