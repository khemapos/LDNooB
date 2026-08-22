<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
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

<div class="flex-1 flex flex-col h-full gap-4 overflow-y-auto max-w-2xl">
  <!-- General Configuration Card -->
  <div
    class="p-5 rounded-2xl bg-white/80 dark:bg-[#0e1018]/90 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl space-y-4 shadow-xs"
  >
    <div class="flex items-center gap-2 pb-2 border-b border-slate-200 dark:border-white/[0.06]">
      <Icon name="settings" size={16} class="text-cyan-500" />
      <h3 class="text-xs font-bold text-slate-900 dark:text-white uppercase tracking-wider">
        Emulator Engine Configuration
      </h3>
    </div>

    <!-- LDPlayer Directory Input -->
    <div class="space-y-1.5">
      <label for="ld-path" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        LDPlayer Installation Path
      </label>
      <div class="flex gap-2">
        <input
          id="ld-path"
          type="text"
          bind:value={ldPath}
          class="flex-1 px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white font-mono focus:outline-none focus:border-cyan-500 shadow-inner"
        />
        <button
          type="button"
          disabled={isDetecting}
          onclick={handleAutoDetect}
          class="px-3 py-2 text-xs font-semibold rounded-xl text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.12] transition-colors cursor-pointer disabled:opacity-50"
        >
          {isDetecting ? "Detecting..." : "Auto Detect"}
        </button>
      </div>
      <p class="text-[11px] text-slate-400">
        The directory containing <code class="font-mono text-cyan-600 dark:text-cyan-400">ldconsole.exe</code> (e.g. C:\LDPlayer\LDPlayer9).
      </p>
    </div>

    <!-- Save Button -->
    <div class="pt-2 flex justify-end">
      <button
        type="button"
        onclick={handleSave}
        class="flex items-center gap-1.5 px-4 py-2 text-xs font-semibold rounded-xl text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-xs cursor-pointer"
      >
        <Icon name="check" size={14} />
        <span>{isSaved ? "Saved!" : "Save Settings"}</span>
      </button>
    </div>
  </div>
</div>
