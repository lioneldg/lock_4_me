use bt_discover::*;
use futures::stream::StreamExt;
use futures::Stream;
use log::{error, info};
use serde_json::json;
use std::fmt;
use std::pin::Pin;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio::task::JoinHandle;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

// Custom error structure
#[derive(Debug)]
pub enum BluetoothError {
    Discovery(String),
    UuidParse(uuid::Error),
}

impl fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discovery(e) => write!(f, "Bluetooth discovery error: {}", e),
            Self::UuidParse(e) => write!(f, "UUID parsing error: {}", e),
        }
    }
}

impl std::error::Error for BluetoothError {}

impl From<uuid::Error> for BluetoothError {
    fn from(err: uuid::Error) -> Self {
        BluetoothError::UuidParse(err)
    }
}

// This is a wrapper around a Mutex to allow for the Bluetooth listener to be stopped
pub struct BluetoothListenerHandle(pub Mutex<Option<JoinHandle<()>>>);

async fn get_bluetooth_stream(
    target_uuid: Option<Uuid>,
) -> Result<Pin<Box<dyn Stream<Item = DiscoveredDevice> + Send>>, BluetoothError> {
    discover_bluetooth_devices(target_uuid)
        .await
        .map_err(|e| BluetoothError::Discovery(e.to_string()))
}

/// Process a discovered device, handling RSSI and emitting events
fn process_device(
    app_handle: &AppHandle,
    device: DiscoveredDevice,
    initial_rssi: &mut Option<i16>,
    rssi_delta_max: Option<i16>,
) -> bool {
    let rssi = match device.rssi {
        Some(rssi) => rssi,
        None => return false,
    };

    if initial_rssi.is_none() {
        *initial_rssi = Some(rssi);
    }

    let diff_rssi = rssi - initial_rssi.unwrap();
    if rssi_delta_max.map_or(true, |delta_max| delta_max + diff_rssi > 0) {
        let event_type = device.event_type;
        let id = device.id;
        let local_name = device.local_name.unwrap_or_else(|| id.clone());

        let _ = app_handle.emit(
            "bluetooth-event",
            json!({
                "event_type": event_type,
                "local_name": local_name,
                "id": id,
                "rssi": rssi,
                "diff_rssi": diff_rssi
            }),
        );
        true
    } else {
        info!("Over delta rssi: {}", diff_rssi);
        let _ = app_handle.emit(
            "bluetooth-over-delta-rssi",
            json!({ "diff_rssi": diff_rssi }),
        );
        false
    }
}

async fn run_bluetooth_listener(
    app_handle: AppHandle,
    target_uuid: Option<Uuid>,
    rssi_delta_max: Option<i16>,
) {
    const TIMEOUT_DURATION: Duration = Duration::from_secs(15);
    const REFRESH_BACKOFF: Duration = Duration::from_secs(1);
    const ERROR_BACKOFF: Duration = Duration::from_secs(3);

    let mut initial_rssi: Option<i16> = None;
    let mut successives_timeout = 0;

    // Main loop to restart the stream if it stops
    loop {
        let mut device_stream = match get_bluetooth_stream(target_uuid).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Error discovering bluetooth devices: {}", e);
                tokio::time::sleep(ERROR_BACKOFF).await;
                continue;
            }
        };

        // Inner loop to process the current stream
        loop {
            let next_event = timeout(TIMEOUT_DURATION, device_stream.next()).await;

            match next_event {
                Ok(Some(device)) => {
                    process_device(&app_handle, device, &mut initial_rssi, rssi_delta_max);
                    successives_timeout = 0; // Reset timeout counter on successful events
                }
                _ => {
                    // The first timeout reloads a new stream by exiting the inner loop
                    // The second consecutive timeout emits the refresh timeout event
                    successives_timeout += 1;
                    if successives_timeout <= 1 {
                        break;
                    } else {
                        successives_timeout = 0;
                        info!("Refresh time out");
                        let _ = app_handle.emit("bluetooth-refresh-timeout", {});
                    }
                }
            }
        }

        // Wait before restarting the stream
        tokio::time::sleep(REFRESH_BACKOFF).await;
    }
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

    // Parse UUID if provided
    let target_uuid = match target_uuid {
        Some(uuid_str) => Some(Uuid::parse_str(&uuid_str).map_err(|e| e.to_string())?),
        None => None,
    };

    // Spawn the new listener task
    let handle = tokio::spawn(async move {
        run_bluetooth_listener(app_handle.clone(), target_uuid, rssi_delta_max).await
    });

    // Store the new handle
    *state.0.lock().unwrap() = Some(handle);

    Ok(())
}
