<script lang="ts">
import BaseModal from "../common/BaseModal.svelte";
import SettingsView from "../views/SettingsView.svelte";

interface Props {
  open: boolean;
  onClose?: () => void;
}

let { open = $bindable(false), onClose }: Props = $props();
</script>

<BaseModal
  bind:open
  title="Application Settings"
  icon="settings"
  maxWidth="max-w-3xl"
  {onClose}
>
  <div class="p-4 max-h-[75vh] overflow-y-auto custom-scrollbar">
    <SettingsView isModal={true} onSaved={() => (open = false)} />
  </div>
</BaseModal>
