<script lang="ts">
import { logsStore } from "$lib/stores/logs.svelte";
import type { WorkflowStep } from "$lib/types";
import Icon from "../ui/Icon.svelte";

let steps = $state<WorkflowStep[]>([
  {
    id: "step-1",
    name: "Launch Target Application",
    type: "start_app",
    params: { packageName: "com.facebook.katana" },
    enabled: true,
  },
  {
    id: "step-2",
    name: "Wait For Screen Load",
    type: "delay",
    params: { seconds: 3 },
    enabled: true,
  },
  {
    id: "step-3",
    name: "Human Scroll Feed",
    type: "human_swipe",
    params: { x1: 360, y1: 900, x2: 360, y2: 300, duration: 400 },
    enabled: true,
  },
]);

let isExecuting = $state(false);

function executeWorkflow() {
  isExecuting = true;
  logsStore.info("Workflow", "Started executing automated workflow steps");
  setTimeout(() => {
    isExecuting = false;
    logsStore.success("Workflow", "Workflow execution completed successfully");
  }, 2000);
}
</script>

<div class="flex-1 flex flex-col h-full gap-3 overflow-hidden">
  <!-- Toolbar -->
  <div
    class="flex items-center justify-between p-3 bg-white/80 dark:bg-[#0e1018]/90 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-xl rounded-2xl shadow-xs"
  >
    <div class="flex items-center gap-2">
      <button
        type="button"
        disabled={isExecuting}
        onclick={executeWorkflow}
        class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-semibold text-slate-950 bg-gradient-to-r from-cyan-400 to-blue-500 hover:from-cyan-300 hover:to-blue-400 transition-all shadow-xs cursor-pointer disabled:opacity-50"
      >
        <Icon name="play" size={12} />
        <span>{isExecuting ? "Executing..." : "Run Workflow"}</span>
      </button>
    </div>

    <div class="text-xs font-mono text-slate-500 dark:text-slate-400">
      {steps.length} Steps Defined
    </div>
  </div>

  <!-- Steps List Container -->
  <div
    class="flex-1 p-4 rounded-2xl bg-white/70 dark:bg-[#0c0d13]/80 border border-slate-200/90 dark:border-white/[0.08] backdrop-blur-md overflow-y-auto space-y-2.5 shadow-inner"
  >
    {#each steps as step, index (step.id)}
      <div
        class="flex items-center justify-between p-3.5 rounded-xl bg-slate-50 dark:bg-[#12141e] border border-slate-200/80 dark:border-white/[0.06] transition-all hover:border-cyan-500/30"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-6 h-6 rounded-lg bg-cyan-500/10 text-cyan-600 dark:text-cyan-400 text-xs font-mono font-bold flex items-center justify-center border border-cyan-500/20"
          >
            {index + 1}
          </div>
          <div>
            <div class="text-xs font-semibold text-slate-900 dark:text-white">
              {step.name}
            </div>
            <div class="text-[11px] font-mono text-slate-500 dark:text-slate-400">
              Type: {step.type} • Params: {JSON.stringify(step.params)}
            </div>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <input
            type="checkbox"
            bind:checked={step.enabled}
            class="w-4 h-4 rounded border-slate-300 dark:border-white/20 text-cyan-600 cursor-pointer"
          />
        </div>
      </div>
    {/each}
  </div>
</div>
