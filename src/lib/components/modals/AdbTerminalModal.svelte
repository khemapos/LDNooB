<script lang="ts">
  import BaseModal from "../common/BaseModal.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import { logsStore } from "$lib/stores/logs.svelte";

  interface Props {
    open: boolean;
    emulatorIndex?: number;
  }

  let { open = $bindable(false), emulatorIndex = 0 }: Props = $props();

  let command = $state("getprop ro.product.model");
  let output = $state("");
  let isRunning = $state(false);

  async function handleExecute() {
    if (!command.trim()) return;
    isRunning = true;
    try {
      const path = settingsStore.settings.ldplayerPath;
      const res = await invoke<string>("run_adb_command", {
        ldplayerDir: path,
        index: emulatorIndex,
        adbCommand: command.trim(),
      });
      output = res || "(Command completed with no output)";
      logsStore.adb(`[Emulator #${emulatorIndex}] $ ${command}`);
    } catch (e) {
      output = `Error: ${e}`;
      logsStore.error("ADB", `Failed to execute: ${e}`);
    } finally {
      isRunning = false;
    }
  }
</script>

<BaseModal
  bind:open
  title="ADB Shell Terminal"
  subtitle="Direct shell interface for emulator #{emulatorIndex}"
  icon="terminal"
  maxWidth="max-w-2xl"
>
  <div class="space-y-3">
    <!-- Input Command -->
    <form onsubmit={(e) => { e.preventDefault(); handleExecute(); }} class="flex gap-2">
      <input
        type="text"
        placeholder="e.g. pm list packages or input keyevent 3"
        bind:value={command}
        class="flex-1 px-3.5 py-2 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white focus:outline-none focus:border-cyan-500 font-mono shadow-inner"
      />
      <button
        type="submit"
        disabled={isRunning}
        class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all cursor-pointer disabled:opacity-50"
      >
        {isRunning ? "Running..." : "Execute"}
      </button>
    </form>

    <!-- Terminal Output -->
    <div
      class="w-full h-64 p-4 rounded-xl bg-[#07080d] border border-white/[0.08] font-mono text-xs text-emerald-400 overflow-y-auto whitespace-pre-wrap select-text"
    >
      {output || "# Output will appear here after execution..."}
    </div>
  </div>
</BaseModal>
