<script lang="ts">
import BaseModal from "./BaseModal.svelte";
import CustomButton from "./CustomButton.svelte";

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
  open = false;
  onConfirm();
}

function handleCancel() {
  open = false;
  onCancel?.();
}
</script>

<BaseModal
  bind:open
  {title}
  icon={isDestructive ? "trash" : "cube"}
  maxWidth="max-w-md"
>
  <p class="text-xs text-[#d9d9d9] font-sans leading-relaxed">
    {message}
  </p>

  {#snippet footer()}
    <CustomButton
      variant="secondary"
      size="md"
      onclick={handleCancel}
    >
      {cancelText}
    </CustomButton>

    <CustomButton
      variant={isDestructive ? "danger" : "primary"}
      size="md"
      onclick={handleConfirm}
    >
      {confirmText}
    </CustomButton>
  {/snippet}
</BaseModal>
