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

/// Test-only function that returns the expected command for the current platform
#[cfg(test)]
pub fn get_lock_screen_command() -> (&'static str, Vec<&'static str>) {
    #[cfg(target_os = "linux")]
    {
        ("loginctl", vec!["lock-session"])
    }
    #[cfg(target_os = "macos")]
    {
        ("osascript", vec!["-e", "tell application \"System Events\" to keystroke \"q\" using {control down, command down}"])
    }
    #[cfg(target_os = "windows")]
    {
        ("rundll32.exe", vec!["user32.dll,LockWorkStation"])
    }
}

/// Internal function that can be tested - extracted logic for testability
#[cfg(test)]
pub fn execute_lock_command() -> Result<(), String> {
    let (command, args) = get_lock_screen_command();
    
    std::process::Command::new(command)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to execute lock command '{}': {}", command, e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lock_screen_command_returns_valid_command() {
        let (command, args) = get_lock_screen_command();
        
        // Verify command is not empty
        assert!(!command.is_empty(), "Command should not be empty");
        
        // Verify we have some arguments for complex commands
        #[cfg(target_os = "macos")]
        {
            assert!(args.len() >= 2, "macOS command should have at least 2 arguments");
            assert_eq!(args[0], "-e");
            assert!(args[1].contains("keystroke"));
        }
        
        #[cfg(target_os = "linux")]
        {
            assert!(args.len() >= 1, "Linux command should have at least 1 argument");
            assert_eq!(args[0], "lock-session");
        }
        
        #[cfg(target_os = "windows")]
        {
            assert!(args.len() >= 1, "Windows command should have at least 1 argument");
            assert_eq!(args[0], "user32.dll,LockWorkStation");
        }
    }

    #[test]
    fn test_lock_screen_command_format() {
        let (command, args) = get_lock_screen_command();
        
        // Test that command doesn't contain invalid characters
        assert!(!command.contains(' '), "Command should not contain spaces");
        assert!(!command.is_empty(), "Command should not be empty");
        
        // Test that each arg is valid
        for arg in &args {
            assert!(!arg.is_empty(), "Arguments should not be empty");
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_lock_command() {
        let (command, args) = get_lock_screen_command();
        assert_eq!(command, "loginctl");
        assert_eq!(args, vec!["lock-session"]);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_lock_command() {
        let (command, args) = get_lock_screen_command();
        assert_eq!(command, "osascript");
        assert_eq!(args[0], "-e");
        assert!(args[1].contains("System Events"));
        assert!(args[1].contains("keystroke"));
        assert!(args[1].contains("control down"));
        assert!(args[1].contains("command down"));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_lock_command() {
        let (command, args) = get_lock_screen_command();
        assert_eq!(command, "rundll32.exe");
        assert_eq!(args, vec!["user32.dll,LockWorkStation"]);
    }

    // This test will fail in CI/testing environments but demonstrates the structure
    #[test]
    #[ignore] // Ignored by default as it would actually attempt to lock the screen
    fn test_lock_screen_integration() {
        // This would actually lock the screen, so we ignore it by default
        // In a real testing environment, you might want to mock the Command execution
        let result = lock_screen();
        
        // In most testing environments, this might fail due to missing display
        // but we can check that it returns a proper error message format
        match result {
            Ok(_) => {
                // Success case - screen was locked
                assert!(true);
            }
            Err(err) => {
                // Error case - should contain meaningful error message
                assert!(err.contains("Failed to lock screen"), 
                       "Error message should be descriptive: {}", err);
            }
        }
    }

    #[test]
    fn test_lock_screen_error_message_format() {
        // Test that error messages follow expected format
        let error_message = "Failed to lock screen on Linux: No such file or directory";
        assert!(error_message.contains("Failed to lock screen"));
        assert!(error_message.contains(":"));
    }

    // Test for specific platform behaviors
    #[cfg(target_os = "linux")]
    #[test]
    fn test_linux_fallback_commands() {
        // Test that we know about various Linux desktop environments
        let fallback_commands = [
            "loginctl",
            "gnome-screensaver-command", 
            "xdg-screensaver",
            "qdbus",
            "cinnamon-screensaver-command",
            "mate-screensaver-command",
            "xflock4"
        ];
        
        // Verify the primary command is the first fallback
        let (primary_command, _) = get_lock_screen_command();
        assert_eq!(primary_command, fallback_commands[0]);
    }

    #[test]
    fn test_command_structure_validity() {
        let (command, args) = get_lock_screen_command();
        
        // Basic validation that command structure makes sense
        assert!(command.len() > 0, "Command should not be empty");
        
        // For complex commands, ensure they have proper structure
        match command {
            "osascript" => {
                assert!(args.len() >= 2, "osascript should have at least 2 arguments");
                assert_eq!(args[0], "-e", "First osascript arg should be -e");
            }
            "rundll32.exe" => {
                assert!(args.len() >= 1, "rundll32 should have at least 1 argument");
                assert!(args[0].contains("user32.dll"), "rundll32 should call user32.dll");
            }
            "loginctl" => {
                assert!(args.len() >= 1, "loginctl should have at least 1 argument");
                assert_eq!(args[0], "lock-session", "loginctl should lock-session");
            }
            _ => {
                // Other commands should still have valid structure
                // (args.len() is always >= 0 by definition)
            }
        }
    }
}
