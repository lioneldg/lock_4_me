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
