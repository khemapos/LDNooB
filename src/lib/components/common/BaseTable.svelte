<script lang="ts">
  import Icon from "../ui/Icon.svelte";

  export interface TableColumn {
    key: string;
    label: string;
    width?: string;
    align?: "left" | "center" | "right";
  }

  interface Props {
    columns: TableColumn[];
    items: any[];
    selectedKeys?: any[];
    itemKey?: string;
    onSelectAll?: () => void;
    onToggleSelect?: (key: any) => void;
    onRowClick?: (item: any) => void;
    rowSnippet?: any;
  }

  let {
    columns,
    items,
    selectedKeys = [],
    itemKey = "index",
    onSelectAll,
    onToggleSelect,
    onRowClick,
    rowSnippet,
  }: Props = $props();

  let allSelected = $derived(
    items.length > 0 && selectedKeys.length === items.length
  );
  let someSelected = $derived(
    selectedKeys.length > 0 && selectedKeys.length < items.length
  );
</script>

<div
  class="w-full h-full overflow-auto rounded-xl border border-slate-200/90 dark:border-white/[0.08] bg-white/70 dark:bg-[#0c0d13]/80 backdrop-blur-md shadow-inner transition-colors"
>
  <table class="w-full text-left border-collapse text-xs">
    <!-- Table Header -->
    <thead
      class="sticky top-0 z-20 bg-slate-100/90 dark:bg-[#12141e]/95 backdrop-blur-md border-b border-slate-200 dark:border-white/[0.08] text-slate-500 dark:text-slate-400 font-mono select-none"
    >
      <tr>
        {#if onSelectAll}
          <th class="w-10 px-3 py-2.5 text-center">
            <input
              type="checkbox"
              checked={allSelected}
              indeterminate={someSelected}
              onchange={onSelectAll}
              class="w-3.5 h-3.5 rounded border-slate-300 dark:border-white/20 text-cyan-600 focus:ring-cyan-500/20 cursor-pointer"
            />
          </th>
        {/if}

        {#each columns as col}
          <th
            class="px-3 py-2.5 font-semibold text-[11px] uppercase tracking-wider {col.width ||
              ''} {col.align === 'center'
              ? 'text-center'
              : col.align === 'right'
                ? 'text-right'
                : 'text-left'}"
          >
            {col.label}
          </th>
        {/each}
      </tr>
    </thead>

    <!-- Table Body -->
    <tbody class="divide-y divide-slate-100 dark:divide-white/[0.04]">
      {#if items.length === 0}
        <tr>
          <td
            colspan={columns.length + (onSelectAll ? 1 : 0)}
            class="py-16 text-center text-slate-400 dark:text-slate-500 font-sans"
          >
            <div class="flex flex-col items-center justify-center gap-2">
              <Icon name="cube" size={28} class="opacity-40" />
              <p class="text-xs">No emulator instances found</p>
            </div>
          </td>
        </tr>
      {:else}
        {#each items as item (item[itemKey])}
          {@const isSelected = selectedKeys.includes(item[itemKey])}
          <tr
            onclick={() => onRowClick?.(item)}
            class="transition-colors duration-100 cursor-pointer {isSelected
              ? 'bg-cyan-500/[0.08] dark:bg-cyan-500/[0.12] hover:bg-cyan-500/[0.12] dark:hover:bg-cyan-500/[0.16]'
              : 'hover:bg-slate-50 dark:hover:bg-white/[0.03]'}"
          >
            {#if onToggleSelect}
              <td
                class="w-10 px-3 py-2 text-center"
                onclick={(e) => e.stopPropagation()}
              >
                <input
                  type="checkbox"
                  checked={isSelected}
                  onchange={() => onToggleSelect?.(item[itemKey])}
                  class="w-3.5 h-3.5 rounded border-slate-300 dark:border-white/20 text-cyan-600 focus:ring-cyan-500/20 cursor-pointer"
                />
              </td>
            {/if}

            {#if rowSnippet}
              {@render rowSnippet(item)}
            {:else}
              {#each columns as col}
                <td class="px-3 py-2 text-slate-700 dark:text-slate-300 font-mono">
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
