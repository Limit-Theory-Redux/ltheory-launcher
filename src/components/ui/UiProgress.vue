<template>
  <div
    class="relative h-[5px] overflow-hidden rounded-full bg-white/[0.08]"
    role="progressbar"
    :aria-label="label"
    :aria-valuenow="indeterminate ? undefined : safeValue"
    aria-valuemin="0"
    aria-valuemax="100"
  >
    <span
      class="block h-full rounded-[inherit] bg-gradient-to-r from-accent-strong to-[#96edf8] shadow-[0_0_12px_rgba(114,215,239,0.48)] transition-[width] duration-200"
      :class="indeterminate && 'w-[42%] animate-indeterminate'"
      :style="barStyle"
    />
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    value?: number;
    indeterminate?: boolean;
    label?: string;
  }>(),
  {
    value: 0,
    indeterminate: false,
    label: "Progress",
  },
);

const safeValue = computed(() => Math.min(100, Math.max(0, props.value)));
const barStyle = computed(() =>
  props.indeterminate ? undefined : { width: `${safeValue.value}%` },
);
</script>
