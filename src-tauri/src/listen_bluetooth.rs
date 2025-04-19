use bt_discover::*;
use futures::stream::StreamExt;
use serde_json::json;
use tauri::Emitter;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

#[tauri::command]
pub async fn listen_bluetooth(
    app_handle: tauri::AppHandle,
    target_uuid: String,
    rssi_delta_max: i16,
) -> Result<(), String> {
    let target_uuid = Uuid::parse_str(&target_uuid).map_err(|e| e.to_string())?;
    let mut device_stream = discover_bluetooth_devices(Some(target_uuid))
        .await
        .map_err(|e| e.to_string())?;
    let mut initial_rssi = 0;

    loop {
        let next_event = timeout(Duration::from_secs(15), device_stream.next())
            .await
            .map_err(|e| e.to_string())?;
        let device = match next_event {
            Some(device) => device,
            _ => break,
        };

        let rssi = match device.rssi {
            Some(rssi) => rssi,
            None => continue,
        };

        if initial_rssi == 0 {
            initial_rssi = rssi;
        }

        let diff_rssi = rssi - initial_rssi;
        if rssi_delta_max + diff_rssi > 0 {
            let event_type = device.event_type;
            let local_name = device.local_name.unwrap_or_else(|| "Unknown".to_string());
            let id = device.id;
            let rssi = device.rssi.unwrap_or(0);

            println!(
                "{}: {:?} (id: {}, RSSI: {:?} dBm, diff: {:?} dBm)",
                event_type, local_name, id, rssi, diff_rssi
            );
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
            break;
        }
    }
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
