<template>
  <div class="flex flex-col items-center w-screen h-screen">
    <img
      src="/assets/LTR_Logo.svg"
      class="mt-auto mb-4 w-32 drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)] noselect"
      draggable="false"
    />
    <img
      src="/assets/LTR_Title.svg"
      class="mb-4 w-[22rem] drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)] noselect"
      draggable="false"
    />
    <div
      class="flex flex-col mb-4 drop-shadow-[0_1px_2px_rgba(0,0,0,0.7)] text-white font-semibold"
    >
      <a
        href="https://github.com/Limit-Theory-Redux/ltheory"
        target="_blank"
        draggable="false"
        class="mb-2"
      >
        <div
          class="flex flex-row noselect border-white border-solid border-2 py-1 px-8 rounded-3xl"
        >
          <v-icon class="i-mdi:github" />
          <p class="ml-4">Source</p>
        </div>
      </a>
      <a href="https://discord.gg/MrfRR5ytJF" target="_blank" draggable="false">
        <div
          class="flex flex-row noselect border-white border-solid border-2 py-1 px-8 rounded-3xl"
          draggable="false"
        >
          <v-icon class="i-bi:discord" />
          <p class="ml-4">Discord</p>
        </div>
      </a>
    </div>
    <v-btn
      v-if="!gameInstalled && !gameDownloadUpdateInstalling"
      class="mb-auto w-48"
      size="large"
      @click="installGame()"
      :disabled="gameDownloadUpdateInstalling"
      >Install</v-btn
    >
    <v-btn
      v-else-if="!gameInstalled && gameDownloadUpdateInstalling"
      class="mb-2 w-48"
      size="large"
      @click="installGame()"
      :disabled="gameDownloadUpdateInstalling"
      >Install</v-btn
    >
    <v-btn
      v-else
      class="mb-2 w-48"
      size="large"
      :disabled="!gameInstalled || gameDownloadUpdateInstalling"
      @click="launchGame()"
      >Launch</v-btn
    >
    <v-btn
      v-if="
        gameInstalled && gameUpdateAvailable && !gameDownloadUpdateInstalling
      "
      class="mb-2 w-48"
      size="large"
      :disabled="!gameUpdateAvailable || gameDownloadUpdateInstalling"
      @click="installGameUpdate()"
      >Update</v-btn
    >
    <v-btn
      v-if="gameInstalled && !configFound"
      class="mb-auto w-48"
      size="large"
      :disabled="!gameInstalled"
      @click="createConfig()"
      >Create Config</v-btn
    >
    <v-btn
      v-else-if="gameInstalled && gameUpdateAvailable"
      class="mb-2 w-48"
      size="large"
      :disabled="!gameInstalled || !configFound"
      @click="openConfig()"
      >Config</v-btn
    >
    <v-btn
      v-else-if="gameInstalled && configFound"
      class="mb-2 w-48"
      size="large"
      :disabled="!gameInstalled || !configFound"
      @click="openConfig()"
      >Config</v-btn
    >
    <div
      v-else-if="gameDownloadUpdateInstalling"
      class="mb-auto text-blue-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Downloading
    </div>
    <div
      v-if="gameVersion.length > 0"
      class="mt-auto w-auto text-white text-center font-semibold font-mono noselect drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)]"
    >
      Game Version
      <div class="text-blue-400 font-normal">{{ gameVersion }}</div>
    </div>
    <div
      v-else
      class="mt-auto w-auto text-white text-center font-semibold font-mono noselect drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)]"
    >
      Game Version
      <div class="text-red font-normal">None</div>
    </div>
    <div
      v-if="
        gameInstalled && gameUpdateAvailable && !gameDownloadUpdateInstalling
      "
      class="flex-row text-green-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Update Available
    </div>
    <div
      class="mb-auto w-auto text-white text-center font-semibold font-mono noselect drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)]"
    >
      Launcher Version
      <div class="text-blue-400 font-normal">{{ appVersion }}</div>
    </div>
    <div class="mb-auto">
      <v-select
        v-show="
          gameInstalled && gameAvailableStates && !gameDownloadUpdateInstalling
        "
        :items="gameAvailableStates"
        v-model="gameSelectedState"
        variant="underlined"
        density="compact"
        focused
        class="absolute bottom-4 right-4 font-mono text-white z-10 w-72"
        hide-details
      ></v-select>
    </div>
    <div class="w-full" v-if="gameDownloadUpdateInstalling">
      <p
        class="text-white text-right font-light text-sm mr-2 noselect"
        v-if="!gameDownloadUpdateExtracting"
      >
        {{ gameDownloadUpdateSpeed }} MB/s
      </p>
      <p class="text-white text-right font-light text-sm mr-2 noselect" v-else>
        Extracting Files ({{ gameDownloadUpdateExtractingFilesRemaining }}
        left)
      </p>
      <v-progress-linear
        v-if="!gameDownloadUpdateExtracting"
        class="animate-slide-in-bottom mb-0"
        :model-value="gameDownloadUpdateProgress"
        height="8"
        color="light-blue"
        buffer-value="0"
        stream
      ></v-progress-linear>
      <v-progress-linear
        v-else
        class="animate-slide-in-bottom mb-0"
        :model-value="gameDownloadUpdateProgress"
        height="8"
        color="light-blue"
        buffer-value="0"
        indeterminate
      ></v-progress-linear>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getVersion } from "@tauri-apps/api/app";
import { homeDir } from "@tauri-apps/api/path";
import { open, confirm, message } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import type { releaseInfo } from "../types/index.ts";

interface GameInfo {
  installed: boolean;
  version: string | null;
  states: string[];
}

const appVersion = await getVersion();
const gameVersion = ref("");
const gamePath = ref("");
const gameInstalled = ref(false);
const gameAvailableStates = ref<string[]>(["LTheoryRedux"]);
const gameSelectedState = ref("LTheoryRedux");
const gameDownloadUpdateProgress = ref(0);
const gameDownloadUpdateSpeed = ref(0);
const gameDownloadUpdateInstalling = ref(false);
const gameDownloadUpdateExtracting = ref(false);
const gameDownloadUpdateExtractingFilesRemaining = ref(0);
const gameUpdateAvailable = ref(false);
const configFound = ref(false);

interface TauriEmitEvent {
  payload: number;
}

const unlistenProgress = await listen(
  "download-progress",
  (event: TauriEmitEvent) => {
    gameDownloadUpdateProgress.value = event.payload;
    console.log("Download progress: " + event.payload);
  }
);

const unlistenSpeed = await listen(
  "download-speed",
  (event: TauriEmitEvent) => {
    let value = Math.floor((event.payload / 1024) * 10) / 10;
    gameDownloadUpdateSpeed.value = value;
    console.log("Download speed: " + value);
  }
);

const unlistenExtracting = await listen(
  "download-extracting",
  (event: TauriEmitEvent) => {
    gameDownloadUpdateExtracting.value = true;
    console.log("Files Extracting.");
  }
);

const unlistenExtractingFilesRemaining = await listen(
  "extracting-files",
  (event: TauriEmitEvent) => {
    gameDownloadUpdateExtractingFilesRemaining.value = event.payload;
    console.log("Files remaining:", event.payload);
  }
);

const unlistenCompleted = await listen("install-complete", (event) => {
  gameDownloadUpdateInstalling.value = false;
  gameDownloadUpdateExtracting.value = false;
  console.log("Install completed");
  loadGameData();
  checkConfigExists();
});

// run on page load
await loadGameData();
checkConfigExists();

async function loadGameData() {
  try {
    const info: GameInfo = await invoke("get_game_info");
    gameInstalled.value = info.installed;
    gameVersion.value = info.version || "";
    gameAvailableStates.value = info.states;

    if (info.installed && info.version) {
      await checkUpdateAvailable();
    }

    // Also get installation path for update functionality
    try {
      gamePath.value = await invoke("get_installation_path");
    } catch {
      // Path not found, game not installed
    }
  } catch (err) {
    console.error("Error loading game data:", err);
  }
}

async function checkConfigExists() {
  try {
    configFound.value = await invoke("check_config_exists");
  } catch (err) {
    console.error(err);
    configFound.value = false;
  }
}

async function checkUpdateAvailable() {
  const response = await fetch(
    "https://api.github.com/repos/Limit-Theory-Redux/ltheory/releases/tags/latest"
  );
  const info: releaseInfo = await response.json();

  if (info.name && info.name.indexOf(gameVersion.value) == -1) {
    console.log("Update found. Installed:", gameVersion.value, "| Latest:", info.name);
    gameUpdateAvailable.value = true;
  } else {
    console.log("No update found. Installed:", gameVersion.value, "| Latest:", info.name);
    gameUpdateAvailable.value = false;
  }
}

async function openConfig() {
  try {
    await invoke("open_config");
  } catch (err) {
    console.error(err);
  }
}

async function createConfig() {
  await message(
    "This feature was not implemented yet. The game will automatically generate a config once you start & modify the settings or exit it using the menu",
    "Not implemented"
  );
}

async function installGame() {
  const selected = await open({
    title: "Select Installation Folder",
    multiple: false,
    directory: true,
    defaultPath: await homeDir(),
  });

  if (selected) {
    const confirmed = await confirm(
      "Are you sure? Limit Theory Redux will be installed to: " +
        selected +
        "\\Limit Theory Redux"
    );

    if (confirmed) {
      invoke("download_game", { installPath: selected });
      gameDownloadUpdateInstalling.value = true;
    }
  }
}

async function installGameUpdate() {
  if (gamePath.value.length > 0) {
    let path = gamePath.value.replace("\\Limit Theory Redux", "");
    console.log(path);
    invoke("download_game", { installPath: path });
    gameDownloadUpdateInstalling.value = true;
  }
}

async function launchGame() {
  try {
    invoke("launch_game", { state: gameSelectedState.value });
  } catch (err) {
    console.error("Error while launching the game.");
  }
}
</script>

<style>
.v-field__outline {
  display: none !important;
}
.v-select__selection {
  width: 100%;
  justify-content: right;
}
</style>
