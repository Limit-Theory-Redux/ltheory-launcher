<template>
  <main class="relative isolate h-screen w-screen overflow-hidden bg-void text-ink">
    <GameStartup
      v-if="startupPreview"
      :mode="startupPreview"
      :status="startupStatus"
      :lines="startupLines"
      :error="startupError"
      @dismiss="dismissStartup"
    />

    <template v-else>
    <video
      v-if="dynamicBackground"
      class="absolute inset-0 -z-30 h-full w-full object-cover"
      :src="backgroundVideo"
      autoplay
      loop
      muted
      playsinline
      @loadedmetadata="slowBackgroundVideo"
    />
    <img
      v-else
      class="absolute inset-0 -z-30 h-full w-full object-cover"
      :src="splashImage"
      alt=""
      draggable="false"
    />
    <div
      class="absolute inset-0 -z-20 bg-[radial-gradient(circle_at_76%_20%,rgba(81,217,220,0.1),transparent_34%),linear-gradient(180deg,rgba(3,10,16,0.3)_0%,rgba(3,10,16,0.52)_48%,rgba(3,10,16,0.92)_100%)] backdrop-saturate-[0.86]"
      aria-hidden="true"
    />

    <div class="absolute top-0 right-[126px] left-0 z-30 h-11 select-none" data-tauri-drag-region />
    <div class="absolute top-2 right-2.5 z-40 flex h-[30px] overflow-hidden rounded-lg border border-white/10 bg-black/30 backdrop-blur-xl">
      <button class="grid w-9 cursor-pointer place-items-center border-0 bg-transparent transition hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-inset focus-visible:outline-accent" type="button" aria-label="Minimize" @click="minimizeWindow">
        <svg class="size-3.5 fill-none stroke-current stroke-[1.35] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 16 16" aria-hidden="true"><path d="M3 11.5h10" /></svg>
      </button>
      <button class="grid w-9 cursor-pointer place-items-center border-0 bg-transparent transition hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-inset focus-visible:outline-accent" type="button" :aria-label="windowMaximized ? 'Restore' : 'Maximize'" @click="toggleMaximized">
        <svg class="size-3.5 fill-none stroke-current stroke-[1.35] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 16 16" aria-hidden="true">
          <rect v-if="!windowMaximized" x="3" y="3" width="10" height="10" rx="1" />
          <path v-else d="M5 5V3h8v8h-2M3 5h8v8H3z" />
        </svg>
      </button>
      <button class="grid w-9 cursor-pointer place-items-center border-0 bg-transparent transition hover:bg-[#c73746] focus-visible:outline-2 focus-visible:outline-inset focus-visible:outline-accent" type="button" aria-label="Close" @click="closeWindow">
        <svg class="size-3.5 fill-none stroke-current stroke-[1.35] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 16 16" aria-hidden="true"><path d="m4 4 8 8m0-8-8 8" /></svg>
      </button>
    </div>

    <NuxtPage class="relative" />

    <div class="absolute right-[18px] bottom-4 z-10 max-[820px]:right-auto max-[820px]:bottom-3.5 max-[820px]:left-[15px]">
      <UiToggle v-model="dynamicBackground" label="Animated background" />
    </div>

    <aside
      v-if="launcherUpdateDownloading || launcherUpdateError"
      class="absolute right-[15px] bottom-[13px] left-[15px] z-20 rounded-xl border border-white/15 bg-panel-strong p-3.5 shadow-panel"
      aria-live="polite"
    >
      <div class="mb-2 flex justify-between text-xs text-muted">
        <span>{{ launcherUpdateError || launcherUpdateStatus }}</span>
        <strong v-if="launcherUpdateDownloading && launcherUpdateProgress > 0" class="text-accent">
          {{ launcherUpdateProgress }}%
        </strong>
      </div>
      <UiProgress
        v-if="launcherUpdateDownloading"
        :value="launcherUpdateProgress"
        :indeterminate="launcherUpdateProgress === 0"
        label="Launcher update download"
      />
    </aside>
    </template>
  </main>
</template>

<script lang="ts" setup>
import { getVersion } from "@tauri-apps/api/app";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { confirm } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import backgroundVideo from "./assets/LTR_BG_VID.mp4";
import splashImage from "./assets/LTR_SPLASH.png";
import useBlockContextMenu from "./composables/useBlockContextMenu";
import useBlockFileDrop from "./composables/useBlockFileDrop";

const runningInTauri = isTauri();
const appWindow = runningInTauri ? getCurrentWindow() : undefined;
const dynamicBackground = ref(readBoolean("dynamicBackground", true));
const windowMaximized = ref(false);
const launcherUpdateProgress = ref(0);
const launcherUpdateDownloading = ref(false);
const launcherUpdateStatus = ref("Downloading launcher update…");
const launcherUpdateError = ref("");
const startupPreview = ref<"fullscreen" | "splash" | null>(null);
const startupStatus = ref("Preparing the main menu");
const startupError = ref("");
const startupLines = ref([
  "[engine] Starting PHX runtime",
  "[render] Creating game window and graphics context",
  "[lua] Loading application configuration",
  "[lua] Initializing LimitTheoryRedux state",
  "[audio] Loading music and sound banks",
  "[world] Generating main-menu star system",
  "[ui] Preparing Main_Menu",
]);
const startupUnlisteners: UnlistenFn[] = [];

useBlockFileDrop();
useBlockContextMenu();

onMounted(async () => {
  const params = new URLSearchParams(window.location.search);
  const startupMode = params.get("startupPreview");
  if (startupMode === "fullscreen" || startupMode === "splash") {
    startupPreview.value = startupMode;
    if (params.get("startupError") === "true") {
      startupStatus.value = "Startup failed";
      startupError.value = "The game stopped before startup completed. Your installation was not changed.";
      return;
    }
    if (runningInTauri && params.get("startupLive") === "true") {
      startupLines.value = [];
      startupStatus.value = "Starting Limit Theory Redux";
      startupUnlisteners.push(
        await listen<{ line: string }>("game-launch-output", ({ payload }) => {
          startupLines.value = [...startupLines.value.slice(-99), payload.line];
        }),
        await listen<string>("game-launch-status", ({ payload }) => {
          startupStatus.value = payload;
        }),
        await listen<string>("game-launch-failed", ({ payload }) => {
          startupStatus.value = "Startup failed";
          startupError.value = payload;
        }),
      );

      const state = params.get("state") || "LTheoryRedux";
      try {
        await invoke("launch_game", { state });
      } catch (error) {
        startupStatus.value = "Startup failed";
        startupError.value = typeof error === "string" ? error : "The game could not be started.";
      }
    }
    return;
  }

  if (!runningInTauri) {
    if (new URLSearchParams(window.location.search).get("launcherUpdate") === "true") {
      launcherUpdateDownloading.value = true;
      launcherUpdateProgress.value = 71;
      launcherUpdateStatus.value = "Downloading launcher update…";
    }
    return;
  }

  try {
    windowMaximized.value = await appWindow!.isMaximized();
    await appWindow!.show();
  } catch (error) {
    console.error("Unable to initialize the launcher window", error);
  }

  await checkForUpdate();
});

onUnmounted(() => {
  for (const unlisten of startupUnlisteners) unlisten();
});

watch(dynamicBackground, (enabled) => {
  localStorage.setItem("dynamicBackground", String(enabled));
});

function readBoolean(key: string, fallback: boolean) {
  const value = localStorage.getItem(key);
  return value === null ? fallback : value === "true";
}

function slowBackgroundVideo(event: Event) {
  (event.currentTarget as HTMLVideoElement).playbackRate = 0.25;
}

async function minimizeWindow() {
  if (!runningInTauri) return;
  await appWindow!.minimize();
}

async function toggleMaximized() {
  if (!runningInTauri) return;
  windowMaximized.value = await appWindow!.isMaximized();
  if (windowMaximized.value) {
    await appWindow!.unmaximize();
  } else {
    await appWindow!.maximize();
  }
  windowMaximized.value = !windowMaximized.value;
}

async function closeWindow() {
  if (!runningInTauri) return;
  await appWindow!.close();
}

async function dismissStartup() {
  if (!runningInTauri) return;
  await invoke("dismiss_game_startup");
}

async function checkForUpdate() {
  try {
    const currentVersion = await getVersion();
    const update = await check();
    if (!update) return;

    const confirmed = await confirm(
      `Launcher ${update.version} is available (installed: ${currentVersion}).`,
      {
        title: "Update available",
        okLabel: "Download update",
        cancelLabel: "Later",
      },
    );

    if (!confirmed) return;

    launcherUpdateDownloading.value = true;
    launcherUpdateError.value = "";
    let downloaded = 0;
    let contentLength = 0;

    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength ?? 0;
          launcherUpdateStatus.value = "Downloading launcher update…";
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          launcherUpdateProgress.value = contentLength > 0
            ? Math.min(100, Math.round((downloaded / contentLength) * 100))
            : 0;
          break;
        case "Finished":
          launcherUpdateProgress.value = 100;
          launcherUpdateStatus.value = "Installing launcher update…";
          break;
      }
    });

    await relaunch();
  } catch (error) {
    launcherUpdateDownloading.value = false;
    launcherUpdateError.value = "Launcher update failed. Please try again later.";
    console.error("Launcher update failed", error);
  }
}
</script>
