import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "$lib/types";

const DEFAULT_SETTINGS: AppSettings = {
  ldplayerPath: "C:\\LDPlayer\\LDPlayer9",
  autoDetectEngine: true,
  rememberWindowPosition: true,
  defaultCpu: 2,
  defaultMemory: 2048,
  defaultWidth: 720,
  defaultHeight: 1280,
  defaultDpi: 320,
  defaultFps: 60,
};

class SettingsStore {
  settings = $state<AppSettings>(DEFAULT_SETTINGS);
  isLoaded = $state(false);

  async init() {
    try {
      const stored = await invoke<string | null>("db_get", {
        key: "app_settings",
      });
      if (stored) {
        this.settings = { ...DEFAULT_SETTINGS, ...JSON.parse(stored) };
      } else {
        const detected = await invoke<string | null>("auto_detect_ldplayer");
        if (detected) {
          this.settings.ldplayerPath = detected;
        }
      }
    } catch {
      // Fallback
    } finally {
      this.isLoaded = true;
    }
  }

  async save(newSettings: Partial<AppSettings>) {
    this.settings = { ...this.settings, ...newSettings };
    try {
      await invoke("db_set", {
        key: "app_settings",
        value: JSON.stringify(this.settings),
      });
    } catch (e) {
      console.warn("Failed to persist settings to SQLite:", e);
    }
  }
}

export const settingsStore = new SettingsStore();
