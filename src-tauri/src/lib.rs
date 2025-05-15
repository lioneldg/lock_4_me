use listen_bluetooth::BluetoothListenerHandle;
use std::sync::Mutex;
use tauri::{
    tray::{MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Builder, Manager, WindowEvent,
};
use tauri_plugin_log;
mod listen_bluetooth;
mod lock_screen;
mod read_write_settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .level(log::LevelFilter::Info)
                .filter(|metadata| {
                    // Keep only logs of our application
                    metadata.target().starts_with(env!("CARGO_CRATE_NAME"))
                })
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(BluetoothListenerHandle(Mutex::new(None)))
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let main_window = app.get_webview_window("main").unwrap();
            let window_ref = main_window.clone();

            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window_ref.hide().unwrap();
                }
            });

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button_state, .. } = event {
                        if button_state == MouseButtonState::Up {
                            let app_handle = tray.app_handle();
                            let window = app_handle.get_webview_window("main").unwrap();
                            if window.is_visible().unwrap() {
                                window.hide().unwrap();
                                window.set_skip_taskbar(true).unwrap();
                            } else {
                                window.set_skip_taskbar(false).unwrap();
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                    }
                })
                .build(app)
                .expect("failed to create tray icon");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            listen_bluetooth::listen_bluetooth,
            read_write_settings::read_settings,
            read_write_settings::write_settings,
            lock_screen::lock_screen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
