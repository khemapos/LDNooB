<script lang="ts">
import Icon from "../ui/Icon.svelte";
import BaseModal from "./BaseModal.svelte";

interface Props {
  open: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  isDestructive?: boolean;
  onConfirm: () => void;
  onCancel?: () => void;
}

let {
  open = $bindable(false),
  title,
  message,
  confirmText = "Confirm",
  cancelText = "Cancel",
  isDestructive = false,
  onConfirm,
  onCancel,
}: Props = $props();

function handleConfirm() {
  onConfirm();
  open = false;
}

function handleCancel() {
  onCancel?.();
  open = false;
}
</script>

<BaseModal
  bind:open
  {title}
  icon="alert"
  maxWidth="max-w-md"
  onClose={handleCancel}
>
  <p class="text-sm text-slate-600 dark:text-slate-300 leading-relaxed">
    {message}
  </p>

  {#snippet footer()}
    <button
      type="button"
      onclick={handleCancel}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-white/[0.06] hover:bg-slate-200 dark:hover:bg-white/[0.12] transition-colors cursor-pointer"
    >
      {cancelText}
    </button>
    <button
      type="button"
      onclick={handleConfirm}
      class="px-4 py-2 text-xs font-semibold rounded-xl text-white transition-all shadow-sm cursor-pointer {isDestructive
        ? 'bg-rose-600 hover:bg-rose-500 shadow-rose-600/20'
        : 'bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 shadow-cyan-500/20'}"
    >
      {confirmText}
    </button>
  {/snippet}
</BaseModal>
