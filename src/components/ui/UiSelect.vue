<template>
  <div ref="root" class="mt-3 flex items-center justify-between gap-3 border-t border-white/[0.07] pt-3">
    <span :id="labelId" class="text-[11px] text-muted">{{ label }}</span>
    <div class="relative min-w-[170px]">
      <button
        :id="triggerId"
        class="flex h-[33px] w-full items-center justify-between gap-3 rounded-lg border border-white/15 bg-black/25 px-3 text-left text-[11px] text-ink outline-none transition hover:border-white/25 hover:bg-white/[0.05] focus-visible:border-accent/60 focus-visible:ring-2 focus-visible:ring-accent/20 disabled:cursor-not-allowed disabled:opacity-40"
        type="button"
        role="combobox"
        aria-haspopup="listbox"
        :aria-labelledby="`${labelId} ${triggerId}`"
        :aria-controls="listboxId"
        :aria-expanded="open"
        :disabled="disabled"
        @click="toggleMenu"
        @keydown="onTriggerKeydown"
      >
        <span class="truncate">{{ model }}</span>
        <svg class="size-[15px] shrink-0 fill-none stroke-muted stroke-[1.7] transition-transform [stroke-linecap:round] [stroke-linejoin:round]" :class="open && 'rotate-180'" viewBox="0 0 20 20" aria-hidden="true">
          <path d="m5 7.5 5 5 5-5" />
        </svg>
      </button>

      <div
        v-if="open"
        :id="listboxId"
        class="absolute right-0 bottom-[calc(100%+7px)] z-[80] max-h-[238px] w-[220px] overflow-y-auto rounded-xl border border-white/15 bg-[#07131d]/98 p-1.5 shadow-[0_18px_48px_rgba(0,0,0,0.48)] backdrop-blur-2xl"
        role="listbox"
        :aria-labelledby="labelId"
      >
        <button
          v-for="(option, index) in options"
          :key="option"
          class="flex min-h-8 w-full items-center justify-between gap-3 rounded-lg border-0 px-2.5 text-left text-[11px] outline-none transition"
          :class="option === model
            ? 'bg-accent/12 text-accent'
            : index === activeIndex
              ? 'bg-white/[0.08] text-ink'
              : 'bg-transparent text-white/60 hover:bg-white/[0.055] hover:text-ink'"
          type="button"
          role="option"
          tabindex="-1"
          :aria-selected="option === model"
          :data-option-index="index"
          @mouseenter="activeIndex = index"
          @click="selectOption(option)"
          @keydown="onOptionKeydown($event, index)"
        >
          <span class="truncate">{{ option }}</span>
          <svg v-if="option === model" class="size-3.5 shrink-0 fill-none stroke-current stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 16 16" aria-hidden="true"><path d="m3 8.2 3 3L13 4.8" /></svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, useId, watch } from "vue";

const props = withDefaults(
  defineProps<{
    label: string;
    options: string[];
    disabled?: boolean;
  }>(),
  { disabled: false },
);

const model = defineModel<string>({ required: true });
const root = ref<HTMLElement>();
const open = ref(false);
const activeIndex = ref(0);
const uid = useId();
const labelId = `select-label-${uid}`;
const triggerId = `select-trigger-${uid}`;
const listboxId = `select-listbox-${uid}`;

function selectedIndex() {
  const index = props.options.indexOf(model.value);
  return index >= 0 ? index : 0;
}

async function openMenu(direction = 0) {
  if (props.disabled || props.options.length === 0) return;
  const baseIndex = selectedIndex();
  activeIndex.value = Math.min(props.options.length - 1, Math.max(0, baseIndex + direction));
  open.value = true;
  await nextTick();
  focusActiveOption();
}

function closeMenu(restoreFocus = false) {
  open.value = false;
  if (restoreFocus) document.getElementById(triggerId)?.focus();
}

function toggleMenu() {
  if (open.value) closeMenu();
  else void openMenu();
}

function selectOption(option: string) {
  model.value = option;
  closeMenu(true);
}

function focusActiveOption() {
  root.value?.querySelector<HTMLElement>(`[data-option-index="${activeIndex.value}"]`)?.focus();
}

async function moveActive(nextIndex: number) {
  activeIndex.value = Math.min(props.options.length - 1, Math.max(0, nextIndex));
  await nextTick();
  focusActiveOption();
}

function onTriggerKeydown(event: KeyboardEvent) {
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      void openMenu(open.value ? 1 : 0);
      break;
    case "ArrowUp":
      event.preventDefault();
      void openMenu(open.value ? -1 : 0);
      break;
    case "Enter":
    case " ":
      event.preventDefault();
      toggleMenu();
      break;
    case "Escape":
      closeMenu();
      break;
  }
}

function onOptionKeydown(event: KeyboardEvent, index: number) {
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      void moveActive(index + 1);
      break;
    case "ArrowUp":
      event.preventDefault();
      void moveActive(index - 1);
      break;
    case "Home":
      event.preventDefault();
      void moveActive(0);
      break;
    case "End":
      event.preventDefault();
      void moveActive(props.options.length - 1);
      break;
    case "Enter":
    case " ":
      event.preventDefault();
      selectOption(props.options[index]!);
      break;
    case "Escape":
    case "Tab":
      closeMenu(event.key === "Escape");
      break;
  }
}

function onPointerDown(event: PointerEvent) {
  if (open.value && !root.value?.contains(event.target as Node)) closeMenu();
}

watch(() => props.disabled, (disabled) => {
  if (disabled) closeMenu();
});

onMounted(() => document.addEventListener("pointerdown", onPointerDown));
onBeforeUnmount(() => document.removeEventListener("pointerdown", onPointerDown));
</script>
