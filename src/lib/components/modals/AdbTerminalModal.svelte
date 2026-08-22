<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import BaseModal from "../common/BaseModal.svelte";
import CustomButton from "../common/CustomButton.svelte";
import CustomInput from "../common/CustomInput.svelte";

interface Props {
  open: boolean;
  emulatorIndex: number;
}

let { open = $bindable(false), emulatorIndex = 0 }: Props = $props();

let commandInput = $state("shell getprop ro.product.model");
let output = $state<string[]>([]);
let isExecuting = $state(false);

async function handleExecute() {
  if (!commandInput.trim()) return;
  isExecuting = true;
  output = [...output, `> adb -s emulator-5554 ${commandInput}`];

  try {
    const res = await invoke<string>("run_adb_command", {
      ldPath: settingsStore.settings.ldplayerPath,
      index: emulatorIndex,
      adbCommand: commandInput.trim(),
    });
    output = [...output, res || "(Command completed with no output)"];
  } catch (err: any) {
    output = [...output, `Error: ${err.toString()}`];
    logsStore.error("ADB", `ADB error on #${emulatorIndex}: ${err.toString()}`);
  } finally {
    isExecuting = false;
  }
}
</script>

<BaseModal
  bind:open
  title="ADB Command Shell"
  subtitle="Connected to emulator #{emulatorIndex}"
  icon="terminal"
>
  <div class="space-y-3 font-sans">
    <!-- Command Output Terminal -->
    <div
      class="h-64 p-3.5 bg-[#0a0b0d] border border-[#25272b] rounded-xl overflow-y-auto font-mono text-[11px] text-[#00b578] space-y-1 select-text"
    >
      <div class="text-[#8c8c8c]">--- ADB Shell Session Initialized ---</div>
      {#each output as line}
        <div class="whitespace-pre-wrap">{line}</div>
      {/each}
    </div>

    <!-- Command Input Bar -->
    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleExecute();
      }}
      class="flex items-center gap-2"
    >
      <CustomInput
        placeholder="e.g. shell input keyevent 4"
        bind:value={commandInput}
        icon="terminal"
        class="flex-1"
      />
      <CustomButton
        type="submit"
        variant="primary"
        size="md"
        disabled={!commandInput.trim() || isExecuting}
        loading={isExecuting}
      >
        Send
      </CustomButton>
    </form>
  </div>

  {#snippet footer()}
    <CustomButton
      variant="secondary"
      size="md"
      onclick={() => (output = [])}
    >
      Clear Screen
    </CustomButton>
    <CustomButton
      variant="secondary"
      size="md"
      onclick={() => (open = false)}
    >
      Close
    </CustomButton>
  {/snippet}
</BaseModal>
