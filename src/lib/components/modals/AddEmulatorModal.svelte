<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import Icon from "../ui/Icon.svelte";

interface Props {
  open: boolean;
}

let { open = $bindable(false) }: Props = $props();

let name = $state("");
let amount = $state(1);
let resPreset = $state("mobile-540");
let cpu = $state("2");
let memory = $state("2048");

let root = $state(true);
let rememberWnd = $state(true);
let autoRotate = $state(false);
let lockWindow = $state(true);
let systemDiskWritable = $state(true);

let isSubmitting = $state(false);

function resetForm() {
  name = "";
  amount = 1;
  resPreset = "mobile-540";
  cpu = "2";
  memory = "2048";
  root = true;
  rememberWnd = true;
  autoRotate = false;
  lockWindow = true;
  systemDiskWritable = true;
  isSubmitting = false;
}

function getResolutionString(preset: string): string {
  switch (preset) {
    case "mobile-540":
      return "540,960,240";
    case "mobile-720":
      return "720,1280,320";
    case "mobile-1080":
      return "1080,1920,480";
    case "tablet-1280":
      return "1280,720,240";
    case "tablet-1600":
      return "1600,900,240";
    case "tablet-1920":
      return "1920,1080,280";
    default:
      return "540,960,240";
  }
}

async function handleCreate() {
  isSubmitting = true;
  const path = settingsStore.settings.ldplayerPath;
  const count = Math.max(1, Math.min(50, amount));
  const createdIndices: number[] = [];

  try {
    for (let i = 0; i < count; i++) {
      let finalName = name.trim();
      if (finalName && count > 1) {
        finalName = `${finalName}_${i + 1}`;
      }

      const emuIndex = await invoke<number>("add_emulator", {
        ldplayerDir: path,
        name: finalName || null,
        rememberWnd,
        autoRotate,
        lockWindow,
        systemDiskWritable,
      });

      if (emuIndex !== undefined && emuIndex !== null && emuIndex >= 0) {
        const resStr = getResolutionString(resPreset);
        await invoke("modify_emulator", {
          ldplayerDir: path,
          index: emuIndex,
          resolution: resStr,
          cpu: Number.parseInt(cpu, 10) || 2,
          memory: Number.parseInt(memory, 10) || 2048,
          root: root ? "1" : "0",
        });
        createdIndices.push(emuIndex);
      }
    }

    logsStore.success("Instance", `Created and configured ${createdIndices.length} instance(s)`);
    await emulatorsStore.refresh();
    open = false;
    resetForm();
  } catch (e) {
    logsStore.error("Instance", `Failed to create instance: ${e}`);
  } finally {
    isSubmitting = false;
  }
}
</script>

{#if open}
  <!-- Modal Backdrop -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-4 select-none font-sans animate-in fade-in duration-150"
    role="presentation"
    onclick={() => (open = false)}
  >
    <!-- Modal Card Dialog (matching D:\ldremote) -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-label="Add Emulator Instance"
      class="bg-bg-panel border border-border-default rounded-3xl max-w-2xl w-full p-6 shadow-2xl flex flex-col gap-4 text-text-default relative"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="flex items-center justify-between pb-3 border-b border-border-default/40">
        <div class="flex items-center gap-2 text-text-hover">
          <span class="text-[#00b578] font-black text-base">+</span>
          <h2 class="text-xs font-black uppercase tracking-wider">
            Add Emulator Instance
          </h2>
        </div>

        <!-- Crimson Red Close Button (matching screenshot) -->
        <button
          type="button"
          onclick={() => (open = false)}
          class="w-7 h-7 rounded-lg bg-[#ff4d4f] hover:bg-[#ff7875] text-white flex items-center justify-center cursor-pointer transition-colors shadow-xs"
          title="Close Modal"
        >
          <Icon name="close" size={13} />
        </button>
      </div>

      <!-- Main Form Body -->
      <div class="flex flex-col gap-3.5">
        <!-- Row 1: Name and Amount -->
        <div class="flex items-center gap-3">
          <!-- Emulator Name Field -->
          <div
            class="flex-1 flex flex-col text-left border border-border-default rounded-xl px-4 py-2 bg-bg-app focus-within:border-[#00b578] focus-within:ring-2 focus-within:ring-[#00b578]/15 transition-all"
          >
            <label
              for="emu-name-input"
              class="text-[10px] font-extrabold uppercase tracking-widest text-text-muted mb-0.5"
            >
              Emulator Name
            </label>
            <div class="flex items-center gap-2">
              <span class="text-text-muted/60">
                <Icon name="cube" size={13} />
              </span>
              <input
                id="emu-name-input"
                type="text"
                bind:value={name}
                placeholder="Leave empty for default"
                class="w-full bg-transparent text-xs font-semibold outline-none border-none p-0 text-text-default placeholder:text-text-muted/50"
              />
            </div>
          </div>

          <!-- Amount Field -->
          <div
            class="w-36 flex flex-col text-left border border-border-default rounded-xl px-4 py-2 bg-bg-app focus-within:border-[#00b578] focus-within:ring-2 focus-within:ring-[#00b578]/15 transition-all"
          >
            <label
              for="emu-amount-input"
              class="text-[10px] font-extrabold uppercase tracking-widest text-text-muted mb-0.5"
            >
              Amount
            </label>
            <input
              id="emu-amount-input"
              type="number"
              min="1"
              max="50"
              bind:value={amount}
              class="w-full bg-transparent text-xs font-semibold outline-none border-none p-0 text-text-default font-mono"
            />
          </div>
        </div>

        <!-- Row 2: Resolution Preset, CPU Cores, RAM Size -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
          <!-- Resolution Preset -->
          <div
            class="flex flex-col text-left border border-border-default rounded-xl px-4 py-2 bg-bg-app"
          >
            <label
              for="emu-res-select"
              class="text-[10px] font-extrabold uppercase tracking-widest text-text-muted mb-0.5"
            >
              Resolution Preset
            </label>
            <select
              id="emu-res-select"
              bind:value={resPreset}
              class="bg-transparent text-xs font-semibold outline-none border-none p-0 text-text-default cursor-pointer"
            >
              <option value="mobile-540" class="bg-bg-card text-text-default">
                Mobile 540×960 (240 DPI)
              </option>
              <option value="mobile-720" class="bg-bg-card text-text-default">
                Mobile 720×1280 (320 DPI)
              </option>
              <option value="mobile-1080" class="bg-bg-card text-text-default">
                Mobile 1080×1920 (480 DPI)
              </option>
              <option value="tablet-1280" class="bg-bg-card text-text-default">
                Tablet 1280×720 (240 DPI)
              </option>
              <option value="tablet-1600" class="bg-bg-card text-text-default">
                Tablet 1600×900 (240 DPI)
              </option>
              <option value="tablet-1920" class="bg-bg-card text-text-default">
                Tablet 1920×1080 (280 DPI)
              </option>
            </select>
          </div>

          <!-- CPU Cores -->
          <div
            class="flex flex-col text-left border border-border-default rounded-xl px-4 py-2 bg-bg-app"
          >
            <label
              for="emu-cpu-select"
              class="text-[10px] font-extrabold uppercase tracking-widest text-text-muted mb-0.5"
            >
              CPU Cores
            </label>
            <select
              id="emu-cpu-select"
              bind:value={cpu}
              class="bg-transparent text-xs font-semibold outline-none border-none p-0 text-text-default cursor-pointer"
            >
              <option value="1" class="bg-bg-card text-text-default">1 Core</option>
              <option value="2" class="bg-bg-card text-text-default">2 Cores</option>
              <option value="3" class="bg-bg-card text-text-default">3 Cores</option>
              <option value="4" class="bg-bg-card text-text-default">4 Cores</option>
              <option value="6" class="bg-bg-card text-text-default">6 Cores</option>
              <option value="8" class="bg-bg-card text-text-default">8 Cores</option>
            </select>
          </div>

          <!-- RAM Size -->
          <div
            class="flex flex-col text-left border border-border-default rounded-xl px-4 py-2 bg-bg-app"
          >
            <label
              for="emu-ram-select"
              class="text-[10px] font-extrabold uppercase tracking-widest text-text-muted mb-0.5"
            >
              RAM Size
            </label>
            <select
              id="emu-ram-select"
              bind:value={memory}
              class="bg-transparent text-xs font-semibold outline-none border-none p-0 text-text-default cursor-pointer"
            >
              <option value="1024" class="bg-bg-card text-text-default">
                1024 MB (1GB)
              </option>
              <option value="2048" class="bg-bg-card text-text-default">
                2048 MB (2GB)
              </option>
              <option value="3072" class="bg-bg-card text-text-default">
                3072 MB (3GB)
              </option>
              <option value="4096" class="bg-bg-card text-text-default">
                4096 MB (4GB)
              </option>
              <option value="6144" class="bg-bg-card text-text-default">
                6144 MB (6GB)
              </option>
              <option value="8192" class="bg-bg-card text-text-default">
                8192 MB (8GB)
              </option>
            </select>
          </div>
        </div>

        <!-- Configuration Switches (2 Columns matching screenshot) -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-3.5 pt-3 border-t border-border-default/40">
          <!-- Root Access -->
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-text-default">
              Root Access (Superuser)
            </span>
            <button
              type="button"
              aria-label="Toggle Root Access"
              onclick={() => (root = !root)}
              class="w-11 h-6 rounded-full transition-colors duration-200 focus:outline-none relative p-0.5 cursor-pointer shrink-0 {root
                ? 'bg-[#00b578]'
                : 'bg-zinc-700'}"
            >
              <span
                class="block w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200 {root
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>

          <!-- Remember Window Size & Position -->
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-text-default">
              Remember window size and position
            </span>
            <button
              type="button"
              aria-label="Toggle Remember Window Size and Position"
              onclick={() => (rememberWnd = !rememberWnd)}
              class="w-11 h-6 rounded-full transition-colors duration-200 focus:outline-none relative p-0.5 cursor-pointer shrink-0 {rememberWnd
                ? 'bg-[#00b578]'
                : 'bg-zinc-700'}"
            >
              <span
                class="block w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200 {rememberWnd
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>

          <!-- Automatically Rotate Window -->
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-text-default">
              Automatically rotate the window
            </span>
            <button
              type="button"
              aria-label="Toggle Automatically Rotate Window"
              onclick={() => (autoRotate = !autoRotate)}
              class="w-11 h-6 rounded-full transition-colors duration-200 focus:outline-none relative p-0.5 cursor-pointer shrink-0 {autoRotate
                ? 'bg-[#00b578]'
                : 'bg-zinc-700'}"
            >
              <span
                class="block w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200 {autoRotate
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>

          <!-- Fix Window Size -->
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-text-default">
              Fix window size (Lock Size)
            </span>
            <button
              type="button"
              aria-label="Toggle Fix Window Size"
              onclick={() => (lockWindow = !lockWindow)}
              class="w-11 h-6 rounded-full transition-colors duration-200 focus:outline-none relative p-0.5 cursor-pointer shrink-0 {lockWindow
                ? 'bg-[#00b578]'
                : 'bg-zinc-700'}"
            >
              <span
                class="block w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200 {lockWindow
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>
        </div>

        <!-- Writable System Disk -->
        <div
          class="flex items-center justify-between pt-3 border-t border-border-default/40"
        >
          <span class="text-xs font-bold text-text-default">
            Writable System Disk
          </span>
          <button
            type="button"
            aria-label="Toggle Writable System Disk"
            onclick={() => (systemDiskWritable = !systemDiskWritable)}
            class="w-11 h-6 rounded-full transition-colors duration-200 focus:outline-none relative p-0.5 cursor-pointer shrink-0 {systemDiskWritable
              ? 'bg-[#00b578]'
              : 'bg-zinc-700'}"
          >
            <span
              class="block w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200 {systemDiskWritable
                ? 'translate-x-5'
                : 'translate-x-0'}"
            ></span>
          </button>
        </div>
      </div>

      <!-- Footer Actions (matching screenshot) -->
      <div class="flex items-center justify-end gap-3 pt-4 border-t border-border-default/40">
        <button
          type="button"
          disabled={isSubmitting}
          onclick={() => (open = false)}
          class="px-5 py-2 text-xs font-bold rounded-xl border border-border-default bg-bg-card hover:bg-bg-card-hover text-text-default hover:text-text-hover transition-colors cursor-pointer"
        >
          Cancel
        </button>

        <button
          type="button"
          disabled={isSubmitting}
          onclick={handleCreate}
          class="px-6 py-2 text-xs font-bold rounded-xl bg-gradient-to-b from-[#00c985] to-[#00b578] hover:from-[#00d78e] hover:to-[#00c07f] text-white border border-[#00b578] shadow-[0_2px_10px_rgba(0,181,120,0.3)] active:scale-[0.98] transition-all cursor-pointer disabled:opacity-50"
        >
          {isSubmitting
            ? "Creating..."
            : amount > 1
              ? "Create Instances"
              : "Create Instance"}
        </button>
      </div>
    </div>
  </div>
{/if}
