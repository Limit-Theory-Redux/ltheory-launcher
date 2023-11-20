export default function useBlockFileDrop() {
  function blockFileDrop(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "none";
      e.dataTransfer.dropEffect = "none";
    }
  }

  window.addEventListener(
    "dragenter",
    blockFileDrop
  );

  window.addEventListener(
    "dragover",
    blockFileDrop
  );

  window.addEventListener(
    "drop",
    blockFileDrop
  );
}
