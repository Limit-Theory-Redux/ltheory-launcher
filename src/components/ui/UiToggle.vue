<template>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md text-[10px] text-white/70 outline-none transition focus-visible:ring-2 focus-visible:ring-accent/60"
    role="switch"
    :aria-checked="model"
    @click="model = !model"
  >
    <span
      class="relative h-[17px] w-[29px] rounded-full border transition"
      :class="model ? 'border-accent/40 bg-accent-strong/20' : 'border-white/20 bg-black/35'"
    >
      <span
        class="absolute top-[3px] left-[3px] size-[9px] rounded-full transition"
        :class="model ? 'translate-x-3 bg-accent' : 'bg-[#8d9ba3]'"
      />
    </span>
    <span>{{ label }}</span>
  </button>
</template>

<script setup lang="ts">
defineProps<{ label: string }>();
const model = defineModel<boolean>({ required: true });
</script>
