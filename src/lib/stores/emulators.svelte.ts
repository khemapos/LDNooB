import { invoke } from "@tauri-apps/api/core";
import type { Emulator, EmulatorFilterStatus } from "$lib/types";
import { logsStore } from "./logs.svelte";
import { settingsStore } from "./settings.svelte";

class EmulatorsStore {
  instances = $state<Emulator[]>([]);
  selectedIndices = $state<number[]>([]);
  isLoading = $state(false);
  filterStatus = $state<EmulatorFilterStatus>("all");
  searchQuery = $state("");
  selectedGroup = $state("all");
  private pollTimer: ReturnType<typeof setInterval> | null = null;

  filteredInstances = $derived<Emulator[]>(
    this.instances.filter((inst) => {
      if (this.filterStatus === "running" && !inst.is_running) return false;
      if (this.filterStatus === "stopped" && inst.is_running) return false;
      if (this.selectedGroup !== "all" && inst.group !== this.selectedGroup) {
        return false;
      }
      if (this.searchQuery.trim() !== "") {
        const q = this.searchQuery.toLowerCase();
        return (
          inst.name.toLowerCase().includes(q) ||
          inst.index.toString().includes(q) ||
          inst.imei.toLowerCase().includes(q) ||
          inst.model.toLowerCase().includes(q)
        );
      }
      return true;
    })
  );

  runningCount = $derived<number>(this.instances.filter((i) => i.is_running).length);
  stoppedCount = $derived<number>(this.instances.filter((i) => !i.is_running).length);

  constructor() {
    this.startPolling();
  }

  startPolling() {
    if (this.pollTimer) return;
    this.pollTimer = setInterval(() => {
      this.syncStatusSilent();
    }, 3000);
  }

  stopPolling() {
    if (this.pollTimer) {
      clearInterval(this.pollTimer);
      this.pollTimer = null;
    }
  }

  async syncStatusSilent() {
    try {
      const path = settingsStore.settings.ldplayerPath;
      if (!path) return;
      const res = await invoke<Emulator[]>("list_emulators", {
        ldplayerDir: path,
      });
      this.instances = res;
    } catch {
      // Silent in background poll
    }
  }

  async refresh() {
    this.isLoading = true;
    try {
      const path = settingsStore.settings.ldplayerPath;
      const res = await invoke<Emulator[]>("list_emulators", {
        ldplayerDir: path,
      });
      this.instances = res;
      logsStore.info("Engine", `Loaded ${res.length} emulator instances`);
    } catch (e) {
      logsStore.error("Engine", `Failed to list emulators: ${e}`);
    } finally {
      this.isLoading = false;
    }
  }

  async launch(index: number) {
    try {
      // Optimistic UI state update
      const target = this.instances.find((i) => i.index === index);
      if (target) {
        target.is_running = true;
      }

      const path = settingsStore.settings.ldplayerPath;
      await invoke("launch_emulator", { ldplayerDir: path, index });
      logsStore.success("Instance", `Launched emulator #${index}`);

      setTimeout(() => this.syncStatusSilent(), 800);
      setTimeout(() => this.syncStatusSilent(), 2000);
      setTimeout(() => this.syncStatusSilent(), 4000);
    } catch (e) {
      logsStore.error("Instance", `Failed to launch emulator #${index}: ${e}`);
      this.syncStatusSilent();
    }
  }

  async quit(index: number) {
    try {
      // Optimistic UI state update
      const target = this.instances.find((i) => i.index === index);
      if (target) {
        target.is_running = false;
        target.pid = -1;
      }

      const path = settingsStore.settings.ldplayerPath;
      await invoke("quit_emulator", { ldplayerDir: path, index });
      logsStore.info("Instance", `Closed emulator #${index}`);

      setTimeout(() => this.syncStatusSilent(), 600);
      setTimeout(() => this.syncStatusSilent(), 1500);
    } catch (e) {
      logsStore.error("Instance", `Failed to quit emulator #${index}: ${e}`);
      this.syncStatusSilent();
    }
  }

  async batchLaunch() {
    for (const index of this.selectedIndices) {
      await this.launch(index);
    }
  }

  async batchQuit() {
    for (const index of this.selectedIndices) {
      await this.quit(index);
    }
  }

  async quitAll() {
    try {
      for (const inst of this.instances) {
        inst.is_running = false;
        inst.pid = -1;
      }
      const path = settingsStore.settings.ldplayerPath;
      await invoke("quit_all_emulators", { ldplayerDir: path });
      logsStore.info("Engine", "Closed all running emulators");
      setTimeout(() => this.syncStatusSilent(), 1000);
    } catch (e) {
      logsStore.error("Engine", `Failed to quit all emulators: ${e}`);
    }
  }

  async sortWindows() {
    try {
      const path = settingsStore.settings.ldplayerPath;
      await invoke("sort_windows", { ldplayerDir: path });
      logsStore.info("Windows", "Arranged emulator windows on screen");
    } catch (e) {
      logsStore.error("Windows", `Failed to arrange windows: ${e}`);
    }
  }

  async addInstance(name: string) {
    try {
      const path = settingsStore.settings.ldplayerPath;
      await invoke("add_emulator", { ldplayerDir: path, name });
      logsStore.success("Instance", `Created new emulator: ${name}`);
      await this.refresh();
    } catch (e) {
      logsStore.error("Instance", `Failed to add emulator: ${e}`);
    }
  }

  async copyInstance(name: string, fromIndex: number) {
    try {
      const path = settingsStore.settings.ldplayerPath;
      await invoke("copy_emulator", {
        ldplayerDir: path,
        name,
        fromIndex,
      });
      logsStore.success("Instance", `Cloned emulator #${fromIndex} as ${name}`);
      await this.refresh();
    } catch (e) {
      logsStore.error("Instance", `Failed to clone emulator: ${e}`);
    }
  }

  async deleteInstance(index: number) {
    try {
      const path = settingsStore.settings.ldplayerPath;
      await invoke("remove_emulator", { ldplayerDir: path, index });
      this.selectedIndices = this.selectedIndices.filter((i) => i !== index);
      logsStore.warn("Instance", `Deleted emulator #${index}`);
      await this.refresh();
    } catch (e) {
      logsStore.error("Instance", `Failed to delete emulator #${index}: ${e}`);
    }
  }

  async renameInstance(index: number, newTitle: string) {
    try {
      const path = settingsStore.settings.ldplayerPath;
      await invoke("rename_emulator", {
        ldplayerDir: path,
        index,
        title: newTitle,
      });
      logsStore.info("Instance", `Renamed emulator #${index} to "${newTitle}"`);
      await this.refresh();
    } catch (e) {
      logsStore.error("Instance", `Failed to rename emulator #${index}: ${e}`);
    }
  }

  toggleSelection(index: number) {
    if (this.selectedIndices.includes(index)) {
      this.selectedIndices = this.selectedIndices.filter((i) => i !== index);
    } else {
      this.selectedIndices = [...this.selectedIndices, index];
    }
  }

  selectAll() {
    this.selectedIndices = this.filteredInstances.map((i) => i.index);
  }

  deselectAll() {
    this.selectedIndices = [];
  }
}

export const emulatorsStore = new EmulatorsStore();
