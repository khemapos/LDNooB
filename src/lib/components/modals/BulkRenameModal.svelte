<script lang="ts">
  import BaseModal from "../common/BaseModal.svelte";
  import { emulatorsStore } from "$lib/stores/emulators.svelte";

  interface Props {
    open: boolean;
  }

  let { open = $bindable(false) }: Props = $props();

  let prefix = $state("Profile");
  let startNumber = $state(1);
  let isSubmitting = $state(false);

  async function handleRename() {
    isSubmitting = true;
    try {
      let current = startNumber;
      for (const index of emulatorsStore.selectedIndices) {
        const title = `${prefix}-${current}`;
        await emulatorsStore.renameInstance(index, title);
        current++;
      }
      open = false;
    } finally {
      isSubmitting = false;
    }
  }
</script>

<BaseModal
  bind:open
  title="Bulk Rename Emulators"
  subtitle="Rename {emulatorsStore.selectedIndices.length} selected instances using a pattern"
  icon="edit"
>
  <div class="space-y-4">
    <div class="space-y-1.5">
      <label for="prefix-name" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        Name Prefix
      </label>
      <input
        id="prefix-name"
        type="text"
        placeholder="e.g. Account"
        bind:value={prefix}
        class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono shadow-inner"
      />
    </div>

    <div class="space-y-1.5">
      <label for="start-num" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        Start Index Number
      </label>
      <input
        id="start-num"
        type="number"
        min="1"
        bind:value={startNumber}
        class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono shadow-inner"
      />
    </div>

    <div class="p-3 bg-slate-100 dark:bg-white/[0.04] rounded-xl text-[11px] font-mono text-slate-500 dark:text-slate-400">
      Preview: {prefix}-{startNumber}, {prefix}-{startNumber + 1}, ...
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
      disabled={isSubmitting}
      onclick={handleRename}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-sm cursor-pointer disabled:opacity-50"
    >
      {isSubmitting ? "Renaming..." : "Apply Rename"}
    </button>
  {/snippet}
</BaseModal>
