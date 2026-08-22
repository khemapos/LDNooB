<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { logsStore } from "$lib/stores/logs.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import CustomButton from "../common/CustomButton.svelte";
import Icon from "../ui/Icon.svelte";

let emulatorEngine = $state(
  (typeof localStorage !== "undefined" && localStorage.getItem("settings_emulator_engine")) ||
    "ldplayer"
);
let ldPath = $state(settingsStore.settings.ldplayerPath || "C:\\LDPlayer\\LDPlayer9");
let mumuPath = $state(
  (typeof localStorage !== "undefined" && localStorage.getItem("settings_mumuplayer_path")) ||
    "C:\\Program Files\\MuMuPlayer-12.0"
);

let appToggleShortcut = $state(
  (typeof localStorage !== "undefined" && localStorage.getItem("settings_shortcut_app_toggle")) ||
    "Ctrl+Space"
);
let toggleShortcut = $state(
  (typeof localStorage !== "undefined" && localStorage.getItem("settings_shortcut_toggle")) ||
    "Ctrl+Shift+Space"
);

// Automation Switches
let autoConnectVpn = $state(
  typeof localStorage !== "undefined" &&
    localStorage.getItem("settings_auto_connect_vpn") === "true"
);
let autoConnectProxy = $state(
  typeof localStorage !== "undefined"
    ? localStorage.getItem("settings_auto_connect_proxy") !== "false"
    : true
);
let autoReconnectProxy = $state(
  typeof localStorage !== "undefined"
    ? localStorage.getItem("settings_auto_reconnect_vpn") !== "false"
    : true
);
let autoStopAfterTask = $state(
  typeof localStorage !== "undefined"
    ? localStorage.getItem("settings_auto_stop_after_task") === "true"
    : false
);
let autoCaptureScreenshots = $state(
  typeof localStorage !== "undefined"
    ? localStorage.getItem("settings_auto_capture_screenshots") !== "false"
    : true
);
let proxyKillSwitch = $state(
  typeof localStorage !== "undefined"
    ? localStorage.getItem("settings_proxy_killswitch") === "true"
    : false
);

let geminiApiKey = $state(
  (typeof localStorage !== "undefined" && localStorage.getItem("settings_gemini_api_key")) || ""
);

let isDetecting = $state(false);
let isTestingGemini = $state(false);
let isSaved = $state(false);

async function handleAutoDetect() {
  isDetecting = true;
  try {
    const detected = await invoke<string | null>("auto_detect_ldplayer");
    if (detected) {
      ldPath = detected;
      logsStore.success("Settings", `Auto-detected LDPlayer at: ${detected}`);
    } else {
      logsStore.warn("Settings", "LDPlayer installation could not be auto-detected");
    }
  } finally {
    isDetecting = false;
  }
}

function handleShortcutKeyDown(e: KeyboardEvent, setter: (val: string) => void) {
  e.preventDefault();
  e.stopPropagation();

  const keys: string[] = [];
  if (e.ctrlKey) keys.push("Ctrl");
  if (e.shiftKey) keys.push("Shift");
  if (e.altKey) keys.push("Alt");

  const key = e.key;
  if (key !== "Control" && key !== "Shift" && key !== "Alt" && key !== "Meta") {
    if (key === " ") {
      keys.push("Space");
    } else if (key.length === 1) {
      keys.push(key.toUpperCase());
    } else {
      keys.push(key);
    }
  }

  if (keys.length > 0) {
    setter(keys.join("+"));
  }
}

async function handleTestGemini() {
  if (!geminiApiKey.trim()) {
    logsStore.warn("AI", "Please enter a Gemini API Key first");
    return;
  }

  isTestingGemini = true;
  try {
    const geminiUrl = `https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=${geminiApiKey.trim()}`;
    const res = await fetch(geminiUrl, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        contents: [{ parts: [{ text: 'Respond with JSON: {"status":"ok"}' }] }],
      }),
    });

    if (!res.ok) {
      throw new Error(`API returned HTTP ${res.status}`);
    }
    logsStore.success("AI", "Successfully verified Gemini Vision API connection!");
  } catch (e: any) {
    logsStore.error("AI", `Gemini API test failed: ${e?.message || e}`);
  } finally {
    isTestingGemini = false;
  }
}

async function handleSave() {
  await settingsStore.save({ ldplayerPath: ldPath });

  if (typeof localStorage !== "undefined") {
    localStorage.setItem("settings_emulator_engine", emulatorEngine);
    localStorage.setItem("settings_ldplayer_path", ldPath);
    localStorage.setItem("settings_mumuplayer_path", mumuPath);
    localStorage.setItem("settings_shortcut_app_toggle", appToggleShortcut);
    localStorage.setItem("settings_shortcut_toggle", toggleShortcut);
    localStorage.setItem("settings_auto_connect_vpn", String(autoConnectVpn));
    localStorage.setItem("settings_auto_connect_proxy", String(autoConnectProxy));
    localStorage.setItem("settings_auto_reconnect_vpn", String(autoReconnectProxy));
    localStorage.setItem("settings_auto_stop_after_task", String(autoStopAfterTask));
    localStorage.setItem("settings_auto_capture_screenshots", String(autoCaptureScreenshots));
    localStorage.setItem("settings_proxy_killswitch", String(proxyKillSwitch));
    localStorage.setItem("settings_gemini_api_key", geminiApiKey.trim());
  }

  isSaved = true;
  logsStore.success("Settings", "Saved application preferences to local configuration");
  setTimeout(() => (isSaved = false), 2000);
}
</script>

<div class="flex-1 flex flex-col h-full gap-5 overflow-y-auto font-sans p-2 custom-scrollbar">
  <!-- Section 1: Emulator Engine (100% Fidelity with D:\ldremote) -->
  <div class="flex flex-col gap-3 p-5 rounded-2xl bg-bg-panel border border-border-default shadow-xs">
    <div class="flex items-center justify-between">
      <h3 class="text-[11px] font-extrabold uppercase tracking-widest text-text-muted select-none">
        Emulator Engine
      </h3>
      <button
        type="button"
        onclick={handleAutoDetect}
        disabled={isDetecting}
        class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl border border-border-default hover:border-[#00b578]/50 bg-bg-card hover:bg-[#00b578]/10 text-text-default hover:text-[#00b578] text-[10px] font-extrabold uppercase tracking-wider transition-all cursor-pointer shadow-xs active:scale-95 disabled:opacity-50"
      >
        <Icon
          name="search"
          size={12}
          class={isDetecting ? "animate-spin text-[#00b578]" : "text-[#00b578]"}
        />
        <span>{isDetecting ? "Detecting..." : "Auto Detect"}</span>
      </button>
    </div>

    <!-- Engine Selector Segmented Pill & Description -->
    <div class="flex flex-wrap items-center gap-4 pt-1">
      <div class="inline-flex items-center p-1 rounded-xl bg-bg-app border border-border-default shadow-inner">
        <!-- LDPlayer Option -->
        <button
          type="button"
          onclick={() => (emulatorEngine = "ldplayer")}
          class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all duration-150 cursor-pointer {emulatorEngine ===
          'ldplayer'
            ? 'bg-[#00b578] text-white shadow-md shadow-[#00b578]/25'
            : 'text-text-muted hover:text-text-hover hover:bg-bg-card'}"
        >
          <Icon name="cube" size={14} />
          <span>LDPlayer</span>
        </button>

        <!-- MuMu Player Option -->
        <button
          type="button"
          onclick={() => (emulatorEngine = "mumu")}
          class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all duration-150 cursor-pointer {emulatorEngine ===
          'mumu'
            ? 'bg-[#00b578] text-white shadow-md shadow-[#00b578]/25'
            : 'text-text-muted hover:text-text-hover hover:bg-bg-card'}"
        >
          <Icon name="smartphone" size={14} />
          <span>MuMu Player</span>
        </button>
      </div>

      <p class="text-xs text-text-muted select-none">
        Configure default actions to launch and manage instances via ldconsole.
      </p>
    </div>

    <!-- Dual Install Folders Cards Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
      <!-- LDPlayer Install Folder Card -->
      <div
        class="flex items-center justify-between gap-3 p-3.5 px-4 rounded-xl border border-border-default bg-bg-card/40 focus-within:border-[#00b578] focus-within:ring-2 focus-within:ring-[#00b578]/15 transition-all shadow-xs"
      >
        <div class="flex items-center gap-3 min-w-0 flex-1">
          <div class="text-text-muted shrink-0">
            <Icon name="folder" size={18} />
          </div>
          <div class="flex flex-col min-w-0 flex-1">
            <span class="text-[9px] font-extrabold uppercase tracking-wider text-text-muted select-none">
              LDPlayer Install Folder
            </span>
            <input
              type="text"
              bind:value={ldPath}
              placeholder="e.g. C:\LDPlayer\LDPlayer9"
              class="w-full bg-transparent border-none outline-none text-xs font-mono font-bold text-text-default placeholder:text-text-muted pt-0.5"
            />
          </div>
        </div>
        <button
          type="button"
          onclick={() => handleAutoDetect()}
          class="px-3 py-1.5 rounded-lg border border-border-default hover:border-border-hover bg-bg-card hover:bg-bg-card-hover text-text-default text-xs font-bold transition-all cursor-pointer shrink-0 shadow-xs"
        >
          Browse
        </button>
      </div>

      <!-- MuMu Player Install Folder Card -->
      <div
        class="flex items-center justify-between gap-3 p-3.5 px-4 rounded-xl border border-border-default bg-bg-card/40 focus-within:border-[#00b578] focus-within:ring-2 focus-within:ring-[#00b578]/15 transition-all shadow-xs"
      >
        <div class="flex items-center gap-3 min-w-0 flex-1">
          <div class="text-text-muted shrink-0">
            <Icon name="folder" size={18} />
          </div>
          <div class="flex flex-col min-w-0 flex-1">
            <span class="text-[9px] font-extrabold uppercase tracking-wider text-text-muted select-none">
              MuMu Player Install Folder
            </span>
            <input
              type="text"
              bind:value={mumuPath}
              placeholder="e.g. C:\Program Files\MuMuPlayer-12.0"
              class="w-full bg-transparent border-none outline-none text-xs font-mono font-bold text-text-default placeholder:text-text-muted pt-0.5"
            />
          </div>
        </div>
        <button
          type="button"
          class="px-3 py-1.5 rounded-lg border border-border-default hover:border-border-hover bg-bg-card hover:bg-bg-card-hover text-text-default text-xs font-bold transition-all cursor-pointer shrink-0 shadow-xs"
        >
          Browse
        </button>
      </div>
    </div>
  </div>

  <!-- Section 2: Keyboard Shortcuts -->
  <div class="flex flex-col gap-3 p-5 rounded-2xl bg-bg-panel border border-border-default shadow-xs">
    <h3 class="text-[11px] font-extrabold uppercase tracking-widest text-text-muted select-none">
      Keyboard Shortcuts
    </h3>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Toggle Application Window -->
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] font-extrabold text-text-muted select-none">
          Toggle Application Window (Global)
        </span>
        <div class="relative flex items-center h-9 w-full">
          <input
            type="text"
            value={appToggleShortcut}
            readonly
            placeholder="Click to record hotkey..."
            onkeydown={(e) =>
              handleShortcutKeyDown(e, (v) => (appToggleShortcut = v))}
            class="h-full w-full border border-border-default rounded-xl px-3.5 pr-14 text-xs font-mono font-bold bg-bg-card/40 text-text-default placeholder:text-text-muted cursor-pointer shadow-xs focus:border-[#00b578] focus:ring-2 focus:ring-[#00b578]/15 transition-all"
          />
          <button
            type="button"
            onclick={() => (appToggleShortcut = "Ctrl+Space")}
            class="absolute right-3 text-[10px] font-bold text-[#00b578] hover:underline bg-transparent border-none cursor-pointer"
          >
            Reset
          </button>
        </div>
      </div>

      <!-- Toggle Emulators Visibility -->
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] font-extrabold text-text-muted select-none">
          Toggle Emulators Visibility (Local)
        </span>
        <div class="relative flex items-center h-9 w-full">
          <input
            type="text"
            value={toggleShortcut}
            readonly
            placeholder="Click to record hotkey..."
            onkeydown={(e) =>
              handleShortcutKeyDown(e, (v) => (toggleShortcut = v))}
            class="h-full w-full border border-border-default rounded-xl px-3.5 pr-14 text-xs font-mono font-bold bg-bg-card/40 text-text-default placeholder:text-text-muted cursor-pointer shadow-xs focus:border-[#00b578] focus:ring-2 focus:ring-[#00b578]/15 transition-all"
          />
          <button
            type="button"
            onclick={() => (toggleShortcut = "Ctrl+Shift+Space")}
            class="absolute right-3 text-[10px] font-bold text-[#00b578] hover:underline bg-transparent border-none cursor-pointer"
          >
            Reset
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Section 3: Automation & Route Protection -->
  <div class="flex flex-col gap-3 p-5 rounded-2xl bg-bg-panel border border-border-default shadow-xs">
    <h3 class="text-[11px] font-extrabold uppercase tracking-widest text-text-muted select-none">
      Automation & Route Protection
    </h3>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5">
      <!-- Auto-Connect VPN -->
      <div
        role="button"
        tabindex="0"
        onclick={() => (autoConnectVpn = !autoConnectVpn)}
        onkeydown={(e) => e.key === "Enter" && (autoConnectVpn = !autoConnectVpn)}
        class="flex items-center justify-between p-3.5 rounded-xl border transition-all cursor-pointer select-none shadow-xs {autoConnectVpn
          ? 'border-[#00b578]/40 bg-[#00b578]/5'
          : 'border-border-default bg-bg-card/30 hover:bg-bg-card/60'}"
      >
        <div class="flex items-start gap-3 min-w-0">
          <div
            class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 mt-0.5 {autoConnectVpn
              ? 'bg-[#00b578]/15 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
          >
            <Icon name="shield" size={15} />
          </div>
          <div class="min-w-0">
            <p class="text-xs font-bold text-text-hover">Auto-Connect VPN on Startup</p>
            <p class="text-[10px] text-text-muted mt-0.5 leading-relaxed">
              Connect assigned OpenVPN profile upon emulator boot.
            </p>
          </div>
        </div>
        <div
          class="w-7 h-4 rounded-full p-0.5 transition-colors duration-200 shrink-0 ml-2 {autoConnectVpn
            ? 'bg-[#00b578]'
            : 'bg-zinc-700'}"
        >
          <div
            class="w-3 h-3 rounded-full bg-white transition-transform duration-200 {autoConnectVpn
              ? 'translate-x-3'
              : 'translate-x-0'}"
          ></div>
        </div>
      </div>

      <!-- Auto-Inject Proxy -->
      <div
        role="button"
        tabindex="0"
        onclick={() => (autoConnectProxy = !autoConnectProxy)}
        onkeydown={(e) => e.key === "Enter" && (autoConnectProxy = !autoConnectProxy)}
        class="flex items-center justify-between p-3.5 rounded-xl border transition-all cursor-pointer select-none shadow-xs {autoConnectProxy
          ? 'border-[#00b578]/40 bg-[#00b578]/5'
          : 'border-border-default bg-bg-card/30 hover:bg-bg-card/60'}"
      >
        <div class="flex items-start gap-3 min-w-0">
          <div
            class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 mt-0.5 {autoConnectProxy
              ? 'bg-[#00b578]/15 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
          >
            <Icon name="network" size={15} />
          </div>
          <div class="min-w-0">
            <p class="text-xs font-bold text-text-hover">Auto-Inject Proxy on Startup</p>
            <p class="text-[10px] text-text-muted mt-0.5 leading-relaxed">
              Apply HTTP/SOCKS5 proxy settings to emulator instance.
            </p>
          </div>
        </div>
        <div
          class="w-7 h-4 rounded-full p-0.5 transition-colors duration-200 shrink-0 ml-2 {autoConnectProxy
            ? 'bg-[#00b578]'
            : 'bg-zinc-700'}"
        >
          <div
            class="w-3 h-3 rounded-full bg-white transition-transform duration-200 {autoConnectProxy
              ? 'translate-x-3'
              : 'translate-x-0'}"
          ></div>
        </div>
      </div>

      <!-- Auto-Reconnect Proxy -->
      <div
        role="button"
        tabindex="0"
        onclick={() => (autoReconnectProxy = !autoReconnectProxy)}
        onkeydown={(e) => e.key === "Enter" && (autoReconnectProxy = !autoReconnectProxy)}
        class="flex items-center justify-between p-3.5 rounded-xl border transition-all cursor-pointer select-none shadow-xs {autoReconnectProxy
          ? 'border-[#00b578]/40 bg-[#00b578]/5'
          : 'border-border-default bg-bg-card/30 hover:bg-bg-card/60'}"
      >
        <div class="flex items-start gap-3 min-w-0">
          <div
            class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 mt-0.5 {autoReconnectProxy
              ? 'bg-[#00b578]/15 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
          >
            <Icon name="refresh" size={15} />
          </div>
          <div class="min-w-0">
            <p class="text-xs font-bold text-text-hover">Auto-Reconnect Dropped Tunnels</p>
            <p class="text-[10px] text-text-muted mt-0.5 leading-relaxed">
              Monitor latency and revive dropped proxy streams.
            </p>
          </div>
        </div>
        <div
          class="w-7 h-4 rounded-full p-0.5 transition-colors duration-200 shrink-0 ml-2 {autoReconnectProxy
            ? 'bg-[#00b578]'
            : 'bg-zinc-700'}"
        >
          <div
            class="w-3 h-3 rounded-full bg-white transition-transform duration-200 {autoReconnectProxy
              ? 'translate-x-3'
              : 'translate-x-0'}"
          ></div>
        </div>
      </div>

      <!-- Proxy Kill-Switch -->
      <div
        role="button"
        tabindex="0"
        onclick={() => (proxyKillSwitch = !proxyKillSwitch)}
        onkeydown={(e) => e.key === "Enter" && (proxyKillSwitch = !proxyKillSwitch)}
        class="flex items-center justify-between p-3.5 rounded-xl border transition-all cursor-pointer select-none shadow-xs {proxyKillSwitch
          ? 'border-[#00b578]/40 bg-[#00b578]/5'
          : 'border-border-default bg-bg-card/30 hover:bg-bg-card/60'}"
      >
        <div class="flex items-start gap-3 min-w-0">
          <div
            class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 mt-0.5 {proxyKillSwitch
              ? 'bg-[#00b578]/15 text-[#00b578]'
              : 'bg-bg-app text-text-muted'}"
          >
            <Icon name="alert" size={15} />
          </div>
          <div class="min-w-0">
            <p class="text-xs font-bold text-text-hover">Proxy Kill-Switch Protection</p>
            <p class="text-[10px] text-text-muted mt-0.5 leading-relaxed">
              Block leakages if VPN or proxy tunnel unexpectedly drops.
            </p>
          </div>
        </div>
        <div
          class="w-7 h-4 rounded-full p-0.5 transition-colors duration-200 shrink-0 ml-2 {proxyKillSwitch
            ? 'bg-[#00b578]'
            : 'bg-zinc-700'}"
        >
          <div
            class="w-3 h-3 rounded-full bg-white transition-transform duration-200 {proxyKillSwitch
              ? 'translate-x-3'
              : 'translate-x-0'}"
          ></div>
        </div>
      </div>
    </div>
  </div>

  <!-- Section 4: AI & Gemini API -->
  <div class="flex flex-col gap-3 p-5 rounded-2xl bg-bg-panel border border-border-default shadow-xs">
    <h3 class="text-[11px] font-extrabold uppercase tracking-widest text-text-muted select-none">
      AI Engine & Vision Integration
    </h3>

    <div class="flex flex-col gap-2">
      <span class="text-[10px] font-extrabold text-text-muted select-none">
        Google Gemini API Key
      </span>
      <div class="flex items-center gap-3">
        <input
          type="password"
          bind:value={geminiApiKey}
          placeholder="Enter your Gemini API key (AIzaSy...)"
          class="flex-1 h-9.5 px-3.5 rounded-xl border border-border-default bg-bg-card/40 font-mono text-xs text-text-default focus:border-[#00b578] focus:ring-2 focus:ring-[#00b578]/15 outline-none transition-all shadow-xs"
        />
        <button
          type="button"
          onclick={handleTestGemini}
          disabled={isTestingGemini || !geminiApiKey.trim()}
          class="h-9.5 px-4 rounded-xl border border-border-default hover:border-[#00b578]/50 bg-bg-card hover:bg-[#00b578]/10 text-text-default hover:text-[#00b578] text-xs font-bold transition-all cursor-pointer shrink-0 disabled:opacity-40 disabled:pointer-events-none shadow-xs active:scale-95"
        >
          {isTestingGemini ? "Testing..." : "Test Connection"}
        </button>
      </div>
      <p class="text-[11px] text-text-muted mt-0.5">
        Used for intelligent OCR parsing, layout reasoning, and computer vision node workflows.
      </p>
    </div>
  </div>

  <!-- Footer Actions -->
  <div class="flex items-center justify-end pt-2 pb-6">
    <button
      type="button"
      onclick={handleSave}
      class="inline-flex items-center gap-2 h-10 px-6 rounded-xl font-bold text-xs text-white bg-gradient-to-b from-[#00c985] to-[#00b578] hover:from-[#00d78e] hover:to-[#00c07f] active:scale-[0.98] border border-[#00b578] shadow-[0_2px_12px_rgba(0,181,120,0.3),inset_0_1px_0_rgba(255,255,255,0.2)] transition-all cursor-pointer"
    >
      <Icon name="check" size={15} />
      <span>{isSaved ? "Saved Successfully!" : "Save Application Settings"}</span>
    </button>
  </div>
</div>
