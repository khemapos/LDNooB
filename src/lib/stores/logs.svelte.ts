import type { LogEntry } from "$lib/types";

class LogsStore {
  entries = $state<LogEntry[]>([]);
  maxEntries = $state(200);

  add(level: LogEntry["level"], category: string, message: string) {
    const entry: LogEntry = {
      id: Math.random().toString(36).substring(2, 9),
      timestamp: new Date().toLocaleTimeString(),
      level,
      category,
      message,
    };
    this.entries = [entry, ...this.entries.slice(0, this.maxEntries - 1)];
  }

  info(category: string, message: string) {
    this.add("info", category, message);
  }

  success(category: string, message: string) {
    this.add("success", category, message);
  }

  warn(category: string, message: string) {
    this.add("warn", category, message);
  }

  error(category: string, message: string) {
    this.add("error", category, message);
  }

  adb(message: string) {
    this.add("adb", "ADB", message);
  }

  clear() {
    this.entries = [];
  }
}

export const logsStore = new LogsStore();
