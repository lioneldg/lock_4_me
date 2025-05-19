use listen_bluetooth::BluetoothListenerHandle;
use log::error;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Builder, Manager, WindowEvent,
};
use tauri_plugin_log;
mod listen_bluetooth;
mod lock_screen;
mod read_write_settings;

/// Handles showing or hiding the main window
fn toggle_window_visibility(window: &tauri::WebviewWindow) -> Result<(), String> {
    match window.is_visible() {
        Ok(is_visible) => {
            if is_visible {
                if let Err(err) = window.hide() {
                    error!("Failed to hide window: {}", err);
                    return Err(format!("Failed to hide window: {}", err));
                }
                if let Err(err) = window.set_skip_taskbar(true) {
                    error!("Failed to set skip taskbar: {}", err);
                    return Err(format!("Failed to set skip taskbar: {}", err));
                }
            } else {
                if let Err(err) = window.set_skip_taskbar(false) {
                    error!("Failed to set skip taskbar: {}", err);
                    return Err(format!("Failed to set skip taskbar: {}", err));
                }
                if let Err(err) = window.show() {
                    error!("Failed to show window: {}", err);
                    return Err(format!("Failed to show window: {}", err));
                }
                if let Err(err) = window.set_focus() {
                    error!("Failed to set focus: {}", err);
                    return Err(format!("Failed to set focus: {}", err));
                }
            }
            Ok(())
        }
        Err(err) => {
            error!("Failed to check window visibility: {}", err);
            Err(format!("Failed to check window visibility: {}", err))
        }
    }
}

/// Configures the system tray icon
fn setup_tray(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    // Get the default icon
    let icon = app
        .default_window_icon()
        .ok_or_else(|| {
            tauri::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Default window icon not found",
            ))
        })?
        .clone();

    // Create menu items
    let stop_i = MenuItem::with_id(app, "stop", "Stop", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&stop_i])?;

    // Configure the system tray icon
    TrayIconBuilder::new()
        .icon(icon)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button_state, .. } = event {
                if button_state == MouseButtonState::Up {
                    let app_handle = tray.app_handle();
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = toggle_window_visibility(&window);
                    } else {
                        error!("Main window not found");
                    }
                }
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "stop" => {
                app.exit(0);
            }
            _ => {}
        })
        .menu(&menu)
        .show_menu_on_left_click(false)
        .build(app)
        .expect("failed to create tray icon");

    Ok(())
}

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
            // Use accessory on macos to prevent the app from being shown in the dock
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Configure the main window
            let main_window = app
                .get_webview_window("main")
                .ok_or_else(|| tauri::Error::WebviewNotFound)?;
            let window_ref = main_window.clone();

            // Handle close event
            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Err(err) = window_ref.hide() {
                        error!("Failed to hide window: {}", err);
                    }
                }
            });

            // Set up system tray icon with menu
            setup_tray(&app.app_handle())?;

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
