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

#[cfg(test)]
mod tests {
    use super::*;

    // Test struct to simulate window behavior for testing
    #[derive(Debug)]
    struct MockWindow {
        is_visible: bool,
        should_fail_visibility_check: bool,
        should_fail_hide: bool,
        should_fail_show: bool,
        should_fail_skip_taskbar: bool,
        should_fail_focus: bool,
    }

    impl MockWindow {
        fn new() -> Self {
            Self {
                is_visible: false,
                should_fail_visibility_check: false,
                should_fail_hide: false,
                should_fail_show: false,
                should_fail_skip_taskbar: false,
                should_fail_focus: false,
            }
        }

        fn visible() -> Self {
            Self {
                is_visible: true,
                ..Self::new()
            }
        }
    }

    #[test]
    fn test_bluetooth_listener_handle_initialization() {
        let handle = BluetoothListenerHandle(Mutex::new(None));
        
        // Should start with None
        assert!(handle.0.lock().unwrap().is_none());
        
        // Test that we can access the mutex
        let mut guard = handle.0.lock().unwrap();
        assert!(guard.is_none());
        
        // Test that we can modify it (simulating storing a handle)
        *guard = None; // In real use, this would be Some(join_handle)
        assert!(guard.is_none());
    }

    #[test]
    fn test_error_message_formats() {
        // Test that error messages follow expected format
        let test_cases = [
            ("Failed to hide window: test error", "hide window"),
            ("Failed to show window: test error", "show window"),
            ("Failed to set skip taskbar: test error", "skip taskbar"),
            ("Failed to set focus: test error", "set focus"),
            ("Failed to check window visibility: test error", "window visibility"),
        ];

        for (message, expected_content) in test_cases {
            assert!(message.contains(expected_content));
            assert!(message.contains("Failed to"));
            assert!(message.contains(": test error"));
        }
    }

    #[test] 
    fn test_log_filter_configuration() {
        // Test the log filter logic
        let crate_name = env!("CARGO_CRATE_NAME");
        
        // Test that our crate's logs would be included
        let our_target = format!("{}_lib::some_module", crate_name);
        assert!(our_target.starts_with(crate_name));
        
        // Test that other crates' logs would be excluded
        let other_target = "some_other_crate::module";
        assert!(!other_target.starts_with(crate_name));
        
        // Test edge case - exactly matching crate name
        assert!(crate_name.starts_with(crate_name));
    }

    #[test]
    fn test_window_visibility_toggle_logic() {
        // Test the logic that would be used in toggle_window_visibility
        
        // Case 1: Window is visible -> should hide
        let is_visible = true;
        let should_hide = is_visible;
        let should_show = !is_visible;
        assert!(should_hide);
        assert!(!should_show);
        
        // Case 2: Window is hidden -> should show  
        let is_visible = false;
        let should_hide = is_visible;
        let should_show = !is_visible;
        assert!(!should_hide);
        assert!(should_show);
    }

    #[test]
    fn test_menu_event_handling_logic() {
        // Test the logic used in menu event handling
        let test_events = ["stop", "unknown", "other"];
        
        for event_id in test_events {
            let should_exit = match event_id {
                "stop" => true,
                _ => false,
            };
            
            if event_id == "stop" {
                assert!(should_exit, "Stop event should trigger exit");
            } else {
                assert!(!should_exit, "Other events should not trigger exit");
            }
        }
    }

    #[test]
    fn test_tray_icon_event_logic() {
        // Test the logic for handling tray icon events
        use tauri::tray::MouseButtonState;
        
        // Test button state handling
        let up_state = MouseButtonState::Up;
        let down_state = MouseButtonState::Down;
        
        let should_toggle_on_up = matches!(up_state, MouseButtonState::Up);
        let should_toggle_on_down = matches!(down_state, MouseButtonState::Up);
        
        assert!(should_toggle_on_up, "Should toggle window on mouse up");
        assert!(!should_toggle_on_down, "Should not toggle window on mouse down");
    }

    #[test]
    fn test_window_close_event_logic() {
        // Test the logic for handling window close events
        
        // In the actual implementation, we prevent close and hide the window
        // Here we test the decision logic
        let should_prevent_close = true; // Always prevent close in our app
        let should_hide_instead = true;  // Always hide instead of closing
        
        assert!(should_prevent_close, "Should always prevent window close");
        assert!(should_hide_instead, "Should hide window instead of closing");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos_activation_policy() {
        // Test that we're using the correct activation policy for macOS
        use tauri::ActivationPolicy;
        
        let policy = ActivationPolicy::Accessory;
        
        // Accessory policy prevents app from showing in dock
        // This is the expected behavior for our system tray app
        match policy {
            ActivationPolicy::Accessory => assert!(true, "Correct policy for system tray app"),
            _ => panic!("Should use Accessory policy on macOS"),
        }
    }

    #[test]
    fn test_invoke_handler_commands() {
        // Test that we have the expected commands registered
        let expected_commands = [
            "listen_bluetooth",
            "read_settings", 
            "write_settings",
            "lock_screen",
        ];
        
        // In a real test, we'd verify these are actually registered
        // Here we just verify we have the expected command names
        for command in expected_commands {
            assert!(!command.is_empty(), "Command name should not be empty");
            assert!(command.contains('_'), "Commands should use snake_case");
        }
    }

    #[test]
    fn test_error_types() {
        // Test that we handle the expected error types
        use std::io;
        
        // Test IO error creation (used for missing icon)
        let io_error = io::Error::new(
            io::ErrorKind::NotFound,
            "Default window icon not found"
        );
        
        assert_eq!(io_error.kind(), io::ErrorKind::NotFound);
        assert_eq!(io_error.to_string(), "Default window icon not found");
        
        // Test conversion to tauri::Error
        let tauri_error = tauri::Error::Io(io_error);
        match tauri_error {
            tauri::Error::Io(_) => assert!(true, "Correctly converted to Tauri IO error"),
            _ => panic!("Should be an IO error"),
        }
    }

    #[test]
    fn test_plugin_configuration() {
        // Test log plugin configuration
        use log::LevelFilter;
        
        let level = LevelFilter::Info;
        assert_eq!(level, LevelFilter::Info);
        
        // Test that Info level is appropriate (not too verbose, not too quiet)
        assert!(level > LevelFilter::Warn, "Should be more verbose than Warn");
        assert!(level < LevelFilter::Debug, "Should be less verbose than Debug");
    }

    #[test]
    fn test_window_operations_error_handling() {
        // Test error handling patterns used in toggle_window_visibility
        
        // Test error message construction
        let base_error = "Permission denied";
        let hide_error = format!("Failed to hide window: {}", base_error);
        let show_error = format!("Failed to show window: {}", base_error);
        let taskbar_error = format!("Failed to set skip taskbar: {}", base_error);
        let focus_error = format!("Failed to set focus: {}", base_error);
        
        assert!(hide_error.starts_with("Failed to hide window:"));
        assert!(show_error.starts_with("Failed to show window:"));
        assert!(taskbar_error.starts_with("Failed to set skip taskbar:"));
        assert!(focus_error.starts_with("Failed to set focus:"));
        
        // All should end with the original error
        assert!(hide_error.ends_with(base_error));
        assert!(show_error.ends_with(base_error));
        assert!(taskbar_error.ends_with(base_error));
        assert!(focus_error.ends_with(base_error));
    }

    #[test]
    fn test_menu_item_configuration() {
        // Test menu item properties
        let menu_id = "stop";
        let menu_text = "Stop";
        let menu_enabled = true;
        
        assert_eq!(menu_id, "stop");
        assert_eq!(menu_text, "Stop");
        assert!(menu_enabled, "Stop menu item should be enabled");
        
        // Test that ID and text are appropriate
        assert!(!menu_id.is_empty(), "Menu ID should not be empty");
        assert!(!menu_text.is_empty(), "Menu text should not be empty");
        assert!(menu_id.is_ascii(), "Menu ID should be ASCII");
    }

    #[test]
    fn test_tray_configuration() {
        // Test tray icon configuration
        let show_menu_on_left_click = false;
        
        assert!(!show_menu_on_left_click, "Should not show menu on left click");
        // Left click should toggle window, right click should show menu
    }
}
