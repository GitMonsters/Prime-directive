//! Desktop Platform Implementation
//!
//! Provides GUI automation for Windows, macOS, and Linux desktops.

use crate::mimicry::gui_agent::{
    ConnectionType, DeviceInfo, GuiAction, GuiError, GuiPlatform, ImageFormat, NormalizedPoint,
    Platform, PressKey, Screenshot, SwipeDirection, SwipeTarget,
};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Desktop platform implementation
pub struct DesktopPlatform {
    /// Platform type
    platform: Platform,
    /// Screen resolution
    resolution: Option<(u32, u32)>,
    /// Whether connected (always true for local)
    connected: bool,
}

impl DesktopPlatform {
    /// Create a new desktop platform instance
    pub fn new(platform: Platform) -> Self {
        Self {
            platform,
            resolution: None,
            connected: false,
        }
    }

    /// Detect the current platform
    pub fn detect_current() -> Self {
        let platform = match std::env::consts::OS {
            "macos" => Platform::MacOs,
            "windows" => Platform::Windows,
            "linux" => Platform::Linux,
            _ => Platform::Linux,
        };
        Self::new(platform)
    }

    /// Get screen resolution
    fn get_resolution(&mut self) -> Result<(u32, u32), GuiError> {
        if let Some(res) = self.resolution {
            return Ok(res);
        }

        let res = match self.platform {
            Platform::MacOs => self.get_macos_resolution(),
            Platform::Windows => self.get_windows_resolution(),
            Platform::Linux => self.get_linux_resolution(),
            _ => Err(GuiError::PlatformNotSupported),
        }?;

        self.resolution = Some(res);
        Ok(res)
    }

    fn get_macos_resolution(&self) -> Result<(u32, u32), GuiError> {
        let output = Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
            .map_err(|e| GuiError::Internal(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse "Resolution: 2560 x 1440"
        for line in stdout.lines() {
            if line.contains("Resolution:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let (Ok(w), Ok(h)) = (parts[1].parse(), parts[3].parse()) {
                        return Ok((w, h));
                    }
                }
            }
        }

        Ok((1920, 1080)) // Default fallback
    }

    fn get_windows_resolution(&self) -> Result<(u32, u32), GuiError> {
        let output = Command::new("wmic")
            .args([
                "path",
                "Win32_VideoController",
                "get",
                "CurrentHorizontalResolution,CurrentVerticalResolution",
            ])
            .output()
            .map_err(|e| GuiError::Internal(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(w), Ok(h)) = (parts[0].parse(), parts[1].parse()) {
                    return Ok((w, h));
                }
            }
        }

        Ok((1920, 1080))
    }

    fn get_linux_resolution(&self) -> Result<(u32, u32), GuiError> {
        let output = Command::new("xdpyinfo")
            .output()
            .map_err(|e| GuiError::Internal(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if line.contains("dimensions:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let dims: Vec<&str> = parts[1].split('x').collect();
                    if dims.len() == 2 {
                        if let (Ok(w), Ok(h)) = (dims[0].parse(), dims[1].parse()) {
                            return Ok((w, h));
                        }
                    }
                }
            }
        }

        Ok((1920, 1080))
    }

    /// Convert normalized point to absolute
    fn to_absolute(&mut self, point: &NormalizedPoint) -> Result<(u32, u32), GuiError> {
        let (width, height) = self.get_resolution()?;
        Ok(point.to_absolute(width, height))
    }
}

impl Default for DesktopPlatform {
    fn default() -> Self {
        Self::detect_current()
    }
}

impl GuiPlatform for DesktopPlatform {
    fn platform(&self) -> Platform {
        self.platform
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn capture_screenshot(&self) -> Result<Screenshot, GuiError> {
        let temp_path = format!("/tmp/desktop_screenshot_{}.png", std::process::id());

        let result = match self.platform {
            Platform::MacOs => Command::new("screencapture")
                .args(["-x", &temp_path])
                .output(),
            Platform::Linux => {
                // Try scrot, then import (ImageMagick), then gnome-screenshot
                Command::new("scrot").arg(&temp_path).output().or_else(|_| {
                    Command::new("import")
                        .args(["-window", "root", &temp_path])
                        .output()
                })
            }
            Platform::Windows => {
                // PowerShell screenshot
                Command::new("powershell")
                    .args([
                        "-Command",
                        &format!(
                            "Add-Type -AssemblyName System.Windows.Forms; \
                             [System.Windows.Forms.Screen]::PrimaryScreen | \
                             ForEach-Object {{ $bitmap = New-Object System.Drawing.Bitmap($_.Bounds.Width, $_.Bounds.Height); \
                             $graphics = [System.Drawing.Graphics]::FromImage($bitmap); \
                             $graphics.CopyFromScreen($_.Bounds.Location, [System.Drawing.Point]::Empty, $_.Bounds.Size); \
                             $bitmap.Save('{}') }}",
                            temp_path
                        ),
                    ])
                    .output()
            }
            _ => return Err(GuiError::PlatformNotSupported),
        };

        result.map_err(|e| GuiError::ScreenshotFailed(e.to_string()))?;

        let data =
            std::fs::read(&temp_path).map_err(|e| GuiError::ScreenshotFailed(e.to_string()))?;

        let _ = std::fs::remove_file(&temp_path);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let (width, height) = self.resolution.unwrap_or((1920, 1080));

        Ok(Screenshot {
            data,
            format: ImageFormat::Png,
            width,
            height,
            timestamp,
            platform: self.platform,
        })
    }

    fn execute_action(&self, action: &GuiAction) -> Result<(), GuiError> {
        match self.platform {
            Platform::MacOs => self.execute_macos(action),
            Platform::Linux => self.execute_linux(action),
            Platform::Windows => self.execute_windows(action),
            _ => Err(GuiError::PlatformNotSupported),
        }
    }

    fn get_context(&self) -> Result<String, GuiError> {
        match self.platform {
            Platform::MacOs => {
                let output = Command::new("osascript")
                    .args(["-e", "tell application \"System Events\" to get name of first process whose frontmost is true"])
                    .output()
                    .map_err(|e| GuiError::Internal(e.to_string()))?;
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
            Platform::Linux => {
                let output = Command::new("xdotool")
                    .args(["getactivewindow", "getwindowname"])
                    .output()
                    .map_err(|e| GuiError::Internal(e.to_string()))?;
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
            Platform::Windows => {
                let output = Command::new("powershell")
                    .args(["-Command", "(Get-Process | Where-Object {$_.MainWindowHandle -eq (Get-Process -Id $PID).MainWindowHandle}).ProcessName"])
                    .output()
                    .map_err(|e| GuiError::Internal(e.to_string()))?;
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
            _ => Ok("unknown".to_string()),
        }
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>, GuiError> {
        Ok(vec![DeviceInfo {
            id: "local".to_string(),
            name: format!("Local {}", self.platform),
            platform: self.platform,
            os_version: Some(std::env::consts::OS.to_string()),
            resolution: self.resolution,
            connection: ConnectionType::Usb,
            active: self.connected,
        }])
    }

    fn connect(&mut self, device_id: &str) -> Result<(), GuiError> {
        if device_id == "local" {
            self.connected = true;
            let _ = self.get_resolution();
            Ok(())
        } else {
            Err(GuiError::NoDevice)
        }
    }

    fn disconnect(&mut self) -> Result<(), GuiError> {
        self.connected = false;
        Ok(())
    }
}

impl DesktopPlatform {
    fn execute_macos(&self, action: &GuiAction) -> Result<(), GuiError> {
        // Handle TYPE
        if let Some(ref text) = action.type_text {
            let script = format!(
                "tell application \"System Events\" to keystroke \"{}\"",
                text.replace("\"", "\\\"")
            );
            Command::new("osascript")
                .args(["-e", &script])
                .output()
                .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            return Ok(());
        }

        // Handle PRESS
        if let Some(ref key) = action.press {
            let key_code = match key {
                PressKey::Enter => "return",
                PressKey::Back => "delete",
                _ => {
                    return Err(GuiError::ActionFailed(format!(
                        "Key {:?} not supported",
                        key
                    )))
                }
            };
            let script = format!(
                "tell application \"System Events\" to key code {}",
                key_code
            );
            Command::new("osascript")
                .args(["-e", &script])
                .output()
                .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            return Ok(());
        }

        // Handle click
        if let Some(ref point) = action.point {
            let mut platform = DesktopPlatform::new(Platform::MacOs);
            platform.resolution = self.resolution;
            let (x, y) = platform.to_absolute(point)?;

            if action.to.is_none() {
                // Simple click using cliclick or AppleScript
                Command::new("cliclick")
                    .args(["c:", &format!("{},{}", x, y)])
                    .output()
                    .or_else(|_| {
                        let script = format!(
                            "tell application \"System Events\" to click at {{{}, {}}}",
                            x, y
                        );
                        Command::new("osascript").args(["-e", &script]).output()
                    })
                    .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            }
        }

        // Handle wait
        if let Some(duration) = action.duration {
            if action.point.is_none() {
                std::thread::sleep(std::time::Duration::from_millis(duration as u64));
            }
        }

        Ok(())
    }

    fn execute_linux(&self, action: &GuiAction) -> Result<(), GuiError> {
        // Handle TYPE
        if let Some(ref text) = action.type_text {
            Command::new("xdotool")
                .args(["type", "--", text])
                .output()
                .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            return Ok(());
        }

        // Handle PRESS
        if let Some(ref key) = action.press {
            let key_name = match key {
                PressKey::Enter => "Return",
                PressKey::Back => "BackSpace",
                PressKey::Home => "Home",
                _ => {
                    return Err(GuiError::ActionFailed(format!(
                        "Key {:?} not supported",
                        key
                    )))
                }
            };
            Command::new("xdotool")
                .args(["key", key_name])
                .output()
                .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            return Ok(());
        }

        // Handle click
        if let Some(ref point) = action.point {
            let mut platform = DesktopPlatform::new(Platform::Linux);
            platform.resolution = self.resolution;
            let (x, y) = platform.to_absolute(point)?;

            if action.to.is_none() {
                Command::new("xdotool")
                    .args(["mousemove", &x.to_string(), &y.to_string(), "click", "1"])
                    .output()
                    .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            }
        }

        // Handle wait
        if let Some(duration) = action.duration {
            if action.point.is_none() {
                std::thread::sleep(std::time::Duration::from_millis(duration as u64));
            }
        }

        Ok(())
    }

    fn execute_windows(&self, action: &GuiAction) -> Result<(), GuiError> {
        // Handle TYPE
        if let Some(ref text) = action.type_text {
            let script = format!(
                "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('{}')",
                text.replace("'", "''")
            );
            Command::new("powershell")
                .args(["-Command", &script])
                .output()
                .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            return Ok(());
        }

        // Handle PRESS
        if let Some(ref key) = action.press {
            let key_code = match key {
                PressKey::Enter => "{ENTER}",
                PressKey::Back => "{BACKSPACE}",
                PressKey::Home => "{HOME}",
                _ => {
                    return Err(GuiError::ActionFailed(format!(
                        "Key {:?} not supported",
                        key
                    )))
                }
            };
            let script = format!(
                "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('{}')",
                key_code
            );
            Command::new("powershell")
                .args(["-Command", &script])
                .output()
                .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            return Ok(());
        }

        // Handle click - requires external tool like nircmd or AutoHotkey
        if let Some(ref point) = action.point {
            let mut platform = DesktopPlatform::new(Platform::Windows);
            platform.resolution = self.resolution;
            let (x, y) = platform.to_absolute(point)?;

            if action.to.is_none() {
                let script = format!(
                    "Add-Type -MemberDefinition '[DllImport(\"user32.dll\")] public static extern bool SetCursorPos(int x, int y); \
                     [DllImport(\"user32.dll\")] public static extern void mouse_event(int flags, int dx, int dy, int data, int info);' \
                     -Name Win32 -Namespace System; \
                     [System.Win32]::SetCursorPos({}, {}); \
                     [System.Win32]::mouse_event(2, 0, 0, 0, 0); \
                     [System.Win32]::mouse_event(4, 0, 0, 0, 0)",
                    x, y
                );
                Command::new("powershell")
                    .args(["-Command", &script])
                    .output()
                    .map_err(|e| GuiError::ActionFailed(e.to_string()))?;
            }
        }

        // Handle wait
        if let Some(duration) = action.duration {
            if action.point.is_none() {
                std::thread::sleep(std::time::Duration::from_millis(duration as u64));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_platform_creation() {
        let platform = DesktopPlatform::detect_current();
        assert!(matches!(
            platform.platform(),
            Platform::MacOs | Platform::Windows | Platform::Linux
        ));
    }

    #[test]
    fn test_list_devices() {
        let platform = DesktopPlatform::detect_current();
        let devices = platform.list_devices().unwrap();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].id, "local");
    }
}
