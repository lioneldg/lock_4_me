use listen_bluetooth::BluetoothListenerHandle;
use std::sync::Arc;
use tokio::sync::Mutex;
mod listen_bluetooth;
mod lock_screen;
mod read_write_settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(BluetoothListenerHandle(Mutex::new(None))))
        .invoke_handler(tauri::generate_handler![
            listen_bluetooth::listen_bluetooth,
            read_write_settings::read_settings,
            read_write_settings::write_settings,
            lock_screen::lock_screen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
