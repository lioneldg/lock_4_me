use listen_bluetooth::BluetoothListenerHandle;
use std::sync::Mutex;
use tauri_plugin_log;
mod listen_bluetooth;
mod lock_screen;
mod read_write_settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .level(log::LevelFilter::Info)
                .filter(|metadata| {
                    // Ne garder que les logs de notre application
                    metadata.target().starts_with(env!("CARGO_CRATE_NAME"))
                })
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(BluetoothListenerHandle(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            listen_bluetooth::listen_bluetooth,
            read_write_settings::read_settings,
            read_write_settings::write_settings,
            lock_screen::lock_screen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
