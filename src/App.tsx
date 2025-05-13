import './style.css';
import { BrowserRouter, Routes, Route } from 'react-router';
import HomeView from './views/HomeView';
import SettingsView from './views/SettingsView';
import { ThemeProvider, useTheme } from './hooks/ThemeContext';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useSettingsStore } from './store/settingsStore';
import { useAppStore } from './store/appStore';

function App() {
  const { colors, setTheme } = useTheme();
  const { i18n } = useTranslation();
  const { settings, loadSettings } = useSettingsStore();
  const { setIsLoading } = useAppStore();

  useEffect(() => {
    document.documentElement.style.setProperty(
      'background-color',
      colors.backgroundColor,
      'important'
    );
    document.body.style.setProperty('background-color', colors.backgroundColor, 'important');
    document
      .getElementById('root')
      ?.style.setProperty('background-color', colors.backgroundColor, 'important');

    document.body.style.setProperty('color', colors.backgroundColor, 'important');
  }, [colors.backgroundColor]);

  useEffect(() => {
    setIsLoading(true);
    loadSettings().finally(() => setIsLoading(false));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (settings.theme) {
      setTheme(settings.theme as 'light' | 'dark');
    }
  }, [settings.theme, setTheme]);

  useEffect(() => {
    if (settings.language && i18n.language !== settings.language) {
      i18n.changeLanguage(settings.language);
    }
  }, [settings.language, i18n]);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomeView />} />
        <Route path="/settings" element={<SettingsView />} />
      </Routes>
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
