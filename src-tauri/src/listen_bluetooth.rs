use bt_discover::*;
use futures::stream::StreamExt;
use serde_json::json;
use tauri::Emitter;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

#[tauri::command(rename_all = "snake_case")]
pub async fn listen_bluetooth(
    app_handle: tauri::AppHandle,
    target_uuid: Option<String>,
    rssi_delta_max: Option<i16>,
) -> Result<(), String> {
    let target_uuid = match target_uuid {
        Some(uuid_str) => Some(Uuid::parse_str(&uuid_str).map_err(|e| e.to_string())?),
        None => None,
    };
    let mut device_stream = discover_bluetooth_devices(target_uuid)
        .await
        .map_err(|e| e.to_string())?;
    let mut initial_rssi = 0;

    loop {
        let exit_loop = || {
            app_handle
                .emit("bluetooth-listener-closed", json!({}))
                .unwrap();
            return true;
        };

        let next_event = timeout(Duration::from_secs(15), device_stream.next()).await;

        let device = match next_event {
            Ok(Some(device)) => device,
            Ok(None) | Err(_) => {
                exit_loop();
                break;
            }
        };

        let rssi = match device.rssi {
            Some(rssi) => rssi,
            None => continue,
        };

        if initial_rssi == 0 {
            initial_rssi = rssi;
        }

        let diff_rssi = rssi - initial_rssi;
        if rssi_delta_max.map_or(true, |delta_max| delta_max + diff_rssi > 0) {
            let event_type = device.event_type;
            let local_name = device.local_name.unwrap_or_else(|| "Unknown".to_string());
            let id = device.id;
            let rssi = device.rssi.unwrap_or(0);

            app_handle
                .emit(
                    "bluetooth-event",
                    json!({
                        "event_type": event_type,
                        "local_name": local_name,
                        "id": id,
                        "rssi": rssi,
                        "diff_rssi": diff_rssi
                    }),
                )
                .unwrap();
        } else {
            exit_loop();
            break;
        }
    }
    Ok(())
}
