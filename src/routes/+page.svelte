<script lang="ts">
import { onMount } from "svelte";
import NavigationTabs from "$lib/components/layout/NavigationTabs.svelte";
import AccountsView from "$lib/components/views/AccountsView.svelte";
import ActivityLogPanel from "$lib/components/views/ActivityLogPanel.svelte";
import ProfilesView from "$lib/components/views/ProfilesView.svelte";
import SettingsView from "$lib/components/views/SettingsView.svelte";
import { emulatorsStore } from "$lib/stores/emulators.svelte";
import { settingsStore } from "$lib/stores/settings.svelte";
import type { ActiveTab } from "$lib/types";

let activeTab = $state<ActiveTab>("profiles");

onMount(async () => {
  await settingsStore.init();
  await emulatorsStore.refresh();
});
</script>

<div class="flex-1 w-full h-full flex flex-col overflow-hidden select-none">
  <!-- Padded View Area: Navigation & Content -->
  <div class="flex-1 w-full flex flex-col p-3 gap-3 overflow-hidden min-h-0">
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
      {:else if activeTab === "settings"}
        <SettingsView />
      {/if}
    </div>
  </div>

  <!-- Full-Width Activity Log Console (Matching D:\ldremote) -->
  <ActivityLogPanel />
</div>
