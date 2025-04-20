import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { appDataDir, join } from "@tauri-apps/api/path";
import "./App.css";
import { Settings, DiscoveredDevice } from "./types";
import { useBluetoothStore } from "./store/bluetoothStore";
import { BrowserRouter, Routes, Route, Link } from "react-router";

const TARGET_UUID = "a87e3669-e2de-d0e3-52ce-93a023ceef37";
const RSSI_DELTA_MAX = 15;

async function listen_bluetooth(targetUuid: string, rssiDeltaMax: number) {
  await invoke("listen_bluetooth", {
    target_uuid: targetUuid,
    rssi_delta_max: rssiDeltaMax,
  });
}

async function writeSettings(filePath: string, settings: Settings) {
  await invoke("write_settings", {
    file_path: filePath,
    settings: settings,
  });
}

async function loadSettings(filePath: string) {
  return await invoke("read_settings", { file_path: filePath });
}

async function lockScreen() {
  await invoke("lock_screen");
}

function HomePage() {
  const { addEvent, events } = useBluetoothStore();

  useEffect(() => {
    listen_bluetooth(TARGET_UUID, RSSI_DELTA_MAX);
    const unlistenBTEventPromise = listen("bluetooth-event", (event) =>
      addEvent(event.payload as DiscoveredDevice)
    );
    const unlistenBTClosedPromise = listen("bluetooth-listener-closed", () => {
      lockScreen();
    });
    return () => {
      unlistenBTEventPromise.then((unlisten) => unlisten());
      unlistenBTClosedPromise.then((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    appDataDir().then(async (dir) => {
      let filePath = await join(dir, "settings.json");
      let settings;
      loadSettings(filePath)
        .then((_settings) => {
          settings = _settings;
        })
        .catch((error) => {
          console.error(error);
        });
      if (!settings) {
        writeSettings(filePath, {
          target_uuid: TARGET_UUID,
          rssi_delta_max: RSSI_DELTA_MAX,
        }).catch((error) => {
          console.error(error);
        });
      }
    });
  }, []);

  return (
    <main className="container">
      <h2>Bluetooth Events</h2>
      <ul>
        {Array.from(events.values()).map((event) => (
          <p key={event.id}>
            {event.local_name} {"=>"} RSSI: {event.rssi}, Δ: {event.diff_rssi}
          </p>
        ))}
      </ul>
    </main>
  );
}

function SettingsPage() {
  return (
    <main className="container">
      <h2>Settings</h2>
      <p>Settings page content goes here.</p>
    </main>
  );
}

function App() {
  return (
    <BrowserRouter>
      <nav style={{ display: "flex", gap: "1rem", marginBottom: "1rem" }}>
        <Link to="/">Home</Link>
        <Link to="/settings">Settings</Link>
      </nav>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
