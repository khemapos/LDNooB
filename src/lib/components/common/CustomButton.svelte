<script lang="ts">
import type { Snippet } from "svelte";

export type ButtonVariant =
  | "primary"
  | "secondary"
  | "danger"
  | "success"
  | "blue"
  | "ghost"
  | "outline"
  | "link"
  | "icon";

export type ButtonSize = "xs" | "sm" | "md" | "lg" | "icon";

interface Props {
  type?: "button" | "submit" | "reset";
  variant?: ButtonVariant;
  size?: ButtonSize;
  loading?: boolean;
  disabled?: boolean;
  title?: string;
  class?: string;
  onclick?: (e: MouseEvent) => void;
  leftIcon?: Snippet;
  rightIcon?: Snippet;
  children?: Snippet;
}

let {
  type = "button",
  variant = "primary",
  size = "md",
  loading = false,
  disabled = false,
  title,
  class: className = "",
  onclick,
  leftIcon,
  rightIcon,
  children,
}: Props = $props();

const baseClasses =
  "inline-flex items-center justify-center gap-1.5 whitespace-nowrap transition-all duration-150 select-none cursor-pointer font-sans disabled:pointer-events-none disabled:opacity-40";

let variantClass = $derived(
  {
    primary:
      "bg-[#00b578] hover:bg-[#00c985] text-white font-bold border border-white/10 shadow-[0_2px_10px_rgba(0,181,120,0.3),_inset_0_1px_0_rgba(255,255,255,0.2)] active:scale-[0.98]",
    success:
      "bg-[#00b578] hover:bg-[#00c985] text-white font-bold border border-white/10 shadow-[0_2px_10px_rgba(0,181,120,0.3)] active:scale-[0.98]",
    blue: "bg-[#1877f2] hover:bg-[#166fe5] text-white font-bold border border-white/10 shadow-[0_2px_10px_rgba(24,119,242,0.3)] active:scale-[0.98]",
    danger:
      "bg-[#ff4d4f] hover:bg-[#ff7875] text-white font-bold border border-white/10 shadow-[0_2px_10px_rgba(255,77,79,0.25)] active:scale-[0.98]",
    secondary:
      "bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover border border-border-default hover:border-border-hover shadow-xs active:scale-[0.98]",
    ghost:
      "bg-transparent hover:bg-bg-card-hover text-text-muted hover:text-text-hover border border-transparent",
    outline:
      "border border-border-default hover:border-border-hover bg-transparent text-text-muted hover:text-text-hover hover:bg-bg-card",
    link: "text-[#00b578] hover:underline bg-transparent p-0 border-none font-semibold",
    icon: "bg-bg-card hover:bg-bg-card-hover text-text-muted hover:text-text-hover border border-border-default rounded-xl flex items-center justify-center shrink-0",
  }[variant]
);

let sizeClass = $derived(
  variant === "link"
    ? ""
    : {
        xs: "h-7 px-2.5 rounded-lg text-[10px] font-semibold",
        sm: "h-8 px-3 rounded-lg text-[11px] font-semibold",
        md: "h-9 px-4 rounded-xl text-xs font-semibold",
        lg: "w-full h-10 rounded-xl text-xs font-semibold",
        icon: "w-9 h-9 rounded-xl flex items-center justify-center shrink-0",
      }[size]
);
</script>

<button
  {type}
  {title}
  disabled={disabled || loading}
  {onclick}
  class="{baseClasses} {variantClass} {sizeClass} {className}"
>
  {#if loading}
    <span
      class="inline-block animate-spin border-2 border-current/30 border-t-current rounded-full w-3.5 h-3.5 mr-0.5"
    ></span>
  {/if}

  {#if !loading && leftIcon}
    {@render leftIcon()}
  {/if}

  {#if children}
    {@render children()}
  {/if}

  {#if !loading && rightIcon}
    {@render rightIcon()}
  {/if}
</button>
