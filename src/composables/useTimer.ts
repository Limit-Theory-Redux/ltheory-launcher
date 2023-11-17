import { ref, type Ref } from "vue";

export function useTimerContinuous(callback: () => void, interval: number) {
  const id: Ref<NodeJS.Timeout | null> = ref(null);
  const started: Ref<Date | null> = ref(null);
  const running: Ref<boolean> = ref(false);

  function start() {
    running.value = true;
    started.value = new Date();
    id.value = setInterval(callback, interval);
  }

  function stop() {
    running.value = false;
    if (id.value) {
      clearInterval(id.value);
    }
  }

  function getStateRunning() {
    return running.value;
  }

  return {
    start,
    stop,
    getStateRunning,
  };
}
