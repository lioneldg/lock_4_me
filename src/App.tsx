import "./style.css";
import { BrowserRouter, Routes, Route, Link } from "react-router";
import HomeView from "./views/HomeView";
import SettingsView from "./views/SettingsView";
import { ThemeProvider, useTheme } from "./hooks/ThemeContext";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { useSettingsStore } from "./store/settingsStore";

function App() {
  const { colors, setTheme } = useTheme();
  const { t, i18n } = useTranslation();
  const { settings, loadSettings } = useSettingsStore();

  useEffect(() => {
    document.body.style.background = colors.background;
    document.body.style.color = colors.text;
  }, [colors.background, colors.text]);

  useEffect(() => {
    loadSettings();
  }, []);

  useEffect(() => {
    if (settings.theme) {
      setTheme(settings.theme as "light" | "dark");
    }
  }, [settings.theme, setTheme]);

  useEffect(() => {
    if (settings.language && i18n.language !== settings.language) {
      i18n.changeLanguage(settings.language);
    }
  }, [settings.language, i18n]);

  return (
    <BrowserRouter>
      <nav style={{ display: "flex", gap: "1rem", marginBottom: "1rem" }}>
        <Link to="/">{t("nav.home")}</Link>
        <Link to="/settings">{t("nav.settings")}</Link>
      </nav>
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
