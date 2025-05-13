import styles from './style.module.css';
import React, { useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { DiscoveredDevice } from '../../types';
import { useBluetoothStore } from '../../store/bluetoothStore';
import { useTranslation } from 'react-i18next';
import { useSettingsStore } from '../../store/settingsStore';
import Icon from '../../components/Icon';
import { ICON_TYPE } from '../../components/Icon/Icon';
import { useNavigate } from 'react-router';
import FormattedText from '../../components/FormattedText';
import { useAppStore } from '../../store/appStore';

async function listen_bluetooth(targetUuid?: string, rssiDeltaMax?: number) {
  await invoke('listen_bluetooth', {
    target_uuid: targetUuid,
    rssi_delta_max: rssiDeltaMax
  });
}

async function lockScreen() {
  await invoke('lock_screen');
}

const HomeView: React.FC = () => {
  const { addEvent, events, clearEvents } = useBluetoothStore();
  const { t } = useTranslation();
  const { settings, setSettings } = useSettingsStore();
  const { isDiscoveryMode, setIsDiscoveryMode } = useAppStore();
  const navigate = useNavigate();

  const { target_uuid, rssi_delta_max } = useMemo(
    () => ({
      target_uuid: settings.target_uuid || undefined,
      rssi_delta_max: settings.target_uuid ? settings.rssi_delta_max : undefined
    }),
    [settings.target_uuid, settings.rssi_delta_max]
  );

  useEffect(() => {
    listen_bluetooth(target_uuid, rssi_delta_max);

    const unlistenBTEventPromise = listen('bluetooth-event', (event) =>
      addEvent(event.payload as DiscoveredDevice)
    );
    const unlistenBTClosedPromise = listen('bluetooth-listener-closed', () => {
      lockScreen();
    });
    return () => {
      unlistenBTEventPromise.then((unlisten) => unlisten());
      unlistenBTClosedPromise.then((unlisten) => unlisten());
    };
  }, [target_uuid, rssi_delta_max, addEvent]);

  function selectDevice(device: DiscoveredDevice) {
    if (isDiscoveryMode) {
      clearEvents();
      setSettings({ target_uuid: device.id });
      setIsDiscoveryMode(false);
      navigate('/settings');
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
      <FormattedText style={{ fontSize: 24, fontWeight: 'bold' }}>{t('home.title')}</FormattedText>
    </div>
  );

  const modeInfo = (
    <div className={styles.more_info}>
      <FormattedText>
        {isDiscoveryMode
          ? t('home.discoveryMode')
          : t('home.targetMode', { uuid: settings.target_uuid })}
      </FormattedText>
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
            style={isDiscoveryMode ? { cursor: 'pointer' } : {}}
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
