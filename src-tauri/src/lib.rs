mod listen_bluetooth;
mod lock_screen;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Clone le handle AVANT de passer dans la tâche asynchrone
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Paramètres à adapter selon ton besoin
                let uuid = "a87e3669-e2de-d0e3-52ce-93a023ceef37".to_string();
                let rssi_delta_max = 15;
                let result =
                    listen_bluetooth::listen_bluetooth(app_handle, uuid, rssi_delta_max).await;
                // Quand listen_bluetooth se termine, on lock l'écran
                lock_screen::lock_screen();
                if let Err(e) = result {
                    println!("Bluetooth listening error: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            listen_bluetooth::listen_bluetooth
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
