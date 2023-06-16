<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import Greet from "./components/Greet.vue";

async function isWindowMaximized(){
  const maximized = await appWindow.isMaximized();
  console.log(maximized)
  if (maximized == false) {
    appWindow.maximize()
  }
  else {
    appWindow.unmaximize()
  }
}
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div @click="appWindow.minimize" class="titlebar-button" id="titlebar-minimize">
      <img
        src="/assets/mdi-window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div @click="isWindowMaximized()" appWindow.unmaximize class="titlebar-button" id="titlebar-maximize">
      <img
      src="/assets/mdi-window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div @click="appWindow.close" class="titlebar-button" id="titlebar-close">
      <img src="/assets/mdi-close.svg" alt="close" />
    </div>
  </div>

  <router-view />
</template>

<style scoped>

</style>
