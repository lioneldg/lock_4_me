import './style.css';
import { BrowserRouter, Routes, Route } from 'react-router';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import HomeView from './views/HomeView';
import SettingsView from './views/SettingsView';
import { ThemeProvider, useTheme } from './hooks/ThemeContext';
import { useEffect, useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { useSettingsStore } from './store/settingsStore';
import { useAppStore } from './store/appStore';
import LoadingSpinner from './components/LoadingSpinner';
import { useBluetoothStore } from './store/bluetoothStore';
import { DiscoveredDevice } from './types';

async function listen_bluetooth(targetUuid?: string, rssiDeltaMax?: number) {
  await invoke('listen_bluetooth', {
    target_uuid: targetUuid,
    rssi_delta_max: rssiDeltaMax
  });
}

async function lockScreen() {
  await invoke('lock_screen');
}

function App() {
  const { colors, setTheme } = useTheme();
  const { i18n } = useTranslation();
  const { settings, loadSettings } = useSettingsStore();
  const { setIsLoading, isLoading } = useAppStore();
  const { addEvent, events } = useBluetoothStore();

  useEffect(() => {
    if (events.size === 0 && !isLoading) {
      setIsLoading(true);
    } else if (events.size > 0 && isLoading) {
      setIsLoading(false);
    }
  }, [events.size, isLoading, setIsLoading]);

  useEffect(() => {
    document
      .getElementById('root')
      ?.style.setProperty('background-color', colors.backgroundColor, 'important');
  }, [colors.backgroundColor]);

  useEffect(() => {
    setIsLoading(true);
    loadSettings().finally(() => setIsLoading(false));
  }, [loadSettings, setIsLoading]);

  useEffect(() => {
    if (settings.theme) {
      setTheme(settings.theme);
    }
    if (settings.language && i18n.language !== settings.language) {
      i18n.changeLanguage(settings.language);
    }
  }, [settings.theme, settings.language, i18n, setTheme]);

  const { target_uuid, rssi_delta_max } = useMemo(
    () => ({
      target_uuid: settings.target_uuid || undefined,
      rssi_delta_max: settings.target_uuid ? settings.rssi_delta_max : undefined
    }),
    [settings.target_uuid, settings.rssi_delta_max]
  );

  useEffect(() => {
    listen_bluetooth(target_uuid, rssi_delta_max);

    const unlistenPromises = [
      listen('bluetooth-event', (event) => addEvent(event.payload as DiscoveredDevice)),
      listen('bluetooth-refresh-timeout', () => lockScreen()),
      listen('bluetooth-over-delta-rssi', () => lockScreen())
    ];

    return () => {
      unlistenPromises.forEach((promise) => promise.then((unlisten) => unlisten()));
    };
  }, [target_uuid, rssi_delta_max, addEvent]);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomeView />} />
        <Route path="/settings" element={<SettingsView />} />
      </Routes>
      {isLoading && <LoadingSpinner />}
    </BrowserRouter>
  );
}

export default function AppWithProvider() {
  return (
    <ThemeProvider>
      <App />
    </ThemeProvider>
  );
}
