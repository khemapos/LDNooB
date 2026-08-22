import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

export interface WindowPreset {
  label: string;
  width: number;
  height: number;
  description: string;
}

export const WINDOW_PRESETS: WindowPreset[] = [
  { label: "Compact HD", width: 1024, height: 720, description: "Compact 1024 × 720 layout" },
  { label: "Standard WXGA", width: 1280, height: 800, description: "Standard 1280 × 800 layout" },
  { label: "Full HD", width: 1440, height: 900, description: "Spacious 1440 × 900 layout" },
  { label: "Widescreen 1080p", width: 1920, height: 1080, description: "Maximum 1920 × 1080 layout" },
];

class WindowStore {
  width = $state(1280);
  height = $state(720);
  isMaximized = $state(false);
  isTauri = $state(true);

  init() {
    if (typeof window === "undefined") return;

    this.width = window.innerWidth;
    this.height = window.innerHeight;

    window.addEventListener("resize", () => {
      this.width = window.innerWidth;
      this.height = window.innerHeight;
    });

    try {
      const win = getCurrentWindow();
      win.isMaximized().then((val) => {
        this.isMaximized = val;
      }).catch(() => {
        this.isTauri = false;
      });

      const unlisten = win.onResized(async () => {
        try {
          this.isMaximized = await win.isMaximized();
          const size = await win.innerSize();
          const scale = await win.scaleFactor();
          const logical = size.toLogical(scale);
          this.width = Math.round(logical.width);
          this.height = Math.round(logical.height);
        } catch {}
      });

      return () => {
        unlisten.then((fn) => fn?.()).catch(() => {});
      };
    } catch {
      this.isTauri = false;
    }
  }

  async setSize(width: number, height: number) {
    try {
      const win = getCurrentWindow();
      if (this.isMaximized) {
        await win.unmaximize();
        this.isMaximized = false;
      }
      await win.setSize(new LogicalSize(width, height));
      this.width = width;
      this.height = height;
    } catch {
      try {
        await invoke("app_set_window_size", { width, height });
        this.width = width;
        this.height = height;
      } catch (err) {
        console.warn("Could not resize window:", err);
      }
    }
  }

  async toggleMaximize() {
    try {
      const win = getCurrentWindow();
      await win.toggleMaximize();
      this.isMaximized = await win.isMaximized();
    } catch {
      try {
        this.isMaximized = await invoke<boolean>("app_toggle_maximize");
      } catch (err) {
        console.warn("Could not toggle maximize:", err);
      }
    }
  }

  async minimize() {
    try {
      const win = getCurrentWindow();
      await win.minimize();
    } catch {
      try {
        await invoke("app_minimize");
      } catch (err) {
        console.warn("Could not minimize window:", err);
      }
    }
  }

  async close() {
    try {
      const win = getCurrentWindow();
      await win.close();
    } catch {
      try {
        await invoke("app_close");
      } catch (err) {
        console.warn("Could not close window:", err);
      }
    }
  }
}

export const windowStore = new WindowStore();
