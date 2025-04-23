import React, { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DiscoveredDevice } from "../../types";
import { useBluetoothStore } from "../../store/bluetoothStore";
import { useTranslation } from "react-i18next";
import { useSettingsStore } from "../../store/settingsStore";

async function listen_bluetooth(targetUuid: string, rssiDeltaMax: number) {
  await invoke("listen_bluetooth", {
    target_uuid: targetUuid,
    rssi_delta_max: rssiDeltaMax,
  });
}

async function lockScreen() {
  await invoke("lock_screen");
}

const HomeView: React.FC = () => {
  const { addEvent, events } = useBluetoothStore();
  const { t } = useTranslation();
  const { settings } = useSettingsStore();

  useEffect(() => {
    listen_bluetooth(settings.target_uuid, settings.rssi_delta_max);
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
  }, [settings.target_uuid, settings.rssi_delta_max]);

  return (
    <main>
      <h1>{t("home.title")}</h1>
      <p>{t("home.description")}</p>
      <h2>Bluetooth Events</h2>
      <ul>
        {Array.from(events.values()).map((event) => (
          <p key={event.id}>
            {event.local_name} {"=>"} RSSI: {event.rssi} dBm, Δ:{" "}
            {event.diff_rssi} dBm
          </p>
        ))}
      </ul>
    </main>
  );
};

export default HomeView;
