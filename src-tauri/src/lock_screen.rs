/// Locks the screen on the current operating system.
///
/// Attempts to lock the screen using platform-specific methods:
/// - Linux: Tries various desktop environment commands
/// - macOS: Uses keyboard shortcut simulation (Cmd+Ctrl+Q)
/// - Windows: Uses LockWorkStation from user32.dll
///
/// # Returns
///
/// A `Result` indicating whether the operation succeeded or failed with an error message.
#[tauri::command]
pub fn lock_screen() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        // Try common Linux lock screen commands for various desktop environments
        std::process::Command::new("loginctl")
            .arg("lock-session")
            .status()
            .or_else(|_| {
                std::process::Command::new("gnome-screensaver-command")
                    .arg("--lock")
                    .status()
            })
            .or_else(|_| {
                std::process::Command::new("xdg-screensaver")
                    .arg("lock")
                    .status()
            })
            .or_else(|_| {
                std::process::Command::new("qdbus")
                    .args(&["org.freedesktop.ScreenSaver", "/ScreenSaver", "Lock"])
                    .status()
            })
            .or_else(|_| {
                std::process::Command::new("cinnamon-screensaver-command")
                    .arg("--lock")
                    .status()
            })
            .or_else(|_| {
                std::process::Command::new("mate-screensaver-command")
                    .arg("--lock")
                    .status()
            })
            .or_else(|_| std::process::Command::new("xflock4").status())
            .map_err(|e| format!("Failed to lock screen on Linux: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: lock the screen using the standard keyboard shortcut Cmd+Ctrl+Q
        std::process::Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke \"q\" using {control down, command down}")
            .status()
            .map_err(|e| format!("Failed to lock screen on macOS: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: lock the workstation
        std::process::Command::new("rundll32.exe")
            .arg("user32.dll,LockWorkStation")
            .status()
            .map_err(|e| format!("Failed to lock screen on Windows: {}", e))?;
    }

    Ok(())
}
