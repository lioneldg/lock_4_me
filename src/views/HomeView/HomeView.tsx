import styles from './style.module.css';
import React, { useState } from 'react';
import { DiscoveredDevice } from '../../types';
import { useBluetoothStore } from '../../store/bluetoothStore';
import { useTranslation } from 'react-i18next';
import { useSettingsStore } from '../../store/settingsStore';
import Icon from '../../components/Icon';
import { ICON_TYPE } from '../../components/Icon/Icon';
import { useNavigate } from 'react-router';
import FormattedText from '../../components/FormattedText';
import { useAppStore } from '../../store/appStore';

const HomeView: React.FC = () => {
  const { events, clearEvents } = useBluetoothStore();
  const { t } = useTranslation();
  const { settings, setSettings } = useSettingsStore();
  const { isDiscoveryMode, setIsDiscoveryMode } = useAppStore();
  const navigate = useNavigate();
  const [hoveredDevice, setHoveredDevice] = useState<string | null>(null);
  const homeTitleText = t('home.title');
  const discoveryModeText = t('home.discoveryMode');
  const targetModeText = t('home.targetMode', { uuid: settings.target_uuid });

  function selectDevice(device: DiscoveredDevice) {
    if (isDiscoveryMode) {
      clearEvents();
      setSettings({ target_uuid: device.id });
      setIsDiscoveryMode(false);
    }
  }

  const settingsButton = (
    <div
      className={styles.settings_icon}
      onClick={() => {
        navigate('/settings');
      }}
    >
      <Icon type={ICON_TYPE.SETTINGS} />
    </div>
  );

  const title = (
    <div className={styles.title}>
      <FormattedText style={{ fontSize: 24, fontWeight: 'bold' }}>{homeTitleText}</FormattedText>
    </div>
  );

  const modeInfo = (
    <div className={styles.more_info}>
      <FormattedText>{isDiscoveryMode ? discoveryModeText : targetModeText}</FormattedText>
    </div>
  );

  const deviceList = (
    <div className={styles.device_list}>
      <ul>
        {Array.from(events.values()).map((discoveredDevice) => (
          <div
            key={discoveredDevice.id}
            onClick={() => selectDevice(discoveredDevice)}
            className="button"
            onMouseEnter={() => setHoveredDevice(discoveredDevice.id)}
            onMouseLeave={() => setHoveredDevice(null)}
            style={
              isDiscoveryMode
                ? {
                    cursor: 'pointer',
                    transition: 'transform 0.2s ease',
                    transform: hoveredDevice === discoveredDevice.id ? 'scale(1.1)' : 'scale(1)'
                  }
                : {}
            }
          >
            <FormattedText>
              {discoveredDevice.local_name} {'=>'} RSSI: {discoveredDevice.rssi} dBm
              {!isDiscoveryMode && `, Δ: ${discoveredDevice.diff_rssi} dBm`}
            </FormattedText>
          </div>
        ))}
      </ul>
    </div>
  );

  return (
    <>
      <header className={styles.header}>
        {title}
        {settingsButton}
      </header>
      <main>
        {modeInfo}
        {deviceList}
      </main>
    </>
  );
};

export default HomeView;
