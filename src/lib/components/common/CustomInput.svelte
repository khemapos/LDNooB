<script lang="ts">
import type { Snippet } from "svelte";
import Icon from "../ui/Icon.svelte";

interface Props {
  label?: string;
  type?: string;
  placeholder?: string;
  value?: string | number;
  disabled?: boolean;
  error?: boolean;
  class?: string;
  id?: string;
  icon?: any;
  rightElement?: Snippet;
  oninput?: (e: Event) => void;
}

let {
  label,
  type = "text",
  placeholder = "",
  value = $bindable(""),
  disabled = false,
  error = false,
  class: className = "",
  id,
  icon,
  rightElement,
  oninput,
}: Props = $props();

let showPassword = $state(false);
let isPassword = $derived(type === "password");
let inputType = $derived(isPassword ? (showPassword ? "text" : "password") : type);
</script>

<div
  class="flex items-center w-full border rounded-xl px-3.5 py-2 transition-all duration-150 shadow-xs cursor-text bg-bg-card hover:bg-bg-card-hover focus-within:ring-2 focus-within:ring-[#00b578]/20 focus-within:border-[#00b578] {error
    ? 'border-[#ff4d4f]/60 focus-within:ring-[#ff4d4f]/20 focus-within:border-[#ff4d4f]'
    : 'border-border-default hover:border-border-hover'} {disabled
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
    <input
      {id}
      type={inputType}
      {placeholder}
      bind:value
      {disabled}
      {oninput}
      autocomplete="off"
      spellcheck="false"
      class="w-full text-xs font-semibold bg-transparent outline-none border-none p-0 focus:ring-0 leading-tight text-text-default placeholder:text-text-muted font-sans"
    />
  </div>

  {#if isPassword}
    <button
      type="button"
      title={showPassword ? "Hide password" : "Show password"}
      onclick={() => (showPassword = !showPassword)}
      class="p-1 text-text-muted hover:text-text-hover transition-colors cursor-pointer shrink-0"
    >
      <Icon name={showPassword ? "eye" : "eye"} size={14} />
    </button>
  {/if}

  {#if rightElement}
    <div class="pl-2 shrink-0">
      {@render rightElement()}
    </div>
  {/if}
</div>
