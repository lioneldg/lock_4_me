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

  return (
    <main>
      <BackToMain />
      <FormattedText style={{ fontSize: 24, fontWeight: 'bold' }}>
        {t('settings.title')}
      </FormattedText>
      <section>
        <ThemeSwitch />
      </section>
      <section style={{ marginTop: 16 }}>
        <Dropdown
          label={t('settings.language')}
          options={languageOptions}
          value={settings.language}
          onChange={handleLanguageChange}
        />
      </section>
      <section style={{ marginTop: 16 }}>
        <Button width={10} text={t('settings.select_bluetooth')} onPress={handleBluetoothSelect} />
      </section>
      <section style={{ marginTop: 16 }}>
        <Slider
          id="rssi-slider"
          label={t('settings.rssi_sensitivity')}
          value={localRssi ?? 0}
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
