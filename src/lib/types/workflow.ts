export type WorkflowActionType =
  | "adb_command"
  | "human_swipe"
  | "human_type"
  | "start_app"
  | "stop_app"
  | "delay"
  | "screenshot"
  | "inject_proxy"
  | "clear_cache";

export interface WorkflowStep {
  id: string;
  name: string;
  type: WorkflowActionType;
  params: Record<string, any>;
  enabled: boolean;
}

export interface WorkflowPreset {
  id: string;
  name: string;
  description: string;
  steps: WorkflowStep[];
}

export interface HierarchyNode {
  tag: string;
  text?: string;
  resourceId?: string;
  className?: string;
  packageName?: string;
  bounds?: [number, number, number, number]; // [left, top, right, bottom]
  children: HierarchyNode[];
}
