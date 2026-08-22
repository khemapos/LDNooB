<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import Icon from "../ui/Icon.svelte";

let selectedIndex = $state<number>(0);
let rawXml = $state("");
let isDumping = $state(false);

async function handleDump() {
  isDumping = true;
  try {
    const path = settingsStore.settings.ldplayerPath;
    const cmd = "uiautomator dump /data/local/tmp/uidump.xml && cat /data/local/tmp/uidump.xml";
    const res = await invoke<string>("run_adb_command", {
      ldplayerDir: path,
      index: selectedIndex,
      adbCommand: cmd,
    });
    rawXml = res || "(No UI dump returned)";
    logsStore.success("Inspector", `Dumped UI hierarchy for instance #${selectedIndex}`);
  } catch (e) {
    rawXml = `Error dumping hierarchy: ${e}`;
    logsStore.error("Inspector", `UI dump failed: ${e}`);
  } finally {
    isDumping = false;
  }
}
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden">
  <!-- Toolbar -->
  <div
    class="flex items-center justify-between p-3 bg-white/80 dark:bg-[#0e1018]/90 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl rounded-2xl shadow-xs"
  >
    <div class="flex items-center gap-3">
      <label for="inspect-idx" class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        Target Emulator:
      </label>
      <select
        id="inspect-idx"
        bind:value={selectedIndex}
        class="px-3 py-1.5 text-xs rounded-xl bg-slate-50 dark:bg-[#07080d] border border-slate-200 dark:border-white/[0.08] text-slate-900 dark:text-white font-mono focus:outline-none focus:border-cyan-500"
      >
        {#each emulatorsStore.instances as inst}
          <option value={inst.index}>#{inst.index} - {inst.name} ({inst.is_running ? "Running" : "Stopped"})</option>
        {/each}
      </select>

      <button
        type="button"
        disabled={isDumping}
        onclick={handleDump}
        class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-semibold text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-xs cursor-pointer disabled:opacity-50"
      >
        <Icon name="eye" size={13} />
        <span>{isDumping ? "Dumping UI..." : "Capture Hierarchy"}</span>
      </button>
    </div>
  </div>

  <!-- XML View Panel -->
  <div
    class="flex-1 p-4 rounded-2xl bg-[#07080d] border border-white/[0.08] font-mono text-xs text-slate-300 overflow-auto select-text whitespace-pre-wrap leading-relaxed shadow-inner"
  >
    {rawXml || "<!-- Click 'Capture Hierarchy' to inspect Android UI nodes & XML elements on target emulator -->"}
  </div>
</div>
