<script lang="ts">
import { proxiesStore } from "$lib/stores/proxies.svelte";
import BaseModal from "../common/BaseModal.svelte";
import CustomButton from "../common/CustomButton.svelte";

interface Props {
  open: boolean;
}

let { open = $bindable(false) }: Props = $props();

let proxyInput = $state("");

async function handleAdd() {
  if (!proxyInput.trim()) return;
  const lines = proxyInput.split("\n");
  for (const line of lines) {
    if (line.trim()) {
      await proxiesStore.addProxy(line.trim());
    }
  }
  proxyInput = "";
}
</script>

<BaseModal
  bind:open
  title="Proxy Pool & Network Manager"
  subtitle="Format: IP:Port or IP:Port:User:Pass (one proxy per line)"
  icon="network"
>
  <div class="space-y-4 font-sans">
    <!-- Paste Box -->
    <div class="flex flex-col gap-1.5 text-left">
      <label
        for="proxy_input"
        class="text-[9px] font-extrabold uppercase tracking-widest text-[#8c8c8c]"
      >
        Import New Proxies
      </label>
      <textarea
        id="proxy_input"
        rows="4"
        placeholder="192.168.1.100:8080&#10;10.0.0.5:3128:admin:password"
        bind:value={proxyInput}
        class="w-full p-3 text-xs rounded-xl bg-[#0e0f11] border border-[#25272b] text-white placeholder-[#8c8c8c] focus:outline-none focus:border-[#00b578] font-mono shadow-inner resize-none"
      ></textarea>
      <div class="flex justify-end pt-1">
        <CustomButton
          variant="primary"
          size="sm"
          disabled={!proxyInput.trim()}
          onclick={handleAdd}
        >
          Add to Pool
        </CustomButton>
      </div>
    </div>

    <!-- Active Proxies List -->
    <div class="border-t border-[#25272b] pt-3">
      <div class="flex items-center justify-between mb-2">
        <span class="text-[10px] font-extrabold uppercase tracking-widest text-[#8c8c8c]">
          Proxy Pool ({proxiesStore.proxies.length})
        </span>
        <CustomButton
          variant="secondary"
          size="xs"
          loading={proxiesStore.isLoading}
          onclick={async () => {
            for (const p of proxiesStore.proxies) {
              await proxiesStore.testProxy(p.id);
            }
          }}
        >
          Test All
        </CustomButton>
      </div>

      <div class="max-h-48 overflow-y-auto space-y-1.5 pr-1 font-mono text-xs">
        {#if proxiesStore.proxies.length === 0}
          <div class="py-6 text-center text-[#8c8c8c] italic text-[11px]">
            No proxies in pool. Import above to assign to instances.
          </div>
        {:else}
          {#each proxiesStore.proxies as p}
            <div
              class="flex items-center justify-between p-2.5 rounded-xl bg-[#0e0f11] border border-[#25272b]"
            >
              <div class="flex items-center gap-2 truncate">
                <span
                  class="w-2 h-2 rounded-full {p.status === 'active'
                    ? 'bg-[#00b578]'
                    : p.status === 'testing'
                      ? 'bg-amber-400 animate-pulse'
                      : p.status === 'error'
                        ? 'bg-[#ff4d4f]'
                        : 'bg-[#8c8c8c]'}"
                ></span>
                <span class="truncate text-white font-medium">{p.host}:{p.port}</span>
              </div>
              <div class="flex items-center gap-2">
                <span
                  class="text-[11px] {p.latency_ms && p.latency_ms < 200
                    ? 'text-[#00b578]'
                    : 'text-[#8c8c8c]'}"
                >
                  {p.latency_ms !== undefined ? `${p.latency_ms}ms` : "-"}
                </span>
                <button
                  type="button"
                  title="Remove Proxy"
                  onclick={() => proxiesStore.removeProxy(p.id)}
                  class="p-1 text-[#8c8c8c] hover:text-[#ff4d4f] transition-colors cursor-pointer"
                >
                  &times;
                </button>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>

  {#snippet footer()}
    <CustomButton
      variant="secondary"
      size="md"
      onclick={() => (open = false)}
    >
      Done
    </CustomButton>
  {/snippet}
</BaseModal>
