import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [bluetoothEvents, setBluetoothEvents] = useState<any[]>([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
    const unlistenPromise = listen("bluetooth-event", (event) => {
      setBluetoothEvents((prev) => [...prev, event.payload]);
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <h2>Bluetooth Events</h2>
      <ul>
        {bluetoothEvents.map((e, i) => (
          <li key={i}>
            {e.local_name} (RSSI: {e.rssi}, Δ: {e.diff_rssi})
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
