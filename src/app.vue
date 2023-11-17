<template>
<div class="relative">
  <video v-if="dynamicBg" autoplay loop muted onloadstart="this.playbackRate=0.20" src="./assets/LTR_BG_VID.mp4" type="video/mp4" class="absolute inset-0 w-full h-full object-cover z-0"/>
  <img v-else src="./assets/LTR_BG_1.png" class="absolute inset-0 w-full h-full object-cover z-0"/>
  <v-switch class="absolute bottom-0 left-4 h-14 z-10 text-white" v-model="dynamicBg" label="Play Video"/>
  <NuxtPage class="backdrop-saturate-[75%]"/>
</div>
</template>

<script lang="ts" setup>
import useBlockContextMenu from "./composables/useBlockContextMenu";

const { cleanup } = useBlockContextMenu();
// TODO: Store this info
const dynamicBg = ref(true);
</script>
