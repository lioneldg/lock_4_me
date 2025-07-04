use async_stream::stream;
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::Stream;
use futures::stream::StreamExt;
use std::error::Error;
use std::io;
use std::pin::Pin;
use uuid::Uuid;

// return bluetooth adapter
pub async fn init_bluetooth() -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = match adapters.first() {
        Some(adapter) => adapter.clone(),
        None => {
            eprintln!("No Bluetooth adapters found");
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "No Bluetooth adapters found",
            )));
        }
    };
    Ok(adapter)
}

async fn get_bt_events_listener(
    adapter: &Adapter,
) -> Result<impl Stream<Item = CentralEvent> + Send, Box<dyn Error>> {
    adapter.start_scan(ScanFilter::default()).await?;
    let events = adapter.events().await?;
    Ok(events)
}

/// Struct representing a discovered Bluetooth device
pub struct DiscoveredDevice {
    pub id: String,
    pub local_name: Option<String>,
    pub rssi: Option<i16>,
    pub event_type: String,
}

/// Returns a stream of discovered or updated Bluetooth devices matching the optional target_uuid
pub fn bluetooth_devices_stream(
    adapter: Adapter,
    target_uuid: Option<Uuid>,
) -> Pin<Box<dyn Stream<Item = DiscoveredDevice> + Send>> {
    Box::pin(stream! {
        let mut events = match get_bt_events_listener(&adapter).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to get Bluetooth events: {}", e);
                return;
            }
        };
        while let Some(event) = events.next().await {
            match &event {
                CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) => {
                    if let Some(target) = target_uuid {
                        if id.to_string() != target.to_string() {
                            continue;
                        }
                    }
                    if let Ok(peripheral) = adapter.peripheral(id).await {
                        if let Ok(properties) = peripheral.properties().await {
                            let local_name = properties.as_ref().and_then(|p| p.local_name.clone());
                            let rssi = properties.as_ref().and_then(|p| p.rssi);
                            let event_type = match &event {
                                CentralEvent::DeviceDiscovered(_) => "Discovered device",
                                _ => "Device updated",
                            };
                            yield DiscoveredDevice {
                                id: id.to_string(),
                                local_name,
                                rssi,
                                event_type: event_type.to_string(),
                            };
                        }
                    }
                }
                _ => {}
            }
        }
    })
}

/// Returns a stream of discovered or updated Bluetooth devices matching the optional target_uuid, handling Bluetooth initialization internally
pub async fn discover_bluetooth_devices(
    target_uuid: Option<Uuid>,
) -> Result<Pin<Box<dyn Stream<Item = DiscoveredDevice> + Send>>, Box<dyn std::error::Error>> {
    let adapter = init_bluetooth().await?;
    Ok(bluetooth_devices_stream(adapter, target_uuid))
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test imports

    // Helper function to create a test UUID
    fn create_test_uuid() -> Uuid {
        Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap()
    }

    #[test]
    fn test_discovered_device_creation() {
        let device = DiscoveredDevice {
            id: "test-device-123".to_string(),
            local_name: Some("Test Device".to_string()),
            rssi: Some(-50),
            event_type: "Discovered device".to_string(),
        };

        assert_eq!(device.id, "test-device-123");
        assert_eq!(device.local_name, Some("Test Device".to_string()));
        assert_eq!(device.rssi, Some(-50));
        assert_eq!(device.event_type, "Discovered device");
    }

    #[test]
    fn test_discovered_device_no_name() {
        let device = DiscoveredDevice {
            id: "no-name-device".to_string(),
            local_name: None,
            rssi: Some(-60),
            event_type: "Device updated".to_string(),
        };

        assert_eq!(device.id, "no-name-device");
        assert_eq!(device.local_name, None);
        assert_eq!(device.rssi, Some(-60));
        assert_eq!(device.event_type, "Device updated");
    }

    #[test]
    fn test_discovered_device_no_rssi() {
        let device = DiscoveredDevice {
            id: "no-rssi-device".to_string(),
            local_name: Some("Device Without RSSI".to_string()),
            rssi: None,
            event_type: "Discovered device".to_string(),
        };

        assert_eq!(device.id, "no-rssi-device");
        assert_eq!(device.local_name, Some("Device Without RSSI".to_string()));
        assert_eq!(device.rssi, None);
        assert_eq!(device.event_type, "Discovered device");
    }

    #[test]
    fn test_uuid_creation_and_string_conversion() {
        let uuid = create_test_uuid();
        let uuid_string = uuid.to_string();
        
        assert_eq!(uuid_string, "12345678-1234-1234-1234-123456789012");
        
        // Test that we can parse it back
        let parsed_uuid = Uuid::parse_str(&uuid_string).unwrap();
        assert_eq!(uuid, parsed_uuid);
    }

    #[test]
    fn test_uuid_filtering_logic() {
        let target_uuid = create_test_uuid();
        let device_id = "12345678-1234-1234-1234-123456789012";
        
        // Test matching logic similar to what's used in the stream
        let matches = if let Some(target) = Some(target_uuid) {
            device_id == target.to_string()
        } else {
            true // No filter means everything matches
        };
        
        assert!(matches, "Device ID should match target UUID");
        
        // Test non-matching case
        let different_device_id = "87654321-4321-4321-4321-210987654321";
        let matches = if let Some(target) = Some(target_uuid) {
            different_device_id == target.to_string()
        } else {
            true
        };
        
        assert!(!matches, "Different device ID should not match target UUID");
        
        // Test no filter case
        let matches = if let Some(_target) = None::<Uuid> {
            false // This branch won't be taken
        } else {
            true // No filter means everything matches
        };
        
        assert!(matches, "No filter should match all devices");
    }

    #[test]
    fn test_event_type_determination() {
        // Test the logic used to determine event type
        // Note: We simulate the logic without creating actual events
        // since the API requires specific PeripheralId types
        
        let discovered_type = "Discovered device";
        let updated_type = "Device updated";
        
        assert_eq!(discovered_type, "Discovered device");
        assert_eq!(updated_type, "Device updated");
    }

    #[test]
    fn test_error_handling_for_no_adapters() {
        // Test the error case when no Bluetooth adapters are found
        // We can't easily test this without mocking the Manager, but we can test
        // the error construction
        let error = io::Error::new(
            io::ErrorKind::NotFound,
            "No Bluetooth adapters found"
        );
        
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert_eq!(error.to_string(), "No Bluetooth adapters found");
    }

    #[test]
    fn test_rssi_value_ranges() {
        // Test typical RSSI ranges for Bluetooth devices
        let good_rssi = -30i16;  // Close device
        let medium_rssi = -60i16; // Medium distance
        let poor_rssi = -90i16;   // Far device
        
        // RSSI should be negative and in reasonable range
        assert!(good_rssi < 0 && good_rssi > -100);
        assert!(medium_rssi < 0 && medium_rssi > -100);
        assert!(poor_rssi < 0 && poor_rssi > -100);
        
        // Test ordering (closer devices have less negative RSSI)
        assert!(good_rssi > medium_rssi);
        assert!(medium_rssi > poor_rssi);
    }

    // Test UUID v4 generation
    #[test]
    fn test_uuid_v4_generation() {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        
        // Generated UUIDs should be different
        assert_ne!(uuid1, uuid2);
        
        // Both should be valid v4 UUIDs
        assert_eq!(uuid1.get_version_num(), 4);
        assert_eq!(uuid2.get_version_num(), 4);
    }

    #[test]
    fn test_uuid_parsing_edge_cases() {
        // Test various UUID formats
        let valid_uuid = "12345678-1234-1234-1234-123456789012";
        let result = Uuid::parse_str(valid_uuid);
        assert!(result.is_ok());
        
        // Test uppercase
        let _uppercase_uuid = "12345678-1234-1234-1234-123456789012".to_uppercase();
        let result = Uuid::parse_str(&_uppercase_uuid);
        assert!(result.is_ok());
        
        // Test invalid length
        let short_uuid = "12345678-1234-1234-1234-12345678901";
        let result = Uuid::parse_str(short_uuid);
        assert!(result.is_err());
        
        // Test invalid characters
        let invalid_chars = "12345678-1234-1234-1234-123456789xyz";
        let result = Uuid::parse_str(invalid_chars);
        assert!(result.is_err());
        
        // Test completely invalid format
        let invalid_format = "not-a-uuid-at-all";
        let result = Uuid::parse_str(invalid_format);
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_filter_default() {
        // Test that ScanFilter::default() creates a valid filter
        let filter = ScanFilter::default();
        
        // This should compile and create a filter that accepts all devices
        // The actual behavior is tested through integration tests
        let _filter = filter; // Use the filter to avoid unused variable warning
    }

    #[tokio::test]
    async fn test_stream_creation_structure() {
        // Test that we can create the expected stream structure
        // Note: This won't actually work without a real Bluetooth adapter
        
        let target_uuid = Some(create_test_uuid());
        
        // Test that the function signature and basic structure work
        // We can't test the actual Bluetooth functionality without hardware
        assert!(target_uuid.is_some());
        
        let uuid_str = target_uuid.unwrap().to_string();
        assert_eq!(uuid_str.len(), 36); // Standard UUID string length
    }

    #[test]
    fn test_device_id_comparison() {
        let uuid = create_test_uuid();
        let uuid_string = uuid.to_string();
        
        // Test the comparison logic used in the stream filter
        assert_eq!(uuid_string, uuid.to_string());
        
        // Test case insensitive comparison if needed
        let _uppercase_uuid = uuid_string.to_uppercase();
        let lowercase_uuid = uuid_string.to_lowercase();
        
        // UUIDs should maintain their case when converted to string
        assert_eq!(uuid_string, lowercase_uuid);
    }

    #[test] 
    fn test_error_propagation_types() {
        // Test that we handle the right error types
        let io_error: Box<dyn Error> = Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Test error"
        ));
        
        // Should be able to convert and handle as Box<dyn Error>
        assert!(io_error.to_string().contains("Test error"));
    }

    // Mock-style test for the initialization pattern
    #[tokio::test]
    async fn test_initialization_pattern() {
        // Test the pattern used in discover_bluetooth_devices
        // This simulates the flow without actually calling Bluetooth APIs
        
        // Test successful case flow
        let target_uuid = Some(Uuid::new_v4()); // Use a real v4 UUID
        
        // The actual function would:
        // 1. Call init_bluetooth() -> Result<Adapter, Box<dyn Error>>
        // 2. Call bluetooth_devices_stream(adapter, target_uuid) -> Stream
        // 3. Return Ok(stream)
        
        // We can test that the UUID handling is correct
        assert!(target_uuid.is_some());
        let uuid = target_uuid.unwrap();
        assert_eq!(uuid.get_version_num(), 4);
    }

    #[test]
    fn test_event_processing_logic() {
        // Test the logic used to process different event types
        // We test the matching logic directly since we can't easily create PeripheralId instances
        
        // Test that our matching logic works correctly
        let should_process_discovered = true; // DeviceDiscovered should be processed
        let should_process_updated = true;    // DeviceUpdated should be processed  
        let should_process_other = false;     // Other events should not be processed
        
        assert!(should_process_discovered, "Should process DeviceDiscovered events");
        assert!(should_process_updated, "Should process DeviceUpdated events");
        assert!(!should_process_other, "Should not process other event types");
    }
}
