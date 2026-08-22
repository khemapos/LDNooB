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

// Column Widths Map
let columnWidths = $state<Record<string, number>>({});

// Pagination State
let currentPage = $state(1);
let isPageSizeOpen = $state(false);
let jumpPageInput = $state("");

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

let pageNumbers = $derived(getPageNumbers(currentPage, totalPages));

function goToPage(page: number) {
  if (page >= 1 && page <= totalPages && page !== currentPage) {
    currentPage = page;
  }
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

// Selection Computes
let isAllPageSelected = $derived(
  paginatedItems.length > 0 && paginatedItems.every((item) => selectedKeys.includes(item[itemKey]))
);

let isSomePageSelected = $derived(
  paginatedItems.some((item) => selectedKeys.includes(item[itemKey])) && !isAllPageSelected
);

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
  if (selectedKeys.includes(key)) {
    selectedKeys = selectedKeys.filter((k) => k !== key);
  } else {
    selectedKeys = [...selectedKeys, key];
  }
  onUpdateSelectedKeys?.(selectedKeys);
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

function toggleColumnVisibility(key: string) {
  columns = columns.map((col) => {
    if (col.key === key && col.canHide) {
      return { ...col, visible: !col.visible };
    }
    return col;
  });
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
                <div class="flex items-center justify-end gap-1 relative" bind:this={columnSelectorRef}>
                  <span>{col.label}</span>
                  <button
                    type="button"
                    title="Customize Visible Columns"
                    onclick={(e) => {
                      e.stopPropagation();
                      showColumnSelector = !showColumnSelector;
                    }}
                    class="p-1 rounded-md text-text-muted hover:text-text-hover hover:bg-bg-card-hover transition-colors cursor-pointer"
                  >
                    <Icon name="filter" size={12} />
                  </button>

                  <!-- Column Visibility Selector Dropdown -->
                  {#if showColumnSelector}
                    <div
                      class="absolute right-0 top-full mt-1.5 w-48 p-2 bg-bg-card border border-border-default rounded-xl shadow-xl z-50 flex flex-col gap-1 text-left text-text-default normal-case font-normal animate-in fade-in zoom-in-95 duration-100"
                    >
                      <span class="text-[10px] font-extrabold uppercase tracking-widest text-text-muted px-2 py-1">
                        Display Columns
                      </span>
                      {#each columns as c}
                        <label
                          class="flex items-center gap-2 px-2 py-1.5 rounded-lg hover:bg-bg-card-hover cursor-pointer text-xs {c.canHide
                            ? 'text-text-default'
                            : 'opacity-50 cursor-not-allowed'}"
                        >
                          <input
                            type="checkbox"
                            checked={c.visible}
                            disabled={!c.canHide}
                            onchange={() => toggleColumnVisibility(c.key)}
                            class="{checkboxClassName} cursor-pointer"
                          />
                          <span class="font-medium text-xs truncate">{c.label}</span>
                        </label>
                      {/each}
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
              onclick={(e) => onRowClick?.(item, e)}
              onmousedown={(e) => onRowMouseDown?.(item, e)}
              onmouseenter={(e) => onRowMouseEnter?.(item, e)}
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
                  style="width: {width}; min-width: {width}; max-width: {width}; text-align: {col.align || (col.key === 'actions' ? 'right' : 'left')};"
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

  <!-- Pagination Footer (100% Parity with D:\ldremote) -->
  {#if paginate && isConnected}
    <footer
      class="shrink-0 flex flex-col lg:flex-row items-center justify-between border-t border-border-default px-5 py-3 select-none z-10 relative bg-bg-panel gap-3 font-sans"
    >
      <!-- Left: Counter Details Badge -->
      <div
        class="flex items-center select-none w-full lg:w-auto justify-center lg:justify-start"
      >
        <div
          class="h-9 px-4 rounded-xl border border-border-default bg-bg-card text-xs text-text-muted font-medium flex items-center gap-2 shadow-xs"
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
          <span class="font-semibold text-text-hover font-mono text-[12.5px]">
            {showingStart}-{showingEnd}
          </span>
          <span>of</span>
          <span class="font-semibold text-[#00b578] font-mono text-[12.5px]">
            {totalItems}
          </span>
          <span>{itemLabel}</span>
        </div>
      </div>

      <!-- Center: Optional Center Slot -->
      {#if renderFooterCenter}
        <div class="w-full lg:w-auto flex justify-center lg:absolute lg:left-1/2 lg:top-1/2 lg:-translate-x-1/2 lg:-translate-y-1/2 my-1 lg:my-0">
          {@render renderFooterCenter()}
        </div>
      {/if}

      <!-- Right: Page Size, Quick Jump, and Navigation Controls -->
      <div
        class="flex flex-wrap items-center justify-center lg:justify-end gap-3 w-full lg:w-auto"
      >
        <!-- Custom Page Size Dropdown -->
        <div class="relative h-9 flex items-center">
          <button
            type="button"
            onclick={() => (isPageSizeOpen = !isPageSizeOpen)}
            class="h-9 border border-border-default hover:border-border-hover text-xs font-bold rounded-xl px-3.5 flex items-center gap-2 transition-all duration-150 cursor-pointer shadow-xs active:scale-95 bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover"
          >
            <span>{pageSize} / page</span>
            <span class="transition-transform duration-200 {isPageSizeOpen ? 'rotate-180' : ''}">
              <Icon name="chevronDown" size={12} />
            </span>
          </button>

          {#if isPageSizeOpen}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <div
              class="fixed inset-0 z-30"
              role="presentation"
              onclick={() => (isPageSizeOpen = false)}
            ></div>
            <div
              class="absolute bottom-full mb-1 left-0 z-40 w-28 rounded-xl border border-border-default p-1 shadow-lg flex flex-col gap-0.5 bg-bg-card text-text-default font-sans"
            >
              {#each pageSizes as size}
                <button
                  type="button"
                  onclick={() => changePageSize(size)}
                  class="px-3 py-1.5 text-left text-xs font-bold rounded-lg transition-colors cursor-pointer w-full {pageSize === size
                    ? 'bg-[#00b578] text-white'
                    : 'hover:bg-bg-card-hover text-text-muted hover:text-text-hover'}"
                >
                  {size} / page
                </button>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Quick Jump Box -->
        <div class="flex items-center gap-2">
          <span class="text-xs text-text-muted font-medium">Go to:</span>
          <div
            class="relative flex items-center bg-bg-card border border-border-default hover:border-border-hover focus-within:border-[#00b578] focus-within:ring-2 focus-within:ring-[#00b578]/15 rounded-xl overflow-hidden transition-all duration-150 w-16 h-9 shadow-xs"
          >
            <input
              type="number"
              min="1"
              max={totalPages}
              bind:value={jumpPageInput}
              onkeydown={handleJumpPage}
              onblur={() => {
                if (jumpPageInput) {
                  const pageVal = Number.parseInt(jumpPageInput, 10);
                  if (pageVal >= 1 && pageVal <= totalPages) {
                    currentPage = pageVal;
                  }
                  jumpPageInput = "";
                }
              }}
              placeholder={String(currentPage)}
              class="w-full h-full text-center bg-transparent border-none focus:outline-none text-xs font-mono font-bold text-text-default placeholder:text-text-muted pr-5 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
            <!-- Custom mini spin buttons -->
            <div class="absolute right-1 flex flex-col h-full justify-center z-10">
              <button
                type="button"
                aria-label="Next Page"
                onclick={() => (currentPage = Math.min(currentPage + 1, totalPages))}
                class="p-0.5 hover:text-[#00b578] text-text-muted transition-colors cursor-pointer flex items-center justify-center bg-transparent border-none"
                title="Next Page"
              >
                <Icon name="chevronUp" size={10} />
              </button>
              <button
                type="button"
                aria-label="Previous Page"
                onclick={() => (currentPage = Math.max(currentPage - 1, 1))}
                class="p-0.5 hover:text-[#00b578] text-text-muted transition-colors cursor-pointer flex items-center justify-center bg-transparent border-none"
                title="Previous Page"
              >
                <Icon name="chevronDown" size={10} />
              </button>
            </div>
          </div>
        </div>

        <!-- Navigation Controls Container -->
        <div
          class="h-9 flex items-center gap-1 p-1 rounded-xl border border-border-default bg-bg-app"
        >
          <!-- Prev Button -->
          <button
            type="button"
            aria-label="Previous Page"
            disabled={currentPage === 1}
            onclick={prevPage}
            class="h-7 w-7 rounded-lg hover:scale-105 active:scale-95 transition-all duration-150 cursor-pointer flex items-center justify-center disabled:cursor-not-allowed disabled:transform-none disabled:opacity-20 border border-border-default hover:border-border-hover bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover"
            title="Previous Page"
          >
            <Icon name="chevronLeft" size={13} />
          </button>

          <!-- Page Numbers -->
          <div class="flex items-center gap-1">
            {#each pageNumbers as page, idx}
              {#if page === "..."}
                <span
                  class="text-text-muted font-bold px-2 select-none font-mono text-xs"
                >
                  ...
                </span>
              {:else}
                {@const pageNum = Number(page)}
                <button
                  type="button"
                  onclick={() => goToPage(pageNum)}
                  class="h-7 min-w-7 px-2 text-xs font-bold rounded-lg font-mono transition-all duration-150 hover:scale-105 active:scale-95 cursor-pointer flex items-center justify-center border {currentPage ===
                  pageNum
                    ? 'bg-[#00b578] hover:bg-[#00c985] text-white border-transparent shadow-md shadow-[#00b578]/20'
                    : 'bg-bg-card border-border-default hover:border-border-hover text-text-muted hover:text-text-hover hover:bg-bg-card-hover'}"
                >
                  {page}
                </button>
              {/if}
            {/each}
          </div>

          <!-- Next Button -->
          <button
            type="button"
            aria-label="Next Page"
            disabled={currentPage === totalPages}
            onclick={nextPage}
            class="h-7 w-7 rounded-lg hover:scale-105 active:scale-95 transition-all duration-150 cursor-pointer flex items-center justify-center disabled:cursor-not-allowed disabled:transform-none disabled:opacity-20 border border-border-default hover:border-border-hover bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover"
            title="Next Page"
          >
            <Icon name="chevronRight" size={13} />
          </button>
        </div>
      </div>
    </footer>
  {/if}
</div>
