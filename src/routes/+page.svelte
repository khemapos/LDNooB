<script lang="ts">
import { onMount } from "svelte";
import NavigationTabs from "$lib/components/layout/NavigationTabs.svelte";
import AccountsView from "$lib/components/views/AccountsView.svelte";
import ActivityLogPanel from "$lib/components/views/ActivityLogPanel.svelte";
import HierarchyInspectorView from "$lib/components/views/HierarchyInspectorView.svelte";
import ProfilesView from "$lib/components/views/ProfilesView.svelte";
import SettingsView from "$lib/components/views/SettingsView.svelte";
import VisualWorkflowsView from "$lib/components/views/VisualWorkflowsView.svelte";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import type { ActiveTab } from "$lib/types";

let activeTab = $state<ActiveTab>("profiles");
let showActivityLog = $state(true);

onMount(async () => {
  await settingsStore.init();
  await emulatorsStore.refresh();
});
</script>

<main class="flex-1 w-full h-full flex flex-col p-3 gap-3 overflow-hidden select-none">
  <!-- Top Navigation Bar -->
  <div class="flex items-center justify-between shrink-0">
    <NavigationTabs
      bind:activeTab
      onTabChange={(t) => (activeTab = t)}
    />
  </div>

  <!-- Active Viewport Container -->
  <div class="flex-1 overflow-hidden flex flex-col min-h-0">
    {#if activeTab === "profiles"}
      <ProfilesView />
    {:else if activeTab === "accounts"}
      <AccountsView />
    {:else if activeTab === "workflows"}
      <VisualWorkflowsView />
    {:else if activeTab === "inspector"}
      <HierarchyInspectorView />
    {:else if activeTab === "settings"}
      <SettingsView />
    {/if}
  </div>

  <!-- Diagnostic Activity Log Console -->
  <ActivityLogPanel />
</main>
