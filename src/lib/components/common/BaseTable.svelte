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
  paginate = false,
  pageSize = $bindable(50),
  pageSizes = [50, 100, 200, 300],
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

// Column Widths Map
let columnWidths = $state<Record<string, number>>({});

// Pagination State
let currentPage = $state(1);

onMount(() => {
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

// Visible Columns
let visibleColumns = $derived(columns.filter((c) => c.visible));

// Pagination Computes
let totalItems = $derived(items.length);
let totalPages = $derived(Math.max(1, Math.ceil(totalItems / pageSize)));
let startIndex = $derived((currentPage - 1) * pageSize);
let endIndex = $derived(Math.min(startIndex + pageSize, totalItems));
let paginatedItems = $derived(paginate ? items.slice(startIndex, endIndex) : items);

// Selection Computes
let isAllPageSelected = $derived(
  paginatedItems.length > 0 && paginatedItems.every((item) => selectedKeys.includes(item[itemKey]))
);

function toggleSelectAllPage() {
  const pageKeys = paginatedItems.map((item) => item[itemKey]);
  if (isAllPageSelected) {
    selectedKeys = selectedKeys.filter((k) => !pageKeys.includes(k));
  } else {
    selectedKeys = Array.from(new Set([...selectedKeys, ...pageKeys]));
  }
  onUpdateSelectedKeys?.(selectedKeys);
}

function toggleSelect(key: any) {
  if (!key && key !== 0) return;
  if (selectedKeys.includes(key)) {
    selectedKeys = selectedKeys.filter((k) => k !== key);
  } else {
    selectedKeys = [...selectedKeys, key];
  }
  onUpdateSelectedKeys?.(selectedKeys);
}

// -------------------------------------------------------------
// Drag-to-select and Click Select Logic
// -------------------------------------------------------------
let isDragging = $state(false);
let dragAnchorIndex = $state<number | null>(null);
let lastClickedIndex = $state<number | null>(null);

function handleRowMouseDownInternal(event: MouseEvent, item: any, index: number) {
  onRowMouseDown?.(item, event);

  if (event.button !== 0 || event.ctrlKey || event.metaKey || event.shiftKey) {
    return;
  }

  const itemKeyVal = item[itemKey];
  isDragging = true;
  dragAnchorIndex = index;
  lastClickedIndex = index;

  selectedKeys = [itemKeyVal];
  onUpdateSelectedKeys?.([itemKeyVal]);

  function handleDragEnd() {
    setTimeout(() => {
      isDragging = false;
      dragAnchorIndex = null;
    }, 50);
    window.removeEventListener("mouseup", handleDragEnd);
  }

  window.addEventListener("mouseup", handleDragEnd);
  event.preventDefault();
}

function handleRowMouseEnterInternal(event: MouseEvent, item: any, index: number) {
  onRowMouseEnter?.(item, event);

  if (!isDragging || dragAnchorIndex === null) return;

  const start = Math.min(dragAnchorIndex, index);
  const end = Math.max(dragAnchorIndex, index);
  const rangeKeys = paginatedItems.slice(start, end + 1).map((p) => p[itemKey]);

  selectedKeys = rangeKeys;
  onUpdateSelectedKeys?.(rangeKeys);
}

function handleRowClickInternal(event: MouseEvent, item: any, index: number) {
  onRowClick?.(item, event);

  if (isDragging) return;

  const itemKeyVal = item[itemKey];
  const isMeta = event.ctrlKey || event.metaKey;
  const isShift = event.shiftKey;

  if (isShift && lastClickedIndex !== null) {
    event.preventDefault();
    const start = Math.min(lastClickedIndex, index);
    const end = Math.max(lastClickedIndex, index);
    const rangeKeys = paginatedItems.slice(start, end + 1).map((p) => p[itemKey]);

    if (isMeta) {
      selectedKeys = Array.from(new Set([...selectedKeys, ...rangeKeys]));
    } else {
      selectedKeys = rangeKeys;
    }
    onUpdateSelectedKeys?.(selectedKeys);
  } else if (isMeta) {
    toggleSelect(itemKeyVal);
  } else {
    selectedKeys = [itemKeyVal];
    onUpdateSelectedKeys?.([itemKeyVal]);
  }

  lastClickedIndex = index;
}

// -------------------------------------------------------------
// Column Resizing Logic
// -------------------------------------------------------------
let isResizing = false;
let startX = 0;
let startWidth = 0;
let activeResizeCol = "";

function startResize(event: MouseEvent, colKey: string) {
  isResizing = true;
  startX = event.clientX;
  startWidth = columnWidths[colKey] || 100;
  activeResizeCol = colKey;

  document.body.style.userSelect = "none";
  window.addEventListener("mousemove", handleResize);
  window.addEventListener("mouseup", stopResize, { once: true });
  event.preventDefault();
}

function handleResize(event: MouseEvent) {
  if (!isResizing || !activeResizeCol) return;
  const dx = event.clientX - startX;
  columnWidths[activeResizeCol] = Math.max(50, startWidth + dx);
}

function stopResize() {
  isResizing = false;
  activeResizeCol = "";
  document.body.style.userSelect = "";
  window.removeEventListener("mousemove", handleResize);
}

// Column Visibility Controls
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

<div class="flex-1 flex flex-col min-h-0 overflow-hidden border border-border-default rounded-2xl bg-bg-panel shadow-xs font-sans">
  <!-- Scrollable Table Container -->
  <div class="overflow-x-auto overflow-y-auto flex-1 min-h-[280px]">
    <table
      class="w-full border-collapse text-left text-[11px] text-text-default table-fixed"
    >
      <!-- Table Header (matching D:\ldremote) -->
      <thead>
        <tr
          class="bg-bg-card border-b border-border-default text-text-muted font-bold uppercase tracking-wider select-none sticky top-0 z-40 shadow-xs"
        >
          <!-- Checkbox Header (Sticky Left) -->
          <th
            class="p-0 w-[40px] min-w-[40px] max-w-[40px] sticky left-0 z-50 bg-bg-card border-r border-border-default/20"
          >
            <div class="flex items-center justify-center py-2.5 w-full h-full">
              <input
                type="checkbox"
                checked={isAllPageSelected}
                onchange={toggleSelectAllPage}
                disabled={!isConnected}
                class="{checkboxClassName} cursor-pointer"
              />
            </div>
          </th>

          <!-- Dynamic Column Headers -->
          {#each visibleColumns as col (col.key)}
            {@const width = (columnWidths[col.key] || col.width || 120) + "px"}
            <th
              class="py-2.5 px-3 border-r border-border-default/20 relative select-none bg-bg-card text-[10.5px] font-extrabold {col.key ===
              'index'
                ? 'text-center font-mono'
                : ''} {col.key === 'actions'
                ? 'sticky right-0 z-50'
                : ''}"
              style="width: {width}; min-width: {width}; max-width: {width};"
            >
              {#if col.key === "actions"}
                <div
                  bind:this={columnSelectorRef}
                  class="flex items-center justify-between gap-1.5 w-full h-full relative"
                >
                  <span class="flex-1 text-center">{col.label}</span>
                  <button
                    type="button"
                    onclick={(e) => {
                      e.stopPropagation();
                      showColumnSelector = !showColumnSelector;
                    }}
                    class="p-1 hover:bg-bg-card-hover text-text-muted hover:text-text-hover rounded cursor-pointer transition-colors flex items-center justify-center shrink-0 {showColumnSelector
                      ? 'text-text-hover bg-bg-card-hover'
                      : ''}"
                    title="Custom Columns"
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

                  <!-- Custom Columns Dropdown Popover -->
                  {#if showColumnSelector}
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      class="absolute right-0 top-full mt-2 w-56 rounded-xl border border-border-default bg-bg-card shadow-2xl z-50 text-left normal-case tracking-normal p-2.5 font-sans"
                      onclick={(e) => e.stopPropagation()}
                    >
                      <div
                        class="flex items-center justify-between gap-1 pb-2 mb-2 border-b border-border-default text-[10px] text-text-muted font-bold"
                      >
                        <button
                          type="button"
                          onclick={resetColumns}
                          class="hover:text-[#00b578] cursor-pointer transition-colors px-1 py-0.5"
                        >
                          Reset
                        </button>
                        <span class="text-border-default">|</span>
                        <button
                          type="button"
                          onclick={hideEmptyColumns}
                          class="hover:text-[#00b578] cursor-pointer transition-colors px-1 py-0.5"
                        >
                          Hide Empty
                        </button>
                        <span class="text-border-default">|</span>
                        <button
                          type="button"
                          onclick={showAllColumns}
                          class="hover:text-[#00b578] cursor-pointer transition-colors px-1 py-0.5"
                        >
                          Show All
                        </button>
                      </div>

                      <div class="py-1 max-h-56 overflow-y-auto space-y-1 pr-1">
                        {#each columns as c}
                          {#if c.key !== "actions"}
                            <label
                              class="flex items-center justify-between px-2 py-1 rounded-lg hover:bg-bg-card-hover text-text-default text-[11px] font-medium cursor-pointer {c.canHide
                                ? ''
                                : 'opacity-40 cursor-not-allowed'}"
                            >
                              <span>{c.label}</span>
                              <input
                                type="checkbox"
                                bind:checked={c.visible}
                                disabled={!c.canHide}
                                class="custom-checkbox"
                              />
                            </label>
                          {/if}
                        {/each}
                      </div>
                    </div>
                  {/if}
                </div>
              {:else if renderHeader}
                {@render renderHeader(col.key, col)}
              {:else}
                <span>{col.label}</span>
              {/if}

              <!-- Column Resizer Handle -->
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <div
                role="separator"
                tabindex="-1"
                aria-orientation="vertical"
                class="absolute top-0 right-0 h-full w-1.5 cursor-col-resize hover:bg-[#00b578]/40 active:bg-[#00b578] transition-colors z-20"
                onmousedown={(e) => {
                  e.stopPropagation();
                  e.preventDefault();
                  startResize(e, col.key);
                }}
              ></div>
            </th>
          {/each}
        </tr>
      </thead>

      <!-- Table Body (matching D:\ldremote) -->
      <tbody>
        {#if paginatedItems.length === 0}
          {#if renderEmptyState}
            {@render renderEmptyState()}
          {:else}
            <tr class="text-center">
              <td
                colspan={visibleColumns.length + 1}
                class="py-16 text-text-muted"
              >
                <div
                  class="flex flex-col items-center justify-center gap-2 max-w-sm mx-auto font-sans"
                >
                  <Icon name="cube" size={28} class="opacity-40" />
                  <p class="font-bold text-xs text-text-default">
                    No items found.
                  </p>
                </div>
              </td>
            </tr>
          {/if}
        {:else}
          {#each paginatedItems as item, index (item[itemKey] ?? index)}
            {@const rowKey = item[itemKey] ?? index}
            {@const isSelected = selectedKeys.includes(item[itemKey])}
            <tr
              id="table-row-{rowKey}"
              class="group h-10 border-0 transition-all duration-150 ease-out {isSelected
                ? 'bg-[#00b578]/12 text-text-hover'
                : 'hover:bg-bg-card-hover text-text-default'}"
              onclick={(e) => handleRowClickInternal(e, item, index)}
              onmousedown={(e) => handleRowMouseDownInternal(e, item, index)}
              onmouseenter={(e) => handleRowMouseEnterInternal(e, item, index)}
              oncontextmenu={(e) => {
                e.preventDefault();
                onRowContextMenu?.(item, e);
              }}
            >
              <!-- Checkbox Cell (Sticky Left) -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <td
                class="p-0 w-[40px] min-w-[40px] max-w-[40px] sticky left-0 z-20 transition-colors duration-150 relative border-r {isSelected
                  ? 'bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-panel))] group-hover:bg-[color-mix(in_srgb,#00b578_12%,var(--color-bg-card-hover))] border-b border-[#00b578]/20 border-r-border-default/20'
                  : 'bg-bg-panel group-hover:bg-bg-card-hover border-b border-border-default/20 border-r-border-default/20'}"
                onclick={(e) => e.stopPropagation()}
                onmousedown={(e) => e.stopPropagation()}
              >
                {#if isSelected}
                  <div class="absolute left-0 top-0 bottom-0 w-[3.5px] bg-[#00b578] pointer-events-none z-30"></div>
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
                  style="width: {width}; min-width: {width}; max-width: {width};"
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

  <!-- Pagination Footer (matching D:\ldremote) -->
  {#if paginate && isConnected && totalItems > 0}
    <footer
      class="shrink-0 flex flex-col lg:flex-row items-center justify-between border-t border-border-default px-4 py-2.5 select-none z-10 relative bg-bg-panel gap-3 font-sans text-xs"
    >
      <!-- Left: Counter Pill -->
      <div
        class="flex items-center select-none w-full lg:w-auto justify-center lg:justify-start"
      >
        <div
          class="h-8 px-3 rounded-xl border border-border-default bg-bg-card text-xs text-text-muted font-medium flex items-center gap-2 shadow-xs"
        >
          <span class="relative flex h-2 w-2 mr-0.5">
            <span
              class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#00b578]/30 opacity-75"
            ></span>
            <span
              class="relative inline-flex rounded-full h-2 w-2 bg-[#00b578]"
            ></span>
          </span>
          <span>Showing</span>
          <span class="font-bold text-text-hover font-mono">{startIndex + 1}</span>
          <span>to</span>
          <span class="font-bold text-text-hover font-mono">{endIndex}</span>
          <span>of</span>
          <span class="font-bold text-text-hover font-mono">{totalItems}</span>
          <span>items</span>
        </div>
      </div>

      <!-- Center: Optional Filters Snippet -->
      {#if renderFooterCenter}
        {@render renderFooterCenter()}
      {/if}

      <!-- Right: Page Size & Navigation Controls -->
      <div class="flex items-center gap-3">
        <select
          bind:value={pageSize}
          class="h-8 px-2.5 rounded-xl border border-border-default bg-bg-card text-xs text-text-hover font-mono focus:outline-none focus:border-[#00b578] cursor-pointer"
        >
          {#each pageSizes as size}
            <option value={size}>{size} / page</option>
          {/each}
        </select>

        <div class="flex items-center gap-1 font-mono text-xs">
          <button
            type="button"
            disabled={currentPage <= 1}
            onclick={() => (currentPage = Math.max(1, currentPage - 1))}
            class="h-8 px-3 rounded-xl border border-border-default bg-bg-card text-text-default hover:text-text-hover hover:bg-bg-card-hover disabled:opacity-40 cursor-pointer"
          >
            Prev
          </button>
          <span class="px-2 text-text-hover font-semibold">
            {currentPage} / {totalPages}
          </span>
          <button
            type="button"
            disabled={currentPage >= totalPages}
            onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
            class="h-8 px-3 rounded-xl border border-border-default bg-bg-card text-text-default hover:text-text-hover hover:bg-bg-card-hover disabled:opacity-40 cursor-pointer"
          >
            Next
          </button>
        </div>
      </div>
    </footer>
  {/if}
</div>
