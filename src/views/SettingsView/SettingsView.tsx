import React, { useState } from "react";
import Dropdown from "../../components/Dropdown";
import ThemeSwitch from "../../components/ThemeSwitch";

const languageOptions = [
  { value: "en", label: "English" },
  { value: "fr", label: "Français" },
];

const SettingsView: React.FC = () => {
  const [language, setLanguage] = useState("en");
  const [bluetoothDevice, setBluetoothDevice] = useState<string | null>(null);

  const handleBluetoothSelect = () => {
    // TODO: Implement Bluetooth device selection logic
    // setBluetoothDevice(selectedDevice);
    alert("Bluetooth device selection not implemented yet.");
  };

  return (
    <main>
      <h2>Settings</h2>
      <section>
        <ThemeSwitch />
      </section>
      <section style={{ marginTop: 16 }}>
        <Dropdown
          label="Language"
          options={languageOptions}
          value={language}
          onChange={setLanguage}
        />
      </section>
      <section style={{ marginTop: 16 }}>
        <button type="button" onClick={handleBluetoothSelect}>
          Select Bluetooth device
        </button>
        {bluetoothDevice && (
          <span style={{ marginLeft: 12 }}>Selected: {bluetoothDevice}</span>
        )}
      </section>
    </main>
  );
};

export default SettingsView;
