import React, { useState } from 'react';
import Dropdown from '../../components/Dropdown';
import ThemeSwitch from '../../components/ThemeSwitch';
import { useTranslation } from 'react-i18next';
import Slider from '../../components/Slider';
import { useSettingsStore } from '../../store/settingsStore';
import { useDebounce } from '../../hooks/useDebounce';
import { Button } from '../../components/Button';
import BackToMain from '../../components/Button/BackToMain';
import FormattedText from '../../components/FormattedText';
import { useAppStore } from '../../store/appStore';
import { useNavigate } from 'react-router';
import styles from './style.module.css';
const languageOptions = [
  { value: 'en', label: 'English' },
  { value: 'fr', label: 'Français' }
];

const SettingsView: React.FC = () => {
  const { t, i18n } = useTranslation();
  const { settings, setSettings } = useSettingsStore();
  const { setIsDiscoveryMode } = useAppStore();
  const navigate = useNavigate();
  const [localRssi, setLocalRssi] = useState(settings.rssi_delta_max);
  const debouncedRssi = useDebounce(localRssi, 400);
  const settingsTitleText = t('settings.title');
  const languageText = t('settings.language');
  const selectBluetoothText = t('settings.select_bluetooth');
  const rssiSensitivityText = t('settings.rssi_sensitivity');

  React.useEffect(() => {
    setLocalRssi(settings.rssi_delta_max);
  }, [settings.rssi_delta_max]);

  React.useEffect(() => {
    if (debouncedRssi !== settings.rssi_delta_max) {
      setSettings({ rssi_delta_max: debouncedRssi });
    }
  }, [debouncedRssi, setSettings, settings.rssi_delta_max]);

  const handleLanguageChange = (lang: string) => {
    i18n.changeLanguage(lang);
    setSettings({ language: lang });
  };

  const handleBluetoothSelect = () => {
    setIsDiscoveryMode(true);
    setSettings({ target_uuid: '' });
    navigate('/');
  };

  const title = (
    <div className={styles.title}>
      <FormattedText style={{ fontSize: 24, fontWeight: 'bold' }}>
        {settingsTitleText}
      </FormattedText>
    </div>
  );

  const languageDropdown = (
    <Dropdown
      label={languageText}
      options={languageOptions}
      value={settings.language}
      onChange={handleLanguageChange}
    />
  );

  const chooseBluetoothDeviceButton = (
    <Button noBorder height={1.6} text={selectBluetoothText} onPress={handleBluetoothSelect} />
  );

  const rssiSlider = (
    <Slider
      id="rssi-slider"
      label={rssiSensitivityText}
      value={localRssi ?? 0}
      min={1}
      max={60}
      onChange={setLocalRssi}
      style={{ width: 200 }}
      unit="dBm"
    />
  );

  return (
    <>
      <header className={styles.header}>
        <div className={styles.back_to_main}>
          <BackToMain />
        </div>
        {title}
      </header>
      <main className={styles.main}>
        <section>
          {chooseBluetoothDeviceButton}
          {rssiSlider}
        </section>
        <section>
          <ThemeSwitch />
          {languageDropdown}
        </section>
      </main>
    </>
  );
};

export default SettingsView;
