<script lang="ts">
import Icon from "../ui/Icon.svelte";

export interface SelectOption {
  value: string | number;
  label: string;
}

interface Props {
  label?: string;
  value?: string | number;
  options: SelectOption[];
  disabled?: boolean;
  class?: string;
  id?: string;
  icon?: any;
  onchange?: (e: Event) => void;
}

let {
  label,
  value = $bindable(""),
  options = [],
  disabled = false,
  class: className = "",
  id,
  icon,
  onchange,
}: Props = $props();
</script>

<div
  class="flex items-center w-full border rounded-xl px-3.5 py-2 transition-all duration-150 shadow-xs cursor-pointer bg-bg-card hover:bg-bg-card-hover focus-within:ring-2 focus-within:ring-[#00b578]/20 focus-within:border-[#00b578] border-border-default hover:border-border-hover {disabled
    ? 'opacity-50 cursor-not-allowed'
    : ''} {className}"
>
  {#if icon}
    <div class="flex items-center justify-center min-w-[18px] text-text-muted shrink-0">
      <Icon name={icon} size={14} />
    </div>
    <div class="border-r h-5 mx-3 border-border-default select-none shrink-0"></div>
  {/if}

  <div class="flex-1 flex flex-col text-left overflow-hidden">
    {#if label}
      <label
        for={id}
        class="text-[9px] font-extrabold uppercase tracking-widest text-text-muted select-none truncate"
      >
        {label}
      </label>
    {/if}
    <select
      {id}
      bind:value
      {disabled}
      {onchange}
      class="w-full text-xs font-semibold bg-transparent outline-none border-none p-0 focus:ring-0 leading-tight text-text-default font-sans cursor-pointer"
    >
      {#each options as opt}
        <option value={opt.value} class="bg-bg-card text-text-default">
          {opt.label}
        </option>
      {/each}
    </select>
  </div>

  <div class="pl-2 text-text-muted pointer-events-none shrink-0">
    <Icon name="chevronDown" size={13} />
  </div>
</div>
