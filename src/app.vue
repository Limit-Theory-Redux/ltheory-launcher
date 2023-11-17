<template>
  <div
    class="w-screen h-screen bg-[url('/assets/lt_mazing2.png')] bg-cover bg-center"
  >
    <NuxtPage />
  </div>
</template>

<script lang="ts" setup>
import useBlockContextMenu from "./composables/useBlockContextMenu";

const { cleanup } = useBlockContextMenu();
</script>
