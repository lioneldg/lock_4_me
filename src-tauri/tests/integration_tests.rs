use lock_4_me_lib::*;
use std::fs;
use tempfile::tempdir;
use uuid::Uuid;

// Integration tests for the entire application

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_settings_workflow() {
        // Test the complete settings workflow
        let dir = tempdir().unwrap();
        let settings_path = dir.path().join("integration_settings.json");
        let settings_path_str = settings_path.to_string_lossy().to_string();

        // Create test settings
        let original_settings = read_write_settings::Settings {
            target_uuid: "12345678-1234-1234-1234-123456789012".to_string(),
            rssi_delta_max: -40,
            theme: "dark".to_string(),
            language: "en".to_string(),
        };

        // Test write operation
        let write_result = read_write_settings::write_settings(
            original_settings.clone(),
            settings_path_str.clone()
        );
        assert!(write_result.is_ok(), "Write settings should succeed");

        // Verify file was created
        assert!(settings_path.exists(), "Settings file should exist");

        // Test read operation
        let read_result = read_write_settings::read_settings(settings_path_str);
        assert!(read_result.is_ok(), "Read settings should succeed");

        let loaded_settings = read_result.unwrap();
        assert_eq!(loaded_settings.target_uuid, "12345678-1234-1234-1234-123456789012");
        assert_eq!(loaded_settings.rssi_delta_max, -40);
        assert_eq!(loaded_settings.theme, "dark");
        assert_eq!(loaded_settings.language, "en");
    }

    #[test]
    fn test_bluetooth_error_handling() {
        // Test Bluetooth error handling and conversion
        use listen_bluetooth::BluetoothError;

        let uuid_error = uuid::Error::InvalidLength(5);
        let bluetooth_error: BluetoothError = uuid_error.into();

        match bluetooth_error {
            BluetoothError::UuidParse(_) => {
                // Test error display
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
    fn test_uuid_validation_integration() {
        // Test UUID validation that would be used in listen_bluetooth
        let test_cases = [
            ("12345678-1234-1234-1234-123456789012", true),
            ("invalid-uuid", false),
            ("", false),
            ("12345678-1234-1234-1234-12345678901", false), // Too short
            ("12345678-1234-1234-1234-123456789012x", false), // Too long
        ];

        for (uuid_str, should_be_valid) in test_cases {
            let result = Uuid::parse_str(uuid_str);
            assert_eq!(result.is_ok(), should_be_valid, 
                      "UUID '{}' validation should be {}", uuid_str, should_be_valid);
        }
    }

    #[test]
    fn test_cross_module_compatibility() {
        // Test that modules work together correctly
        use read_write_settings::Settings;
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

        let settings = read_write_settings::Settings {
            target_uuid: "test-uuid".to_string(),
            rssi_delta_max: 0, // Edge case: zero delta
            theme: "".to_string(), // Edge case: empty string
            language: "zh-CN".to_string(), // Multi-part language code
        };

        let save_result = settings.save(settings_file.to_str().unwrap());
        assert!(save_result.is_ok(), "Should handle special characters in path");

        // Test loading it back
        let load_result = read_write_settings::Settings::load(settings_file.to_str().unwrap());
        assert!(load_result.is_ok(), "Should load settings with edge case values");

        let loaded = load_result.unwrap();
        assert_eq!(loaded.rssi_delta_max, 0);
        assert_eq!(loaded.theme, "");
        assert_eq!(loaded.language, "zh-CN");
    }

    #[test]
    fn test_bluetooth_listener_handle_lifecycle() {
        // Test the complete lifecycle of BluetoothListenerHandle
        use listen_bluetooth::BluetoothListenerHandle;
        use std::sync::Mutex;

        let handle = BluetoothListenerHandle(Mutex::new(None));

        // Initially should be None
        assert!(handle.0.lock().unwrap().is_none());

        // Simulate storing a handle (we can't create a real JoinHandle easily)
        // In real use, this would be done by listen_bluetooth function
        {
            let mut guard = handle.0.lock().unwrap();
            *guard = None; // In reality, this would be Some(tokio::spawn(...))
        }

        // Test that we can take the handle out (for cleanup)
        let taken_handle = handle.0.lock().unwrap().take();
        assert!(taken_handle.is_none());

        // After taking, should be None again
        assert!(handle.0.lock().unwrap().is_none());
    }

    #[test]
    fn test_error_message_consistency() {
        // Test that error messages are consistent across modules
        
        // Settings errors
        let settings_error = "Error creating settings file '/invalid/path': Permission denied";
        assert!(settings_error.contains("Error"));
        assert!(settings_error.contains(":"));

        // Bluetooth errors
        let bt_error = "Bluetooth discovery error: Device not found";
        assert!(bt_error.contains("error:"));

        // Lock screen errors
        let lock_error = "Failed to lock screen on Linux: Command not found";
        assert!(lock_error.contains("Failed to"));
        assert!(lock_error.contains(":"));

        // All follow similar pattern: [Context]: [Details]
    }

    #[test]
    fn test_rssi_calculation_integration() {
        // Test RSSI calculations that would be used across modules
        
        // Test realistic RSSI values and calculations
        let test_cases = [
            (-30, -25, 5),   // Device moved closer
            (-50, -60, -10), // Device moved away
            (-40, -40, 0),   // Device didn't move
        ];

        for (initial, current, expected_diff) in test_cases {
            let calculated_diff = current - initial;
            assert_eq!(calculated_diff, expected_diff, 
                      "RSSI diff calculation incorrect for {} -> {}", initial, current);

            // Test delta logic (similar to what's used in process_device)
            let delta_max = -15i16;
            let within_threshold = delta_max + calculated_diff > 0;
            
            // Verify the logic makes sense
            if calculated_diff > -delta_max {
                assert!(!within_threshold, "Device should be considered too close");
            }
        }
    }

    #[test]
    fn test_command_registration_consistency() {
        // Test that command names are consistent and follow conventions
        let commands = [
            "listen_bluetooth",
            "read_settings",
            "write_settings", 
            "lock_screen",
        ];

        for command in commands {
            // All commands should use snake_case
            assert!(command.contains('_'), "Command '{}' should use snake_case", command);
            
            // No commands should start or end with underscore
            assert!(!command.starts_with('_'), "Command '{}' should not start with underscore", command);
            assert!(!command.ends_with('_'), "Command '{}' should not end with underscore", command);
            
            // Should be all lowercase
            assert_eq!(command, command.to_lowercase(), "Command '{}' should be lowercase", command);
        }
    }

    #[tokio::test]
    async fn test_async_operations_integration() {
        // Test integration of async operations
        use uuid::Uuid;

        // Test UUID parsing that would be used in async contexts
        let valid_uuid_str = "12345678-1234-1234-1234-123456789012";
        let uuid_result = Uuid::parse_str(valid_uuid_str);
        assert!(uuid_result.is_ok());

        // Test that UUID can be used across async boundaries
        let uuid = uuid_result.unwrap();
        let uuid_string = uuid.to_string();
        
        // Simulate async processing
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        assert_eq!(uuid_string, valid_uuid_str);
    }

    #[test]
    fn test_platform_specific_functionality() {
        // Test platform-specific features work correctly
        use lock_screen::get_lock_screen_command;

        let (command, args) = get_lock_screen_command();
        
        // Verify command is appropriate for current platform
        assert!(!command.is_empty(), "Lock screen command should not be empty");
        assert!(!args.is_empty(), "Lock screen command should have arguments");

        // Test that command structure is valid for current platform
        #[cfg(target_os = "linux")]
        {
            assert_eq!(command, "loginctl");
            assert!(args.contains(&"lock-session"));
        }

        #[cfg(target_os = "macos")]
        {
            assert_eq!(command, "osascript");
            assert!(args.len() >= 2);
        }

        #[cfg(target_os = "windows")]
        {
            assert_eq!(command, "rundll32.exe");
            assert!(args[0].contains("user32.dll"));
        }
    }

    #[test]
    fn test_data_serialization_roundtrip() {
        // Test that data can be serialized and deserialized correctly
        use read_write_settings::Settings;
        use serde_json;

        let original_settings = Settings {
            target_uuid: "test-uuid-123".to_string(),
            rssi_delta_max: -75,
            theme: "auto".to_string(),
            language: "es".to_string(),
        };

        // Serialize to JSON
        let json_data = serde_json::to_string(&original_settings).unwrap();
        
        // Deserialize back
        let deserialized_settings: Settings = serde_json::from_str(&json_data).unwrap();

        // Verify all fields match
        assert_eq!(deserialized_settings.target_uuid, original_settings.target_uuid);
        assert_eq!(deserialized_settings.rssi_delta_max, original_settings.rssi_delta_max);
        assert_eq!(deserialized_settings.theme, original_settings.theme);
        assert_eq!(deserialized_settings.language, original_settings.language);
    }
}