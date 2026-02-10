//! Android Platform Implementation
//!
//! Provides GUI automation for Android devices via ADB (Android Debug Bridge).

use crate::mimicry::gui_agent::{
    ConnectionType, DeviceInfo, GuiAction, GuiError, GuiPlatform, ImageFormat, NormalizedPoint,
    Platform, PressKey, Screenshot, SwipeDirection, SwipeTarget,
};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Android platform implementation using ADB
pub struct AndroidPlatform {
    /// Connected device ID
    device_id: Option<String>,
    /// Screen resolution (cached)
    resolution: Option<(u32, u32)>,
    /// ADB path
    adb_path: String,
}

impl AndroidPlatform {
    /// Create a new Android platform instance
    pub fn new() -> Self {
        Self {
            device_id: None,
            resolution: None,
            adb_path: Self::find_adb(),
        }
    }

    /// Find ADB executable
    fn find_adb() -> String {
        // Check common locations
        let paths = [
            "adb",
            "/usr/bin/adb",
            "/usr/local/bin/adb",
            "~/Android/Sdk/platform-tools/adb",
        ];

        for path in paths {
            if Command::new(path).arg("version").output().is_ok() {
                return path.to_string();
            }
        }

        "adb".to_string() // Default fallback
    }

    /// Run an ADB command
    fn adb_command(&self, args: &[&str]) -> Result<String, GuiError> {
        let mut cmd = Command::new(&self.adb_path);

        if let Some(ref device_id) = self.device_id {
            cmd.arg("-s").arg(device_id);
        }

        let output = cmd
            .args(args)
            .output()
            .map_err(|e| GuiError::Internal(format!("ADB command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GuiError::ActionFailed(format!("ADB error: {}", stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get screen resolution
    fn get_resolution(&mut self) -> Result<(u32, u32), GuiError> {
        if let Some(res) = self.resolution {
            return Ok(res);
        }

        let output = self.adb_command(&["shell", "wm", "size"])?;

        // Parse "Physical size: 1080x1920"
        for line in output.lines() {
            if line.contains("Physical size") || line.contains("Override size") {
                if let Some(size) = line.split(':').nth(1) {
                    let parts: Vec<&str> = size.trim().split('x').collect();
                    if parts.len() == 2 {
                        if let (Ok(w), Ok(h)) = (parts[0].parse(), parts[1].parse()) {
                            self.resolution = Some((w, h));
                            return Ok((w, h));
                        }
                    }
                }
            }
        }

        Err(GuiError::Internal(
            "Could not determine screen resolution".to_string(),
        ))
    }

    /// Convert normalized point to absolute coordinates
    fn to_absolute(&mut self, point: &NormalizedPoint) -> Result<(u32, u32), GuiError> {
        let (width, height) = self.get_resolution()?;
        Ok(point.to_absolute(width, height))
    }
}

impl Default for AndroidPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiPlatform for AndroidPlatform {
    fn platform(&self) -> Platform {
        Platform::Android
    }

    fn is_connected(&self) -> bool {
        self.device_id.is_some()
    }

    fn capture_screenshot(&self) -> Result<Screenshot, GuiError> {
        // Capture to temp file on device
        self.adb_command(&["shell", "screencap", "-p", "/sdcard/screenshot.png"])?;

        // Pull the file
        let output = Command::new(&self.adb_path)
            .args(if let Some(ref id) = self.device_id {
                vec!["-s", id, "exec-out", "cat", "/sdcard/screenshot.png"]
            } else {
                vec!["exec-out", "cat", "/sdcard/screenshot.png"]
            })
            .output()
            .map_err(|e| GuiError::ScreenshotFailed(e.to_string()))?;

        if output.stdout.is_empty() {
            return Err(GuiError::ScreenshotFailed(
                "Empty screenshot data".to_string(),
            ));
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        // Get resolution
        let (width, height) = self.resolution.unwrap_or((1080, 1920));

        Ok(Screenshot {
            data: output.stdout,
            format: ImageFormat::Png,
            width,
            height,
            timestamp,
            platform: Platform::Android,
        })
    }

    fn execute_action(&self, action: &GuiAction) -> Result<(), GuiError> {
        // Handle TYPE first (doesn't need coordinates)
        if let Some(ref text) = action.type_text {
            // Escape special characters for shell
            let escaped = text.replace(' ', "%s").replace('\'', "\\'");
            self.adb_command(&["shell", "input", "text", &escaped])?;
            return Ok(());
        }

        // Handle PRESS
        if let Some(ref key) = action.press {
            let keycode = match key {
                PressKey::Home => "KEYCODE_HOME",
                PressKey::Back => "KEYCODE_BACK",
                PressKey::Enter => "KEYCODE_ENTER",
                PressKey::Recent => "KEYCODE_APP_SWITCH",
                PressKey::VolumeUp => "KEYCODE_VOLUME_UP",
                PressKey::VolumeDown => "KEYCODE_VOLUME_DOWN",
                PressKey::Power => "KEYCODE_POWER",
                PressKey::Screenshot => "KEYCODE_SYSRQ",
            };
            self.adb_command(&["shell", "input", "keyevent", keycode])?;
            return Ok(());
        }

        // Handle POINT-based actions (click, swipe, long press)
        if let Some(ref point) = action.point {
            let (x, y) = {
                let mut platform = AndroidPlatform {
                    device_id: self.device_id.clone(),
                    resolution: self.resolution,
                    adb_path: self.adb_path.clone(),
                };
                platform.to_absolute(point)?
            };

            // Check if it's a swipe
            if let Some(ref target) = action.to {
                let (end_x, end_y) = match target {
                    SwipeTarget::Direction(dir) => {
                        let distance = 500u16; // Default swipe distance
                        let end_point = dir.end_point(point, distance);
                        let mut platform = AndroidPlatform {
                            device_id: self.device_id.clone(),
                            resolution: self.resolution,
                            adb_path: self.adb_path.clone(),
                        };
                        platform.to_absolute(&end_point)?
                    }
                    SwipeTarget::Point(end) => {
                        let mut platform = AndroidPlatform {
                            device_id: self.device_id.clone(),
                            resolution: self.resolution,
                            adb_path: self.adb_path.clone(),
                        };
                        platform.to_absolute(end)?
                    }
                };

                let duration = action.duration.unwrap_or(300);
                self.adb_command(&[
                    "shell",
                    "input",
                    "swipe",
                    &x.to_string(),
                    &y.to_string(),
                    &end_x.to_string(),
                    &end_y.to_string(),
                    &duration.to_string(),
                ])?;
            } else if action.duration.map(|d| d >= 200).unwrap_or(false) {
                // Long press
                let duration = action.duration.unwrap();
                self.adb_command(&[
                    "shell",
                    "input",
                    "swipe",
                    &x.to_string(),
                    &y.to_string(),
                    &x.to_string(),
                    &y.to_string(),
                    &duration.to_string(),
                ])?;
            } else {
                // Regular tap
                self.adb_command(&["shell", "input", "tap", &x.to_string(), &y.to_string()])?;
            }

            return Ok(());
        }

        // Handle wait
        if let Some(duration) = action.duration {
            std::thread::sleep(std::time::Duration::from_millis(duration as u64));
            return Ok(());
        }

        Ok(())
    }

    fn get_context(&self) -> Result<String, GuiError> {
        // Get current focused app
        let output = self.adb_command(&[
            "shell",
            "dumpsys",
            "window",
            "windows",
            "|",
            "grep",
            "-E",
            "mCurrentFocus",
        ])?;

        // Parse the package name
        if let Some(start) = output.find('{') {
            if let Some(end) = output.find('}') {
                return Ok(output[start + 1..end].to_string());
            }
        }

        // Fallback: get current activity
        let output = self.adb_command(&[
            "shell",
            "dumpsys",
            "activity",
            "activities",
            "|",
            "grep",
            "mResumedActivity",
        ])?;

        Ok(output.lines().next().unwrap_or("unknown").to_string())
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>, GuiError> {
        let output = Command::new(&self.adb_path)
            .arg("devices")
            .arg("-l")
            .output()
            .map_err(|e| GuiError::Internal(format!("ADB failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                let id = parts[0].to_string();

                // Extract model name if available
                let name = parts
                    .iter()
                    .find(|p| p.starts_with("model:"))
                    .map(|p| p.replace("model:", ""))
                    .unwrap_or_else(|| id.clone());

                let connection = if id.contains(':') {
                    ConnectionType::Wifi
                } else if id.starts_with("emulator") {
                    ConnectionType::Emulator
                } else {
                    ConnectionType::Usb
                };

                devices.push(DeviceInfo {
                    id,
                    name,
                    platform: Platform::Android,
                    os_version: None,
                    resolution: None,
                    connection,
                    active: false,
                });
            }
        }

        Ok(devices)
    }

    fn connect(&mut self, device_id: &str) -> Result<(), GuiError> {
        // Verify device exists
        let devices = self.list_devices()?;
        if !devices.iter().any(|d| d.id == device_id) {
            return Err(GuiError::NoDevice);
        }

        self.device_id = Some(device_id.to_string());

        // Cache resolution
        let _ = self.get_resolution();

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), GuiError> {
        self.device_id = None;
        self.resolution = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_platform_creation() {
        let platform = AndroidPlatform::new();
        assert_eq!(platform.platform(), Platform::Android);
        assert!(!platform.is_connected());
    }

    #[test]
    fn test_find_adb() {
        // This just verifies the function runs without panic
        let path = AndroidPlatform::find_adb();
        assert!(!path.is_empty());
    }
}
