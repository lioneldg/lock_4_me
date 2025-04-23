import React, { useState } from "react";
import Dropdown from "../../components/Dropdown";
import ThemeSwitch from "../../components/ThemeSwitch";
import { useTranslation } from "react-i18next";

const languageOptions = [
  { value: "en", label: "English" },
  { value: "fr", label: "Français" },
];

const SettingsView: React.FC = () => {
  const { t, i18n } = useTranslation();
  const [language, setLanguage] = useState(i18n.language || "en");
  const [bluetoothDevice, setBluetoothDevice] = useState<string | null>(null);

  const handleLanguageChange = (lang: string) => {
    setLanguage(lang);
    i18n.changeLanguage(lang);
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
          value={language}
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
    </main>
  );
};

export default SettingsView;
