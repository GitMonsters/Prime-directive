//! Platform Abstraction Layer
//!
//! This module provides a unified interface for GUI automation across
//! Android, iOS, and Desktop platforms.

pub mod android;
pub mod ios;
pub mod desktop;

use super::gui_agent::{
    ConnectionType, DeviceInfo, GuiAction, GuiError, GuiPlatform, 
    ImageFormat, Platform, Screenshot,
};

/// Factory for creating platform-specific GUI automation instances
pub struct PlatformFactory;

impl PlatformFactory {
    /// Create a platform instance for the given platform type
    pub fn create(platform: Platform) -> Box<dyn GuiPlatform> {
        match platform {
            Platform::Android => Box::new(android::AndroidPlatform::new()),
            Platform::Ios => Box::new(ios::IosPlatform::new()),
            Platform::Windows | Platform::MacOs | Platform::Linux => {
                Box::new(desktop::DesktopPlatform::new(platform))
            }
            Platform::Web => Box::new(desktop::DesktopPlatform::new(platform)),
        }
    }

    /// Detect available devices across all platforms
    pub fn detect_all_devices() -> Vec<DeviceInfo> {
        let mut devices = Vec::new();

        // Check Android
        if let Ok(android_devices) = android::AndroidPlatform::new().list_devices() {
            devices.extend(android_devices);
        }

        // Check iOS
        if let Ok(ios_devices) = ios::IosPlatform::new().list_devices() {
            devices.extend(ios_devices);
        }

        // Add local desktop
        let desktop = desktop::DesktopPlatform::detect_current();
        devices.push(DeviceInfo {
            id: "local".to_string(),
            name: "Local Desktop".to_string(),
            platform: desktop.platform(),
            os_version: Some(std::env::consts::OS.to_string()),
            resolution: None,
            connection: ConnectionType::Usb,
            active: false,
        });

        devices
    }
}

/// A multi-platform device manager
pub struct DeviceManager {
    /// Currently active platform
    active_platform: Option<Box<dyn GuiPlatform>>,
    /// Active device ID
    active_device_id: Option<String>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            active_platform: None,
            active_device_id: None,
        }
    }

    /// List all available devices
    pub fn list_devices(&self) -> Vec<DeviceInfo> {
        PlatformFactory::detect_all_devices()
    }

    /// Connect to a device by ID
    pub fn connect(&mut self, device_id: &str) -> Result<(), GuiError> {
        // Find the device
        let devices = self.list_devices();
        let device = devices
            .iter()
            .find(|d| d.id == device_id)
            .ok_or(GuiError::NoDevice)?;

        // Create platform instance
        let mut platform = PlatformFactory::create(device.platform);
        platform.connect(device_id)?;

        self.active_platform = Some(platform);
        self.active_device_id = Some(device_id.to_string());

        Ok(())
    }

    /// Disconnect from current device
    pub fn disconnect(&mut self) -> Result<(), GuiError> {
        if let Some(ref mut platform) = self.active_platform {
            platform.disconnect()?;
        }
        self.active_platform = None;
        self.active_device_id = None;
        Ok(())
    }

    /// Get the active platform
    pub fn active_platform(&self) -> Option<&dyn GuiPlatform> {
        self.active_platform.as_ref().map(|p| p.as_ref())
    }

    /// Get the active platform mutably
    pub fn active_platform_mut(&mut self) -> Option<&mut Box<dyn GuiPlatform>> {
        self.active_platform.as_mut()
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.active_platform
            .as_ref()
            .map(|p| p.is_connected())
            .unwrap_or(false)
    }

    /// Get active device ID
    pub fn active_device_id(&self) -> Option<&str> {
        self.active_device_id.as_deref()
    }

    /// Capture screenshot from active device
    pub fn capture_screenshot(&self) -> Result<Screenshot, GuiError> {
        self.active_platform
            .as_ref()
            .ok_or(GuiError::NoDevice)?
            .capture_screenshot()
    }

    /// Execute action on active device
    pub fn execute_action(&self, action: &GuiAction) -> Result<(), GuiError> {
        self.active_platform
            .as_ref()
            .ok_or(GuiError::NoDevice)?
            .execute_action(action)
    }

    /// Get current app context
    pub fn get_context(&self) -> Result<String, GuiError> {
        self.active_platform
            .as_ref()
            .ok_or(GuiError::NoDevice)?
            .get_context()
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_factory() {
        let android = PlatformFactory::create(Platform::Android);
        assert_eq!(android.platform(), Platform::Android);

        let ios = PlatformFactory::create(Platform::Ios);
        assert_eq!(ios.platform(), Platform::Ios);

        let macos = PlatformFactory::create(Platform::MacOs);
        assert_eq!(macos.platform(), Platform::MacOs);
    }

    #[test]
    fn test_device_manager() {
        let manager = DeviceManager::new();
        assert!(!manager.is_connected());
        assert!(manager.active_device_id().is_none());
    }
}
