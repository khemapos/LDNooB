<script lang="ts">
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import BaseModal from "../common/BaseModal.svelte";
import CustomButton from "../common/CustomButton.svelte";
import CustomInput from "../common/CustomInput.svelte";
import CustomSelect from "../common/CustomSelect.svelte";

interface Props {
  open: boolean;
}

let { open = $bindable(false) }: Props = $props();

let name = $state("");
let count = $state(1);
let isCloning = $state(false);
let cloneFromIndex = $state<number>(0);
let isSubmitting = $state(false);

let cloneOptions = $derived(
  emulatorsStore.instances.map((i) => ({
    value: i.index,
    label: `#${i.index} - ${i.name}`,
  }))
);

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
  title={isCloning ? "Clone Emulator Instance" : "Create New LDPlayer"}
  subtitle="Configure instance profile, naming pattern, and hardware"
  icon="plus"
>
  <div class="space-y-4 font-sans">
    <!-- Mode Switcher -->
    <div class="grid grid-cols-2 gap-2 p-1 bg-[#0e0f11] border border-[#25272b] rounded-xl">
      <button
        type="button"
        onclick={() => (isCloning = false)}
        class="py-2 text-xs font-bold rounded-lg transition-all cursor-pointer {!isCloning
          ? 'bg-[#18191c] text-[#00b578] shadow-xs'
          : 'text-[#8c8c8c] hover:text-white'}"
      >
        New Blank Instance
      </button>
      <button
        type="button"
        onclick={() => (isCloning = true)}
        class="py-2 text-xs font-bold rounded-lg transition-all cursor-pointer {isCloning
          ? 'bg-[#18191c] text-[#00b578] shadow-xs'
          : 'text-[#8c8c8c] hover:text-white'}"
      >
        Clone Existing Instance
      </button>
    </div>

    <!-- Instance Name Input -->
    <CustomInput
      label="Instance Name"
      placeholder="e.g. LDPlayer-Profile-1"
      bind:value={name}
      icon="cube"
    />

    {#if isCloning}
      <!-- Clone Source Selector -->
      <CustomSelect
        label="Source Emulator to Clone From"
        bind:value={cloneFromIndex}
        options={cloneOptions}
        icon="copy"
      />
    {:else}
      <!-- Quantity Input -->
      <CustomInput
        label="Batch Quantity"
        type="number"
        bind:value={count}
        icon="grid"
      />
    {/if}
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
      disabled={!name.trim() || isSubmitting}
      loading={isSubmitting}
      onclick={handleCreate}
    >
      {isCloning ? "Clone Instance" : "Create LDPlayer"}
    </CustomButton>
  {/snippet}
</BaseModal>
