use bt_discover::*;
use futures::stream::StreamExt;
use log::error;
use serde_json::json;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio::task::JoinHandle;
use tokio::time::{timeout, Duration};
use uuid::Uuid;
// This is a wrapper around a Mutex to allow for the Bluetooth listener to be stopped
pub struct BluetoothListenerHandle(pub Mutex<Option<JoinHandle<()>>>);

#[tauri::command(rename_all = "snake_case")]
pub async fn listen_bluetooth(
    app_handle: AppHandle,
    state: State<'_, BluetoothListenerHandle>,
    target_uuid: Option<String>,
    rssi_delta_max: Option<i16>,
) -> Result<(), String> {
    // Stop previous listener if any
    if let Some(handle) = state.0.lock().unwrap().take() {
        handle.abort();
    }

    // Prepare arguments
    let target_uuid = match target_uuid {
        Some(uuid_str) => Some(Uuid::parse_str(&uuid_str).map_err(|e| e.to_string())?),
        None => None,
    };

    // Clone for move into async task
    let app_handle_clone = app_handle.clone();

    // Spawn the new listener task
    let handle = tokio::spawn(async move {
        let refresh_time_out = || {
            let _ = app_handle_clone.emit("bluetooth-refresh-timeout", {});
        };
        let over_delta_rssi = |diff_rssi: i16| {
            let _ = app_handle_clone.emit(
                "bluetooth-over-delta-rssi",
                json!({ "diff_rssi": diff_rssi }),
            );
        };
        let mut device_stream = match discover_bluetooth_devices(target_uuid).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Error discovering bluetooth devices: {}", e);
                return;
            }
        };
        let mut initial_rssi: Option<i16> = None;

        loop {
            let next_event = timeout(Duration::from_secs(15), device_stream.next()).await;

            let device = match next_event {
                Ok(Some(device)) => device,
                Ok(None) | Err(_) => {
                    refresh_time_out();
                    continue;
                }
            };

            let rssi = match device.rssi {
                Some(rssi) => rssi,
                None => continue,
            };

            if initial_rssi.is_none() {
                initial_rssi = Some(rssi);
            }

            let diff_rssi = rssi - initial_rssi.unwrap();
            if rssi_delta_max.map_or(true, |delta_max| delta_max + diff_rssi > 0) {
                let event_type = device.event_type;
                let id = device.id;
                let local_name = device.local_name.unwrap_or_else(|| id.clone());

                let _ = app_handle_clone.emit(
                    "bluetooth-event",
                    json!({
                        "event_type": event_type,
                        "local_name": local_name,
                        "id": id,
                        "rssi": rssi,
                        "diff_rssi": diff_rssi
                    }),
                );
            } else {
                over_delta_rssi(diff_rssi);
            }
        }
    });

    // Store the new handle
    *state.0.lock().unwrap() = Some(handle);

    Ok(())
}
