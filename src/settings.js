const { invoke } = window.__TAURI__.core;

export async function fetchSettings() {
  try {
    const settings = await invoke('get_settings');
    console.log('Settings:', settings);
    return settings;
  } catch (error) {
    console.log('Error while loading settings:', error);
    return null;
  }
}

export async function saveSettings(newSettings) {
  try {
    await invoke('save_settings', { settings: newSettings });
    console.log('Settings saved!');
  } catch (error) {
    console.log('Error while saving:', error);
  }
}
