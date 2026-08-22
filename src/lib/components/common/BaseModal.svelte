<script lang="ts">
  import Icon from "../ui/Icon.svelte";

  interface Props {
    open: boolean;
    title: string;
    subtitle?: string;
    icon?: any;
    maxWidth?: string;
    onClose?: () => void;
    children?: any;
    footer?: any;
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
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-sm animate-in fade-in duration-150"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) closeModal();
    }}
  >
    <div
      role="dialog"
      aria-modal="true"
      class="relative w-full {maxWidth} bg-white dark:bg-[#0e1018] border border-slate-200 dark:border-white/[0.08] shadow-2xl rounded-2xl overflow-hidden backdrop-blur-2xl flex flex-col transition-all duration-150 ring-1 ring-inset ring-transparent dark:ring-white/[0.03] max-h-[90vh]"
    >
      <!-- Modal Header -->
      <div
        class="flex items-center justify-between px-5 py-4 border-b border-slate-200 dark:border-white/[0.06] bg-slate-50/50 dark:bg-white/[0.02]"
      >
        <div class="flex items-center gap-3">
          {#if icon}
            <div
              class="w-8 h-8 rounded-lg bg-cyan-500/10 dark:bg-cyan-500/20 text-cyan-600 dark:text-cyan-400 border border-cyan-500/25 flex items-center justify-center shrink-0"
            >
              <Icon name={icon} size={16} />
            </div>
          {/if}
          <div>
            <h3 class="text-sm font-bold text-slate-900 dark:text-white tracking-tight">
              {title}
            </h3>
            {#if subtitle}
              <p class="text-xs text-slate-500 dark:text-slate-400">
                {subtitle}
              </p>
            {/if}
          </div>
        </div>

        <button
          type="button"
          title="Close Modal (Esc)"
          onclick={closeModal}
          class="w-7 h-7 rounded-lg flex items-center justify-center text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-white/[0.06] transition-colors cursor-pointer"
        >
          <Icon name="close" size={14} />
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-5 overflow-y-auto flex-1 text-sm text-slate-700 dark:text-slate-300">
        {#if children}
          {@render children()}
        {/if}
      </div>

      <!-- Modal Footer (Optional) -->
      {#if footer}
        <div
          class="flex items-center justify-end gap-2.5 px-5 py-3.5 border-t border-slate-200 dark:border-white/[0.06] bg-slate-50/80 dark:bg-[#08090d]"
        >
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}
