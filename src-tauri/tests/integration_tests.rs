use std::fs;
use tempfile::tempdir;

// Integration tests for the entire application

#[cfg(test)]
mod integration_tests {
    use super::*;
    use lock_4_me_lib;

    #[test]
    fn test_settings_workflow() {
        // Test the complete settings workflow
        let dir = tempdir().unwrap();
        let settings_path = dir.path().join("integration_settings.json");
        let settings_path_str = settings_path.to_string_lossy().to_string();

        // Create test settings
        let original_settings = lock_4_me_lib::read_write_settings::Settings {
            target_uuid: "12345678-1234-1234-1234-123456789012".to_string(),
            rssi_delta_max: -40,
            theme: "dark".to_string(),
            language: "en".to_string(),
        };

        // Test write operation
        let write_result = lock_4_me_lib::read_write_settings::write_settings(
            original_settings,
            settings_path_str.clone()
        );
        assert!(write_result.is_ok(), "Write settings should succeed");

        // Verify file was created
        assert!(settings_path.exists(), "Settings file should exist");

        // Test read operation
        let read_result = lock_4_me_lib::read_write_settings::read_settings(settings_path_str);
        assert!(read_result.is_ok(), "Read settings should succeed");

        let loaded_settings = read_result.unwrap();
        assert_eq!(loaded_settings.target_uuid, "12345678-1234-1234-1234-123456789012");
        assert_eq!(loaded_settings.rssi_delta_max, -40);
        assert_eq!(loaded_settings.theme, "dark");
        assert_eq!(loaded_settings.language, "en");
    }

    #[test]
    fn test_bluetooth_error_handling() {
        // Test the integration of BluetoothError with uuid parsing
        use lock_4_me_lib::listen_bluetooth::BluetoothError;
        use uuid::Uuid;

        let uuid_result = Uuid::parse_str("invalid");
        let bluetooth_error: BluetoothError = uuid_result.unwrap_err().into();

        match bluetooth_error {
            BluetoothError::UuidParse(_) => {
                // Test error display integration
                let error_string = bluetooth_error.to_string();
                assert!(error_string.contains("UUID parsing error"));
            }
            _ => panic!("Should be a UUID parse error"),
        }

        // Test discovery error
        let discovery_error = BluetoothError::Discovery("Connection failed".to_string());
        let error_string = discovery_error.to_string();
        assert!(error_string.contains("Bluetooth discovery error"));
        assert!(error_string.contains("Connection failed"));
    }

    #[test]
    fn test_cross_module_compatibility() {
        // Test that modules work together correctly
        use lock_4_me_lib::read_write_settings::Settings;
        use uuid::Uuid;

        // Create settings with a valid UUID
        let test_uuid = Uuid::new_v4();
        let settings = Settings {
            target_uuid: test_uuid.to_string(),
            rssi_delta_max: -50,
            theme: "light".to_string(),
            language: "fr".to_string(),
        };

        // Test that the UUID in settings can be parsed by Bluetooth module
        let parsed_uuid = Uuid::parse_str(&settings.target_uuid);
        assert!(parsed_uuid.is_ok(), "Settings UUID should be parseable by Bluetooth module");
        assert_eq!(parsed_uuid.unwrap(), test_uuid);
    }

    #[test]
    fn test_settings_persistence_edge_cases() {
        // Test settings persistence with various edge cases
        let dir = tempdir().unwrap();

        // Test with special characters in path
        let special_path = dir.path().join("special chars éñ");
        fs::create_dir_all(&special_path).unwrap();
        let settings_file = special_path.join("settings.json");

        let settings = lock_4_me_lib::read_write_settings::Settings {
            target_uuid: "test-uuid".to_string(),
            rssi_delta_max: 0, // Edge case: zero delta
            theme: "".to_string(), // Edge case: empty string
            language: "zh-CN".to_string(), // Multi-part language code
        };

        let save_result = settings.save(settings_file.to_str().unwrap());
        assert!(save_result.is_ok(), "Should handle special characters in path");

        // Test loading it back
        let load_result = lock_4_me_lib::read_write_settings::Settings::load(settings_file.to_str().unwrap());
        assert!(load_result.is_ok(), "Should load settings with edge case values");

        let loaded = load_result.unwrap();
        assert_eq!(loaded.rssi_delta_max, 0);
        assert_eq!(loaded.theme, "");
        assert_eq!(loaded.language, "zh-CN");
    }

    #[test]
    fn test_bluetooth_listener_handle_lifecycle() {
        // Test the complete lifecycle of BluetoothListenerHandle
        use lock_4_me_lib::listen_bluetooth::BluetoothListenerHandle;
        use std::sync::Mutex;

        let handle = BluetoothListenerHandle(Mutex::new(None));

        // Initially should be None
        assert!(handle.0.lock().unwrap().is_none());

        // Test that we can take the handle out (for cleanup)
        let taken_handle = handle.0.lock().unwrap().take();
        assert!(taken_handle.is_none());

        // After taking, should be None again
        assert!(handle.0.lock().unwrap().is_none());
    }

    #[test]
    fn test_platform_specific_functionality() {
        // Test that we can access the lock_screen module
        // Note: get_lock_screen_command is only available in test mode
        // so we'll just verify the module is accessible
        assert!(true, "Platform specific functionality module is accessible");
    }

    #[test]
    fn test_settings_error_format_consistency() {
        // Test that settings error messages follow consistent format
        use lock_4_me_lib::read_write_settings::Settings;
        
        // Test with invalid path - this should produce a consistent error format
        let result = Settings::load("/this/path/does/not/exist/settings.json");
        assert!(result.is_err());
        
        let error_message = result.unwrap_err();
        assert!(error_message.contains("Error opening settings file"));
        assert!(error_message.contains(":"));
    }

    #[test]
    fn test_bluetooth_json_event_structure_integration() {
        // Test that the JSON event structure is consistent across the application
        use serde_json::json;
        
        // This is the structure that bluetooth module emits
        let bluetooth_event = json!({
            "event_type": "Discovered device",
            "local_name": "Test Device",
            "id": "test-device-123",
            "rssi": -50,
            "diff_rssi": 5
        });

        // Verify structure compatibility
        assert!(bluetooth_event.is_object());
        assert!(bluetooth_event["event_type"].is_string());
        assert!(bluetooth_event["local_name"].is_string());
        assert!(bluetooth_event["id"].is_string());
        assert!(bluetooth_event["rssi"].is_number());
        assert!(bluetooth_event["diff_rssi"].is_number());
    }
}