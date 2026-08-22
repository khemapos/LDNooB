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
  paginate?: boolean;
  pageSize?: number;
  pageSizes?: number[];
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
  paginate = true,
  pageSize = $bindable(50),
  pageSizes = [50, 100, 200, 300],
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

// Pagination State
let currentPage = $state(1);

onMount(() => {
  try {
    const savedWidths = localStorage.getItem("ldnoob_table_column_widths");
    if (savedWidths) {
      columnWidths = JSON.parse(savedWidths);
    }
  } catch {
    // fallback
  }

  columns.forEach((col) => {
    if (!columnWidths[col.key]) {
      columnWidths[col.key] = col.width || 120;
    }
  });

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

// Pagination Computes
let totalItems = $derived(items.length);
let totalPages = $derived(Math.max(1, Math.ceil(totalItems / pageSize)));
let startIndex = $derived((currentPage - 1) * pageSize);
let endIndex = $derived(Math.min(startIndex + pageSize, totalItems));
let paginatedItems = $derived(paginate ? items.slice(startIndex, endIndex) : items);

// Selection Computes
let allSelected = $derived(
  paginatedItems.length > 0 && paginatedItems.every((i) => selectedKeys.includes(i[itemKey]))
);
let someSelected = $derived(
  paginatedItems.some((i) => selectedKeys.includes(i[itemKey])) && !allSelected
);

function toggleSelectAll() {
  const pageKeys = paginatedItems.map((i) => i[itemKey]);
  if (allSelected) {
    selectedKeys = selectedKeys.filter((k) => !pageKeys.includes(k));
  } else {
    selectedKeys = Array.from(new Set([...selectedKeys, ...pageKeys]));
  }
  onUpdateSelectedKeys?.(selectedKeys);
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
    const range = paginatedItems.slice(start, end + 1).map((i) => i[itemKey]);

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
  class="w-full h-full flex flex-col overflow-hidden rounded-2xl border border-[#25272b] dark:border-[#25272b] bg-[#141517] dark:bg-[#141517] shadow-xs select-none"
>
  <!-- Table Scroll Area -->
  <div class="flex-1 w-full overflow-auto relative">
    <table class="w-full text-left border-collapse text-xs">
      <!-- Table Header -->
      <thead
        class="sticky top-0 z-30 bg-[#18191c] border-b border-[#25272b] text-[#8c8c8c] font-sans font-bold text-[11px] uppercase tracking-wider"
      >
        <tr>
          <!-- Checkbox Column (Sticky Left) -->
          <th
            class="w-10 min-w-[40px] max-w-[40px] p-0 sticky left-0 z-40 bg-[#18191c] border-r border-[#25272b]/60"
          >
            <div class="flex items-center justify-center py-2.5 w-full h-full">
              <input
                type="checkbox"
                checked={allSelected}
                indeterminate={someSelected}
                onchange={toggleSelectAll}
                class="custom-checkbox cursor-pointer"
              />
            </div>
          </th>

          <!-- Dynamic Columns -->
          {#each visibleColumns as col (col.key)}
            {@const colWidth = (columnWidths[col.key] || col.width || 120) + "px"}
            <th
              class="py-2.5 px-3 border-r border-[#25272b]/60 relative group {col.key ===
              'index'
                ? 'text-center'
                : col.align === 'right'
                  ? 'text-right'
                  : 'text-left'} {col.key === 'actions'
                ? 'sticky right-0 z-40 bg-[#18191c] shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.35)]'
                : ''}"
              style="width: {colWidth}; min-width: {colWidth}; max-width: {colWidth};"
            >
              <div class="flex items-center justify-between gap-1 overflow-hidden">
                <span class="truncate">{col.label}</span>

                <!-- Custom Column Filter Settings Trigger on Actions header -->
                {#if col.key === "actions"}
                  <div class="relative" bind:this={columnFilterRef}>
                    <button
                      type="button"
                      title="Customize Columns"
                      onclick={(e) => {
                        e.stopPropagation();
                        showColumnFilter = !showColumnFilter;
                      }}
                      class="p-1 rounded-md text-[#8c8c8c] hover:text-white hover:bg-[#25272b] transition-colors cursor-pointer {showColumnFilter
                        ? 'bg-[#25272b] text-[#00b578]'
                        : ''}"
                    >
                      <Icon name="filter" size={13} />
                    </button>

                    <!-- Column Chooser Dropdown Popover -->
                    {#if showColumnFilter}
                      <div
                        class="absolute right-0 top-full mt-1.5 w-56 p-2.5 bg-[#18191c] border border-[#25272b] rounded-xl shadow-2xl z-50 animate-in fade-in zoom-in-95 duration-150 font-sans text-xs"
                      >
                        <!-- Action Row -->
                        <div
                          class="flex items-center justify-between pb-2 mb-2 border-b border-[#25272b] text-[10px] font-bold text-[#8c8c8c]"
                        >
                          <button
                            type="button"
                            onclick={resetColumns}
                            class="hover:text-[#00b578] cursor-pointer"
                          >
                            Reset
                          </button>
                          <span class="text-[#25272b]">|</span>
                          <button
                            type="button"
                            onclick={hideEmptyColumns}
                            class="hover:text-[#00b578] cursor-pointer"
                          >
                            Hide Empty
                          </button>
                          <span class="text-[#25272b]">|</span>
                          <button
                            type="button"
                            onclick={showAllColumns}
                            class="hover:text-[#00b578] cursor-pointer"
                          >
                            Show All
                          </button>
                        </div>

                        <!-- Checkboxes List -->
                        <div class="space-y-1 max-h-52 overflow-y-auto pr-1">
                          {#each columns as c}
                            <label
                              class="flex items-center justify-between px-2 py-1 rounded-lg hover:bg-[#1f2125] text-[#d9d9d9] cursor-pointer {c.canHide
                                ? ''
                                : 'opacity-50 cursor-not-allowed'}"
                            >
                              <span class="text-[11px] font-medium">{c.label}</span>
                              <input
                                type="checkbox"
                                bind:checked={c.visible}
                                disabled={!c.canHide}
                                class="custom-checkbox"
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
                class="absolute top-0 right-0 h-full w-1.5 cursor-col-resize hover:bg-[#00b578]/40 active:bg-[#00b578] transition-colors z-20"
                onmousedown={(e) => startResize(e, col.key)}
              ></div>
            </th>
          {/each}
        </tr>
      </thead>

      <!-- Table Body -->
      <tbody class="divide-y divide-[#25272b]/40">
        {#if paginatedItems.length === 0}
          <tr>
            <td
              colspan={visibleColumns.length + 1}
              class="py-16 text-center text-[#8c8c8c] font-sans"
            >
              <div class="flex flex-col items-center justify-center gap-2">
                <Icon name="cube" size={28} class="opacity-40" />
                <p class="text-xs font-bold text-[#d9d9d9]">No emulator instances found.</p>
              </div>
            </td>
          </tr>
        {:else}
          {#each paginatedItems as item, index (item[itemKey] ?? index)}
            {@const key = item[itemKey] ?? index}
            {@const isSelected = selectedKeys.includes(key)}
            <tr
              onclick={(e) => handleRowClickInternal(item, index, e)}
              oncontextmenu={(e) => {
                e.preventDefault();
                onRowContextMenu?.(item, e);
              }}
              class="group transition-colors duration-100 cursor-pointer {isSelected
                ? 'bg-[#00b578]/10 text-white shadow-[inset_3px_0_0_0_#00b578]'
                : 'hover:bg-[#1f2125] text-[#d9d9d9]'}"
            >
              <!-- Checkbox Cell (Sticky Left) -->
              <td
                class="w-10 min-w-[40px] max-w-[40px] p-0 sticky left-0 z-10 border-r border-[#25272b]/40 transition-colors {isSelected
                  ? 'bg-[#121c18]'
                  : 'bg-[#141517] group-hover:bg-[#1f2125]'}"
                onclick={(e) => e.stopPropagation()}
              >
                <div class="flex items-center justify-center py-2.5 w-full h-full">
                  <input
                    type="checkbox"
                    checked={isSelected}
                    onchange={() => toggleSelect(key)}
                    class="custom-checkbox cursor-pointer"
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
                    class="py-2.5 px-3 border-r border-[#25272b]/30 font-mono truncate {col.key ===
                    'index'
                      ? 'text-center font-bold text-[#8c8c8c]'
                      : ''} {col.key === 'actions'
                      ? `sticky right-0 z-10 shadow-[-4px_0_8px_-4px_rgba(0,0,0,0.3)] ${
                          isSelected
                            ? 'bg-[#121c18]'
                            : 'bg-[#141517] group-hover:bg-[#1f2125]'
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

  <!-- Pagination Footer -->
  {#if paginate && totalItems > 0}
    <footer
      class="shrink-0 flex items-center justify-between border-t border-[#25272b] px-4 py-2.5 bg-[#141517] text-xs text-[#8c8c8c] font-sans"
    >
      <!-- Left: Showing Items Status Indicator -->
      <div class="flex items-center gap-2">
        <span class="w-2 h-2 rounded-full bg-[#00b578] animate-pulse"></span>
        <span>
          Showing <strong class="text-white font-mono">{startIndex + 1}</strong> to{" "}
          <strong class="text-white font-mono">{endIndex}</strong> of{" "}
          <strong class="text-white font-mono">{totalItems}</strong> items
        </span>
      </div>

      <!-- Right: Page Controls & Page Size -->
      <div class="flex items-center gap-3">
        <!-- Page Size Selector -->
        <div class="flex items-center gap-1.5">
          <span>Rows:</span>
          <select
            bind:value={pageSize}
            class="px-2 py-1 text-xs rounded-lg bg-[#18191c] border border-[#25272b] text-white font-mono focus:outline-none focus:border-[#00b578]"
          >
            {#each pageSizes as size}
              <option value={size}>{size} / page</option>
            {/each}
          </select>
        </div>

        <!-- Pagination Navigation Buttons -->
        <div class="flex items-center gap-1">
          <button
            type="button"
            disabled={currentPage <= 1}
            onclick={() => (currentPage = 1)}
            class="px-2 py-1 rounded-md bg-[#18191c] border border-[#25272b] text-[#d9d9d9] hover:text-white hover:bg-[#1f2125] disabled:opacity-40 cursor-pointer font-mono"
          >
            &laquo;
          </button>
          <button
            type="button"
            disabled={currentPage <= 1}
            onclick={() => (currentPage = Math.max(1, currentPage - 1))}
            class="px-2.5 py-1 rounded-md bg-[#18191c] border border-[#25272b] text-[#d9d9d9] hover:text-white hover:bg-[#1f2125] disabled:opacity-40 cursor-pointer font-mono"
          >
            &lsaquo;
          </button>
          <span class="px-2 py-1 font-mono text-white text-xs">
            {currentPage} / {totalPages}
          </span>
          <button
            type="button"
            disabled={currentPage >= totalPages}
            onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
            class="px-2.5 py-1 rounded-md bg-[#18191c] border border-[#25272b] text-[#d9d9d9] hover:text-white hover:bg-[#1f2125] disabled:opacity-40 cursor-pointer font-mono"
          >
            &rsaquo;
          </button>
          <button
            type="button"
            disabled={currentPage >= totalPages}
            onclick={() => (currentPage = totalPages)}
            class="px-2 py-1 rounded-md bg-[#18191c] border border-[#25272b] text-[#d9d9d9] hover:text-white hover:bg-[#1f2125] disabled:opacity-40 cursor-pointer font-mono"
          >
            &raquo;
          </button>
        </div>
      </div>
    </footer>
  {/if}
</div>
