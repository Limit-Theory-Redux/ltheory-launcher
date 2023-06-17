<script setup lang="ts">
  import { ref } from "vue";
  import { getVersion, getName } from '@tauri-apps/api/app';
  import { dirname, configDir } from '@tauri-apps/api/path';
  import { exists, BaseDirectory } from '@tauri-apps/api/fs';
  import { Command } from '@tauri-apps/api/shell'
  import { type } from '@tauri-apps/api/os';
  import { open } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';

  // Get App Name
  const appName = ref('AppName')
  getName().then((name) => {
    appName.value = name
  })
  // Get App Version
  const appVersion = ref('AppVersion')
  getVersion().then((name) => {
    appVersion.value = name
  })

  const gameExecutable = ref('')
  const launchButtonText = ref('')
  let cachedPath = localStorage.getItem("executablePath")

  if ( cachedPath && cachedPath.includes("lt64.exe")) {
    gameExecutable.value = cachedPath
    launchButtonText.value = 'Launch Game'
  } else {
    launchButtonText.value = 'Select Executable'
  }

  async function getExecuteCommandForOs () {
    const osType = await type();
    let shell = osType=="Windows_NT" ? 'cmd' : 'sh'
    let start = osType=="Windows_NT" ? '/C' : 'xdg-open'
    return {shell, start}
  }
  // Get Config Path
  async function openConfig() {
    try {
      var configDirPath = await configDir();
      var path = configDirPath + "\\LTheoryRedux\\LTheoryRedux\\user.ini"

      if (await exists(path, {dir: BaseDirectory.Data})) {
        let command = await getExecuteCommandForOs()
        let output = new Command(command.shell, [command.start, path]).execute()
        console.log(output)
      } else {
        console.log("Config does not exist")
      }
    } catch(err) {
      console.error(err)
    }
  }

  async function launchGame() {
    if (!gameExecutable.value.length || gameExecutable.value.length && !gameExecutable.value.includes("lt64.exe")) {
      let path = await open({
        multiple: false,
        filters: [{
          name: 'LTR Executable',
          extensions: ['exe']
        }]
      });

      // Hardcoded to windows for now
      if (typeof path === "string" && path.includes("lt64.exe")) {
        gameExecutable.value = path
        launchButtonText.value = "Launch Game"
        localStorage.setItem("executablePath", path)
      }
    } else {
      try {
        if (await exists(gameExecutable.value, {dir: BaseDirectory.Home})) {
          invoke("launch_game", { dir: await dirname(gameExecutable.value), path: gameExecutable.value })
        }
      } catch(err) {
        console.error(err)
      }
    }
  }
</script>

<template>
  <main class="home-page">
    <div class="container">
      <div class="row">
        <a href="https://github.com/Limit-Theory-Redux/ltheory" target="_blank">
          <img class="appLogo" src="/assets/LTR_Icon.png" alt="AppLogo"/>
        </a>
      </div>
      <img class="appTitle" src="/assets/LTR_Title.png"/>
      <button class="button-launch" @click="launchGame">{{launchButtonText}}</button>
      <button class="button-configure" @click="openConfig">Configure</button>
      <div class="row">
        <div class="container-appInfo">
          <p class="name">{{ appName }} </p>
          <p class="version">{{ appVersion }}</p>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.home-page{
  width: 100%;
  height: 100%;
	display: flex;
	align-items:center;
	justify-content:center;
  background-position: center center;
  background-image: url("/assets/lt_mazing2.png");
  background-repeat: no-repeat;
  background-size: cover;
}
.container {
  margin: auto auto auto auto;
  padding: var(--titlebar-height) 0 0 0;
}

.container-appInfo {
  margin: 15px auto 5px auto;
}

.name {
  color:rgb(255, 255, 255);
  filter: drop-shadow(0 0 1px black);
  margin: 0 0 0 0;
}

.version {
  color:rgb(255, 255, 255);
  filter: drop-shadow(0 0 1px black);
  margin: 0 0 0 0;
}

.appLogo {
  height: 10em;
  transition: 0.5s;
  filter: drop-shadow(0px 0px 10px black)
}

.appTitle {
  height: 5em;
  transition: 0.5s;
  filter: drop-shadow(1px 1px 2px black)
}

.button-launch {
  margin-top: 15px;
  margin-left: auto;
  margin-right: auto;
  width: 12em;
  background-color: rgba(256, 256, 256, 0.9);
  color: black;
  font-weight: 1000;
  filter: drop-shadow(0px 0px 5px black)
}

.button-configure {
  margin-top: 10px;
  margin-left: auto;
  margin-right: auto;
  width: 12em;
  background-color: rgba(256, 256, 256, 0.9);
  color: black;
  font-weight: 1000;
  filter: drop-shadow(0px 0px 5px black)
}
</style>