<template>
  <div
    data-tauri-drag-region
    class="fixed justify-end text-end z-50 w-full bg-transparent text-xs"
  >
    <v-icon
      @click="appWindow.minimize()"
      class="i-mdi:window-minimize text-white m-1"
    ></v-icon>
    <v-icon
      @click="isWindowMaximized()"
      class="i-mdi:window-maximize text-white m-1"
    ></v-icon>
    <v-icon
      @click="appWindow.close()"
      class="i-mdi:window-close text-white m-1"
    ></v-icon>
  </div>
  <div class="relative">
    <video
      v-show="dynamicBg"
      autoplay
      loop
      muted
      onloadstart="this.playbackRate=0.25"
      src="/assets/LTR_BG_VID.mp4"
      type="video/mp4"
      class="absolute inset-0 w-full h-full object-cover z-0"
      @loadeddata="onLoadVideo()"
    />
    <img
      v-show="!dynamicBg"
      src="/assets/LTR_SPLASH.png"
      class="absolute inset-0 w-full h-full object-cover z-0"
    />
    <v-switch
      class="absolute bottom-0 left-4 h-14 z-10 text-white"
      v-model="dynamicBg"
      color="indigo"
      label="Play Video"
    />
    <NuxtPage
      data-tauri-drag-region
      draggable="false"
      class="backdrop-saturate-[75%]"
    />
  </div>
</template>

<script lang="ts" setup>
import { appWindow } from "@tauri-apps/api/window";
import useBlockContextMenu from "./composables/useBlockContextMenu";
import useBlockFileDrop from "./composables/useBlockFileDrop";
import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { confirm } from "@tauri-apps/api/dialog";

useBlockFileDrop();
useBlockContextMenu();
const dynamicBg = ref(true);
const windowMaximized = ref(false);

const unlisten = await onUpdaterEvent(({ error, status }) => {
  // This will log all updater events, including status updates and errors.
  console.log("Updater event", error, status);
});

// on page load
getBgSettingFromStorage();
getMaximizedFromStorage();

function stringToBoolean(str: string): boolean {
  return str.toLowerCase() === "true";
}

function getBgSettingFromStorage() {
  let fromStorage = localStorage.getItem("dynamicBg");

  if (fromStorage) {
    dynamicBg.value = stringToBoolean(fromStorage);
  }
}

function getMaximizedFromStorage() {
  let fromStorage = localStorage.getItem("isMaximized");

  if (fromStorage) {
    windowMaximized.value = stringToBoolean(fromStorage);
  }

  setMaximized(windowMaximized.value);
}

function setMaximized(maximize: boolean) {
  if (maximize == true) {
    appWindow.maximize();
    windowMaximized.value = true;
  } else {
    appWindow.unmaximize();
    windowMaximized.value = false;
  }
}

async function isWindowMaximized() {
  const maximized = await appWindow.isMaximized();
  setMaximized(!maximized);

  localStorage.setItem("isMaximized", windowMaximized.value.toString());
}

watch(dynamicBg, (dynamicBg) => {
  localStorage.setItem("dynamicBg", dynamicBg.toString());
});

function onLoadVideo() {
  appWindow.show();
  checkForUpdate();
}

async function checkForUpdate() {
  try {
    const { shouldUpdate, manifest } = await checkUpdate();
    if (shouldUpdate) {
      const confirmed = await confirm(
        "An update for the Launcher is available. Do you want to download the update?",
        { title: "Update Available", type: "info" }
      );

      if (confirmed) {
        console.log(`Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`);

        await installUpdate();

        await relaunch();
      }
    }
  } catch (error) {
    console.error(error);
  }
}
</script>
