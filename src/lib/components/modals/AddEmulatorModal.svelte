<script lang="ts">
  import BaseModal from "../common/BaseModal.svelte";
  import { emulatorsStore } from "$lib/stores/emulators.svelte";

  interface Props {
    open: boolean;
  }

  let { open = $bindable(false) }: Props = $props();

  let name = $state("");
  let count = $state(1);
  let isCloning = $state(false);
  let cloneFromIndex = $state<number>(0);
  let isSubmitting = $state(false);

  async function handleCreate() {
    if (!name.trim()) return;
    isSubmitting = true;
    try {
      if (isCloning) {
        await emulatorsStore.copyInstance(name.trim(), cloneFromIndex);
      } else {
        for (let i = 0; i < count; i++) {
          const instanceName = count > 1 ? `${name.trim()}_${i + 1}` : name.trim();
          await emulatorsStore.addInstance(instanceName);
        }
      }
      open = false;
      name = "";
      count = 1;
      isCloning = false;
    } finally {
      isSubmitting = false;
    }
  }
</script>

<BaseModal
  bind:open
  title={isCloning ? "Clone Emulator Instance" : "Create New Emulator"}
  subtitle="Configure new instance profile and settings"
  icon="plus"
>
  <div class="space-y-4">
    <!-- Creation Mode Switch -->
    <div class="grid grid-cols-2 gap-2 p-1 bg-slate-100 dark:bg-white/[0.04] rounded-xl">
      <button
        type="button"
        onclick={() => (isCloning = false)}
        class="py-1.5 text-xs font-semibold rounded-lg transition-all cursor-pointer {!isCloning
          ? 'bg-white dark:bg-[#1a1d2d] text-cyan-600 dark:text-cyan-400 shadow-xs'
          : 'text-slate-500 hover:text-slate-800 dark:hover:text-slate-200'}"
      >
        New Blank Instance
      </button>
      <button
        type="button"
        onclick={() => (isCloning = true)}
        class="py-1.5 text-xs font-semibold rounded-lg transition-all cursor-pointer {isCloning
          ? 'bg-white dark:bg-[#1a1d2d] text-cyan-600 dark:text-cyan-400 shadow-xs'
          : 'text-slate-500 hover:text-slate-800 dark:hover:text-slate-200'}"
      >
        Clone Existing Instance
      </button>
    </div>

    <!-- Name Input -->
    <div class="space-y-1.5">
      <label for="emu-name" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        Instance Name
      </label>
      <input
        id="emu-name"
        type="text"
        placeholder="e.g. LDPlayer-Profile-1"
        bind:value={name}
        class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:border-cyan-500 font-mono shadow-inner"
      />
    </div>

    {#if isCloning}
      <!-- Clone Source Selector -->
      <div class="space-y-1.5">
        <label for="clone-src" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
          Source Emulator to Clone From
        </label>
        <select
          id="clone-src"
          bind:value={cloneFromIndex}
          class="w-full px-3 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono"
        >
          {#each emulatorsStore.instances as inst}
            <option value={inst.index}>#{inst.index} - {inst.name}</option>
          {/each}
        </select>
      </div>
    {:else}
      <!-- Batch Quantity -->
      <div class="space-y-1.5">
        <label for="batch-count" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
          Batch Quantity
        </label>
        <input
          id="batch-count"
          type="number"
          min="1"
          max="20"
          bind:value={count}
          class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono shadow-inner"
        />
      </div>
    {/if}
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
      disabled={!name.trim() || isSubmitting}
      onclick={handleCreate}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-sm cursor-pointer disabled:opacity-50"
    >
      {isSubmitting ? "Creating..." : isCloning ? "Clone Instance" : "Create Instance"}
    </button>
  {/snippet}
</BaseModal>
