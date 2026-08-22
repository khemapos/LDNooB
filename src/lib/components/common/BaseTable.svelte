<script lang="ts">
import type { Snippet } from "svelte";
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
  paginate?: boolean;
  pageSize?: number;
  pageSizes?: number[];
  itemLabel?: string;
  isConnected?: boolean;
  checkboxClassName?: string;
  onUpdateSelectedKeys?: (keys: any[]) => void;
  onRowClick?: (item: any, event: MouseEvent) => void;
  onRowMouseDown?: (item: any, event: MouseEvent) => void;
  onRowMouseEnter?: (item: any, event: MouseEvent) => void;
  onRowContextMenu?: (item: any, event: MouseEvent) => void;
  renderCell?: Snippet<[string, any, number, ColumnConfig]>;
  renderHeader?: Snippet<[string, ColumnConfig]>;
  renderEmptyState?: Snippet<[]>;
  renderFooterCenter?: Snippet<[]>;
}

let {
  columns = $bindable([]),
  items = [],
  selectedKeys = $bindable([]),
  itemKey = "index",
  paginate = true,
  pageSize = $bindable(50),
  pageSizes = [10, 20, 50, 100, 200],
  itemLabel = "instances",
  isConnected = true,
  checkboxClassName = "custom-checkbox",
  onUpdateSelectedKeys,
  onRowClick,
  onRowMouseDown,
  onRowMouseEnter,
  onRowContextMenu,
  renderCell,
  renderHeader,
  renderEmptyState,
  renderFooterCenter,
}: Props = $props();

// Column Selector Dropdown State
let showColumnSelector = $state(false);
let columnSelectorRef = $state<HTMLDivElement | null>(null);
let defaultColumnsBackup: ColumnConfig[] = [];

// Column Drag & Drop Reorder State
let draggedColIndex = $state<number | null>(null);

// Column Widths Map
let columnWidths = $state<Record<string, number>>({});

// Pagination State
let currentPage = $state(1);
let isPageSizeOpen = $state(false);
let jumpPageInput = $state("");

onMount(() => {
  // Save default snapshot
  defaultColumnsBackup = JSON.parse(JSON.stringify(columns));

  // Try load saved column config from localStorage
  const savedConfig = localStorage.getItem(`table_columns_config_${itemLabel}`);
  if (savedConfig) {
    try {
      const parsed = JSON.parse(savedConfig);
      if (Array.isArray(parsed) && parsed.length > 0) {
        const merged = parsed.map((p: any) => {
          const match = columns.find((c) => c.key === p.key);
          return match ? { ...match, visible: p.visible } : p;
        });
        // Add any new columns that were missing
        columns.forEach((c) => {
          if (!merged.some((m: any) => m.key === c.key)) {
            merged.push(c);
          }
        });
        columns = merged;
      }
    } catch {}
  }

  // Initialize default column widths
  columns.forEach((col) => {
    if (col.visible === undefined) {
      col.visible = true;
    }
    if (!columnWidths[col.key]) {
      columnWidths[col.key] = col.width || 120;
    }
  });

  function handleOutsideClick(e: MouseEvent) {
    if (showColumnSelector && columnSelectorRef && !columnSelectorRef.contains(e.target as Node)) {
      showColumnSelector = false;
    }
  }

  window.addEventListener("click", handleOutsideClick);
  return () => window.removeEventListener("click", handleOutsideClick);
});

function saveColumnConfig() {
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(`table_columns_config_${itemLabel}`, JSON.stringify(columns));
  }
}

// Column Customization Helper Actions
function resetColumns() {
  if (defaultColumnsBackup.length > 0) {
    columns = JSON.parse(JSON.stringify(defaultColumnsBackup));
    saveColumnConfig();
  }
}

function showAllColumns() {
  columns = columns.map((col) => ({ ...col, visible: true }));
  saveColumnConfig();
}

function hideEmptyColumns() {
  columns = columns.map((col) => {
    if (!col.canHide) return col;
    // Check if all items are empty for this column
    const hasAnyValue = items.some((item) => {
      const val = item[col.key];
      return val !== undefined && val !== null && val !== "" && val !== "-" && val !== "Unassigned";
    });
    return { ...col, visible: hasAnyValue };
  });
  saveColumnConfig();
}

function toggleColumnVisibility(key: string) {
  columns = columns.map((col) => {
    if (col.key === key && col.canHide) {
      return { ...col, visible: !col.visible };
    }
    return col;
  });
  saveColumnConfig();
}

// Column Drag-and-Drop Reorder Logic
function handleDragStart(index: number) {
  draggedColIndex = index;
}

function handleDragOver(e: DragEvent, targetIndex: number) {
  e.preventDefault();
  if (draggedColIndex === null || draggedColIndex === targetIndex) return;

  const updated = [...columns];
  const [moved] = updated.splice(draggedColIndex, 1);
  updated.splice(targetIndex, 0, moved);
  columns = updated;
  draggedColIndex = targetIndex;
  saveColumnConfig();
}

function handleDragEnd() {
  draggedColIndex = null;
}

// Visible Columns
let visibleColumns = $derived(columns.filter((c) => c.visible));

// Pagination Computes
let totalItems = $derived(items.length);
let totalPages = $derived(Math.max(1, Math.ceil(totalItems / pageSize)));
let showingStart = $derived(totalItems === 0 ? 0 : (currentPage - 1) * pageSize + 1);
let showingEnd = $derived(Math.min(currentPage * pageSize, totalItems));
let paginatedItems = $derived(
  paginate ? items.slice((currentPage - 1) * pageSize, currentPage * pageSize) : items
);

// Reset currentPage if items change and currentPage is out of bounds
$effect(() => {
  if (currentPage > totalPages && totalPages > 0) {
    currentPage = totalPages;
  }
});

function getPageNumbers(curr: number, total: number): (number | string)[] {
  if (total <= 7) {
    return Array.from({ length: total }, (_, i) => i + 1);
  }
  if (curr <= 3) {
    return [1, 2, 3, 4, "...", total];
  }
  if (curr >= total - 2) {
    return [1, "...", total - 3, total - 2, total - 1, total];
  }
  return [1, "...", curr - 1, curr, curr + 1, "...", total];
}

function prevPage() {
  if (currentPage > 1) {
    currentPage -= 1;
  }
}

function nextPage() {
  if (currentPage < totalPages) {
    currentPage += 1;
  }
}

function handleJumpPage(e: KeyboardEvent) {
  if (e.key === "Enter") {
    const pageVal = Number.parseInt(jumpPageInput, 10);
    if (!Number.isNaN(pageVal) && pageVal >= 1 && pageVal <= totalPages) {
      currentPage = pageVal;
    }
    jumpPageInput = "";
  }
}

function changePageSize(size: number) {
  pageSize = size;
  currentPage = 1;
  isPageSizeOpen = false;
}

// Selection Computes & Multi-Select Engine (100% Parity with D:\ldremote)
let isAllPageSelected = $derived(
  paginatedItems.length > 0 && paginatedItems.every((item) => selectedKeys.includes(item[itemKey]))
);

let isSomePageSelected = $derived(
  paginatedItems.some((item) => selectedKeys.includes(item[itemKey])) && !isAllPageSelected
);

let lastClickedIndex = $state<number | null>(null);
let isDragging = $state(false);
let dragAnchorIndex = $state<number | null>(null);

function toggleSelectAll() {
  if (isAllPageSelected) {
    const pageItemKeys = paginatedItems.map((i) => i[itemKey]);
    selectedKeys = selectedKeys.filter((k) => !pageItemKeys.includes(k));
  } else {
    const newKeys = [...selectedKeys];
    paginatedItems.forEach((i) => {
      const k = i[itemKey];
      if (!newKeys.includes(k)) newKeys.push(k);
    });
    selectedKeys = newKeys;
  }
  onUpdateSelectedKeys?.(selectedKeys);
}

function toggleSelect(key: any) {
  if (key === undefined || key === null) return;
  if (selectedKeys.includes(key)) {
    selectedKeys = selectedKeys.filter((k) => k !== key);
  } else {
    selectedKeys = [...selectedKeys, key];
  }
  onUpdateSelectedKeys?.(selectedKeys);
}

function handleRowClick(e: MouseEvent, item: any, index: number) {
  onRowClick?.(item, e);

  // Avoid triggering row click if clicking on an interactive element
  const target = e.target as HTMLElement | null;
  if (
    target &&
    (target.tagName === "BUTTON" ||
      target.tagName === "INPUT" ||
      target.tagName === "SELECT" ||
      target.tagName === "A" ||
      target.closest("button") ||
      target.closest("input") ||
      target.closest("select"))
  ) {
    return;
  }

  const itemKeyVal = item[itemKey];
  const isMeta = e.ctrlKey || e.metaKey;
  const isShift = e.shiftKey;

  if (isShift && lastClickedIndex !== null) {
    e.preventDefault();
    const start = Math.min(lastClickedIndex, index);
    const end = Math.max(lastClickedIndex, index);
    const rangeKeys = paginatedItems.slice(start, end + 1).map((p) => p[itemKey]);

    if (isMeta) {
      selectedKeys = Array.from(new Set([...selectedKeys, ...rangeKeys]));
    } else {
      selectedKeys = rangeKeys;
    }
  } else if (isMeta) {
    toggleSelect(itemKeyVal);
    lastClickedIndex = index;
  } else {
    selectedKeys = [itemKeyVal];
    lastClickedIndex = index;
  }
  onUpdateSelectedKeys?.(selectedKeys);
}

function handleRowMouseDown(e: MouseEvent, item: any, index: number) {
  onRowMouseDown?.(item, e);

  // Only handle left clicks without modifier keys for drag select
  if (e.button !== 0 || e.ctrlKey || e.metaKey || e.shiftKey) return;

  const target = e.target as HTMLElement | null;
  if (target && (target.closest("button") || target.closest("input") || target.closest("select"))) {
    return;
  }

  const itemKeyVal = item[itemKey];
  isDragging = true;
  dragAnchorIndex = index;
  lastClickedIndex = index;

  function handleDragEnd() {
    isDragging = false;
    dragAnchorIndex = null;
    window.removeEventListener("mouseup", handleDragEnd);
  }

  window.addEventListener("mouseup", handleDragEnd);
}

function handleRowMouseEnter(item: any, index: number) {
  onRowMouseEnter?.(item, new MouseEvent("mouseenter"));

  if (isDragging && dragAnchorIndex !== null) {
    const start = Math.min(dragAnchorIndex, index);
    const end = Math.max(dragAnchorIndex, index);
    selectedKeys = paginatedItems.slice(start, end + 1).map((p) => p[itemKey]);
    onUpdateSelectedKeys?.(selectedKeys);
  }
}

// Column Resizing Logic
let resizingColumnKey = $state<string | null>(null);
let startX = 0;
let startWidth = 0;

function handleResizeStart(e: MouseEvent, key: string) {
  e.preventDefault();
  e.stopPropagation();
  resizingColumnKey = key;
  startX = e.clientX;
  startWidth = columnWidths[key] || columns.find((c) => c.key === key)?.width || 120;

  window.addEventListener("mousemove", handleResizeMove);
  window.addEventListener("mouseup", handleResizeEnd);
}

function handleResizeMove(e: MouseEvent) {
  if (!resizingColumnKey) return;
  const diff = e.clientX - startX;
  const newWidth = Math.max(50, startWidth + diff);
  columnWidths[resizingColumnKey] = newWidth;
}

function handleResizeEnd() {
  resizingColumnKey = null;
  window.removeEventListener("mousemove", handleResizeMove);
  window.removeEventListener("mouseup", handleResizeEnd);
}
</script>

<div
  class="relative flex flex-col flex-1 h-full w-full bg-bg-panel border border-border-default rounded-2xl shadow-xs overflow-hidden font-sans text-xs select-none"
>
  <!-- Table Container (Scrollable) -->
  <div class="flex-1 overflow-auto relative custom-scrollbar">
    <table class="w-full border-collapse border-spacing-0 table-fixed text-left select-none">
      <!-- Fixed Table Header -->
      <thead class="sticky top-0 z-30 bg-bg-header/95 backdrop-blur-md shadow-xs border-b border-border-default/40">
        <tr class="h-10 text-[11px] font-extrabold uppercase tracking-wider text-text-muted select-none">
          <!-- Checkbox Column Header -->
          <th
            class="sticky left-0 z-40 bg-bg-header/95 backdrop-blur-md w-10 min-w-10 max-w-10 px-3 text-center border-r border-border-default/20"
          >
            <div class="flex items-center justify-center">
              <input
                type="checkbox"
                checked={isAllPageSelected}
                indeterminate={isSomePageSelected}
                onchange={toggleSelectAll}
                class="{checkboxClassName} cursor-pointer"
                title="Select all on current page"
              />
            </div>
          </th>

          <!-- Dynamic Columns Header -->
          {#each visibleColumns as col}
            {@const width = (columnWidths[col.key] || col.width || 120) + "px"}
            <th
              class="relative px-3 py-2 border-r border-border-default/20 select-none group font-extrabold text-text-muted {col.key ===
              'index'
                ? 'text-center font-mono'
                : ''} {col.key === 'actions'
                ? 'sticky right-0 z-40 bg-bg-header/95 backdrop-blur-md text-right'
                : ''}"
              style="width: {width}; min-width: {width}; max-width: {width}; text-align: {col.align ||
                (col.key === 'actions' ? 'right' : 'left')};"
            >
              {#if col.key === "actions"}
                <!-- Actions Column with Custom View Columns Menu -->
                <div class="flex items-center justify-end gap-1.5 relative pr-1" bind:this={columnSelectorRef}>
                  <span class="truncate">{col.label}</span>
                  <button
                    type="button"
                    title="Custom Columns"
                    onclick={(e) => {
                      e.stopPropagation();
                      showColumnSelector = !showColumnSelector;
                    }}
                    class="p-1 rounded-lg text-text-muted hover:text-text-hover hover:bg-bg-card-hover transition-colors cursor-pointer {showColumnSelector
                      ? 'text-text-hover bg-bg-card-hover'
                      : ''}"
                  >
                    <svg
                      class="w-3.5 h-3.5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                      stroke-width="2.2"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M9 17V7m3 10V7m3 10V7M6 21h12a2 2 0 002-2V5a2 2 0 00-2-2H6a2 2 0 00-2 2v14a2 2 0 002 2z"
                      />
                    </svg>
                  </button>

                  <!-- Column Visibility Selector Dropdown Popover (100% Fidelity with D:\ldremote) -->
                  {#if showColumnSelector}
                    <div
                      role="menu"
                      tabindex="-1"
                      class="absolute right-0 top-full mt-1.5 w-52 bg-bg-panel border border-border-default rounded-xl shadow-2xl z-50 flex flex-col text-left text-text-default normal-case font-normal animate-in fade-in zoom-in-95 duration-100 overflow-hidden"
                      onclick={(e) => e.stopPropagation()}
                      onkeydown={(e) => e.stopPropagation()}
                    >
                      <!-- Top Quick Action Bar -->
                      <div
                        class="flex items-center justify-between gap-1 p-2 border-b border-border-default/60 text-[10px] font-bold text-text-muted bg-bg-card/40 select-none"
                      >
                        <button
                          type="button"
                          onclick={resetColumns}
                          class="hover:text-[#00b578] cursor-pointer transition-colors px-1 py-0.5 bg-transparent border-none"
                        >
                          Reset
                        </button>
                        <span class="text-border-default">|</span>
                        <button
                          type="button"
                          onclick={hideEmptyColumns}
                          class="hover:text-[#00b578] cursor-pointer transition-colors px-1 py-0.5 bg-transparent border-none"
                        >
                          Hide Empty
                        </button>
                        <span class="text-border-default">|</span>
                        <button
                          type="button"
                          onclick={showAllColumns}
                          class="hover:text-[#00b578] cursor-pointer transition-colors px-1 py-0.5 bg-transparent border-none"
                        >
                          Show All
                        </button>
                      </div>

                      <!-- Scrollable Columns Checklist with Drag Handle -->
                      <div
                        class="p-1.5 max-h-64 overflow-y-auto space-y-0.5 custom-scrollbar"
                      >
                        {#each columns as c, colIndex}
                          {#if c.key !== "actions"}
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <div
                              draggable={c.canHide}
                              ondragstart={() => handleDragStart(colIndex)}
                              ondragover={(e) => handleDragOver(e, colIndex)}
                              ondragend={handleDragEnd}
                              class="flex items-center justify-between px-2 py-1.5 rounded-lg select-none text-[11px] font-semibold transition-all duration-100 hover:bg-bg-card-hover {draggedColIndex ===
                              colIndex
                                ? 'opacity-30 bg-[#00b578]/10 border-dashed border-[#00b578]'
                                : ''}"
                            >
                              <label
                                class="flex items-center gap-2 flex-1 min-w-0 {c.canHide
                                  ? 'cursor-pointer'
                                  : 'opacity-40 cursor-not-allowed'}"
                              >
                                <input
                                  type="checkbox"
                                  checked={c.visible}
                                  disabled={!c.canHide}
                                  onchange={() => toggleColumnVisibility(c.key)}
                                  class="{checkboxClassName} cursor-pointer shrink-0"
                                />
                                <span class="truncate">{c.label}</span>
                              </label>

                              {#if c.canHide}
                                <div
                                  class="cursor-grab active:cursor-grabbing text-text-muted hover:text-[#00b578] p-0.5 shrink-0 transition-colors"
                                  title="Drag to reorder"
                                >
                                  <svg
                                    class="w-3.5 h-3.5"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                  >
                                    <path
                                      d="M7 6a1 1 0 100-2 1 1 0 000 2zM7 9a1 1 0 100-2 1 1 0 000 2zM7 12a1 1 0 100-2 1 1 0 000 2zM7 15a1 1 0 100-2 1 1 0 000 2zM13 6a1 1 0 100-2 1 1 0 000 2zM13 9a1 1 0 100-2 1 1 0 000 2zM13 12a1 1 0 100-2 1 1 0 000 2zM13 15a1 1 0 100-2 1 1 0 000 2z"
                                    />
                                  </svg>
                                </div>
                              {/if}
                            </div>
                          {/if}
                        {/each}
                      </div>
                    </div>
                  {/if}
                </div>
              {:else if renderHeader}
                {@render renderHeader(col.key, col)}
              {:else}
                <span class="truncate block">{col.label}</span>
              {/if}

              <!-- Column Resizer Handle -->
              {#if col.key !== "actions"}
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <div
                  role="separator"
                  aria-orientation="vertical"
                  tabindex="-1"
                  class="absolute top-0 right-0 w-1.5 h-full cursor-col-resize hover:bg-[#00b578] transition-colors z-20"
                  onmousedown={(e) => handleResizeStart(e, col.key)}
                ></div>
              {/if}
            </th>
          {/each}
        </tr>
      </thead>

      <!-- Table Body Rows -->
      <tbody class="divide-y divide-border-default/20 text-xs font-normal">
        {#if paginatedItems.length === 0}
          <tr>
            <td
              colspan={visibleColumns.length + 1}
              class="py-16 text-center text-text-muted"
            >
              {#if renderEmptyState}
                {@render renderEmptyState()}
              {:else}
                <div class="flex flex-col items-center justify-center gap-2">
                  <span class="text-text-muted/40">
                    <Icon name="cube" size={32} />
                  </span>
                  <span class="font-medium">No items found</span>
                </div>
              {/if}
            </td>
          </tr>
        {:else}
          {#each paginatedItems as item, index (item[itemKey] ?? index)}
            {@const isSelected = selectedKeys.includes(item[itemKey])}
            <tr
              class="h-10 transition-colors duration-150 group cursor-default {isSelected
                ? 'bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-panel))] hover:bg-[color-mix(in_srgb,#00b578_16%,var(--color-bg-panel))]'
                : 'hover:bg-bg-card-hover/40'}"
              onclick={(e) => handleRowClick(e, item, index)}
              onmousedown={(e) => handleRowMouseDown(e, item, index)}
              onmouseenter={() => handleRowMouseEnter(item, index)}
              oncontextmenu={(e) => onRowContextMenu?.(item, e)}
            >
              <!-- Checkbox Cell with Selection Indicator -->
              <td
                class="sticky left-0 z-20 w-10 min-w-10 max-w-10 px-3 text-center transition-colors duration-150 relative border-r {isSelected
                  ? 'bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-panel))] group-hover:bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-card-hover))] border-b border-[#00b578]/20 border-r-border-default/20'
                  : 'bg-bg-panel group-hover:bg-bg-card-hover border-b border-border-default/20 border-r-border-default/20'}"
                onclick={(e) => e.stopPropagation()}
                onmousedown={(e) => e.stopPropagation()}
              >
                {#if isSelected}
                  <div
                    class="absolute left-0 top-0 bottom-0 w-[3.5px] bg-[#00b578] pointer-events-none z-30"
                  ></div>
                {/if}
                <div class="flex items-center justify-center py-2.5 w-full h-full">
                  <input
                    type="checkbox"
                    checked={isSelected}
                    onchange={() => toggleSelect(item[itemKey])}
                    class="{checkboxClassName} cursor-pointer"
                    onclick={(e) => e.stopPropagation()}
                    onmousedown={(e) => e.stopPropagation()}
                  />
                </div>
              </td>

              <!-- Dynamic Column Cells -->
              {#each visibleColumns as col}
                {@const width =
                  (columnWidths[col.key] || col.width || 120) + "px"}
                <td
                  class="py-2.5 px-3 border-r transition-colors duration-150 {isSelected
                    ? 'border-b border-[#00b578]/20 border-r-border-default/20'
                    : 'border-b border-border-default/20 border-r-border-default/20'} {col.key ===
                  'index'
                    ? 'text-center font-mono'
                    : ''} {col.key === 'name'
                    ? 'font-bold text-text-hover'
                    : ''} {col.key === 'actions'
                    ? `sticky right-0 z-20 ${
                        isSelected
                          ? 'bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-panel))] group-hover:bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-card-hover))]'
                          : 'bg-bg-panel group-hover:bg-bg-card-hover'
                      }`
                    : ''}"
                  style="width: {width}; min-width: {width}; max-width: {width}; text-align: {col.align ||
                    (col.key === 'actions' ? 'right' : 'left')};"
                >
                  {#if renderCell}
                    {@render renderCell(col.key, item, index, col)}
                  {:else}
                    {item[col.key] ?? "-"}
                  {/if}
                </td>
              {/each}
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>

  <!-- Pagination Bottom Bar (100% Parity with D:\ldremote) -->
  {#if paginate}
    <div
      class="h-11 border-t border-border-default flex items-center justify-between px-4 text-[11px] select-none shrink-0 bg-bg-panel/95 backdrop-blur-md"
    >
      <!-- Left: Showing Counter with Animated Emerald Dot -->
      <div class="flex items-center gap-2 text-text-muted font-medium">
        <span class="flex h-1.5 w-1.5 relative">
          <span
            class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#00b578] opacity-75"
          ></span>
          <span
            class="relative inline-flex rounded-full h-1.5 w-1.5 bg-[#00b578]"
          ></span>
        </span>
        <span>
          Showing <span class="font-bold text-text-default">{showingStart}</span>
          -
          <span class="font-bold text-text-default">{showingEnd}</span>
          of
          <span class="font-bold text-[#00b578]">{totalItems}</span>
          {itemLabel}
        </span>
      </div>

      <!-- Center: Custom slot -->
      <div class="flex items-center gap-2">
        {#if renderFooterCenter}
          {@render renderFooterCenter()}
        {/if}
      </div>

      <!-- Right: Page Size Popover, Jump Box, & Compact Page Navigation -->
      <div class="flex items-center gap-3">
        <!-- Upward Page Size Selector Dropdown -->
        <div class="relative">
          <button
            type="button"
            onclick={(e) => {
              e.stopPropagation();
              isPageSizeOpen = !isPageSizeOpen;
            }}
            class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg border border-border-default bg-bg-card hover:bg-bg-card-hover text-text-default text-[11px] font-bold transition-all cursor-pointer shadow-xs active:scale-95"
          >
            <span>{pageSize} / page</span>
            <Icon
              name="chevronDown"
              size={12}
              class="text-text-muted transition-transform {isPageSizeOpen
                ? 'rotate-180'
                : ''}"
            />
          </button>

          <!-- Upward Popover Menu -->
          {#if isPageSizeOpen}
            <div
              class="absolute bottom-full mb-1.5 right-0 w-28 bg-bg-panel border border-border-default rounded-xl shadow-xl z-50 p-1 flex flex-col gap-0.5 text-left font-bold animate-in fade-in slide-in-from-bottom-1 duration-100"
            >
              {#each pageSizes as size}
                <button
                  type="button"
                  onclick={() => changePageSize(size)}
                  class="flex items-center justify-between px-2.5 py-1.5 rounded-lg text-[11px] transition-colors cursor-pointer {pageSize ===
                  size
                    ? 'bg-[#00b578]/15 text-[#00b578]'
                    : 'text-text-muted hover:text-text-default hover:bg-bg-card-hover'}"
                >
                  <span>{size} / page</span>
                  {#if pageSize === size}
                    <Icon name="check" size={12} class="text-[#00b578]" />
                  {/if}
                </button>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Quick Jump Input Box -->
        <div
          class="flex items-center gap-1.5 px-2 py-0.5 rounded-lg border border-border-default bg-bg-card/40 shadow-xs"
        >
          <span class="text-[10.5px] font-medium text-text-muted">Go to:</span>
          <div class="flex items-center gap-0.5">
            <input
              type="text"
              bind:value={jumpPageInput}
              onkeydown={handleJumpPage}
              placeholder={currentPage.toString()}
              class="w-7 h-5 text-center text-[11px] font-mono font-bold bg-transparent text-text-default border-none outline-none p-0"
            />
            <div class="flex flex-col -space-y-0.5">
              <button
                type="button"
                onclick={() => {
                  if (currentPage < totalPages) currentPage += 1;
                }}
                class="text-text-muted hover:text-text-hover leading-none px-0.5 cursor-pointer border-none bg-transparent"
              >
                ▲
              </button>
              <button
                type="button"
                onclick={() => {
                  if (currentPage > 1) currentPage -= 1;
                }}
                class="text-text-muted hover:text-text-hover leading-none px-0.5 cursor-pointer border-none bg-transparent"
              >
                ▼
              </button>
            </div>
          </div>
        </div>

        <!-- Page Numbers Bar (< 1 2 3 ... N >) -->
        <div class="flex items-center gap-1">
          <!-- Previous Button -->
          <button
            type="button"
            onclick={prevPage}
            disabled={currentPage <= 1}
            class="w-6.5 h-6.5 rounded-lg flex items-center justify-center border border-border-default bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer shadow-xs active:scale-95"
            title="Previous Page"
          >
            <Icon name="chevronLeft" size={12} />
          </button>

          <!-- Numbered Page Buttons -->
          {#each getPageNumbers(currentPage, totalPages) as pageNum}
            {#if pageNum === "..."}
              <span class="w-5 text-center text-text-muted font-bold text-xs select-none">
                ...
              </span>
            {:else}
              <button
                type="button"
                onclick={() => (currentPage = pageNum as number)}
                class="w-6.5 h-6.5 rounded-lg text-[11px] font-mono font-bold flex items-center justify-center transition-all cursor-pointer shadow-xs {currentPage ===
                pageNum
                  ? 'bg-[#00b578] text-white shadow-md shadow-[#00b578]/20 border border-[#00b578]'
                  : 'border border-border-default bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover active:scale-95'}"
              >
                {pageNum}
              </button>
            {/if}
          {/each}

          <!-- Next Button -->
          <button
            type="button"
            onclick={nextPage}
            disabled={currentPage >= totalPages}
            class="w-6.5 h-6.5 rounded-lg flex items-center justify-center border border-border-default bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer shadow-xs active:scale-95"
            title="Next Page"
          >
            <Icon name="chevronRight" size={12} />
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
