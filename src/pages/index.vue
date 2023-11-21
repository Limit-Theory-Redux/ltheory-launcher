<template>
  <div class="flex flex-col items-center w-screen h-screen">
    <img
      src="/assets/LTR_Logo.svg"
      class="mt-auto mb-4 w-32 drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)] noselect"
      draggable="false"
    />
    <img
      src="/assets/LTR_Title.svg"
      class="mb-4 w-96 drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)] noselect"
      draggable="false"
    />
    <div
      class="flex flex-col mb-6 drop-shadow-[0_1px_2px_rgba(0,0,0,0.7)] text-white font-semibold"
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
      class="mb-4 w-48"
      size="large"
      @click="installGame()"
      :disabled="gameDownloadUpdateInstalling"
      >Install</v-btn
    >
    <v-btn
      v-else
      class="mb-4 w-48"
      size="large"
      :disabled="!gameInstalled || gameDownloadUpdateInstalling"
      @click="launchGame()"
      >Launch</v-btn
    >
    <v-btn
      v-if="
        gameInstalled && gameUpdateAvailable && !gameDownloadUpdateInstalling
      "
      class="mb-4 w-48"
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
      class="mb-4 w-48"
      size="large"
      :disabled="!gameInstalled || !configFound"
      @click="openConfig()"
      >Config</v-btn
    >
    <v-btn
      v-else-if="gameInstalled && configFound"
      class="mb-auto w-48"
      size="large"
      :disabled="!gameInstalled || !configFound"
      @click="openConfig()"
      >Config</v-btn
    >
    <div
      v-if="
        gameInstalled && gameUpdateAvailable && !gameDownloadUpdateInstalling
      "
      class="mb-auto text-green-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Update Available
    </div>
    <div
      v-else-if="gameDownloadUpdateInstalling"
      class="mb-auto text-blue-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Downloading
    </div>
    <div
      v-if="gameVersion.length > 0"
      class="w-auto text-white text-center font-semibold font-mono noselect drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)]"
    >
      Game Version
      <div class="text-blue-400 font-normal">{{ gameVersion }}</div>
    </div>
    <div
      v-else
      class="w-auto text-white text-center font-semibold font-mono noselect drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)]"
    >
      Game Version
      <div class="text-red font-normal">None</div>
    </div>
    <div
      class="mb-12 w-auto text-white text-center font-semibold font-mono noselect drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)]"
    >
      Launcher Version
      <div class="text-blue-400 font-normal">{{ appVersion }}</div>
    </div>
    <v-progress-linear
      class="animate-slide-in-bottom mb-0"
      v-if="gameDownloadUpdateInstalling"
      :model-value="gameDownloadUpdateProgress"
      height="8"
      color="light-blue"
      buffer-value="0"
      stream
    ></v-progress-linear>
  </div>
</template>

<script lang="ts" setup>
import { getVersion } from "@tauri-apps/api/app";
import { dirname, configDir, homeDir } from "@tauri-apps/api/path";
import { exists, BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
import { type } from "@tauri-apps/api/os";
import { Command } from "@tauri-apps/api/shell";
import { open, confirm, message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

import type { releaseInfo } from "../types/index.ts";

const appVersion = await getVersion();
const gameVersion = ref("");
const gamePath = ref("");
const gameInstalled = ref(false);
const gameDownloadUpdateProgress = ref(0);
const gameDownloadUpdateInstalling = ref(false);
const gameUpdateAvailable = ref(false);
const configFound = ref(false);
const configUrl = "LTheoryRedux\\LTheoryRedux\\data\\user.ini";

interface DownloadProgressEvent {
  payload: number;
}

interface InstallCompleteEvent {
  payload: string;
}

interface LauncherUpdateStatusEvent {
  payload: LauncherUpdateStatusEventPayload;
}

interface LauncherUpdateStatusEventPayload {
  status: string,
  error: string
}

const unlistenProgress = await listen(
  "download-progress",
  (event: DownloadProgressEvent) => {
    gameDownloadUpdateProgress.value = event.payload;
    console.log("Download progress: " + event.payload);
  }
);

const unlistenCompleted = await listen(
  "install-complete",
  (event: InstallCompleteEvent) => {
    gameDownloadUpdateInstalling.value = false;
    console.log("Install completed: " + event.payload);
    getGameInstallationPath();
    checkConfigExists();
  }
);

listen("tauri://update-status", (event: LauncherUpdateStatusEvent) => {
  console.log("New status: ", event);
  if (event.payload.status == "PENDING") {
    gameDownloadUpdateInstalling.value = true
  }
});

// run on page load
await getGameInstallationPath();
checkConfigExists();

async function checkConfigExists() {
  try {
    var configDirPath = await configDir();
    var path = configDirPath + configUrl;

    if (await exists(path, { dir: BaseDirectory.Data })) {
      configFound.value = true;
    } else {
      configFound.value = false;
    }
  } catch (err) {
    console.error(err);
  }
}

async function checkUpdateAvailable() {
  // Todo change to actual repo & use gh app auth
  const response = await fetch(
    "https://api.github.com/repos/Limit-Theory-Redux/ltheory/releases/tags/latest"
  );
  const info: releaseInfo = await response.json();

  if (info.name.indexOf(gameVersion.value) == -1) {
    let str =
      "Update found. Installed version: " +
      gameVersion.value +
      " | Latest version: " +
      info.name;
    console.log(str);
    gameUpdateAvailable.value = true;
  } else {
    let str =
      "No update found. Installed version: " +
      gameVersion.value +
      " | Latest version: " +
      info.name;
    console.log(str);
    gameUpdateAvailable.value = false;
  }
}

async function openConfig() {
  try {
    var configDirPath = await configDir();
    var path = configDirPath + configUrl;

    if (await exists(path, { dir: BaseDirectory.Data })) {
      let command = await getExecuteCommandForOs();
      let output = new Command(command.shell, [command.start, path]).execute();
      console.log(output);
    } else {
      console.log("Config does not exist");
    }
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

async function getGameInstallationPath() {
  try {
    let installationPath: string = await invoke("get_installation_path");
    console.log("Installation Path: " + installationPath);

    gamePath.value = installationPath;
    await checkIfExecutableExists();
  } catch (err) {
    console.error("Installation path not found in registry with error: " + err);
  }
}

async function checkIfExecutableExists() {
  try {
    const binaryFilePath = `${gamePath.value}\\bin\\lt64.exe`;

    if (await exists(binaryFilePath, { dir: BaseDirectory.Home })) {
      gameInstalled.value = true;
      console.log("Game binary found at: " + binaryFilePath);
      await getGameVersion();
    }
  } catch (err) {
    console.error(err);
  }
}

async function getGameVersion() {
  try {
    const versionFilePath = `${gamePath.value}\\script\\Config\\Version.lua`;

    const data = await readTextFile(versionFilePath, {
      dir: BaseDirectory.Home,
    });

    const match = data.match(/Config\.gameVersion\s*=\s*"([^"]+)"/);
    if (match && match[1]) {
      console.log("Found version: " + match[1]);
      gameVersion.value = match[1];
      await checkUpdateAvailable();
    } else {
      throw new Error("Version could not be found");
    }
  } catch (err) {
    gameVersion.value = "";
    console.error("Error while reading game version from Version.lua:", err);
  }
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
    gamePath.value = gamePath.value.replace("\\Limit Theory Redux", "");
    console.log(gamePath.value);
    invoke("download_game", { installPath: gamePath.value });
    gameDownloadUpdateInstalling.value = true;
  }
}

async function launchGame() {
  try {
    invoke("launch_game");
  } catch (err) {
    console.error("Error while launching the game.");
  }
}

async function getExecuteCommandForOs() {
  const osType = await type();
  let shell = osType == "Windows_NT" ? "cmd" : "sh";
  let start = osType == "Windows_NT" ? "/C" : "xdg-open";
  return { shell, start };
}
</script>
