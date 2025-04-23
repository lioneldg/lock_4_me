import React, { useState } from "react";
import Dropdown from "../../components/Dropdown";
import ThemeSwitch from "../../components/ThemeSwitch";
import { useTranslation } from "react-i18next";
import Slider from "../../components/Slider";
import { useSettingsStore } from "../../store/settingsStore";
import { useDebounce } from "../../hooks/useDebounce";

const languageOptions = [
  { value: "en", label: "English" },
  { value: "fr", label: "Français" },
];

const SettingsView: React.FC = () => {
  const { t, i18n } = useTranslation();
  const { settings, setSettings } = useSettingsStore();
  const [bluetoothDevice, setBluetoothDevice] = useState<string | null>(null);
  const [localRssi, setLocalRssi] = useState(settings.rssi_delta_max);
  const debouncedRssi = useDebounce(localRssi, 400);

  React.useEffect(() => {
    if (debouncedRssi !== settings.rssi_delta_max) {
      setSettings({ rssi_delta_max: debouncedRssi });
    }
  }, [debouncedRssi]);

  const handleLanguageChange = (lang: string) => {
    i18n.changeLanguage(lang);
    setSettings({ language: lang });
  };

  const handleBluetoothSelect = () => {
    // TODO: Implement Bluetooth device selection logic
    // setBluetoothDevice(selectedDevice);
    alert("Bluetooth device selection not implemented yet.");
  };

  return (
    <main>
      <h2>{t("settings.title")}</h2>
      <section>
        <ThemeSwitch />
      </section>
      <section style={{ marginTop: 16 }}>
        <Dropdown
          label={t("settings.language")}
          options={languageOptions}
          value={settings.language}
          onChange={handleLanguageChange}
        />
      </section>
      <section style={{ marginTop: 16 }}>
        <button type="button" onClick={handleBluetoothSelect}>
          {t("settings.select_bluetooth")}
        </button>
        {bluetoothDevice && (
          <span style={{ marginLeft: 12 }}>
            {t("settings.selected")} {bluetoothDevice}
          </span>
        )}
      </section>
      <section style={{ marginTop: 16 }}>
        <Slider
          id="rssi-slider"
          label={t("settings.rssi_sensitivity")}
          value={localRssi}
          min={1}
          max={60}
          onChange={setLocalRssi}
          style={{ width: 200 }}
          unit="dBm"
        />
      </section>
    </main>
  );
};

export default SettingsView;
