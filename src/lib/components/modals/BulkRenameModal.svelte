<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import BaseModal from "../common/BaseModal.svelte";
import CustomButton from "../common/CustomButton.svelte";
import CustomInput from "../common/CustomInput.svelte";

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
  <div class="space-y-4 font-sans">
    <CustomInput
      label="Name Prefix"
      placeholder="e.g. Account"
      bind:value={prefix}
      icon="edit"
    />

    <CustomInput
      label="Start Index Number"
      type="number"
      bind:value={startNumber}
      icon="grid"
    />

    <!-- Preview Box -->
    <div class="p-3.5 bg-[#0e0f11] border border-[#25272b] rounded-xl text-[11px] font-mono text-[#8c8c8c]">
      Preview: <span class="text-[#00b578] font-bold">{prefix}-{startNumber}</span>, <span class="text-[#00b578] font-bold">{prefix}-{startNumber + 1}</span>, ...
    </div>
  </div>

  {#snippet footer()}
    <CustomButton
      variant="secondary"
      size="md"
      onclick={() => (open = false)}
    >
      Cancel
    </CustomButton>

    <CustomButton
      variant="primary"
      size="md"
      disabled={isSubmitting}
      loading={isSubmitting}
      onclick={handleRename}
    >
      Apply Rename
    </CustomButton>
  {/snippet}
</BaseModal>
