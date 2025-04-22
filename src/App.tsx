import "./style.css";
import { BrowserRouter, Routes, Route, Link } from "react-router";
import HomeView from "./views/HomeView";
import SettingsView from "./views/SettingsView";

function App() {
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
