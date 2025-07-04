use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
#[derive(Serialize, Deserialize)]
pub struct Settings {
    target_uuid: String,
    rssi_delta_max: i16,
    theme: String,
    language: String,
}

impl Settings {
    pub fn save(&self, file_path: &str) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(file_path).parent() {
            create_dir_all(parent)
                .map_err(|e| format!("Error creating directory '{}': {}", parent.display(), e))?;
        }
        let mut settings_file = File::create(file_path)
            .map_err(|e| format!("Error creating settings file '{}': {}", file_path, e))?;
        let settings_json = serde_json::to_string(self)
            .map_err(|e| format!("Error serializing settings: {}", e))?;
        settings_file
            .write_all(settings_json.as_bytes())
            .map_err(|e| format!("Error writing settings to file '{}': {}", file_path, e))
    }

    pub fn load(file_path: &str) -> Result<Self, String> {
        let settings_file = File::open(file_path)
            .map_err(|e| format!("Error opening settings file '{}': {}", file_path, e))?;
        serde_json::from_reader(settings_file)
            .map_err(|e| format!("Error parsing JSON in settings file '{}': {}", file_path, e))
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn write_settings(settings: Settings, file_path: String) -> Result<(), String> {
    settings.save(&file_path)
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_settings(file_path: String) -> Result<Settings, String> {
    Settings::load(&file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    fn create_test_settings() -> Settings {
        Settings {
            target_uuid: "12345678-1234-1234-1234-123456789012".to_string(),
            rssi_delta_max: -50,
            theme: "dark".to_string(),
            language: "en".to_string(),
        }
    }

    #[test]
    fn test_settings_serialization() {
        let settings = create_test_settings();
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("12345678-1234-1234-1234-123456789012"));
        assert!(json.contains("-50"));
        assert!(json.contains("dark"));
        assert!(json.contains("en"));
    }

    #[test]
    fn test_settings_deserialization() {
        let json = r#"{
            "target_uuid": "12345678-1234-1234-1234-123456789012",
            "rssi_delta_max": -50,
            "theme": "dark",
            "language": "en"
        }"#;
        let settings: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.target_uuid, "12345678-1234-1234-1234-123456789012");
        assert_eq!(settings.rssi_delta_max, -50);
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.language, "en");
    }

    #[test]
    fn test_settings_save_and_load() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_settings.json");
        let file_path_str = file_path.to_str().unwrap();

        let original_settings = create_test_settings();
        
        // Test save
        let save_result = original_settings.save(file_path_str);
        assert!(save_result.is_ok(), "Failed to save settings: {:?}", save_result);

        // Verify file exists and contains expected content
        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("12345678-1234-1234-1234-123456789012"));

        // Test load
        let loaded_settings = Settings::load(file_path_str);
        assert!(loaded_settings.is_ok(), "Failed to load settings: {:?}", loaded_settings);
        
        let loaded_settings = loaded_settings.unwrap();
        assert_eq!(loaded_settings.target_uuid, original_settings.target_uuid);
        assert_eq!(loaded_settings.rssi_delta_max, original_settings.rssi_delta_max);
        assert_eq!(loaded_settings.theme, original_settings.theme);
        assert_eq!(loaded_settings.language, original_settings.language);
    }

    #[test]
    fn test_settings_save_with_nested_directory() {
        let dir = tempdir().unwrap();
        let nested_path = dir.path().join("nested").join("directory").join("settings.json");
        let file_path_str = nested_path.to_str().unwrap();

        let settings = create_test_settings();
        let result = settings.save(file_path_str);
        
        assert!(result.is_ok(), "Failed to save settings in nested directory: {:?}", result);
        assert!(nested_path.exists());
        
        // Verify we can load it back
        let loaded = Settings::load(file_path_str);
        assert!(loaded.is_ok());
    }

    #[test]
    fn test_settings_load_nonexistent_file() {
        let result = Settings::load("/nonexistent/path/settings.json");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Error opening settings file"));
    }

    #[test]
    fn test_settings_load_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid_settings.json");
        let file_path_str = file_path.to_str().unwrap();

        // Create file with invalid JSON
        fs::write(&file_path, "{ invalid json }").unwrap();

        let result = Settings::load(file_path_str);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Error parsing JSON"));
    }

    #[test]
    fn test_write_settings_command() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("command_test.json");
        let file_path_str = file_path.to_string_lossy().to_string();

        let settings = create_test_settings();
        let result = write_settings(settings, file_path_str.clone());
        
        assert!(result.is_ok());
        assert!(file_path.exists());
        
        // Verify content
        let loaded = read_settings(file_path_str);
        assert!(loaded.is_ok());
        let loaded_settings = loaded.unwrap();
        assert_eq!(loaded_settings.target_uuid, "12345678-1234-1234-1234-123456789012");
    }

    #[test]
    fn test_read_settings_command() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("read_test.json");
        let file_path_str = file_path.to_string_lossy().to_string();

        // Create settings file manually
        let json_content = r#"{
            "target_uuid": "test-uuid",
            "rssi_delta_max": -30,
            "theme": "light",
            "language": "en"
        }"#;
        fs::write(&file_path, json_content).unwrap();

        let result = read_settings(file_path_str);
        assert!(result.is_ok());
        
        let settings = result.unwrap();
        assert_eq!(settings.target_uuid, "test-uuid");
        assert_eq!(settings.rssi_delta_max, -30);
        assert_eq!(settings.theme, "light");
        assert_eq!(settings.language, "en");
    }
}
