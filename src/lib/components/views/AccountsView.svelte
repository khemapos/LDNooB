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

let columns = $state<ColumnConfig[]>([
  { key: "index", label: "Index", visible: true, canHide: true, width: 65, align: "center" },
  { key: "hostEmulator", label: "Host Emulator", visible: true, canHide: true, width: 140 },
  { key: "uid", label: "UID / Name", visible: true, canHide: false, width: 140 },
  { key: "profileName", label: "Profile Name", visible: true, canHide: true, width: 130 },
  { key: "password", label: "Password", visible: true, canHide: true, width: 120 },
  { key: "twoFA", label: "2FA Key", visible: true, canHide: true, width: 110 },
  { key: "proxy", label: "Proxy", visible: true, canHide: true, width: 140 },
  { key: "status", label: "FB Status", visible: true, canHide: true, width: 100, align: "center" },
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
    class="flex flex-wrap items-center justify-between gap-3 p-3 bg-[#141517] border border-[#25272b] rounded-2xl shadow-xs"
  >
    <div class="flex items-center gap-2">
      <!-- Import Accounts Button (Brand Blue) -->
      <CustomButton
        variant="blue"
        size="md"
        onclick={() => (showImportModal = true)}
      >
        <Icon name="plus" size={14} />
        <span>Import Accounts</span>
      </CustomButton>

      <CustomButton
        variant="secondary"
        size="sm"
        onclick={() => (showProxyModal = true)}
      >
        <Icon name="network" size={12} />
        <span>Proxy Pool ({proxiesStore.proxies.length})</span>
      </CustomButton>
    </div>

    <div class="text-xs font-mono text-[#8c8c8c]">
      Total Accounts: <strong class="text-white">{accountsStore.accounts.length}</strong>
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
      {#snippet rowSnippet(item: FacebookAccount, isSelected: boolean, index: number)}
        <!-- Index -->
        {#if columns.find(c => c.key === "index")?.visible}
          <td class="py-2.5 px-3 text-center font-mono font-bold text-[#8c8c8c] border-r border-[#25272b]/30">
            {index + 1}
          </td>
        {/if}

        <!-- Host Emulator -->
        {#if columns.find(c => c.key === "hostEmulator")?.visible}
          <td class="py-2.5 px-3 font-sans text-xs text-[#8c8c8c] border-r border-[#25272b]/30">
            {#if item.emuIndex >= 0}
              <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-[#18191c] text-[#00b578] font-mono text-[11px]">
                #{item.emuIndex}
              </span>
            {:else}
              <span class="text-[#8c8c8c] italic text-[11px]">Unassigned</span>
            {/if}
          </td>
        {/if}

        <!-- UID -->
        {#if columns.find(c => c.key === "uid")?.visible}
          <td class="py-2.5 px-3 font-mono font-bold text-white border-r border-[#25272b]/30">
            {item.uid}
          </td>
        {/if}

        <!-- Profile Name -->
        {#if columns.find(c => c.key === "profileName")?.visible}
          <td class="py-2.5 px-3 text-[#d9d9d9] font-sans text-xs border-r border-[#25272b]/30">
            {item.username || "-"}
          </td>
        {/if}

        <!-- Password -->
        {#if columns.find(c => c.key === "password")?.visible}
          <td class="py-2.5 px-3 font-mono text-xs text-[#8c8c8c] border-r border-[#25272b]/30">
            ••••••••
          </td>
        {/if}

        <!-- 2FA -->
        {#if columns.find(c => c.key === "twoFA")?.visible}
          <td class="py-2.5 px-3 font-mono text-xs text-[#8c8c8c] border-r border-[#25272b]/30">
            {item.twoFA ? "••••••••" : "-"}
          </td>
        {/if}

        <!-- Proxy -->
        {#if columns.find(c => c.key === "proxy")?.visible}
          <td class="py-2.5 px-3 font-mono text-xs text-[#8c8c8c] border-r border-[#25272b]/30">
            {item.proxy || "Direct"}
          </td>
        {/if}

        <!-- Status -->
        {#if columns.find(c => c.key === "status")?.visible}
          <td class="py-2.5 px-3 text-center border-r border-[#25272b]/30">
            <span
              class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider bg-[#1877f2]/10 text-[#1877f2] border border-[#1877f2]/20"
            >
              {item.status}
            </span>
          </td>
        {/if}

        <!-- Actions -->
        {#if columns.find(c => c.key === "actions")?.visible}
          <td
            class="py-2.5 px-3 text-right sticky right-0 z-10 shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.3)] {isSelected
              ? 'bg-[#121c18]'
              : 'bg-[#141517] group-hover:bg-[#1f2125]'}"
            onclick={(e) => e.stopPropagation()}
          >
            <button
              type="button"
              title="Delete Account"
              onclick={() => accountsStore.removeAccount(item.uid)}
              class="p-1.5 rounded-lg text-[#8c8c8c] hover:text-[#ff4d4f] hover:bg-[#ff4d4f]/10 transition-colors cursor-pointer"
            >
              <Icon name="trash" size={13} />
            </button>
          </td>
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
  <div class="space-y-3">
    <textarea
      rows="6"
      placeholder="1000847291029|MyPassword123|JBSWY3DPEHPK3PXP|sb=123..."
      bind:value={importInput}
      class="w-full p-3 text-xs rounded-xl bg-[#0e0f11] border border-[#25272b] text-white placeholder-[#8c8c8c] focus:outline-none focus:border-[#1877f2] font-mono shadow-inner resize-none"
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
