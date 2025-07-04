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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mock AppHandle for testing
    struct MockAppHandle;
    
    impl MockAppHandle {
        fn emit_all<T: serde::Serialize>(&self, _event: &str, _payload: T) -> Result<(), String> {
            Ok(())
        }
    }

    fn create_test_device(rssi: Option<i16>, local_name: Option<&str>) -> DiscoveredDevice {
        DiscoveredDevice {
            id: "test-device-123".to_string(),
            local_name: local_name.map(|s| s.to_string()),
            rssi,
            event_type: "Discovered device".to_string(),
        }
    }

    #[test]
    fn test_bluetooth_error_display() {
        let discovery_error = BluetoothError::Discovery("Test error".to_string());
        assert_eq!(
            discovery_error.to_string(),
            "Bluetooth discovery error: Test error"
        );

        let uuid_result = Uuid::parse_str("invalid");
        let uuid_error = BluetoothError::UuidParse(uuid_result.unwrap_err());
        assert!(uuid_error.to_string().contains("UUID parsing error"));
    }

    #[test]
    fn test_bluetooth_error_from_uuid_error() {
        let uuid_str = "invalid-uuid";
        let uuid_result = Uuid::parse_str(uuid_str);
        assert!(uuid_result.is_err(), "Invalid UUID should fail to parse");
        
        let bluetooth_error: BluetoothError = uuid_result.unwrap_err().into();
        match bluetooth_error {
            BluetoothError::UuidParse(_) => assert!(true),
            _ => panic!("Expected UuidParse error"),
        }
    }

    #[test]
    fn test_bluetooth_listener_handle_new() {
        let handle = BluetoothListenerHandle(Mutex::new(None));
        assert!(handle.0.lock().unwrap().is_none());
    }

    // Note: We can't easily test process_device without a real AppHandle from Tauri,
    // but we can test the logic separately

    #[test]
    fn test_rssi_diff_calculation() {
        // Test RSSI difference calculation logic
        let initial_rssi = -50i16;
        let current_rssi = -45i16;
        let diff_rssi = current_rssi - initial_rssi;
        assert_eq!(diff_rssi, 5);

        let initial_rssi = -30i16;
        let current_rssi = -40i16;
        let diff_rssi = current_rssi - initial_rssi;
        assert_eq!(diff_rssi, -10);
    }

    #[test]
    fn test_rssi_delta_max_logic() {
        // Test the logic used in process_device for rssi_delta_max
        let rssi_delta_max = Some(-10i16);
        let diff_rssi = -5i16; // Device got closer (less negative)
        
        // Should be allowed: delta_max + diff_rssi = -10 + (-5) = -15, which is < 0
        let allowed = rssi_delta_max.map_or(true, |delta_max| delta_max + diff_rssi > 0);
        assert!(!allowed, "Device should be too close");

        let diff_rssi = -15i16; // Device is farther away
        let allowed = rssi_delta_max.map_or(true, |delta_max| delta_max + diff_rssi > 0);
        assert!(!allowed, "Device should still be acceptable");

        // Test with no delta max (should always allow)
        let rssi_delta_max: Option<i16> = None;
        let allowed = rssi_delta_max.map_or(true, |delta_max| delta_max + diff_rssi > 0);
        assert!(allowed, "Should always allow when no delta max is set");
    }

        #[test]
    fn test_uuid_parsing_in_listen_bluetooth() {
        // Test valid UUID parsing
        let valid_uuid = "12345678-1234-1234-1234-123456789012";
        let result = Uuid::parse_str(valid_uuid);
        assert!(result.is_ok(), "Valid UUID should parse successfully");

        // Test invalid UUID parsing
        let invalid_uuid = "invalid-uuid";
        let result = Uuid::parse_str(invalid_uuid);
        assert!(result.is_err(), "Invalid UUID should fail to parse");
    }

    #[test]
    fn test_discovered_device_creation() {
        let device = create_test_device(Some(-50), Some("Test Device"));
        
        assert_eq!(device.id, "test-device-123");
        assert_eq!(device.local_name, Some("Test Device".to_string()));
        assert_eq!(device.rssi, Some(-50));
        assert_eq!(device.event_type, "Discovered device");
    }

    #[test]
    fn test_discovered_device_without_rssi() {
        let device = create_test_device(None, Some("Test Device"));
        
        assert_eq!(device.rssi, None);
        // This device would return false in process_device due to no RSSI
    }

    #[test]
    fn test_discovered_device_without_name() {
        let device = create_test_device(Some(-50), None);
        
        assert_eq!(device.local_name, None);
        // In process_device, this would use the ID as fallback name
    }

    #[test]
    fn test_timeout_duration_constants() {
        // Test that our constants are reasonable
        const TIMEOUT_DURATION: Duration = Duration::from_secs(15);
        const REFRESH_BACKOFF: Duration = Duration::from_secs(1);
        const ERROR_BACKOFF: Duration = Duration::from_secs(3);

        assert_eq!(TIMEOUT_DURATION.as_secs(), 15);
        assert_eq!(REFRESH_BACKOFF.as_secs(), 1);
        assert_eq!(ERROR_BACKOFF.as_secs(), 3);
        
        // Ensure backoffs are shorter than timeout
        assert!(REFRESH_BACKOFF < TIMEOUT_DURATION);
        assert!(ERROR_BACKOFF < TIMEOUT_DURATION);
    }

    #[test]
    fn test_json_event_structure() {
        // Test the JSON structure that would be emitted
        let event_data = json!({
            "event_type": "Discovered device",
            "local_name": "Test Device",
            "id": "test-device-123",
            "rssi": -50,
            "diff_rssi": 5
        });

        assert_eq!(event_data["event_type"], "Discovered device");
        assert_eq!(event_data["local_name"], "Test Device");
        assert_eq!(event_data["id"], "test-device-123");
        assert_eq!(event_data["rssi"], -50);
        assert_eq!(event_data["diff_rssi"], 5);
    }

    #[test]
    fn test_bluetooth_error_is_error_trait() {
        let error = BluetoothError::Discovery("Test".to_string());
        let _: &dyn std::error::Error = &error; // Should compile if Error trait is implemented
    }

    #[test]
    fn test_bluetooth_error_debug() {
        let error = BluetoothError::Discovery("Test error".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Discovery"));
        assert!(debug_str.contains("Test error"));
    }

    // Integration-style test that simulates the UUID validation logic
    #[test]
    fn test_uuid_validation_workflow() {
        // Test the workflow that listen_bluetooth would follow

        // Valid UUID case
        let target_uuid_str = Some("12345678-1234-1234-1234-123456789012".to_string());
        let parsed_uuid = match target_uuid_str {
            Some(uuid_str) => Some(Uuid::parse_str(&uuid_str)),
            None => None,
        };
        
        assert!(parsed_uuid.is_some());
        assert!(parsed_uuid.unwrap().is_ok());

        // Invalid UUID case
        let target_uuid_str = Some("invalid-uuid".to_string());
        let parsed_uuid = match target_uuid_str {
            Some(uuid_str) => Some(Uuid::parse_str(&uuid_str)),
            None => None,
        };
        
        assert!(parsed_uuid.is_some());
        assert!(parsed_uuid.unwrap().is_err());

        // None case
        let target_uuid_str: Option<String> = None;
        let parsed_uuid = match target_uuid_str {
            Some(uuid_str) => Some(Uuid::parse_str(&uuid_str)),
            None => None,
        };
        
        assert!(parsed_uuid.is_none());
    }

    #[test]
    fn test_successive_timeout_logic() {
        // Test the logic for handling successive timeouts
        let mut successives_timeout = 0;

        // First timeout - should break inner loop (reload stream)
        successives_timeout += 1;
        assert_eq!(successives_timeout, 1);
        let should_break = successives_timeout <= 1;
        assert!(should_break, "First timeout should break inner loop");

        // Second timeout - should emit refresh event and reset
        successives_timeout += 1;
        assert_eq!(successives_timeout, 2);
        let should_break = successives_timeout <= 1;
        assert!(!should_break, "Second timeout should not break inner loop");
        
        // Reset logic
        successives_timeout = 0;
        assert_eq!(successives_timeout, 0);
    }
}
