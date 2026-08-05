<template>
  <section class="fixed inset-0 z-[100] isolate overflow-hidden bg-[#050b11] text-ink">
    <img class="absolute inset-0 z-0 h-full w-full object-cover" :src="splashImage" alt="" draggable="false" />
    <div
      class="absolute inset-0 z-[1]"
      :class="mode === 'fullscreen'
        ? 'bg-[radial-gradient(circle_at_72%_38%,rgba(48,153,170,0.12),transparent_34%),linear-gradient(105deg,rgba(2,8,13,0.96)_0%,rgba(2,8,13,0.72)_48%,rgba(2,8,13,0.88)_100%)]'
        : 'bg-[linear-gradient(145deg,rgba(3,10,16,0.78),rgba(3,10,16,0.94))]'"
      aria-hidden="true"
    />

    <div v-if="mode === 'fullscreen'" class="relative z-10 mx-auto grid h-full max-w-[1640px] grid-cols-[minmax(0,1fr)_minmax(480px,600px)] items-end gap-[8vw] px-[7vw] pt-[8vh] pb-[9vh]">
      <div class="pb-2">
        <img class="w-[clamp(320px,27vw,440px)] drop-shadow-[0_10px_24px_rgba(0,0,0,0.65)]" :src="title" alt="Limit Theory Redux" draggable="false" />
        <div class="mt-4 flex items-center gap-2.5">
          <span class="h-4 w-0.5 bg-accent/75" aria-hidden="true" />
          <span class="text-[10px] font-semibold tracking-[0.18em] text-white/60 uppercase">Early development build</span>
        </div>
        <p class="mt-12 text-[10px] font-bold tracking-[0.2em] text-accent uppercase">Starting game</p>
        <h1 class="mt-3 max-w-[700px] text-[clamp(48px,5vw,78px)] leading-[0.96] font-semibold tracking-[-0.05em] text-balance">Preparing the universe.</h1>
        <p class="mt-5 max-w-[570px] text-sm leading-relaxed text-white/60">Limit Theory Redux is initializing its engine, scripts, and main-menu simulation. The game will come forward automatically when it is ready.</p>
      </div>

      <div class="overflow-hidden rounded-2xl border border-white/15 bg-[#07141f]/90 shadow-[0_28px_90px_rgba(0,0,0,0.5)] backdrop-blur-2xl">
        <header class="flex items-center justify-between border-b border-white/[0.08] px-5 py-4">
          <div class="flex items-center gap-3">
            <span class="size-2 rounded-full" :class="error ? 'bg-danger shadow-[0_0_14px_rgba(255,127,142,0.65)]' : 'animate-soft-pulse bg-accent shadow-[0_0_14px_rgba(114,215,239,0.7)]'" />
            <div>
              <p class="m-0 text-[13px] font-bold">{{ status }}</p>
              <p class="m-0 mt-0.5 text-[10px] text-muted">Live startup output</p>
            </div>
          </div>
          <span class="text-[9px] font-bold tracking-[0.12em] text-white/40 uppercase">Process active</span>
        </header>
        <ol class="h-[300px] space-y-2 overflow-hidden px-5 py-5 font-mono text-[11px] leading-relaxed">
          <li v-for="(line, index) in visibleLines" :key="`${index}-${line}`" class="flex gap-3" :class="index === visibleLines.length - 1 ? 'text-white/90' : 'text-white/40'">
            <span class="select-none text-accent/55">{{ String(index + 1).padStart(2, '0') }}</span>
            <span class="min-w-0 break-words">{{ line }}</span>
          </li>
        </ol>
        <div class="px-5 pb-5">
          <UiProgress v-if="!error" :value="0" indeterminate label="Game startup in progress" />
          <p class="mt-3 text-[10px]" :class="error ? 'text-danger' : 'text-white/35'">{{ error || "Do not close the launcher while the game is initializing." }}</p>
          <div v-if="error" class="mt-4 grid grid-cols-2 gap-2">
            <UiButton variant="secondary" @click="copyErrorLog">{{ copyLabel }}</UiButton>
            <UiButton variant="secondary" @click="emit('dismiss')">Return to launcher</UiButton>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="relative z-10 flex h-full flex-col p-7">
      <div :key="error ? 'failure-header' : 'startup-header'" class="flex items-start justify-between gap-6">
        <div>
          <img class="w-[270px] drop-shadow-[0_8px_20px_rgba(0,0,0,0.65)]" :src="title" alt="Limit Theory Redux" draggable="false" />
          <div class="mt-3 flex items-center gap-2">
            <span class="h-3.5 w-0.5 bg-accent/75" aria-hidden="true" />
            <span class="text-[9px] font-semibold tracking-[0.16em] text-white/60 uppercase">Early development build</span>
          </div>
        </div>
        <span class="mt-1 text-[9px] font-bold tracking-[0.14em] text-white/35 uppercase">Starting game</span>
      </div>

      <div class="mt-auto">
        <div class="mb-4 flex items-center gap-3">
          <span class="size-2.5 rounded-full" :class="error ? 'bg-danger shadow-[0_0_16px_rgba(255,127,142,0.65)]' : 'animate-soft-pulse bg-accent shadow-[0_0_16px_rgba(114,215,239,0.72)]'" />
          <div>
            <h1 class="m-0 text-[21px] font-semibold tracking-[-0.025em]">{{ status }}</h1>
            <p class="m-0 mt-0.5 text-[10px]" :class="error ? 'text-danger' : 'text-muted'">{{ error || "The game will open automatically when ready." }}</p>
          </div>
        </div>
        <UiProgress v-if="!error" :value="0" indeterminate label="Game startup in progress" />
        <ol class="mt-4 h-[94px] space-y-1.5 overflow-hidden rounded-lg border border-white/[0.08] bg-black/25 px-3.5 py-3 font-mono text-[9px] leading-relaxed">
          <li v-for="(line, index) in compactLines" :key="`${index}-${line}`" class="truncate" :class="index === compactLines.length - 1 ? 'text-white/85' : 'text-white/35'">
            <span class="mr-2 text-accent/55">›</span>{{ line }}
          </li>
        </ol>
        <div v-if="error" class="mt-3 grid grid-cols-2 gap-2">
          <UiButton variant="secondary" @click="copyErrorLog">{{ copyLabel }}</UiButton>
          <UiButton variant="secondary" @click="emit('dismiss')">Return to launcher</UiButton>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { invoke, isTauri } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { computed, onBeforeUnmount, ref } from "vue";
import splashImage from "../assets/LTR_SPLASH.png";
import title from "../assets/LTR_Title.svg";

const props = withDefaults(
  defineProps<{
    mode?: "fullscreen" | "splash";
    status?: string;
    lines?: string[];
    error?: string;
  }>(),
  {
    mode: "splash",
    status: "Loading game systems",
    lines: () => [],
    error: "",
  },
);

const emit = defineEmits<{ dismiss: [] }>();

const visibleLines = computed(() => props.lines.slice(-9));
const compactLines = computed(() => props.lines.slice(-4));
const copyState = ref<"idle" | "copied" | "failed">("idle");
const copyLabel = computed(() => {
  if (copyState.value === "copied") return "Copied log";
  if (copyState.value === "failed") return "Copy failed";
  return "Copy error log";
});
let copyResetTimer: ReturnType<typeof setTimeout> | undefined;

async function copyErrorLog() {
  const fallbackLog = [
    "Limit Theory Redux startup failure",
    `Error: ${props.error || "Unknown startup error"}`,
    "",
    "Startup output:",
    ...(props.lines.length ? props.lines : ["(no startup output captured)"]),
  ].join("\n");

  try {
    const log = isTauri()
      ? await invoke<string>("get_last_game_launch_log").catch(() => fallbackLog)
      : fallbackLog;
    await writeText(log);
    copyState.value = "copied";
  } catch (error) {
    console.error("Unable to copy the startup error log", error);
    copyState.value = "failed";
  }

  if (copyResetTimer) clearTimeout(copyResetTimer);
  copyResetTimer = setTimeout(() => {
    copyState.value = "idle";
  }, 2200);
}

onBeforeUnmount(() => {
  if (copyResetTimer) clearTimeout(copyResetTimer);
});
</script>
