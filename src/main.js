import { fetchSettings, saveSettings } from "./settings.js";

window.addEventListener("DOMContentLoaded", () => {
  let currentSettings = null;

  // IIFE: Immediately Invoked Function Expression
  (async () => {
    currentSettings = await fetchSettings();
  })();

  document.getElementById("btnLoad").addEventListener("click", async () => {
    currentSettings = await fetchSettings();
  });

  document.getElementById("btnSave").addEventListener("click", async () => {
    if (currentSettings) {
      await saveSettings(currentSettings);
    }
  });
});
