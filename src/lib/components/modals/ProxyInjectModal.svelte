<script lang="ts">
import { proxiesStore } from "$lib/stores/proxies.svelte";
import BaseModal from "../common/BaseModal.svelte";

interface Props {
  open: boolean;
}

let { open = $bindable(false) }: Props = $props();

let proxyInput = $state("");
let isAdding = $state(false);

async function handleAdd() {
  if (!proxyInput.trim()) return;
  isAdding = true;
  try {
    const lines = proxyInput.trim().split("\n");
    for (const line of lines) {
      if (line.trim()) {
        await proxiesStore.addProxy(line.trim());
      }
    }
    proxyInput = "";
    open = false;
  } finally {
    isAdding = false;
  }
}
</script>

<BaseModal
  bind:open
  title="Add & Test Proxies"
  subtitle="Import proxy list (host:port or host:port:user:pass)"
  icon="network"
>
  <div class="space-y-4">
    <div class="space-y-1.5">
      <label for="proxy-text" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        Proxy Endpoints (One per line)
      </label>
      <textarea
        id="proxy-text"
        rows="5"
        placeholder="192.168.1.100:1080
127.0.0.1:9050:username:password"
        bind:value={proxyInput}
        class="w-full p-3 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:border-cyan-500 font-mono shadow-inner resize-none"
      ></textarea>
    </div>
  </div>

  {#snippet footer()}
    <button
      type="button"
      onclick={() => (open = false)}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.12] transition-colors cursor-pointer"
    >
      Cancel
    </button>
    <button
      type="button"
      disabled={!proxyInput.trim() || isAdding}
      onclick={handleAdd}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-sm cursor-pointer disabled:opacity-50"
    >
      {isAdding ? "Testing & Adding..." : "Add & Validate"}
    </button>
  {/snippet}
</BaseModal>
