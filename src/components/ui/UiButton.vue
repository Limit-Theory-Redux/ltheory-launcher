<template>
  <button
    class="relative inline-flex min-h-[42px] items-center justify-center gap-2 overflow-hidden rounded-lg border px-4 text-xs font-bold tracking-[0.01em] transition duration-150 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent disabled:cursor-not-allowed disabled:opacity-40 enabled:hover:-translate-y-px enabled:active:translate-y-0"
    :class="[variantClasses[variant], block && 'w-full']"
    :disabled="disabled || loading"
    :aria-busy="loading"
    type="button"
  >
    <span
      v-if="$slots.icon"
      class="size-[17px] [&>svg]:size-[17px] [&>svg]:fill-none [&>svg]:stroke-current [&>svg]:stroke-[1.6] [&>svg]:[stroke-linecap:round] [&>svg]:[stroke-linejoin:round]"
      aria-hidden="true"
    >
      <slot name="icon" />
    </span>
    <span><slot /></span>
    <span
      v-if="loading"
      class="absolute right-3 size-3.5 animate-spin rounded-full border-2 border-current border-r-transparent"
      aria-hidden="true"
    />
  </button>
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    variant?: "primary" | "secondary" | "success" | "ghost";
    disabled?: boolean;
    loading?: boolean;
    block?: boolean;
  }>(),
  {
    variant: "primary",
    disabled: false,
    loading: false,
    block: false,
  },
);

const variantClasses = {
  primary: "border-transparent bg-gradient-to-br from-[#83e4f7] to-accent-strong text-[#03131a] shadow-[0_8px_20px_rgba(32,169,208,0.17)] enabled:hover:shadow-[0_12px_28px_rgba(32,169,208,0.3)]",
  secondary: "border-white/15 bg-white/[0.055] text-[#dcecf1] enabled:hover:border-accent/35 enabled:hover:bg-accent/10",
  success: "border-positive/30 bg-positive/[0.13] text-[#a8efbf] enabled:hover:bg-positive/20",
  ghost: "border-transparent bg-transparent text-muted enabled:hover:bg-white/[0.06] enabled:hover:text-ink",
} as const;

const { variant, disabled, loading, block } = toRefs(props);
</script>
