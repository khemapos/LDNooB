<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import CustomButton from "../common/CustomButton.svelte";
import CustomInput from "../common/CustomInput.svelte";
import Icon from "../ui/Icon.svelte";

let ldPath = $state(settingsStore.settings.ldplayerPath);
let isDetecting = $state(false);
let isSaved = $state(false);

async function handleAutoDetect() {
  isDetecting = true;
  try {
    const detected = await invoke<string | null>("auto_detect_ldplayer");
    if (detected) {
      ldPath = detected;
      logsStore.success("Settings", `Auto-detected LDPlayer at: ${detected}`);
    } else {
      logsStore.warn("Settings", "LDPlayer installation could not be auto-detected");
    }
  } finally {
    isDetecting = false;
  }
}

async function handleSave() {
  await settingsStore.save({ ldplayerPath: ldPath });
  isSaved = true;
  logsStore.success("Settings", "Saved settings to local database");
  setTimeout(() => (isSaved = false), 2000);
}
</script>

<div class="flex-1 flex flex-col h-full gap-4 overflow-y-auto max-w-2xl font-sans">
  <!-- General Configuration Card -->
  <div
    class="p-5 rounded-2xl bg-[#141517] border border-[#25272b] space-y-4 shadow-xs"
  >
    <div class="flex items-center gap-2 pb-2 border-b border-[#25272b]">
      <Icon name="settings" size={16} class="text-[#00b578]" />
      <h3 class="text-xs font-bold text-white uppercase tracking-wider">
        Emulator Engine Configuration
      </h3>
    </div>

    <!-- LDPlayer Directory Input -->
    <div class="space-y-2">
      <div class="flex items-center gap-2">
        <CustomInput
          label="LDPlayer Installation Directory"
          placeholder="C:\LDPlayer\LDPlayer9"
          bind:value={ldPath}
          icon="folder"
          class="flex-1 font-mono"
        />
        <CustomButton
          variant="secondary"
          size="md"
          loading={isDetecting}
          onclick={handleAutoDetect}
          class="mt-2"
        >
          Auto Detect
        </CustomButton>
      </div>

      <p class="text-[11px] text-[#8c8c8c]">
        The directory containing <code class="font-mono text-[#00b578]">ldconsole.exe</code> (e.g. C:\LDPlayer\LDPlayer9).
      </p>
    </div>

    <!-- Save Button -->
    <div class="pt-2 flex justify-end">
      <CustomButton
        variant="primary"
        size="md"
        onclick={handleSave}
      >
        <Icon name="check" size={14} />
        <span>{isSaved ? "Saved Successfully!" : "Save Settings"}</span>
      </CustomButton>
    </div>
  </div>
</div>
