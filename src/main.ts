import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";

// Tauri API
import { appWindow } from '@tauri-apps/api/window'

// Move to Center & Focus
appWindow.center()
appWindow.setFocus()

// Prevent context menu
document.addEventListener('contextmenu', event => event.preventDefault());

createApp(App).use(router).mount("#app");
