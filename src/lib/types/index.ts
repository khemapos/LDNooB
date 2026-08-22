export * from "./account";
export * from "./emulator";
export * from "./proxy";
export * from "./workflow";

export type ActiveTab = "profiles" | "accounts" | "settings";

export interface LogEntry {
  id: string;
  timestamp: string;
  level: "info" | "warn" | "error" | "adb" | "success";
  category: string;
  message: string;
}

export interface AppSettings {
  ldplayerPath: string;
  mumuPath?: string;
  autoDetectEngine: boolean;
  rememberWindowPosition: boolean;
  defaultCpu: number;
  defaultMemory: number;
  defaultWidth: number;
  defaultHeight: number;
  defaultDpi: number;
  defaultFps: number;
}
