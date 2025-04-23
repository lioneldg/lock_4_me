import "./style.css";
import { BrowserRouter, Routes, Route, Link } from "react-router";
import HomeView from "./views/HomeView";
import SettingsView from "./views/SettingsView";
import { ThemeProvider, useTheme } from "./hocks/ThemeContext";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";

function App() {
  const { colors } = useTheme();
  const { t } = useTranslation();

  useEffect(() => {
    document.body.style.background = colors.background;
    document.body.style.color = colors.text;
  }, [colors.background, colors.text]);

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
