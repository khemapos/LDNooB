<script lang="ts">
import { accountsStore } from "$lib/stores/accounts.svelte";
import { proxiesStore } from "$lib/stores/proxies.svelte";
import type { FacebookAccount } from "$lib/types";
import BaseModal from "../common/BaseModal.svelte";
import BaseTable, { type ColumnConfig } from "../common/BaseTable.svelte";
import ProxyInjectModal from "../modals/ProxyInjectModal.svelte";
import Icon from "../ui/Icon.svelte";

let showImportModal = $state(false);
let showProxyModal = $state(false);
let importInput = $state("");

let columns = $state<ColumnConfig[]>([
  { key: "uid", label: "UID / Account ID", visible: true, canHide: false, width: 170 },
  { key: "status", label: "Status", visible: true, canHide: true, width: 120, align: "center" },
  { key: "proxy", label: "Assigned Proxy", visible: true, canHide: true, width: 200 },
  { key: "twoFA", label: "2FA Secret", visible: true, canHide: true, width: 140 },
  { key: "actions", label: "Actions", visible: true, canHide: false, width: 100, align: "right" },
]);

function handleImport() {
  if (!importInput.trim()) return;
  accountsStore.batchImport(importInput.split("\n"));
  importInput = "";
  showImportModal = false;
}
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden">
  <!-- Accounts Command Toolbar -->
  <div
    class="flex flex-wrap items-center justify-between gap-3 p-3 bg-white/90 dark:bg-[#0e1018]/90 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl rounded-2xl shadow-xs"
  >
    <div class="flex items-center gap-2">
      <!-- Import Accounts Button (Brand Blue) -->
      <button
        type="button"
        onclick={() => (showImportModal = true)}
        class="flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-bold text-white bg-gradient-to-r from-[#1890ff] to-[#096dd9] hover:from-[#40a9ff] hover:to-[#1890ff] transition-all shadow-[0_2px_10px_rgba(24,144,255,0.3)] cursor-pointer"
      >
        <Icon name="plus" size={14} />
        <span>Import Accounts</span>
      </button>

      <button
        type="button"
        onclick={() => (showProxyModal = true)}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-100 dark:bg-white/[0.04] text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.08] transition-colors cursor-pointer"
      >
        <Icon name="network" size={12} />
        <span>Proxy Pool ({proxiesStore.proxies.length})</span>
      </button>
    </div>

    <div class="text-xs font-mono text-slate-500 dark:text-slate-400">
      Total Accounts: {accountsStore.accounts.length}
    </div>
  </div>

  <!-- Accounts Table -->
  <div class="flex-1 overflow-hidden">
    <BaseTable
      bind:columns
      items={accountsStore.accounts}
      bind:selectedKeys={accountsStore.selectedUids}
      itemKey="uid"
    >
      {#snippet rowSnippet(item: FacebookAccount, isSelected: boolean)}
        <!-- UID -->
        {#if columns.find(c => c.key === "uid")?.visible}
          <td class="py-2.5 px-3 font-mono font-bold text-slate-900 dark:text-white border-r border-slate-100 dark:border-white/[0.04]">
            {item.uid}
          </td>
        {/if}

        <!-- Status -->
        {#if columns.find(c => c.key === "status")?.visible}
          <td class="py-2.5 px-3 text-center border-r border-slate-100 dark:border-white/[0.04]">
            <span
              class="inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider bg-blue-500/10 text-blue-600 dark:text-blue-400 border border-blue-500/20"
            >
              {item.status}
            </span>
          </td>
        {/if}

        <!-- Proxy -->
        {#if columns.find(c => c.key === "proxy")?.visible}
          <td class="py-2.5 px-3 text-xs font-mono text-slate-600 dark:text-slate-400 border-r border-slate-100 dark:border-white/[0.04]">
            {item.proxy || "Direct (No Proxy)"}
          </td>
        {/if}

        <!-- 2FA -->
        {#if columns.find(c => c.key === "twoFA")?.visible}
          <td class="py-2.5 px-3 text-xs font-mono text-slate-500 dark:text-slate-400 border-r border-slate-100 dark:border-white/[0.04]">
            {item.twoFA ? "••••••••" : "-"}
          </td>
        {/if}

        <!-- Actions -->
        {#if columns.find(c => c.key === "actions")?.visible}
          <td
            class="py-2.5 px-3 text-right sticky right-0 z-10 shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.1)] dark:shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.4)] {isSelected
              ? 'bg-blue-500/[0.06] dark:bg-[#101520]'
              : 'bg-white dark:bg-[#0c0e15] group-hover:bg-slate-50 dark:group-hover:bg-[#12141f]'}"
            onclick={(e) => e.stopPropagation()}
          >
            <button
              type="button"
              title="Delete Account"
              onclick={() => accountsStore.removeAccount(item.uid)}
              class="p-1.5 rounded-lg text-slate-400 hover:text-rose-600 hover:bg-rose-500/10 transition-colors cursor-pointer"
            >
              <Icon name="trash" size={13} />
            </button>
          </td>
        {/if}
      {/snippet}
    </BaseTable>
  </div>
</div>

<!-- Import Modal -->
<BaseModal
  bind:open={showImportModal}
  title="Batch Import Facebook Accounts"
  subtitle="Format: UID|Password|2FA|Cookie (one account per line)"
  icon="users"
>
  <div class="space-y-3">
    <textarea
      rows="6"
      placeholder="1000847291029|MyPassword123|JBSWY3DPEHPK3PXP|sb=123..."
      bind:value={importInput}
      class="w-full p-3 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:border-blue-500 font-mono shadow-inner resize-none"
    ></textarea>
  </div>

  {#snippet footer()}
    <button
      type="button"
      onclick={() => (showImportModal = false)}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.12] transition-colors cursor-pointer"
    >
      Cancel
    </button>
    <button
      type="button"
      disabled={!importInput.trim()}
      onclick={handleImport}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-white bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-400 hover:to-indigo-500 transition-all shadow-sm cursor-pointer disabled:opacity-50"
    >
      Import Accounts
    </button>
  {/snippet}
</BaseModal>

<!-- Proxy Modal -->
<ProxyInjectModal bind:open={showProxyModal} />
