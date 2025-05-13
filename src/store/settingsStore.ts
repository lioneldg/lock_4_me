import { create } from 'zustand';
import { Settings } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { appDataDir, join } from '@tauri-apps/api/path';

interface SettingsStore {
  settings: Settings;
  setSettings: (settings: Partial<Settings>) => void;
  loadSettings: () => Promise<void>;
}

const defaultSettings: Settings = {
  target_uuid: '',
  rssi_delta_max: 15,
  theme: 'dark',
  language: 'en'
};

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  settings: defaultSettings,
  setSettings: async (newSettings) => {
    const updated = { ...get().settings, ...newSettings };
    set({ settings: updated });
    // Persists to disk
    const dir = await appDataDir();
    const filePath = await join(dir, 'settings.json');
    await invoke('write_settings', { file_path: filePath, settings: updated });
  },
  loadSettings: async () => {
    const dir = await appDataDir();
    const filePath = await join(dir, 'settings.json');
    try {
      const _settings = await invoke<Settings>('read_settings', {
        file_path: filePath
      });
      set({ settings: _settings });
    } catch (_e) {
      // If not found, write default
      await invoke('write_settings', {
        file_path: filePath,
        settings: defaultSettings
      });
      set({ settings: defaultSettings });
    }
  }
}));
