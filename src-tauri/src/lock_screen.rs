pub fn lock_screen() {
    #[cfg(target_os = "linux")]
    {
        // Try common Linux lock screen commands for various desktop environments
        let _ = std::process::Command::new("loginctl")
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
            .or_else(|_| std::process::Command::new("xflock4").status());
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: lock the screen
        let _ = std::process::Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke \"q\" using {control down, command down}")
            .status();
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: lock the workstation
        let _ = std::process::Command::new("rundll32.exe")
            .arg("user32.dll,LockWorkStation")
            .status();
    }
}
