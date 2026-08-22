<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";
import { type LogEntry, logsStore } from "$lib/stores/logs.svelte";
import Icon from "../ui/Icon.svelte";

let selectedCategory = $state("All");
let selectedLevel = $state("All");
let searchQuery = $state("");
let copiedAll = $state(false);
let copiedRowId = $state<string | null>(null);

let viewingImage = $state<string | null>(null);
let imageLoading = $state(false);
let logsContainerRef = $state<HTMLDivElement | null>(null);

const categories = ["All", "Facebook Registration", "Emulator", "Proxy", "System", "ADB"];
const severities = ["All", "Info", "Success", "Warning", "Error"];

// Filtered logs
let filteredLogs = $derived(
  logsStore.entries.filter((log) => {
    const q = searchQuery.toLowerCase().trim();
    const categoryMatch =
      selectedCategory === "All" ||
      log.category.toLowerCase() === selectedCategory.toLowerCase() ||
      (selectedCategory === "Facebook Registration" &&
        log.category.toLowerCase().includes("facebook"));

    const levelMatch =
      selectedLevel === "All" ||
      log.level.toLowerCase() === selectedLevel.toLowerCase() ||
      (selectedLevel === "Info" && log.level.toLowerCase() === "info") ||
      (selectedLevel === "Warning" && log.level.toLowerCase() === "warn");

    const searchMatch =
      !q ||
      log.message.toLowerCase().includes(q) ||
      log.category.toLowerCase().includes(q) ||
      log.level.toLowerCase().includes(q);

    return categoryMatch && levelMatch && searchMatch;
  })
);

// Auto scroll to bottom
$effect(() => {
  if (logsStore.entries.length && logsContainerRef) {
    logsContainerRef.scrollTop = logsContainerRef.scrollHeight;
  }
});

onMount(() => {
  function handleKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "b") {
      e.preventDefault();
      logsStore.togglePanel();
    }
  }

  window.addEventListener("keydown", handleKeyDown);
  return () => window.removeEventListener("keydown", handleKeyDown);
});

function handleCopyAll() {
  if (filteredLogs.length === 0) return;
  const text = filteredLogs
    .map(
      (log) =>
        `[${log.timestamp}] [${log.category.toUpperCase()}] [${log.level.toUpperCase()}] ${log.message}`
    )
    .join("\n");
  navigator.clipboard.writeText(text);
  copiedAll = true;
  setTimeout(() => (copiedAll = false), 2000);
}

function handleCopyRow(log: LogEntry) {
  navigator.clipboard.writeText(log.message);
  copiedRowId = log.id;
  setTimeout(() => {
    if (copiedRowId === log.id) copiedRowId = null;
  }, 1500);
}

function getScreenshotPath(msg: string): string | null {
  const match = msg.match(/\.?\/debug_screenshots\/[^\s'"]+\.png/);
  return match ? match[0] : null;
}

async function handleViewScreenshot(path: string) {
  imageLoading = true;
  try {
    const data = await invoke<number[]>("read_binary_file", { filePath: path });
    const bytes = new Uint8Array(data);
    const blob = new Blob([bytes], { type: "image/png" });
    viewingImage = URL.createObjectURL(blob);
  } catch {
    // fallback
  } finally {
    imageLoading = false;
  }
}

function getCategoryBadgeClass(category: string) {
  const cat = category.toLowerCase();
  if (cat.includes("emulator")) {
    return "bg-[#1877f2]/12 text-[#1877f2] border-[#1877f2]/30";
  }
  if (cat.includes("facebook")) {
    return "bg-[#00b578]/12 text-[#00b578] border-[#00b578]/30";
  }
  if (cat.includes("proxy")) {
    return "bg-purple-500/15 text-purple-400 border-purple-500/30";
  }
  if (cat.includes("adb")) {
    return "bg-cyan-500/15 text-cyan-400 border-cyan-500/30";
  }
  return "bg-zinc-500/15 text-zinc-400 border-zinc-500/30";
}

function getLevelBadgeClass(level: string) {
  const lvl = level.toLowerCase();
  if (lvl === "success") {
    return "bg-[#00b578]/12 text-[#00b578] border-[#00b578]/30";
  }
  if (lvl === "info") {
    return "bg-[#1877f2]/12 text-[#1877f2] border-[#1877f2]/30";
  }
  if (lvl === "warn" || lvl === "warning") {
    return "bg-[#ffc107]/15 text-[#ffc107] border-[#ffc107]/30";
  }
  return "bg-[#ff4d4f]/15 text-[#ff4d4f] border-[#ff4d4f]/30";
}
let catRefs: Record<string, HTMLButtonElement | null> = $state({});
let catIndicator = $state({ left: 2, width: 0 });

let lvlRefs: Record<string, HTMLButtonElement | null> = $state({});
let lvlIndicator = $state({ left: 2, width: 0 });

$effect(() => {
  const el = catRefs[selectedCategory];
  if (el) {
    catIndicator = { left: el.offsetLeft, width: el.offsetWidth };
  }
});

$effect(() => {
  const el = lvlRefs[selectedLevel];
  if (el) {
    lvlIndicator = { left: el.offsetLeft, width: el.offsetWidth };
  }
});
</script>

{#if logsStore.isPanelOpen}
  <div
    class="bg-bg-panel/95 backdrop-blur-md border-t border-border-default h-64 flex flex-col shrink-0 overflow-hidden relative z-20 transition-colors duration-300 font-sans shadow-lg select-none"
  >
    <!-- Title & Controls Header (matching D:\ldremote) -->
    <div
      class="h-11 px-4 flex items-center justify-between select-none shrink-0 gap-3 bg-bg-card border-b border-border-default"
    >
      <!-- Left: Info & Filters -->
      <div class="flex items-center h-7 gap-2 select-none shrink-0">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-[#00b578] animate-pulse"></div>
          <span class="text-xs font-black uppercase tracking-wider text-text-hover font-mono">
            Activity Log
          </span>
          <span
            class="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-bg-app text-text-muted border border-border-default shadow-xs"
          >
            {filteredLogs.length} / {logsStore.entries.length}
          </span>
        </div>

        <span class="h-4 border-l border-border-default mx-1.5"></span>

        <!-- Category Tabs with Smooth Sliding Pill -->
        <div class="flex items-center gap-1.5 select-none font-bold shrink-0">
          <span class="text-[9px] font-black uppercase tracking-wider text-text-muted">
            Category
          </span>
          <div class="flex items-center gap-0.5 p-0.5 h-7 rounded-lg border bg-bg-app border-border-default relative shrink-0">
            {#if catIndicator.width > 0}
              <div
                class="absolute top-0.5 bottom-0.5 rounded-md bg-bg-card border border-border-default shadow-xs transition-all duration-200 cubic-bezier(0.16,1,0.3,1) pointer-events-none z-0"
                style="left: {catIndicator.left}px; width: {catIndicator.width}px;"
              ></div>
            {/if}
            {#each categories as cat}
              {@const isSelected = selectedCategory === cat}
              <button
                type="button"
                bind:this={catRefs[cat]}
                onclick={() => (selectedCategory = cat)}
                title={cat}
                class="px-2.5 min-w-[38px] h-[22px] flex items-center justify-center text-[9px] font-black rounded-md cursor-pointer transition-colors duration-150 relative z-10 shrink-0 {isSelected
                  ? 'text-text-hover font-bold'
                  : 'text-text-muted hover:text-text-hover'}"
              >
                <span>{cat === "Facebook Registration" ? "FB Reg" : cat}</span>
              </button>
            {/each}
          </div>
        </div>

        <span class="h-4 border-l border-border-default mx-1.5"></span>

        <!-- Level Tabs with Smooth Sliding Pill -->
        <div class="flex items-center gap-1.5 select-none shrink-0">
          <span class="text-[9px] font-black uppercase tracking-wider text-text-muted">
            Severity
          </span>
          <div class="flex items-center gap-0.5 p-0.5 h-7 rounded-lg border bg-bg-app border-border-default relative shrink-0">
            {#if lvlIndicator.width > 0}
              <div
                class="absolute top-0.5 bottom-0.5 rounded-md bg-bg-card border border-border-default shadow-xs transition-all duration-200 cubic-bezier(0.16,1,0.3,1) pointer-events-none z-0"
                style="left: {lvlIndicator.left}px; width: {lvlIndicator.width}px;"
              ></div>
            {/if}
            {#each severities as lvl}
              {@const isSelected = selectedLevel === lvl}
              <button
                type="button"
                bind:this={lvlRefs[lvl]}
                onclick={() => (selectedLevel = lvl)}
                class="px-2.5 min-w-[44px] h-[22px] text-[9px] font-bold flex items-center justify-center gap-1 rounded-md cursor-pointer transition-colors duration-150 relative z-10 shrink-0 {isSelected
                  ? 'text-text-hover font-bold'
                  : 'text-text-muted hover:text-text-hover'}"
              >
                {#if lvl !== "All"}
                  <span
                    class="w-1.5 h-1.5 rounded-full {lvl === 'Info'
                      ? 'bg-blue-500'
                      : lvl === 'Success'
                        ? 'bg-emerald-500'
                        : lvl === 'Warning'
                          ? 'bg-amber-500'
                          : 'bg-rose-500'}"
                  ></span>
                {/if}
                <span>{lvl}</span>
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Right: Search & Actions -->
      <div class="flex items-center h-7 gap-2.5">
        <!-- Search Input -->
        <div class="group relative flex items-center shrink-0 h-7">
          <input
            bind:value={searchQuery}
            type="text"
            placeholder="Search operations..."
            class="pl-7 pr-6 h-7 text-[10px] font-bold rounded-lg border border-border-default hover:border-border-hover focus:border-[#00b578] bg-bg-app text-text-default placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-[#00b578]/20 transition-all duration-150 w-44"
          />
          <span class="absolute left-2 text-text-muted pointer-events-none group-focus-within:text-[#00b578]">
            <Icon name="search" size={11} />
          </span>
          {#if searchQuery}
            <button
              type="button"
              onclick={() => (searchQuery = "")}
              class="absolute right-1.5 text-text-muted hover:text-text-hover p-0.5 hover:bg-border-default rounded-md cursor-pointer flex items-center justify-center transition-colors"
            >
              <Icon name="close" size={10} />
            </button>
          {/if}
        </div>

        <div class="flex items-center h-7 gap-1.5 border-l border-border-default pl-2">
          <!-- Copy All Logs Button -->
          <button
            type="button"
            onclick={handleCopyAll}
            disabled={filteredLogs.length === 0}
            class="h-7 px-2.5 border border-border-default hover:border-border-hover shrink-0 flex items-center gap-1.5 rounded-lg cursor-pointer transition-all duration-150 hover:scale-[1.02] active:scale-95 text-[10px] font-bold disabled:opacity-40 disabled:cursor-not-allowed bg-bg-app {copiedAll
              ? 'bg-[#00b578]/15 border-[#00b578]/40 text-[#00b578]'
              : 'text-text-muted hover:text-text-hover hover:bg-bg-card'}"
            title="Copy all filtered logs"
          >
            {#if copiedAll}
              <Icon name="check" size={12} />
              <span>Copied!</span>
            {:else}
              <Icon name="edit" size={11} />
              <span>Copy All</span>
            {/if}
          </button>

          <!-- Clear Logs Button -->
          <button
            type="button"
            onclick={() => logsStore.clear()}
            class="h-7 w-7 border border-border-default hover:border-border-hover shrink-0 flex items-center justify-center rounded-lg cursor-pointer transition-all duration-150 hover:scale-[1.05] active:scale-95 text-text-muted hover:text-[#ff4d4f] hover:bg-[#ff4d4f]/10 bg-bg-app"
            title="Clear all logs"
          >
            <Icon name="trash" size={12} />
          </button>

          <!-- Close Button -->
          <button
            type="button"
            onclick={() => logsStore.setPanelOpen(false)}
            class="h-7 w-7 border border-border-default hover:border-border-hover shrink-0 flex items-center justify-center rounded-lg cursor-pointer transition-all duration-150 hover:scale-[1.05] active:scale-95 text-text-muted hover:text-text-hover hover:bg-bg-card bg-bg-app"
            title="Close Panel (Ctrl+B)"
          >
            <Icon name="close" size={12} />
          </button>
        </div>
      </div>
    </div>

    <!-- Logs Area -->
    <div
      bind:this={logsContainerRef}
      class="flex-1 overflow-y-auto p-3 font-mono text-[10.5px] leading-relaxed space-y-1 bg-bg-app/80 text-text-default select-text"
    >
      {#if filteredLogs.length === 0}
        <div class="h-full flex items-center justify-center italic select-none text-text-muted">
          No activity logs match your selection.
        </div>
      {:else}
        {#each filteredLogs as log, idx (log.id ?? idx)}
          {@const screenshotPath = getScreenshotPath(log.message)}
          <div
            class="flex items-center gap-2 px-2 py-1 rounded-md transition-all duration-150 group relative hover:bg-bg-card-hover/40"
          >
            <!-- Timestamp -->
            <span class="font-bold shrink-0 select-none opacity-60 text-text-muted">
              [{log.timestamp}]
            </span>

            <!-- Category Badge -->
            <span
              class="px-1.5 py-0.2 rounded-full text-[8.5px] font-black uppercase tracking-wide shrink-0 border select-none {getCategoryBadgeClass(
                log.category
              )}"
            >
              {log.category.toLowerCase().includes("facebook") ? "FB Reg" : log.category}
            </span>

            <!-- Level Badge -->
            <span
              class="px-1.5 py-0.2 rounded-full text-[8.5px] font-black uppercase tracking-wide shrink-0 border select-none {getLevelBadgeClass(
                log.level
              )}"
            >
              {log.level}
            </span>

            <!-- Message -->
            <span class="font-medium break-all select-text flex-1 text-text-default">
              {log.message}
            </span>

            <!-- Screenshot View Button (if path detected) -->
            {#if screenshotPath}
              <button
                type="button"
                onclick={() => handleViewScreenshot(screenshotPath!)}
                disabled={imageLoading}
                class="p-1 shrink-0 cursor-pointer transition-all duration-150 hover:scale-105 active:scale-90 rounded-md text-[#1877f2] hover:text-[#40a9ff] hover:bg-bg-card"
                title="View screenshot image"
              >
                <Icon name="eye" size={13} />
              </button>
            {/if}

            <!-- Copy Row Button (visible on hover) -->
            <button
              type="button"
              onclick={() => handleCopyRow(log)}
              class="opacity-0 group-hover:opacity-100 p-1 shrink-0 cursor-pointer transition-all duration-150 hover:scale-105 active:scale-90 rounded-md text-text-muted hover:text-[#00b578] hover:bg-bg-card"
              title={copiedRowId === log.id ? "Copied!" : "Copy log message"}
            >
              {#if copiedRowId === log.id}
                <Icon name="check" size={12} class="text-[#00b578]" />
              {:else}
                <Icon name="edit" size={12} />
              {/if}
            </button>
          </div>
        {/each}
      {/if}
    </div>
  </div>
{/if}

<!-- Lightbox Modal for Screenshots -->
{#if viewingImage}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 bg-black/85 backdrop-blur-sm z-[10000] flex items-center justify-center p-4 select-none animate-in fade-in duration-150"
    role="presentation"
    onclick={() => (viewingImage = null)}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      class="relative max-w-2xl max-h-[90vh] bg-bg-panel border border-border-default rounded-2xl overflow-hidden shadow-2xl flex flex-col font-sans"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="h-10 px-4 flex items-center justify-between border-b border-border-default shrink-0 bg-bg-card">
        <span class="text-[10px] font-black uppercase tracking-wider text-text-muted">
          Failure Screenshot Viewer
        </span>
        <button
          type="button"
          onclick={() => (viewingImage = null)}
          class="p-1 hover:bg-bg-card-hover rounded-md text-text-muted hover:text-text-hover transition-colors cursor-pointer"
        >
          <Icon name="close" size={14} />
        </button>
      </div>

      <!-- Image Content -->
      <div class="p-4 flex items-center justify-center overflow-auto bg-bg-app">
        <img
          src={viewingImage}
          alt="Failure Screenshot"
          class="max-w-full max-h-[75vh] object-contain rounded-lg shadow-md select-text"
        />
      </div>
    </div>
  </div>
{/if}
