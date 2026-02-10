//! iOS Platform Implementation
//!
//! Provides GUI automation for iOS devices via libimobiledevice and related tools.
//! Supports both USB and WiFi connections to iPhones and iPads.

use crate::mimicry::gui_agent::{
    ConnectionType, DeviceInfo, GuiAction, GuiError, GuiPlatform, ImageFormat, NormalizedPoint,
    Platform, PressKey, Screenshot, SwipeDirection, SwipeTarget,
};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// iOS platform implementation using libimobiledevice tools
pub struct IosPlatform {
    /// Connected device UDID
    device_id: Option<String>,
    /// Screen resolution (cached)
    resolution: Option<(u32, u32)>,
    /// Path to idevice tools
    tool_prefix: String,
    /// WebDriverAgent URL (for UI automation)
    wda_url: Option<String>,
}

impl IosPlatform {
    /// Create a new iOS platform instance
    pub fn new() -> Self {
        Self {
            device_id: None,
            resolution: None,
            tool_prefix: Self::find_tools(),
            wda_url: None,
        }
    }

    /// Create with custom WebDriverAgent URL
    pub fn with_wda(wda_url: impl Into<String>) -> Self {
        Self {
            device_id: None,
            resolution: None,
            tool_prefix: Self::find_tools(),
            wda_url: Some(wda_url.into()),
        }
    }

    /// Find libimobiledevice tools
    fn find_tools() -> String {
        // Check if tools are available
        let tools = ["idevice_id", "idevicescreenshot", "ideviceinfo"];

        for prefix in ["", "/usr/local/bin/", "/opt/homebrew/bin/"] {
            let path = format!("{}idevice_id", prefix);
            if Command::new(&path).arg("-l").output().is_ok() {
                return prefix.to_string();
            }
        }

        String::new()
    }

    /// Run an idevice command
    fn idevice_command(&self, tool: &str, args: &[&str]) -> Result<String, GuiError> {
        let tool_path = format!("{}{}", self.tool_prefix, tool);
        let mut cmd = Command::new(&tool_path);

        if let Some(ref device_id) = self.device_id {
            cmd.arg("-u").arg(device_id);
        }

        let output = cmd
            .args(args)
            .output()
            .map_err(|e| GuiError::Internal(format!("idevice command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GuiError::ActionFailed(format!("idevice error: {}", stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute action via WebDriverAgent
    #[cfg(feature = "gui")]
    fn wda_action(&self, action: &WdaAction) -> Result<(), GuiError> {
        let wda_url = self
            .wda_url
            .as_ref()
            .ok_or_else(|| GuiError::Internal("WebDriverAgent not configured".to_string()))?;

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/session/current/wda/{}", wda_url, action.endpoint());

        let response = client
            .post(&url)
            .json(&action.body())
            .send()
            .map_err(|e| GuiError::ActionFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(GuiError::ActionFailed(format!(
                "WDA returned: {}",
                response.status()
            )));
        }

        Ok(())
    }

    #[cfg(not(feature = "gui"))]
    fn wda_action(&self, _action: &WdaAction) -> Result<(), GuiError> {
        Err(GuiError::PlatformNotSupported)
    }

    /// Convert normalized point to absolute coordinates
    fn to_absolute(&self, point: &NormalizedPoint) -> Result<(u32, u32), GuiError> {
        let (width, height) = self
            .resolution
            .ok_or_else(|| GuiError::Internal("Screen resolution not available".to_string()))?;
        Ok(point.to_absolute(width, height))
    }

    /// Get device information
    fn get_device_info(&self, key: &str) -> Result<String, GuiError> {
        self.idevice_command("ideviceinfo", &["-k", key])
            .map(|s| s.trim().to_string())
    }
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

/// WebDriverAgent action types
enum WdaAction {
    Tap {
        x: u32,
        y: u32,
    },
    DoubleTap {
        x: u32,
        y: u32,
    },
    LongPress {
        x: u32,
        y: u32,
        duration: f64,
    },
    Swipe {
        from_x: u32,
        from_y: u32,
        to_x: u32,
        to_y: u32,
        duration: f64,
    },
    TypeText {
        text: String,
    },
    PressButton {
        button: String,
    },
}

impl WdaAction {
    fn endpoint(&self) -> &'static str {
        match self {
            WdaAction::Tap { .. } => "tap",
            WdaAction::DoubleTap { .. } => "doubleTap",
            WdaAction::LongPress { .. } => "touchAndHold",
            WdaAction::Swipe { .. } => "dragFromToForDuration",
            WdaAction::TypeText { .. } => "keys",
            WdaAction::PressButton { .. } => "pressButton",
        }
    }

    fn body(&self) -> serde_json::Value {
        match self {
            WdaAction::Tap { x, y } => serde_json::json!({ "x": x, "y": y }),
            WdaAction::DoubleTap { x, y } => serde_json::json!({ "x": x, "y": y }),
            WdaAction::LongPress { x, y, duration } => {
                serde_json::json!({ "x": x, "y": y, "duration": duration })
            }
            WdaAction::Swipe {
                from_x,
                from_y,
                to_x,
                to_y,
                duration,
            } => {
                serde_json::json!({
                    "fromX": from_x, "fromY": from_y,
                    "toX": to_x, "toY": to_y,
                    "duration": duration
                })
            }
            WdaAction::TypeText { text } => serde_json::json!({ "value": [text] }),
            WdaAction::PressButton { button } => serde_json::json!({ "name": button }),
        }
    }
}

impl GuiPlatform for IosPlatform {
    fn platform(&self) -> Platform {
        Platform::Ios
    }

    fn is_connected(&self) -> bool {
        self.device_id.is_some()
    }

    fn capture_screenshot(&self) -> Result<Screenshot, GuiError> {
        // Create temp file
        let temp_path = format!("/tmp/ios_screenshot_{}.png", std::process::id());

        // Capture screenshot
        self.idevice_command("idevicescreenshot", &[&temp_path])?;

        // Read the file
        let data =
            std::fs::read(&temp_path).map_err(|e| GuiError::ScreenshotFailed(e.to_string()))?;

        // Clean up
        let _ = std::fs::remove_file(&temp_path);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let (width, height) = self.resolution.unwrap_or((1170, 2532)); // iPhone 13 default

        Ok(Screenshot {
            data,
            format: ImageFormat::Png,
            width,
            height,
            timestamp,
            platform: Platform::Ios,
        })
    }

    fn execute_action(&self, action: &GuiAction) -> Result<(), GuiError> {
        // Handle TYPE
        if let Some(ref text) = action.type_text {
            return self.wda_action(&WdaAction::TypeText { text: text.clone() });
        }

        // Handle PRESS
        if let Some(ref key) = action.press {
            let button = match key {
                PressKey::Home => "home",
                PressKey::VolumeUp => "volumeUp",
                PressKey::VolumeDown => "volumeDown",
                _ => {
                    return Err(GuiError::ActionFailed(format!(
                        "Key {:?} not supported on iOS",
                        key
                    )))
                }
            };
            return self.wda_action(&WdaAction::PressButton {
                button: button.to_string(),
            });
        }

        // Handle POINT-based actions
        if let Some(ref point) = action.point {
            let (x, y) = self.to_absolute(point)?;

            // Check if it's a swipe
            if let Some(ref target) = action.to {
                let (end_x, end_y) = match target {
                    SwipeTarget::Direction(dir) => {
                        let distance = 500u16;
                        let end_point = dir.end_point(point, distance);
                        self.to_absolute(&end_point)?
                    }
                    SwipeTarget::Point(end) => self.to_absolute(end)?,
                };

                let duration = action.duration.unwrap_or(300) as f64 / 1000.0;
                return self.wda_action(&WdaAction::Swipe {
                    from_x: x,
                    from_y: y,
                    to_x: end_x,
                    to_y: end_y,
                    duration,
                });
            } else if action.duration.map(|d| d >= 200).unwrap_or(false) {
                // Long press
                let duration = action.duration.unwrap() as f64 / 1000.0;
                return self.wda_action(&WdaAction::LongPress { x, y, duration });
            } else {
                // Regular tap
                return self.wda_action(&WdaAction::Tap { x, y });
            }
        }

        // Handle wait
        if let Some(duration) = action.duration {
            std::thread::sleep(std::time::Duration::from_millis(duration as u64));
            return Ok(());
        }

        Ok(())
    }

    fn get_context(&self) -> Result<String, GuiError> {
        // On iOS, we can get the bundle ID of frontmost app via WDA
        if let Some(ref wda_url) = self.wda_url {
            #[cfg(feature = "gui")]
            {
                let client = reqwest::blocking::Client::new();
                let url = format!("{}/session/current/wda/activeAppInfo", wda_url);

                if let Ok(response) = client.get(&url).send() {
                    if let Ok(json) = response.json::<serde_json::Value>() {
                        if let Some(bundle_id) = json["value"]["bundleId"].as_str() {
                            return Ok(bundle_id.to_string());
                        }
                    }
                }
            }
        }

        // Fallback
        Ok("unknown".to_string())
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>, GuiError> {
        let tool_path = format!("{}idevice_id", self.tool_prefix);
        let output = Command::new(&tool_path)
            .arg("-l")
            .output()
            .map_err(|e| GuiError::Internal(format!("idevice_id failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in stdout.lines() {
            let udid = line.trim();
            if !udid.is_empty() && !udid.contains("ERROR") {
                // Get device name
                let name = Command::new(format!("{}ideviceinfo", self.tool_prefix))
                    .args(["-u", udid, "-k", "DeviceName"])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "iPhone".to_string());

                // Get iOS version
                let os_version = Command::new(format!("{}ideviceinfo", self.tool_prefix))
                    .args(["-u", udid, "-k", "ProductVersion"])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| s.trim().to_string());

                // Determine connection type
                let connection = if udid.contains("-") && udid.len() > 30 {
                    ConnectionType::Wifi
                } else {
                    ConnectionType::Usb
                };

                devices.push(DeviceInfo {
                    id: udid.to_string(),
                    name,
                    platform: Platform::Ios,
                    os_version,
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

        // Try to get resolution
        if let Ok(width) = self.get_device_info("ScreenWidth") {
            if let Ok(height) = self.get_device_info("ScreenHeight") {
                if let (Ok(w), Ok(h)) = (width.parse(), height.parse()) {
                    self.resolution = Some((w, h));
                }
            }
        }

        // Set default resolution based on common devices
        if self.resolution.is_none() {
            self.resolution = Some((1170, 2532)); // iPhone 13/14 default
        }

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), GuiError> {
        self.device_id = None;
        self.resolution = None;
        Ok(())
    }
}

/// iOS Device Types and their resolutions
pub struct IosDeviceResolutions;

impl IosDeviceResolutions {
    /// Get resolution for known device models
    pub fn get_resolution(model: &str) -> Option<(u32, u32)> {
        match model {
            // iPhone 15 series
            "iPhone15,4" | "iPhone15,5" => Some((1179, 2556)), // iPhone 15 Pro Max
            "iPhone15,3" => Some((1179, 2556)),                // iPhone 15 Pro
            "iPhone15,2" => Some((1170, 2532)),                // iPhone 15 Plus
            "iPhone14,7" | "iPhone14,8" => Some((1170, 2532)), // iPhone 14

            // iPhone 13/14 series
            "iPhone14,2" | "iPhone14,3" => Some((1170, 2532)),
            "iPhone13,2" | "iPhone13,3" => Some((1170, 2532)),

            // iPhone SE
            "iPhone14,6" => Some((750, 1334)), // iPhone SE 3rd gen

            // iPad Pro
            "iPad13,4" | "iPad13,5" => Some((2048, 2732)), // iPad Pro 12.9"
            "iPad13,8" | "iPad13,9" => Some((1668, 2388)), // iPad Pro 11"

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_platform_creation() {
        let platform = IosPlatform::new();
        assert_eq!(platform.platform(), Platform::Ios);
        assert!(!platform.is_connected());
    }

    #[test]
    fn test_ios_device_resolutions() {
        assert!(IosDeviceResolutions::get_resolution("iPhone15,4").is_some());
        assert!(IosDeviceResolutions::get_resolution("unknown").is_none());
    }

    #[test]
    fn test_wda_action_body() {
        let tap = WdaAction::Tap { x: 100, y: 200 };
        let body = tap.body();
        assert_eq!(body["x"], 100);
        assert_eq!(body["y"], 200);
    }
}
