use bt_discover::*;
use futures::stream::StreamExt;
use futures::Stream;
use log::error;
use serde_json::json;
use std::pin::Pin;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio::task::JoinHandle;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

// This is a wrapper around a Mutex to allow for the Bluetooth listener to be stopped
pub struct BluetoothListenerHandle(pub Mutex<Option<JoinHandle<()>>>);

async fn get_bluetooth_stream(
    target_uuid: Option<Uuid>,
) -> Result<Pin<Box<dyn Stream<Item = DiscoveredDevice> + Send>>, String> {
    discover_bluetooth_devices(target_uuid)
        .await
        .map_err(|e| e.to_string())
}

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

        let mut initial_rssi: Option<i16> = None;
        let mut successives_timeout = 0;

        // Main loop to restart the stream if it stops
        loop {
            let mut device_stream = match get_bluetooth_stream(target_uuid).await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("Error discovering bluetooth devices: {}", e);
                    // Wait a bit before trying again
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }
            };

            // Inner loop to process the current stream
            loop {
                let next_event = timeout(Duration::from_secs(10), device_stream.next()).await;

                match next_event {
                    Ok(Some(device)) => {
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
                    _ => {
                        // the first timeout reload a new stream by exiting the inner loop, the second timeout throw the event refresh_time_out
                        successives_timeout += 1;
                        if successives_timeout <= 1 {
                            break;
                        } else {
                            successives_timeout = 0;
                            refresh_time_out();
                        }
                    }
                }
            }

            // Wait a short moment before restarting the stream
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // Store the new handle
    *state.0.lock().unwrap() = Some(handle);

    Ok(())
}
