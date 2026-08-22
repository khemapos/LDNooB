<script lang="ts">
import { accountsStore } from "$lib/stores/accounts.svelte";
import { proxiesStore } from "$lib/stores/proxies.svelte";
import type { FacebookAccount } from "$lib/types";
import BaseModal from "../common/BaseModal.svelte";
import BaseTable, { type TableColumn } from "../common/BaseTable.svelte";
import ProxyInjectModal from "../modals/ProxyInjectModal.svelte";
import Icon from "../ui/Icon.svelte";

let showImportModal = $state(false);
let showProxyModal = $state(false);
let importInput = $state("");

const columns: TableColumn[] = [
  { key: "uid", label: "UID / Account ID", width: "w-40" },
  { key: "status", label: "Status", width: "w-28", align: "center" },
  { key: "proxy", label: "Proxy", width: "w-48" },
  { key: "twoFA", label: "2FA Secret", width: "w-36" },
  { key: "actions", label: "Actions", width: "w-24", align: "right" },
];

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
    class="flex flex-wrap items-center justify-between gap-3 p-3 bg-white/80 dark:bg-[#0e1018]/90 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl rounded-2xl shadow-xs"
  >
    <div class="flex items-center gap-2">
      <button
        type="button"
        onclick={() => (showImportModal = true)}
        class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-semibold text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-xs cursor-pointer"
      >
        <Icon name="plus" size={14} />
        <span>Import Accounts</span>
      </button>

      <button
        type="button"
        onclick={() => (showProxyModal = true)}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-medium bg-slate-100 dark:bg-white/[0.04] text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.08] transition-colors cursor-pointer"
      >
        <Icon name="network" size={12} />
        <span>Manage Proxies ({proxiesStore.proxies.length})</span>
      </button>
    </div>

    <div class="text-xs font-mono text-slate-500 dark:text-slate-400">
      Total Accounts: {accountsStore.accounts.length}
    </div>
  </div>

  <!-- Accounts Table -->
  <div class="flex-1 overflow-hidden">
    <BaseTable
      {columns}
      items={accountsStore.accounts}
      itemKey="uid"
    >
      {#snippet rowSnippet(item: FacebookAccount)}
        <!-- UID -->
        <td class="px-3 py-2 font-mono font-bold text-slate-900 dark:text-white">
          {item.uid}
        </td>

        <!-- Status -->
        <td class="px-3 py-2 text-center">
          <span
            class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium bg-cyan-500/10 text-cyan-700 dark:text-cyan-300 border border-cyan-500/20"
          >
            {item.status}
          </span>
        </td>

        <!-- Proxy -->
        <td class="px-3 py-2 text-xs font-mono text-slate-500 dark:text-slate-400">
          {item.proxy || "Direct (No Proxy)"}
        </td>

        <!-- 2FA -->
        <td class="px-3 py-2 text-xs font-mono text-slate-500 dark:text-slate-400">
          {item.twoFA ? "••••••••" : "-"}
        </td>

        <!-- Actions -->
        <td class="px-3 py-2 text-right">
          <button
            type="button"
            title="Delete Account"
            onclick={() => accountsStore.removeAccount(item.uid)}
            class="p-1.5 rounded-lg text-slate-400 hover:text-rose-600 hover:bg-rose-500/10 transition-colors cursor-pointer"
          >
            <Icon name="trash" size={13} />
          </button>
        </td>
      {/snippet}
    </BaseTable>
  </div>
</div>

<!-- Import Modal -->
<BaseModal
  bind:open={showImportModal}
  title="Batch Import Accounts"
  subtitle="Format: UID|Password|2FA|Cookie (one per line)"
  icon="users"
>
  <div class="space-y-3">
    <textarea
      rows="6"
      placeholder="1000847291029|MyPassword123|JBSWY3DPEHPK3PXP|sb=123..."
      bind:value={importInput}
      class="w-full p-3 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:border-cyan-500 font-mono shadow-inner resize-none"
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
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-sm cursor-pointer disabled:opacity-50"
    >
      Import Accounts
    </button>
  {/snippet}
</BaseModal>

<!-- Proxy Modal -->
<ProxyInjectModal bind:open={showProxyModal} />
