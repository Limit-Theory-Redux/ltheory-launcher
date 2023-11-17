import { onMounted } from "vue"

export default function useBlockContextMenu() {
  function blockContextMenu(event: MouseEvent) {
    event.preventDefault();
  }

  // if needed make this a plugin, but why?
  if (process.env.NODE_ENV == "production") {
    onMounted(() => {
      document.addEventListener('contextmenu', blockContextMenu);
    });
  }

  function cleanup() {
    document.removeEventListener('contextmenu', blockContextMenu);
  }

  return {
    cleanup,
  };
}