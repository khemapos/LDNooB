<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import type { Emulator } from "$lib/types";
import BaseModal from "../common/BaseModal.svelte";

interface Props {
  open: boolean;
  emulator?: Emulator | null;
}

let { open = $bindable(false), emulator = null }: Props = $props();

let cpu = $state(2);
let memory = $state(2048);
let resolution = $state("720,1280,320");
let isSaving = $state(false);

$effect(() => {
  if (emulator) {
    cpu = 2;
    memory = 2048;
    resolution = `${emulator.width},${emulator.height},${emulator.dpi}`;
  }
});

async function handleSave() {
  if (!emulator) return;
  isSaving = true;
  try {
    const path = settingsStore.settings.ldplayerPath;
    await invoke("modify_emulator", {
      ldplayerDir: path,
      index: emulator.index,
      resolution,
      cpu,
      memory,
    });
    logsStore.success(
      "Instance",
      `Updated settings for emulator #${emulator.index} (${cpu} Cores, ${memory}MB RAM)`
    );
    await emulatorsStore.refresh();
    open = false;
  } catch (e) {
    logsStore.error("Instance", `Failed to update settings: ${e}`);
  } finally {
    isSaving = false;
  }
}
</script>

<BaseModal
  bind:open
  title="Modify Emulator Settings"
  subtitle="Hardware, CPU, and screen resolution configuration for #{emulator?.index ?? 0}"
  icon="settings"
  maxWidth="max-w-md"
>
  <div class="space-y-4">
    <!-- CPU Cores -->
    <div class="space-y-1.5">
      <label for="cpu-select" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        CPU Cores
      </label>
      <select
        id="cpu-select"
        bind:value={cpu}
        class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono"
      >
        <option value={1}>1 Core (Low)</option>
        <option value={2}>2 Cores (Recommended)</option>
        <option value={4}>4 Cores (High Performance)</option>
        <option value={8}>8 Cores (Ultra)</option>
      </select>
    </div>

    <!-- RAM Memory -->
    <div class="space-y-1.5">
      <label for="ram-select" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        RAM Memory (MB)
      </label>
      <select
        id="ram-select"
        bind:value={memory}
        class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono"
      >
        <option value={1024}>1024 MB (1 GB)</option>
        <option value={2048}>2048 MB (2 GB - Standard)</option>
        <option value={3072}>3072 MB (3 GB)</option>
        <option value={4096}>4096 MB (4 GB - High)</option>
      </select>
    </div>

    <!-- Resolution Presets -->
    <div class="space-y-1.5">
      <label for="res-select" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        Resolution & DPI
      </label>
      <select
        id="res-select"
        bind:value={resolution}
        class="w-full px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono"
      >
        <option value="720,1280,320">720 × 1280 (Phone - 320 DPI)</option>
        <option value="540,960,240">540 × 960 (Phone - 240 DPI / Low RAM)</option>
        <option value="1080,1920,480">1080 × 1920 (Full HD - 480 DPI)</option>
        <option value="1280,720,240">1280 × 720 (Tablet - 240 DPI)</option>
      </select>
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
      disabled={isSaving}
      onclick={handleSave}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-white bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 transition-all shadow-sm cursor-pointer disabled:opacity-50"
    >
      {isSaving ? "Saving..." : "Apply Settings"}
    </button>
  {/snippet}
</BaseModal>
