export interface Emulator {
  index: number;
  name: string;
  top_hwnd: string;
  bind_hwnd: string;
  is_running: boolean;
  pid: number;
  vbox_pid: number;
  width: number;
  height: number;
  dpi: number;
  brand: string;
  model: string;
  imei: string;
  mac: string;
  android_id: string;
  disk_size_bytes: number;
  group?: string;
  proxy?: string;
  fps?: number;
}

export interface EmulatorConfig {
  cpu: number;
  memory: number;
  width: number;
  height: number;
  dpi: number;
  fps: number;
  imei?: string;
  model?: string;
  manufacturer?: string;
  phone_number?: string;
  android_id?: string;
  mac?: string;
}

export interface DeviceModelPreset {
  brand: string;
  model: string;
  manufacturer: string;
}

export type EmulatorFilterStatus = "all" | "running" | "stopped";
