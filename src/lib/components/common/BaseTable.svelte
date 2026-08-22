<script lang="ts">
import { onMount } from "svelte";
import Icon from "../ui/Icon.svelte";

export interface ColumnConfig {
  key: string;
  label: string;
  visible: boolean;
  canHide: boolean;
  width?: number;
  align?: "left" | "center" | "right";
}

interface Props {
  columns: ColumnConfig[];
  items: any[];
  selectedKeys?: any[];
  itemKey?: string;
  checkboxClassName?: string;
  onUpdateSelectedKeys?: (keys: any[]) => void;
  onRowClick?: (item: any, event: MouseEvent) => void;
  onRowContextMenu?: (item: any, event: MouseEvent) => void;
  rowSnippet?: any;
}

let {
  columns = $bindable([]),
  items = [],
  selectedKeys = $bindable([]),
  itemKey = "index",
  checkboxClassName = "custom-checkbox",
  onUpdateSelectedKeys,
  onRowClick,
  onRowContextMenu,
  rowSnippet,
}: Props = $props();

// Column Filter Popover State
let showColumnFilter = $state(false);
let columnFilterRef = $state<HTMLElement | null>(null);

// Column Widths Map
let columnWidths = $state<Record<string, number>>({});

onMount(() => {
  // Load saved column widths from localStorage
  try {
    const savedWidths = localStorage.getItem("ldnoob_table_column_widths");
    if (savedWidths) {
      columnWidths = JSON.parse(savedWidths);
    }
  } catch {
    // fallback
  }

  // Initialize defaults if not saved
  columns.forEach((col) => {
    if (!columnWidths[col.key]) {
      columnWidths[col.key] = col.width || 120;
    }
  });

  // Close popover when clicking outside
  function handleClickOutside(e: MouseEvent) {
    if (showColumnFilter && columnFilterRef && !columnFilterRef.contains(e.target as Node)) {
      showColumnFilter = false;
    }
  }
  window.addEventListener("click", handleClickOutside);
  return () => window.removeEventListener("click", handleClickOutside);
});

function saveWidths() {
  try {
    localStorage.setItem("ldnoob_table_column_widths", JSON.stringify(columnWidths));
  } catch {
    // ignore
  }
}

// Visible Columns
let visibleColumns = $derived(columns.filter((c) => c.visible));

// Selection Computes
let allSelected = $derived(items.length > 0 && selectedKeys.length === items.length);
let someSelected = $derived(selectedKeys.length > 0 && selectedKeys.length < items.length);

function toggleSelectAll() {
  if (allSelected) {
    selectedKeys = [];
    onUpdateSelectedKeys?.([]);
  } else {
    selectedKeys = items.map((i) => i[itemKey]);
    onUpdateSelectedKeys?.(selectedKeys);
  }
}

function toggleSelect(key: any) {
  if (selectedKeys.includes(key)) {
    selectedKeys = selectedKeys.filter((k) => k !== key);
  } else {
    selectedKeys = [...selectedKeys, key];
  }
  onUpdateSelectedKeys?.(selectedKeys);
}

// Shift-click range selection
let lastSelectedIndex = $state<number | null>(null);

function handleRowClickInternal(item: any, index: number, event: MouseEvent) {
  onRowClick?.(item, event);

  const key = item[itemKey];
  if (event.shiftKey && lastSelectedIndex !== null) {
    event.preventDefault();
    const start = Math.min(lastSelectedIndex, index);
    const end = Math.max(lastSelectedIndex, index);
    const range = items.slice(start, end + 1).map((i) => i[itemKey]);

    if (event.ctrlKey || event.metaKey) {
      selectedKeys = Array.from(new Set([...selectedKeys, ...range]));
    } else {
      selectedKeys = range;
    }
    onUpdateSelectedKeys?.(selectedKeys);
  } else if (event.ctrlKey || event.metaKey) {
    toggleSelect(key);
  } else {
    selectedKeys = [key];
    onUpdateSelectedKeys?.(selectedKeys);
  }
  lastSelectedIndex = index;
}

// Column Resizing
let isResizing = false;
let activeResizeCol = "";
let startX = 0;
let startW = 0;

function startResize(e: MouseEvent, colKey: string) {
  e.preventDefault();
  e.stopPropagation();
  isResizing = true;
  activeResizeCol = colKey;
  startX = e.clientX;
  startW = columnWidths[colKey] || 120;

  document.body.style.userSelect = "none";
  window.addEventListener("mousemove", handleMouseMove);
  window.addEventListener("mouseup", stopResize);
}

function handleMouseMove(e: MouseEvent) {
  if (!isResizing || !activeResizeCol) return;
  const dx = e.clientX - startX;
  columnWidths[activeResizeCol] = Math.max(50, startW + dx);
}

function stopResize() {
  if (isResizing) {
    isResizing = false;
    activeResizeCol = "";
    document.body.style.userSelect = "";
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("mouseup", stopResize);
    saveWidths();
  }
}

// Column Filter Actions
function resetColumns() {
  columns = columns.map((c) => ({ ...c, visible: true }));
}

function showAllColumns() {
  columns = columns.map((c) => ({ ...c, visible: true }));
}

function hideEmptyColumns() {
  columns = columns.map((c) => {
    if (!c.canHide) return c;
    const hasValue = items.some(
      (item) => item[c.key] !== undefined && item[c.key] !== null && item[c.key] !== ""
    );
    return { ...c, visible: hasValue };
  });
}
</script>

<div
  class="w-full h-full flex flex-col overflow-hidden rounded-2xl border border-slate-200/90 dark:border-white/[0.08] bg-white dark:bg-[#0c0e15] shadow-xs transition-colors"
>
  <!-- Table Container -->
  <div class="flex-1 w-full overflow-auto select-none relative">
    <table class="w-full text-left border-collapse text-xs">
      <!-- Table Header -->
      <thead
        class="sticky top-0 z-30 bg-slate-100/95 dark:bg-[#12141f] border-b border-slate-200 dark:border-white/[0.08] text-slate-500 dark:text-slate-400 font-mono text-[11px]"
      >
        <tr>
          <!-- Checkbox Column -->
          <th
            class="w-10 min-w-[40px] max-w-[40px] p-0 sticky left-0 z-40 bg-slate-100 dark:bg-[#12141f] border-r border-slate-200/80 dark:border-white/[0.06]"
          >
            <div class="flex items-center justify-center py-2.5 w-full h-full">
              <input
                type="checkbox"
                checked={allSelected}
                indeterminate={someSelected}
                onchange={toggleSelectAll}
                class="w-3.5 h-3.5 rounded border-slate-300 dark:border-white/20 text-emerald-500 focus:ring-emerald-500/20 cursor-pointer"
              />
            </div>
          </th>

          <!-- Dynamic Columns -->
          {#each visibleColumns as col (col.key)}
            {@const colWidth = (columnWidths[col.key] || col.width || 120) + "px"}
            <th
              class="py-2.5 px-3 border-r border-slate-200/60 dark:border-white/[0.06] font-bold uppercase tracking-wider relative group {col.key ===
              'index'
                ? 'text-center'
                : col.align === 'right'
                  ? 'text-right'
                  : 'text-left'} {col.key === 'actions'
                ? 'sticky right-0 z-40 bg-slate-100 dark:bg-[#12141f] shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.1)] dark:shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.4)]'
                : ''}"
              style="width: {colWidth}; min-width: {colWidth}; max-width: {colWidth};"
            >
              <div class="flex items-center justify-between gap-1 overflow-hidden">
                <span class="truncate">{col.label}</span>

                <!-- Column Settings Filter Trigger (on Actions Header) -->
                {#if col.key === "actions"}
                  <div class="relative" bind:this={columnFilterRef}>
                    <button
                      type="button"
                      title="Customize Columns"
                      onclick={(e) => {
                        e.stopPropagation();
                        showColumnFilter = !showColumnFilter;
                      }}
                      class="p-1 rounded-md text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-200 dark:hover:bg-white/10 transition-colors cursor-pointer {showColumnFilter
                        ? 'bg-slate-200 dark:bg-white/10 text-emerald-500'
                        : ''}"
                    >
                      <Icon name="filter" size={12} />
                    </button>

                    <!-- Column Chooser Dropdown Popover -->
                    {#if showColumnFilter}
                      <div
                        class="absolute right-0 top-full mt-1.5 w-52 p-2 bg-white dark:bg-[#161824] border border-slate-200 dark:border-white/[0.10] rounded-xl shadow-2xl z-50 animate-in fade-in zoom-in-95 duration-150 font-sans text-xs"
                      >
                        <!-- Action Row -->
                        <div
                          class="flex items-center justify-between pb-2 mb-2 border-b border-slate-200 dark:border-white/[0.06] text-[10px] font-bold text-slate-500 dark:text-slate-400"
                        >
                          <button
                            type="button"
                            onclick={resetColumns}
                            class="hover:text-emerald-500 cursor-pointer"
                          >
                            Reset
                          </button>
                          <span class="text-slate-300 dark:text-slate-600">|</span>
                          <button
                            type="button"
                            onclick={hideEmptyColumns}
                            class="hover:text-emerald-500 cursor-pointer"
                          >
                            Hide Empty
                          </button>
                          <span class="text-slate-300 dark:text-slate-600">|</span>
                          <button
                            type="button"
                            onclick={showAllColumns}
                            class="hover:text-emerald-500 cursor-pointer"
                          >
                            Show All
                          </button>
                        </div>

                        <!-- Checkboxes List -->
                        <div class="space-y-1 max-h-48 overflow-y-auto pr-1">
                          {#each columns as c}
                            <label
                              class="flex items-center justify-between px-2 py-1 rounded-lg hover:bg-slate-100 dark:hover:bg-white/[0.04] text-slate-700 dark:text-slate-300 cursor-pointer {c.canHide
                                ? ''
                                : 'opacity-60 cursor-not-allowed'}"
                            >
                              <span class="text-[11px]">{c.label}</span>
                              <input
                                type="checkbox"
                                bind:checked={c.visible}
                                disabled={!c.canHide}
                                class="w-3.5 h-3.5 rounded border-slate-300 dark:border-white/20 text-emerald-500 focus:ring-emerald-500/20 cursor-pointer"
                              />
                            </label>
                          {/each}
                        </div>
                      </div>
                    {/if}
                  </div>
                {/if}
              </div>

              <!-- Column Resizer Handle -->
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <div
                role="separator"
                tabindex="-1"
                aria-orientation="vertical"
                class="absolute top-0 right-0 h-full w-1.5 cursor-col-resize hover:bg-emerald-500/40 active:bg-emerald-500 transition-colors z-20"
                onmousedown={(e) => startResize(e, col.key)}
              ></div>
            </th>
          {/each}
        </tr>
      </thead>

      <!-- Table Body -->
      <tbody class="divide-y divide-slate-100 dark:divide-white/[0.04]">
        {#if items.length === 0}
          <tr>
            <td
              colspan={visibleColumns.length + 1}
              class="py-16 text-center text-slate-400 dark:text-slate-500 font-sans"
            >
              <div class="flex flex-col items-center justify-center gap-2">
                <Icon name="cube" size={28} class="opacity-40" />
                <p class="text-xs font-semibold">No instances or items found</p>
              </div>
            </td>
          </tr>
        {:else}
          {#each items as item, index (item[itemKey] ?? index)}
            {@const key = item[itemKey] ?? index}
            {@const isSelected = selectedKeys.includes(key)}
            <tr
              onclick={(e) => handleRowClickInternal(item, index, e)}
              oncontextmenu={(e) => {
                e.preventDefault();
                onRowContextMenu?.(item, e);
              }}
              class="group transition-all duration-100 cursor-pointer {isSelected
                ? 'bg-emerald-500/[0.08] dark:bg-emerald-500/[0.12] text-slate-900 dark:text-white shadow-[inset_3px_0_0_0_#00b578]'
                : 'hover:bg-slate-50 dark:hover:bg-white/[0.03] text-slate-700 dark:text-slate-300'}"
            >
              <!-- Checkbox Cell (Sticky Left) -->
              <td
                class="w-10 min-w-[40px] max-w-[40px] p-0 sticky left-0 z-10 border-r border-slate-200/50 dark:border-white/[0.04] transition-colors {isSelected
                  ? 'bg-emerald-500/[0.06] dark:bg-[#101920]'
                  : 'bg-white dark:bg-[#0c0e15] group-hover:bg-slate-50 dark:group-hover:bg-[#12141f]'}"
                onclick={(e) => e.stopPropagation()}
              >
                <div class="flex items-center justify-center py-2 w-full h-full">
                  <input
                    type="checkbox"
                    checked={isSelected}
                    onchange={() => toggleSelect(key)}
                    class="w-3.5 h-3.5 rounded border-slate-300 dark:border-white/20 text-emerald-500 focus:ring-emerald-500/20 cursor-pointer"
                  />
                </div>
              </td>

              <!-- Custom Snippet Render -->
              {#if rowSnippet}
                {@render rowSnippet(item, isSelected)}
              {:else}
                {#each visibleColumns as col}
                  {@const colWidth = (columnWidths[col.key] || col.width || 120) + "px"}
                  <td
                    class="py-2.5 px-3 border-r border-slate-100 dark:border-white/[0.04] font-mono truncate {col.key ===
                    'index'
                      ? 'text-center font-bold text-slate-400'
                      : ''} {col.key === 'actions'
                      ? `sticky right-0 z-10 shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.1)] dark:shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.4)] ${
                          isSelected
                            ? 'bg-emerald-500/[0.06] dark:bg-[#101920]'
                            : 'bg-white dark:bg-[#0c0e15] group-hover:bg-slate-50 dark:group-hover:bg-[#12141f]'
                        }`
                      : ''}"
                    style="width: {colWidth}; min-width: {colWidth}; max-width: {colWidth};"
                  >
                    {item[col.key] ?? "-"}
                  </td>
                {/each}
              {/if}
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>
