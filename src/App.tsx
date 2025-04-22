import "./style.css";
import { BrowserRouter, Routes, Route, Link } from "react-router";
import HomeView from "./views/HomeView";
import SettingsView from "./views/SettingsView";
import { useTheme } from "./hocks/useTheme";
import { useEffect } from "react";

function App() {
  const { colors } = useTheme();

  useEffect(() => {
    document.body.style.background = colors.background;
    document.body.style.color = colors.text;
  }, [colors.background, colors.text]);

  return (
    <BrowserRouter>
      <nav style={{ display: "flex", gap: "1rem", marginBottom: "1rem" }}>
        <Link to="/">Home</Link>
        <Link to="/settings">Settings</Link>
      </nav>
      <Routes>
        <Route path="/" element={<HomeView />} />
        <Route path="/settings" element={<SettingsView />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
