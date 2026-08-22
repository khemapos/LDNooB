<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import type { Emulator } from "$lib/types";
import BaseModal from "../common/BaseModal.svelte";
import CustomButton from "../common/CustomButton.svelte";
import CustomSelect from "../common/CustomSelect.svelte";

interface Props {
  open: boolean;
  emulator: Emulator | null;
}

let { open = $bindable(false), emulator = null }: Props = $props();

let resolutionPreset = $state("720x1280x320");
let cpu = $state(2);
let memory = $state(2048);
let isSaving = $state(false);

const resolutionOptions = [
  { value: "720x1280x320", label: "Phone: 720 × 1280 (320 DPI)" },
  { value: "540x960x240", label: "Phone: 540 × 960 (240 DPI)" },
  { value: "1080x1920x480", label: "FHD Phone: 1080 × 1920 (480 DPI)" },
  { value: "1280x720x240", label: "Tablet: 1280 × 720 (240 DPI)" },
];

const cpuOptions = [
  { value: 1, label: "1 Core" },
  { value: 2, label: "2 Cores (Recommended)" },
  { value: 4, label: "4 Cores" },
  { value: 8, label: "8 Cores" },
];

const memoryOptions = [
  { value: 1024, label: "1024 MB (1 GB)" },
  { value: 2048, label: "2048 MB (2 GB - Recommended)" },
  { value: 3072, label: "3072 MB (3 GB)" },
  { value: 4096, label: "4096 MB (4 GB)" },
];

$effect(() => {
  if (emulator) {
    cpu = emulator.cpu || 2;
    memory = emulator.memory || 2048;
    resolutionPreset = `${emulator.width}x${emulator.height}x${emulator.dpi}`;
  }
});

async function handleSave() {
  if (!emulator) return;
  isSaving = true;

  const [widthStr, heightStr, dpiStr] = resolutionPreset.split("x");
  const width = parseInt(widthStr, 10);
  const height = parseInt(heightStr, 10);
  const dpi = parseInt(dpiStr, 10);

  try {
    await invoke("modify_emulator_settings", {
      ldPath: settingsStore.settings.ldplayerPath,
      index: emulator.index,
      resolution: [width, height, dpi],
      cpu,
      memory,
    });

    logsStore.info("Settings", `Updated settings for #${emulator.index} (${emulator.name})`);
    await emulatorsStore.refresh();
    open = false;
  } catch (err: any) {
    logsStore.error(
      "Settings",
      `Failed to update settings for #${emulator.index}: ${err.toString()}`
    );
  } finally {
    isSaving = false;
  }
}
</script>

<BaseModal
  bind:open
  title="Modify Emulator Settings"
  subtitle={emulator
    ? `#${emulator.index} - ${emulator.name}`
    : "Instance hardware parameters"}
  icon="settings"
>
  <div class="space-y-4 font-sans">
    <!-- Resolution Preset -->
    <CustomSelect
      label="Resolution & DPI"
      bind:value={resolutionPreset}
      options={resolutionOptions}
      icon="eye"
    />

    <!-- CPU Cores -->
    <CustomSelect
      label="CPU Cores"
      bind:value={cpu}
      options={cpuOptions}
      icon="cube"
    />

    <!-- RAM Memory -->
    <CustomSelect
      label="RAM Allocation"
      bind:value={memory}
      options={memoryOptions}
      icon="network"
    />
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
      disabled={isSaving}
      loading={isSaving}
      onclick={handleSave}
    >
      Save Configuration
    </CustomButton>
  {/snippet}
</BaseModal>
