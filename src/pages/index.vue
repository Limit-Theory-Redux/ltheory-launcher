<template>
  <div class="flex flex-col items-center w-screen h-screen">
    <img
      src="../assets/LTR_Icon.png"
      class="mt-auto mb-4 w-32 drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)] noselect"
    />
    <img
      src="../assets/LTR_Title.png"
      class="mb-4 w-96 drop-shadow-[0_4px_4px_rgba(0,0,0,0.7)] noselect"
    />
    <div
      class="flex flex-col mb-6 drop-shadow-[0_1px_2px_rgba(0,0,0,0.7)] text-white font-semibold"
    >
      <div class="flex flex-row mb-2 noselect">
        <a href="https://github.com/Limit-Theory-Redux/ltheory" target="_blank">
          <v-icon class="i-mdi:github mr-4" /></a
        >
        <p>Source</p>
      </div>
      <div class="flex flex-row noselect">
        <a href="https://discord.gg/MrfRR5ytJF" target="_blank">
          <v-icon class="i-bi:discord mr-4" /></a
        >
        <p>Discord</p>
      </div>
    </div>
    <v-btn v-if="!gameInstalled" class="mb-4 w-48" size="large"
      >Download Game</v-btn
    >
    <v-btn
      v-else
      class="mb-4 w-48"
      size="large"
      :disabled="!gameInstalled || gameUpdateInstalling"
      >Launch Game</v-btn
    >
    <v-btn
      v-if="gameInstalled && gameUpdateAvailable && !gameUpdateInstalling"
      class="mb-4 w-48"
      size="large"
      :disabled="!gameUpdateAvailable || gameUpdateInstalling"
      @click="installGameUpdate()"
      >Update Game</v-btn
    >
    <v-btn
      v-if="!configFound"
      class="mb-4 w-48"
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
      v-if="gameUpdateInstalling"
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
import { dirname, configDir } from "@tauri-apps/api/path";
import { exists, BaseDirectory } from "@tauri-apps/api/fs";
import { type } from "@tauri-apps/api/os";
import { Command } from "@tauri-apps/api/shell";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

const appVersion = await getVersion();
const gameVersion = ref("");
const gameInstalled = ref(true);
const gameUpdateAvailable = ref(true);
const gameUpdateInstalling = ref(false);
const configFound = ref(false);
const configUrl = "LTheoryRedux\\LTheoryRedux\\data\\user.ini";

// run on page load
checkConfigExists();

async function checkConfigExists() {
  console.log("Check Config Exists");
  try {
    var configDirPath = await configDir();
    var path = configDirPath + configUrl;

    if (await exists(path, { dir: BaseDirectory.Data })) {
      console.log("Config exists");
      configFound.value = true;
    } else {
      console.log("Config does not exists");
      configFound.value = false;
    }
  } catch (err) {
    console.error(err);
  }
}

async function openConfig() {
  console.log("Open Config");
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

async function installGameUpdate() {
  gameUpdateInstalling.value = true;
}

async function getExecuteCommandForOs() {
  const osType = await type();
  let shell = osType == "Windows_NT" ? "cmd" : "sh";
  let start = osType == "Windows_NT" ? "/C" : "xdg-open";
  return { shell, start };
}
</script>

<style scoped></style>
