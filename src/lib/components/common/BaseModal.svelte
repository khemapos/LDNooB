<script lang="ts">
import type { Snippet } from "svelte";
import Icon from "../ui/Icon.svelte";

interface Props {
  open: boolean;
  title: string;
  subtitle?: string;
  icon?: any;
  maxWidth?: string;
  onClose?: () => void;
  children?: Snippet;
  footer?: Snippet;
}

let {
  open = $bindable(false),
  title,
  subtitle,
  icon,
  maxWidth = "max-w-xl",
  onClose,
  children,
  footer,
}: Props = $props();

function handleKeydown(e: KeyboardEvent) {
  if (open && e.key === "Escape") {
    e.preventDefault();
    closeModal();
  }
}

function closeModal() {
  open = false;
  onClose?.();
}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-xs animate-in fade-in duration-150 select-none"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) closeModal();
    }}
  >
    <div
      role="dialog"
      aria-modal="true"
      class="relative w-full {maxWidth} bg-[#131416] border border-[#25272b] shadow-2xl rounded-2xl overflow-hidden backdrop-blur-md flex flex-col transition-all duration-200 max-h-[90vh] text-white"
    >
      <!-- Dialog Header matching D:\ldremote -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-[#25272b] bg-[#141517]"
      >
        <div class="flex items-center gap-2.5">
          {#if icon}
            <div class="w-6 h-6 rounded-lg bg-[#00b578]/10 text-[#00b578] flex items-center justify-center shrink-0">
              <Icon name={icon} size={14} />
            </div>
          {/if}
          <div>
            <h3 class="text-[11.5px] font-extrabold uppercase tracking-widest text-[#d9d9d9] font-sans">
              {title}
            </h3>
            {#if subtitle}
              <p class="text-[10px] text-[#8c8c8c] mt-0.5 font-sans">
                {subtitle}
              </p>
            {/if}
          </div>
        </div>

        <button
          type="button"
          title="Close Modal (Esc)"
          onclick={closeModal}
          class="w-6.5 h-6.5 rounded-lg flex items-center justify-center bg-rose-500 hover:bg-rose-600 active:bg-rose-700 text-white transition-colors cursor-pointer shadow-md shadow-rose-500/10"
        >
          <Icon name="close" size={12} />
        </button>
      </div>

      <!-- Dialog Body -->
      <div class="p-6 overflow-y-auto flex-1 text-xs text-[#d9d9d9] font-sans">
        {#if children}
          {@render children()}
        {/if}
      </div>

      <!-- Dialog Footer -->
      {#if footer}
        <div
          class="flex items-center justify-end gap-2.5 px-6 py-4 border-t border-[#25272b] bg-[#141517]"
        >
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}
