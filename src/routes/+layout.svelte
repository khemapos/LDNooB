<script lang="ts">
import { onMount } from "svelte";
import "../app.css";
import BottomBar from "$lib/components/layout/BottomBar.svelte";
import WindowBar from "$lib/components/layout/WindowBar.svelte";
import { themeStore } from "$lib/stores/theme.svelte";

let { children } = $props();

onMount(() => {
  themeStore.init();
});
</script>

<div class="h-screen w-screen flex flex-col overflow-hidden bg-bg-app text-text-default select-none transition-colors duration-150 font-sans antialiased">
  <!-- Desktop Frameless Window Title Bar -->
  <WindowBar title="LDNooB" />

  <!-- Main Viewport / Scroll Area -->
  <div class="flex-1 overflow-auto flex flex-col relative">
    {@render children()}
  </div>

  <!-- Desktop Bottom Status Bar -->
  <BottomBar status="Ready" />
</div>
