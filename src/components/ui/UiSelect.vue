<template>
  <label class="mt-3 flex items-center justify-between gap-3 border-t border-white/[0.07] pt-3">
    <span class="text-[11px] text-muted">{{ label }}</span>
    <span class="relative">
      <select
        v-model="model"
        class="h-[31px] min-w-[150px] appearance-none rounded-lg border border-white/15 bg-black/25 py-0 pr-8 pl-3 text-[11px] text-ink outline-none transition focus:border-accent/60 focus:ring-2 focus:ring-accent/20 disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="disabled"
      >
        <option v-for="option in options" :key="option" :value="option">
          {{ option }}
        </option>
      </select>
      <svg class="pointer-events-none absolute top-2 right-2 size-[15px] fill-none stroke-muted stroke-[1.7] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 20 20" aria-hidden="true">
        <path d="m5 7.5 5 5 5-5" />
      </svg>
    </span>
  </label>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    label: string;
    options: string[];
    disabled?: boolean;
  }>(),
  { disabled: false },
);

const model = defineModel<string>({ required: true });
</script>
