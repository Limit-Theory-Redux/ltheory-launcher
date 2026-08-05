<template>
  <section class="relative mx-auto grid h-screen w-screen max-w-[1720px] grid-cols-[minmax(0,1fr)_minmax(330px,390px)] items-end gap-[clamp(40px,7vw,110px)] px-[clamp(42px,6vw,104px)] pt-[clamp(76px,9vh,118px)] pb-[clamp(64px,8vh,90px)] max-[820px]:flex max-[820px]:flex-col max-[820px]:items-center max-[820px]:gap-0 max-[820px]:px-6 max-[820px]:pt-[60px] max-[820px]:pb-[54px] max-[560px]:px-[18px] [@media(max-height:680px)]:pt-[54px] [@media(max-height:680px)]:pb-11">
    <header class="pointer-events-none absolute top-[clamp(64px,8vh,90px)] left-[clamp(42px,6vw,104px)] z-[35] select-none max-[820px]:top-[62px] max-[820px]:right-5 max-[820px]:left-5 max-[820px]:flex max-[820px]:flex-col max-[820px]:items-center">
      <img class="w-[clamp(420px,36vw,600px)] drop-shadow-[0_10px_24px_rgba(0,0,0,0.58)] max-[820px]:w-[min(82vw,360px)]" :src="title" alt="Limit Theory Redux" draggable="false" />
      <div class="mt-3 flex items-center gap-2.5">
        <span class="h-4 w-0.5 bg-accent/75" aria-hidden="true" />
        <span class="text-[10px] font-semibold tracking-[0.18em] text-white/60 uppercase max-[560px]:text-[9px]">Early development build</span>
      </div>

      <nav class="pointer-events-auto mt-5 flex items-center gap-4" aria-label="Project links">
        <button class="inline-flex min-h-7 items-center gap-2 text-[10px] font-semibold tracking-[0.08em] text-white/45 uppercase transition hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-accent" type="button" @click="openExternal(SOURCE_URL)">
          <svg class="size-3.5 fill-current" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M12 2.4a9.8 9.8 0 0 0-3.1 19.1c.5.1.7-.2.7-.5v-1.9c-2.8.6-3.4-1.2-3.4-1.2-.5-1.2-1.1-1.5-1.1-1.5-.9-.6.1-.6.1-.6 1 0 1.5 1 1.5 1 .9 1.5 2.3 1.1 2.9.8.1-.6.4-1.1.6-1.3-2.2-.3-4.6-1.1-4.6-4.9 0-1.1.4-2 1-2.6-.1-.3-.4-1.3.1-2.6 0 0 .8-.3 2.7 1a9.3 9.3 0 0 1 4.9 0c1.9-1.3 2.7-1 2.7-1 .5 1.3.2 2.3.1 2.6.6.7 1 1.5 1 2.6 0 3.8-2.3 4.6-4.6 4.9.4.3.7.9.7 1.8V21c0 .3.2.6.7.5A9.8 9.8 0 0 0 12 2.4Z" />
          </svg>
          GitHub
          <svg class="size-3 fill-none stroke-current stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 16 16" aria-hidden="true"><path d="M6 3h7v7M13 3 5 11M11 8v4a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h4" /></svg>
        </button>
        <span class="h-3 w-px bg-white/15" aria-hidden="true" />
        <button class="inline-flex min-h-7 items-center gap-2 text-[10px] font-semibold tracking-[0.08em] text-white/45 uppercase transition hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-accent" type="button" @click="openExternal(DISCORD_URL)">
          <svg class="size-3.5 fill-current" viewBox="0 0 24 24" aria-hidden="true"><path d="M19.5 5.3A18 18 0 0 0 15 3.9l-.6 1.2a16.6 16.6 0 0 0-4.9 0l-.6-1.2a18 18 0 0 0-4.5 1.4C1.6 9.5.8 13.6 1.2 17.7a18.2 18.2 0 0 0 5.5 2.8l1.3-1.8c-.7-.3-1.4-.7-2-1.2l.5-.4a13 13 0 0 0 11 0l.6.4c-.7.5-1.4.9-2.1 1.2l1.3 1.8a18.2 18.2 0 0 0 5.5-2.8c.5-4.8-.8-8.8-3.3-12.4ZM8.3 15.2c-1.1 0-2-1-2-2.2 0-1.2.9-2.2 2-2.2s2 1 2 2.2c0 1.2-.9 2.2-2 2.2Zm7.4 0c-1.1 0-2-1-2-2.2 0-1.2.9-2.2 2-2.2s2 1 2 2.2c0 1.2-.9 2.2-2 2.2Z" /></svg>
          Discord
        </button>
      </nav>
    </header>

    <div class="z-[35] flex max-w-[610px] flex-col items-start self-end pb-3 text-left max-[820px]:hidden">

      <div>
        <span class="text-[10px] font-bold tracking-[0.2em] text-accent uppercase">Open-world space simulation</span>
        <h1 class="mt-3 max-w-[570px] text-[clamp(36px,4vw,66px)] leading-[0.98] font-semibold tracking-[-0.045em] text-[#f2f8fb] text-balance drop-shadow-[0_8px_22px_rgba(0,0,0,0.62)] [@media(max-height:680px)]:max-w-[470px] [@media(max-height:680px)]:text-[clamp(30px,3.5vw,48px)]">Build your future among the stars.</h1>
        <p class="mt-[18px] max-w-[560px] text-[clamp(12px,1vw,15px)] leading-relaxed text-white/70 [@media(max-height:680px)]:mt-2 [@media(max-height:680px)]:leading-[1.35]">Explore a living universe. Build, trade, fight, and shape your own future among the stars.</p>
      </div>

    </div>

    <article class="launcher-panel relative w-full max-w-[390px] justify-self-end overflow-hidden rounded-[14px] border border-white/15 bg-[linear-gradient(155deg,rgba(12,28,40,0.93),rgba(5,14,22,0.97))] p-5 shadow-[0_26px_72px_rgba(0,0,0,0.38)] backdrop-blur-2xl before:absolute before:inset-x-0 before:top-0 before:h-px before:bg-gradient-to-r before:from-transparent before:via-accent/45 before:to-transparent max-[820px]:mt-auto max-[820px]:max-w-[420px] max-[820px]:p-[17px] max-[560px]:w-full min-[1500px]:max-w-[420px] min-[1500px]:p-[23px] [@media(max-height:680px)]:p-[15px]">
      <div class="mb-4 flex items-center justify-between border-b border-white/[0.07] pb-3 text-[9px] font-bold tracking-[0.12em] text-white/45 uppercase [@media(max-height:680px)]:mb-2.5 [@media(max-height:680px)]:pb-2">
        <span>{{ settingsOpen ? "Launcher settings" : "Status" }}</span>
        <div class="flex items-center gap-2">
          <span v-if="!settingsOpen" class="text-white/75">{{ gameDataLoading ? "Checking" : gameInstalled ? "Installed" : "Available" }}</span>
        <button
          class="group -m-1 inline-flex size-7 items-center justify-center rounded-md border border-transparent text-white/45 transition hover:border-white/10 hover:bg-white/[0.06] hover:text-white/85 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
          type="button"
          :aria-label="settingsOpen ? 'Close launcher settings' : 'Open launcher settings'"
          :title="settingsOpen ? 'Close settings' : 'Launcher settings'"
          @click="settingsOpen = !settingsOpen"
        >
          <svg v-if="settingsOpen" class="size-4 fill-none stroke-current stroke-[1.7] [stroke-linecap:round]" viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10M15 5 5 15" /></svg>
          <svg v-else class="size-4 fill-none stroke-current stroke-[1.45] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 20 20" aria-hidden="true"><circle cx="10" cy="10" r="2.5" /><path d="M16.4 11.2v-2.4l-1.8-.5a5 5 0 0 0-.5-1.1l.9-1.7-1.7-1.7-1.7.9a5 5 0 0 0-1.1-.5L10 2.4H7.6l-.5 1.8a5 5 0 0 0-1.1.5l-1.7-.9-1.7 1.7.9 1.7A5 5 0 0 0 3 8.3l-1.8.5v2.4l1.8.5a5 5 0 0 0 .5 1.1l-.9 1.7 1.7 1.7 1.7-.9a5 5 0 0 0 1.1.5l.5 1.8H10l.5-1.8a5 5 0 0 0 1.1-.5l1.7.9 1.7-1.7-.9-1.7a5 5 0 0 0 .5-1.1l1.8-.5Z" transform="translate(1.2 0) scale(.88)" /></svg>
        </button>
        </div>
      </div>

      <div v-if="settingsOpen" class="min-h-[268px]">
        <div>
          <h2 class="text-[15px] font-semibold tracking-[-0.01em] text-ink">Game location</h2>
          <p class="mt-1 text-[10px] leading-relaxed text-muted">Point the launcher to an existing installation, or choose a new location when installing.</p>
        </div>

        <div class="mt-4 rounded-xl border border-white/[0.09] bg-black/20 p-3">
          <div class="flex items-center gap-2 text-[9px] font-bold tracking-[0.09em] text-white/40 uppercase">
            <span class="size-1.5 rounded-full" :class="gameInstalled ? 'bg-positive' : 'bg-white/25'" aria-hidden="true" />
            Active game folder
          </div>
          <p class="mt-2 break-all text-[10px] leading-[1.45] text-white/75">{{ gamePath || "No game folder configured" }}</p>
        </div>

        <div class="mt-3 grid gap-2">
          <UiButton variant="secondary" block :disabled="gameDataLoading || gameLaunching || gameDownloadUpdateInstalling" @click="locateExistingGame">
            <template #icon><svg viewBox="0 0 20 20"><path d="M3 6.5h5l1.5 1.7H17v7.3H3v-9Z" /></svg></template>
            {{ gameInstalled ? "Use another game folder" : "Locate existing game" }}
          </UiButton>
          <UiButton variant="ghost" block :disabled="gameDataLoading || gameLaunching || gameDownloadUpdateInstalling" @click="installGame">
            Choose location and install
          </UiButton>
        </div>

        <div class="mt-4 border-t border-white/[0.07] pt-3">
          <div class="flex items-start justify-between gap-3">
            <div>
              <h3 class="text-[11px] font-semibold text-white/80">Game diagnostics</h3>
              <p class="mt-1 text-[9px] leading-relaxed text-muted">Copy all output from the most recent game session, including startup and crash details.</p>
            </div>
          </div>
          <UiButton
            class="mt-2.5"
            variant="ghost"
            block
            :disabled="!lastGameLogAvailable || gameLaunching"
            @click="copyLastGameLog"
          >
            {{ lastGameLogCopyLabel }}
          </UiButton>
        </div>

        <p v-if="settingsError" class="mt-3 rounded-lg border border-danger/20 bg-danger/[0.07] px-2.5 py-2 text-[10px] leading-[1.4] text-danger" role="alert">{{ settingsError }}</p>
      </div>

      <template v-else>
      <div class="flex min-h-[38px] items-center gap-2.5">
        <span class="size-2.5 shrink-0 rounded-full" :class="statusTone" aria-hidden="true" />
        <div class="flex min-w-0 flex-1 flex-col gap-0.5">
          <strong class="text-[13px] leading-tight">{{ statusTitle }}</strong>
          <span class="truncate text-[11px] text-muted">{{ statusDetail }}</span>
        </div>
        <span v-if="gameUpdateAvailable" class="rounded-full border border-positive/25 bg-positive/10 px-2 py-1 text-[9px] font-bold tracking-[0.05em] text-positive uppercase">Update ready</span>
      </div>

      <div v-if="!gameDataLoading && !gameInstalled" class="mt-4 border-t border-white/[0.07] pt-3">
        <span class="text-[9px] font-bold tracking-[0.12em] text-accent uppercase">First-time setup</span>
        <p class="mt-1.5 text-[10px] leading-relaxed text-muted">Install the latest build, or connect an existing Limit Theory Redux folder.</p>
      </div>

      <div class="mt-4 [@media(max-height:680px)]:mt-3">
        <UiButton
          block
          :loading="gameDownloadUpdateInstalling || gameLaunching"
          :disabled="gameDataLoading || gameDownloadUpdateInstalling || gameLaunching"
          @click="primaryAction"
        >
          <template #icon>
            <svg v-if="gameInstalled" viewBox="0 0 20 20"><path d="m7 4 9 6-9 6V4Z" /></svg>
            <svg v-else viewBox="0 0 20 20"><path d="M10 3v10m-4-4 4 4 4-4M4 16h12" /></svg>
          </template>
          {{ primaryActionLabel }}
        </UiButton>

        <div class="mt-2 grid grid-cols-2 gap-2 [&>:only-child]:col-span-2">
          <UiButton
            v-if="gameUpdateAvailable"
            variant="success"
            :disabled="gameDataLoading || gameDownloadUpdateInstalling || gameLaunching"
            @click="installGameUpdate"
          >
            Update game
          </UiButton>
          <UiButton
            v-if="gameInstalled"
            variant="secondary"
            :disabled="gameDataLoading || !gameInstalled || !configFound || gameDownloadUpdateInstalling || gameLaunching"
            @click="openConfig"
          >
            Game config
          </UiButton>
          <UiButton
            v-if="!gameInstalled"
            variant="secondary"
            :disabled="gameDataLoading || gameDownloadUpdateInstalling || gameLaunching"
            @click="locateExistingGame"
          >
            Locate existing
          </UiButton>
        </div>
      </div>

      <UiSelect
        v-if="!gameDataLoading && gameInstalled"
        v-model="gameSelectedState"
        label="Launch state"
        :options="gameAvailableStates"
        :disabled="gameDownloadUpdateInstalling || gameLaunching"
      />

      <div v-if="gameDownloadUpdateInstalling" class="mt-3 border-t border-white/[0.07] pt-3" aria-live="polite">
        <div class="mb-2 flex justify-between text-[10px] text-muted">
          <span>{{ downloadStatus }}</span>
          <strong v-if="!gameDownloadUpdateExtracting" class="text-accent">{{ gameDownloadUpdateProgress.toFixed(0) }}%</strong>
        </div>
        <UiProgress
          :value="gameDownloadUpdateProgress"
          :indeterminate="gameDownloadUpdateExtracting"
          label="Game installation progress"
        />
      </div>

      <p v-if="operationError" class="mt-3 rounded-lg border border-danger/20 bg-danger/[0.07] px-2.5 py-2 text-[10px] leading-[1.4] text-danger" role="alert">{{ operationError }}</p>

      <dl class="mt-3.5 flex items-center border-t border-white/[0.07] pt-3 text-[9px] [@media(max-height:680px)]:mt-2.5">
        <div class="flex min-w-0 flex-1 items-center gap-2 border-r border-white/10 pr-3">
          <dt class="text-white/40 uppercase tracking-[0.07em]">Game</dt>
          <dd class="ml-auto truncate text-white/75">{{ gameDataLoading ? "Checking…" : gameVersion || "Not installed" }}</dd>
        </div>
        <div class="flex min-w-0 flex-1 items-center gap-2 pl-3">
          <dt class="text-white/40 uppercase tracking-[0.07em]">Launcher</dt>
          <dd class="ml-auto truncate text-white/75">{{ appVersion }}</dd>
        </div>
      </dl>
      </template>
    </article>
  </section>
</template>

<script lang="ts" setup>
import { getVersion } from "@tauri-apps/api/app";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { confirm, open } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import title from "../assets/LTR_Title.svg";
import type { ReleaseInfo } from "../types";

interface GameInfo {
  installed: boolean;
  version: string | null;
  states: string[];
}

interface NumericEvent {
  payload: number;
}

const SOURCE_URL = "https://github.com/Limit-Theory-Redux/ltheory";
const DISCORD_URL = "https://discord.gg/MrfRR5ytJF";
const RELEASE_API_URL = "https://api.github.com/repos/Limit-Theory-Redux/ltheory/releases?per_page=1";

const appVersion = ref("—");
const gameVersion = ref("");
const gamePath = ref("");
const gameInstalled = ref(false);
const gameAvailableStates = ref(["LTheoryRedux"]);
const gameSelectedState = ref("LTheoryRedux");
const gameDownloadUpdateProgress = ref(0);
const gameDownloadUpdateSpeed = ref(0);
const gameDownloadUpdateInstalling = ref(false);
const gameDownloadUpdateExtracting = ref(false);
const gameDownloadUpdateExtractingFilesRemaining = ref(0);
const gameLaunching = ref(false);
const gameUpdateAvailable = ref(false);
const configFound = ref(false);
const operationError = ref("");
const settingsError = ref("");
const settingsOpen = ref(false);
const lastGameLogAvailable = ref(false);
const lastGameLogCopyState = ref<"idle" | "copied" | "failed">("idle");
const gameDataLoading = ref(true);
const latestRelease = shallowRef<ReleaseInfo | null>(null);
const unlisteners: UnlistenFn[] = [];
const runningInTauri = isTauri();
let lastGameLogCopyResetTimer: ReturnType<typeof setTimeout> | undefined;

const lastGameLogCopyLabel = computed(() => {
  if (lastGameLogCopyState.value === "copied") return "Copied game log";
  if (lastGameLogCopyState.value === "failed") return "Could not copy log";
  return lastGameLogAvailable.value ? "Copy last game log" : "No game log available";
});

const primaryActionLabel = computed(() => {
  if (gameDataLoading.value) return "Checking installation…";
  if (gameLaunching.value) return "Starting game…";
  if (gameDownloadUpdateExtracting.value) return "Installing files…";
  if (gameDownloadUpdateInstalling.value) return "Downloading…";
  return gameInstalled.value ? "Launch game" : "Install game";
});
const statusTitle = computed(() => {
  if (gameDataLoading.value) return "Checking installation";
  if (operationError.value) return "Action needed";
  if (gameDownloadUpdateExtracting.value) return "Installing files";
  if (gameDownloadUpdateInstalling.value) return "Downloading game";
  if (gameUpdateAvailable.value) return "Update available";
  if (gameInstalled.value) return "Ready to launch";
  return "Game not installed";
});
const statusDetail = computed(() => {
  if (gameDataLoading.value) return "Reading the existing game location";
  if (operationError.value) return "The last operation did not complete";
  if (gameDownloadUpdateInstalling.value) return downloadStatus.value;
  if (gameInstalled.value) return `Version ${gameVersion.value || "unknown"}`;
  return "Choose an install location to get started";
});
const statusTone = computed(() => {
  if (gameDataLoading.value) return "animate-soft-pulse bg-accent shadow-[0_0_14px_rgba(114,215,239,0.62)]";
  if (operationError.value) {
    return "bg-danger shadow-[0_0_0_4px_rgba(255,127,142,0.12),0_0_14px_rgba(255,127,142,0.4)]";
  }
  if (gameDownloadUpdateInstalling.value) {
    return "animate-soft-pulse bg-accent shadow-[0_0_0_4px_rgba(114,215,239,0.12),0_0_14px_rgba(114,215,239,0.45)]";
  }
  if (gameInstalled.value) {
    return "bg-positive shadow-[0_0_0_4px_rgba(125,227,161,0.12),0_0_14px_rgba(125,227,161,0.45)]";
  }
  return "bg-[#70818c] shadow-[0_0_0_4px_rgba(112,129,140,0.12)]";
});
const downloadStatus = computed(() => {
  if (gameDownloadUpdateExtracting.value) {
    return `Extracting ${gameDownloadUpdateExtractingFilesRemaining.value} files`;
  }
  return gameDownloadUpdateSpeed.value > 0
    ? `Downloading at ${gameDownloadUpdateSpeed.value.toFixed(1)} MB/s`
    : "Starting download…";
});

onMounted(async () => {
  if (!runningInTauri) {
    applyPreviewState(new URLSearchParams(window.location.search).get("previewState") || "ready");
    return;
  }

  appVersion.value = await getVersion();
  await registerDownloadListeners();
  unlisteners.push(
    await listen("game-launch-reset", async () => {
      gameLaunching.value = false;
      await refreshLastGameLogAvailability();
    }),
    await listen<number | null>("game-launch-exited", async (event) => {
      if (event.payload !== null && event.payload !== 0) {
        operationError.value = `The game exited with code ${event.payload}. The full log is available in settings.`;
      }
      await refreshLastGameLogAvailability();
    }),
    await listen<string>("game-launch-failed", async () => {
      await refreshLastGameLogAvailability();
    }),
  );
  await loadGameData();
  await checkConfigExists();
  await refreshLastGameLogAvailability();
});

function applyPreviewState(state: string) {
  gameDataLoading.value = false;
  appVersion.value = "0.6.1";
  gameVersion.value = "0.9.4";
  gamePath.value = "C:\\Games\\Limit Theory Redux";
  gameInstalled.value = true;
  gameAvailableStates.value = ["LTheoryRedux", "Sandbox", "FleetTest"];
  gameSelectedState.value = "LTheoryRedux";
  configFound.value = true;
  gameUpdateAvailable.value = false;
  gameDownloadUpdateInstalling.value = false;
  gameDownloadUpdateExtracting.value = false;
  gameDownloadUpdateProgress.value = 0;
  gameDownloadUpdateSpeed.value = 0;
  operationError.value = "";

  switch (state) {
    case "uninstalled":
      gameInstalled.value = false;
      gameVersion.value = "";
      configFound.value = false;
      break;
    case "update":
      gameUpdateAvailable.value = true;
      break;
    case "downloading":
      gameDownloadUpdateInstalling.value = true;
      gameDownloadUpdateProgress.value = 64;
      gameDownloadUpdateSpeed.value = 18.6;
      break;
    case "extracting":
      gameDownloadUpdateInstalling.value = true;
      gameDownloadUpdateExtracting.value = true;
      gameDownloadUpdateProgress.value = 100;
      gameDownloadUpdateExtractingFilesRemaining.value = 428;
      break;
    case "error":
      operationError.value = "The download could not be verified. Your existing installation was left unchanged.";
      break;
  }
}

onUnmounted(() => {
  for (const unlisten of unlisteners) unlisten();
  if (lastGameLogCopyResetTimer) clearTimeout(lastGameLogCopyResetTimer);
});

async function refreshLastGameLogAvailability() {
  try {
    await invoke<string>("get_last_game_launch_log");
    lastGameLogAvailable.value = true;
  } catch {
    lastGameLogAvailable.value = false;
  }
}

async function copyLastGameLog() {
  settingsError.value = "";
  try {
    const log = await invoke<string>("get_last_game_launch_log");
    await writeText(log);
    lastGameLogAvailable.value = true;
    lastGameLogCopyState.value = "copied";
  } catch (error) {
    lastGameLogCopyState.value = "failed";
    settingsError.value = "The most recent game log could not be copied.";
    console.error("Unable to copy the most recent game log", error);
  }

  if (lastGameLogCopyResetTimer) clearTimeout(lastGameLogCopyResetTimer);
  lastGameLogCopyResetTimer = setTimeout(() => {
    lastGameLogCopyState.value = "idle";
  }, 2200);
}

async function registerDownloadListeners() {
  unlisteners.push(
    await listen<NumericEvent["payload"]>("download-progress", (event) => {
      gameDownloadUpdateProgress.value = event.payload;
    }),
    await listen<NumericEvent["payload"]>("download-speed", (event) => {
      gameDownloadUpdateSpeed.value = Math.floor((event.payload / 1024) * 10) / 10;
    }),
    await listen("download-extracting", () => {
      gameDownloadUpdateExtracting.value = true;
    }),
    await listen<NumericEvent["payload"]>("extracting-files", (event) => {
      gameDownloadUpdateExtractingFilesRemaining.value = event.payload;
    }),
    await listen("install-complete", async () => {
      gameDownloadUpdateInstalling.value = false;
      gameDownloadUpdateExtracting.value = false;
      gameDownloadUpdateProgress.value = 100;
      await loadGameData();
      await checkConfigExists();
    }),
    await listen<string>("install-error", (event) => {
      gameDownloadUpdateInstalling.value = false;
      gameDownloadUpdateExtracting.value = false;
      operationError.value = event.payload || "Installation failed.";
    }),
  );
}

async function loadGameData() {
  gameDataLoading.value = true;
  try {
    const info = await invoke<GameInfo>("get_game_info");
    gameInstalled.value = info.installed;
    gameVersion.value = info.version || "";
    gameAvailableStates.value = info.states.length ? info.states : ["LTheoryRedux"];
    if (!gameAvailableStates.value.includes(gameSelectedState.value)) {
      gameSelectedState.value = gameAvailableStates.value[0] ?? "LTheoryRedux";
    }

    if (info.installed && info.version) void checkUpdateAvailable();
    if (info.installed) gamePath.value = await invoke<string>("get_installation_path");
  } catch (error) {
    operationError.value = "Could not read the current game installation.";
    console.error("Unable to load game data", error);
  } finally {
    gameDataLoading.value = false;
  }
}

async function checkConfigExists() {
  try {
    configFound.value = await invoke<boolean>("check_config_exists");
  } catch {
    configFound.value = false;
  }
}

async function checkUpdateAvailable() {
  try {
    const release = await getLatestRelease();
    const nightlyVersion = release.name.match(/^Nightly\s+(.+)$/i)?.[1];
    const latestVersion = (nightlyVersion || release.tag_name).replace(/^v/i, "");
    gameUpdateAvailable.value = Boolean(latestVersion && latestVersion !== gameVersion.value.replace(/^v/i, ""));
  } catch (error) {
    gameUpdateAvailable.value = false;
    console.error("Unable to check for a game update", error);
  }
}

async function getLatestRelease() {
  if (latestRelease.value) return latestRelease.value;

  const response = await fetch(RELEASE_API_URL, { headers: { Accept: "application/vnd.github+json" } });
  if (!response.ok) throw new Error(`GitHub returned ${response.status}`);
  const releases = await response.json() as ReleaseInfo[];
  const release = releases[0];
  if (!release?.tag_name) throw new Error("GitHub did not return a published game release");
  latestRelease.value = release;
  return release;
}

async function openExternal(url: typeof SOURCE_URL | typeof DISCORD_URL) {
  await openUrl(url);
}

async function openConfig() {
  operationError.value = "";
  try {
    await invoke("open_config");
  } catch (error) {
    operationError.value = "The game configuration could not be opened.";
    console.error("Unable to open config", error);
  }
}

async function primaryAction() {
  if (gameInstalled.value) await launchGame();
  else await installGame();
}

async function installGame() {
  operationError.value = "";
  settingsError.value = "";
  const selected = await pickGameFolder("Select installation folder");
  if (typeof selected !== "string") return;

  const confirmed = await confirm(
    `Install Limit Theory Redux in ${selected}\\Limit Theory Redux?`,
    { title: "Confirm installation", okLabel: "Install", cancelLabel: "Cancel" },
  );
  if (!confirmed) return;

  settingsOpen.value = false;
  await startInstall(selected, false);
}

async function locateExistingGame() {
  operationError.value = "";
  settingsError.value = "";
  const selected = await pickGameFolder(
    "Select the Limit Theory Redux game folder",
    gamePath.value || undefined,
  );
  if (typeof selected !== "string") return;

  try {
    gamePath.value = await invoke<string>("set_game_installation_path", { installPath: selected });
    await loadGameData();
    await checkConfigExists();
    settingsOpen.value = false;
  } catch (error) {
    const message = typeof error === "string" ? error : "The selected folder could not be used.";
    settingsError.value = message;
    operationError.value = message;
  }
}

async function pickGameFolder(title: string, defaultPath?: string) {
  try {
    return await open({
      title,
      multiple: false,
      directory: true,
      ...(defaultPath ? { defaultPath } : {}),
    });
  } catch (error) {
    const message = "The native folder picker could not be opened.";
    settingsError.value = message;
    operationError.value = message;
    console.error("Unable to open the native game folder picker", error);
    return null;
  }
}

async function installGameUpdate() {
  await startInstall(null, true);
}

async function startInstall(installPath: string | null, updateExisting: boolean) {
  operationError.value = "";
  gameDownloadUpdateProgress.value = 0;
  gameDownloadUpdateInstalling.value = true;
  gameDownloadUpdateExtracting.value = false;
  try {
    const release = await getLatestRelease();
    await invoke("download_game", { installPath, releaseTag: release.tag_name, updateExisting });
  } catch (error) {
    gameDownloadUpdateInstalling.value = false;
    operationError.value = error instanceof Error
      ? error.message
      : typeof error === "string" ? error : "Installation failed.";
    console.error("Unable to install game", error);
  }
}

async function launchGame() {
  if (gameLaunching.value) return;
  operationError.value = "";
  gameLaunching.value = true;
  try {
    await invoke("prepare_game_launch", { state: gameSelectedState.value });
  } catch (error) {
    gameLaunching.value = false;
    operationError.value = typeof error === "string" ? error : "The game could not be launched.";
    console.error("Unable to launch game", error);
  }
}
</script>
