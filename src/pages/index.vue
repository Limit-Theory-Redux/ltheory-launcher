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
      <div class="flex flex-row mb-2 noselect">
        <a href="https://github.com/Limit-Theory-Redux/ltheory" target="_blank" draggable="false">
          <v-icon class="i-mdi:github"
        /></a>
        <p class="ml-4">Source</p>
      </div>
      <div class="flex flex-row noselect" draggable="false">
        <a href="https://discord.gg/MrfRR5ytJF" target="_blank" draggable="false">
          <v-icon class="i-bi:discord"
        /></a>
        <p class="ml-4">Discord</p>
      </div>
    </div>
    <v-btn
      v-if="!gameInstalled && !gameDownloadInstalling"
      class="mb-auto w-48"
      size="large"
      @click="installGame()"
      :disabled="gameDownloadInstalling"
      >Install</v-btn
    >
    <v-btn
      v-else-if="!gameInstalled && gameDownloadInstalling"
      class="mb-4 w-48"
      size="large"
      @click="installGame()"
      :disabled="gameDownloadInstalling"
      >Install</v-btn
    >
    <v-btn
      v-else
      class="mb-4 w-48"
      size="large"
      :disabled="!gameInstalled || gameUpdateInstalling"
      @click="launchGame()"
      >Launch</v-btn
    >
    <v-btn
      v-if="gameInstalled && gameUpdateAvailable && !gameUpdateInstalling"
      class="mb-4 w-48"
      size="large"
      :disabled="!gameUpdateAvailable || gameUpdateInstalling"
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
      v-if="gameInstalled && gameUpdateAvailable && !gameUpdateInstalling"
      class="mb-auto text-green-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Update Available
    </div>
    <div
      v-else-if="gameUpdateInstalling"
      class="mb-auto text-blue-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Installing Update
    </div>
    <div
      v-else-if="gameDownloadInstalling"
      class="mb-auto text-blue-400 font-mono noselect drop-shadow-[0_1px_1px_rgba(0,0,0,0.7)]"
    >
      Downloading Game
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
      v-if="gameDownloadInstalling || gameUpdateInstalling"
      model-value="25"
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
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

import type { releaseInfo } from "../types/index.ts";

const appVersion = await getVersion();
const gameVersion = ref("");
const gamePath = ref("");
const gameInstalled = ref(false);
const gameDownloadInstalling = ref(false);
const gameUpdateAvailable = ref(false);
const gameUpdateInstalling = ref(false);
const configFound = ref(false);
const configUrl = "LTheoryRedux\\LTheoryRedux\\data\\user.ini";

// run on page load
getGameInstallationPath();
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
  // create default config here
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
    } else {
      throw new Error("Version could not be found");
    }
  } catch (err) {
    console.error("Error while reading game version from Version.lua:", err);
  }
}

async function installGame() {
  // TODO: Authentificate as GitHub App so we don´t get api rate limited
  /* const response = await fetch("https://api.github.com/repos/IllustrisJack/ltheory-redux/releases/tags/latest");
  const info: releaseInfo = await response.json();

  let assets: Asset[] = info.assets

  assets.forEach(asset => {
    console.log(asset.name)
  });
  */

  const selected = await open({
    title: "Select Installation Path",
    multiple: false,
    directory: true,
    defaultPath: await homeDir(),
  });

  console.log(selected);
  gameDownloadInstalling.value = true;
}

async function installGameUpdate() {
  gameUpdateInstalling.value = true;
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
