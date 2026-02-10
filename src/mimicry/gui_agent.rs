//! GUI Agent Module - Platform-Agnostic GUI Action Types and Traits
//!
//! This module provides the core abstractions for GUI-based agent interactions,
//! enabling RustyWorm to observe and mimic GUI interaction patterns across
//! Android, iOS, and Desktop platforms.
//!
//! Based on AgentCPM-GUI action space with extensions for cross-platform support.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Normalized screen coordinate (0-1000 range)
/// Origin is top-left corner of the screen
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NormalizedPoint {
    /// X coordinate (0-1000, left to right)
    pub x: u16,
    /// Y coordinate (0-1000, top to bottom)
    pub y: u16,
}

impl NormalizedPoint {
    /// Create a new normalized point
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x: x.min(1000),
            y: y.min(1000),
        }
    }

    /// Convert from absolute coordinates given screen dimensions
    pub fn from_absolute(abs_x: u32, abs_y: u32, width: u32, height: u32) -> Self {
        Self {
            x: ((abs_x as f64 / width as f64) * 1000.0) as u16,
            y: ((abs_y as f64 / height as f64) * 1000.0) as u16,
        }
    }

    /// Convert to absolute coordinates given screen dimensions
    pub fn to_absolute(&self, width: u32, height: u32) -> (u32, u32) {
        let abs_x = (self.x as f64 / 1000.0 * width as f64) as u32;
        let abs_y = (self.y as f64 / 1000.0 * height as f64) as u32;
        (abs_x, abs_y)
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &NormalizedPoint) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

impl fmt::Display for NormalizedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

/// Swipe direction for gesture actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SwipeDirection {
    /// Get the opposite direction
    pub fn opposite(&self) -> Self {
        match self {
            SwipeDirection::Up => SwipeDirection::Down,
            SwipeDirection::Down => SwipeDirection::Up,
            SwipeDirection::Left => SwipeDirection::Right,
            SwipeDirection::Right => SwipeDirection::Left,
        }
    }

    /// Calculate end point from start point for a standard swipe
    pub fn end_point(&self, start: &NormalizedPoint, distance: u16) -> NormalizedPoint {
        let distance = distance.min(1000);
        match self {
            SwipeDirection::Up => NormalizedPoint::new(start.x, start.y.saturating_sub(distance)),
            SwipeDirection::Down => NormalizedPoint::new(start.x, (start.y + distance).min(1000)),
            SwipeDirection::Left => NormalizedPoint::new(start.x.saturating_sub(distance), start.y),
            SwipeDirection::Right => NormalizedPoint::new((start.x + distance).min(1000), start.y),
        }
    }
}

/// Hardware/navigation button types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PressKey {
    /// Home button - return to launcher/home screen
    Home,
    /// Back button - navigate back
    Back,
    /// Enter/Return key
    Enter,
    /// Recent apps button (Android)
    Recent,
    /// Volume up
    VolumeUp,
    /// Volume down
    VolumeDown,
    /// Power button
    Power,
    /// Screenshot combination
    Screenshot,
}

/// Task status for multi-step interactions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    /// Task is starting
    Start,
    /// Task is in progress
    Continue,
    /// Task completed successfully
    Finish,
    /// Task goal already satisfied (no action needed)
    Satisfied,
    /// Task is impossible to complete
    Impossible,
    /// Task was interrupted
    Interrupt,
    /// Need user feedback to continue
    NeedFeedback,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Continue
    }
}

/// Swipe target - either a direction or specific endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwipeTarget {
    /// Swipe in a direction
    Direction(SwipeDirection),
    /// Swipe to a specific point
    Point(NormalizedPoint),
}

/// A GUI action that can be executed on any platform
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiAction {
    /// Optional thought/reasoning for this action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought: Option<String>,

    /// Click/tap at a point
    #[serde(rename = "POINT", skip_serializing_if = "Option::is_none")]
    pub point: Option<NormalizedPoint>,

    /// Swipe target (direction or endpoint)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<SwipeTarget>,

    /// Duration in milliseconds (for long press, swipe speed, or wait)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// Press a hardware/navigation key
    #[serde(rename = "PRESS", skip_serializing_if = "Option::is_none")]
    pub press: Option<PressKey>,

    /// Type text input
    #[serde(rename = "TYPE", skip_serializing_if = "Option::is_none")]
    pub type_text: Option<String>,

    /// Task status update
    #[serde(rename = "STATUS", skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
}

impl GuiAction {
    /// Create a click action
    pub fn click(x: u16, y: u16) -> Self {
        Self {
            point: Some(NormalizedPoint::new(x, y)),
            thought: None,
            to: None,
            duration: None,
            press: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a click action with thought
    pub fn click_with_thought(x: u16, y: u16, thought: impl Into<String>) -> Self {
        Self {
            point: Some(NormalizedPoint::new(x, y)),
            thought: Some(thought.into()),
            to: None,
            duration: None,
            press: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a long press action
    pub fn long_press(x: u16, y: u16, duration_ms: u32) -> Self {
        Self {
            point: Some(NormalizedPoint::new(x, y)),
            duration: Some(duration_ms.max(200)),
            thought: None,
            to: None,
            press: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a swipe action with direction
    pub fn swipe_direction(x: u16, y: u16, direction: SwipeDirection) -> Self {
        Self {
            point: Some(NormalizedPoint::new(x, y)),
            to: Some(SwipeTarget::Direction(direction)),
            thought: None,
            duration: None,
            press: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a swipe action to a specific point
    pub fn swipe_to(from_x: u16, from_y: u16, to_x: u16, to_y: u16) -> Self {
        Self {
            point: Some(NormalizedPoint::new(from_x, from_y)),
            to: Some(SwipeTarget::Point(NormalizedPoint::new(to_x, to_y))),
            thought: None,
            duration: None,
            press: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a key press action
    pub fn press_key(key: PressKey) -> Self {
        Self {
            press: Some(key),
            thought: None,
            point: None,
            to: None,
            duration: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a type text action
    pub fn type_text(text: impl Into<String>) -> Self {
        Self {
            type_text: Some(text.into()),
            thought: None,
            point: None,
            to: None,
            duration: None,
            press: None,
            status: None,
        }
    }

    /// Create a wait action
    pub fn wait(duration_ms: u32) -> Self {
        Self {
            duration: Some(duration_ms),
            thought: None,
            point: None,
            to: None,
            press: None,
            type_text: None,
            status: None,
        }
    }

    /// Create a status update action
    pub fn status(status: TaskStatus) -> Self {
        Self {
            status: Some(status),
            thought: None,
            point: None,
            to: None,
            duration: None,
            press: None,
            type_text: None,
        }
    }

    /// Add thought to an existing action
    pub fn with_thought(mut self, thought: impl Into<String>) -> Self {
        self.thought = Some(thought.into());
        self
    }

    /// Add status to an existing action
    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Get the action type as a string
    pub fn action_type(&self) -> &'static str {
        if self.type_text.is_some() {
            "type"
        } else if self.press.is_some() {
            "press"
        } else if self.to.is_some() {
            "swipe"
        } else if self.point.is_some() {
            if self.duration.map(|d| d >= 200).unwrap_or(false) {
                "long_press"
            } else {
                "click"
            }
        } else if self.duration.is_some() {
            "wait"
        } else if self.status.is_some() {
            "status"
        } else {
            "unknown"
        }
    }

    /// Check if this is a primitive action (not just status update)
    pub fn is_primitive(&self) -> bool {
        self.point.is_some() || self.press.is_some() || self.type_text.is_some()
    }

    /// Serialize to compact JSON (no whitespace)
    pub fn to_compact_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl Default for GuiAction {
    fn default() -> Self {
        Self {
            thought: None,
            point: None,
            to: None,
            duration: None,
            press: None,
            type_text: None,
            status: Some(TaskStatus::Continue),
        }
    }
}

/// Screenshot data from a device
#[derive(Debug, Clone)]
pub struct Screenshot {
    /// Raw image bytes (PNG or JPEG)
    pub data: Vec<u8>,
    /// Image format
    pub format: ImageFormat,
    /// Original width in pixels
    pub width: u32,
    /// Original height in pixels
    pub height: u32,
    /// Timestamp when captured
    pub timestamp: u64,
    /// Platform that captured it
    pub platform: Platform,
}

/// Image format for screenshots
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
}

/// Platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Android,
    Ios,
    Windows,
    MacOs,
    Linux,
    Web,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Android => write!(f, "android"),
            Platform::Ios => write!(f, "ios"),
            Platform::Windows => write!(f, "windows"),
            Platform::MacOs => write!(f, "macos"),
            Platform::Linux => write!(f, "linux"),
            Platform::Web => write!(f, "web"),
        }
    }
}

/// A recorded GUI interaction (for learning patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiInteraction {
    /// The action that was taken
    pub action: GuiAction,
    /// Screenshot before the action (base64 encoded or path)
    pub screenshot_before: Option<String>,
    /// Screenshot after the action
    pub screenshot_after: Option<String>,
    /// Time taken to execute (ms)
    pub execution_time_ms: u32,
    /// Whether the action succeeded
    pub success: bool,
    /// Platform where interaction occurred
    pub platform: Platform,
    /// App or window context
    pub context: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Configuration for GUI agent behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiAgentConfig {
    /// Maximum wait time for screen to stabilize (ms)
    pub screen_stabilize_timeout_ms: u32,
    /// Default swipe duration (ms)
    pub default_swipe_duration_ms: u32,
    /// Default long press duration (ms)
    pub default_long_press_duration_ms: u32,
    /// Whether to capture screenshots before/after actions
    pub capture_screenshots: bool,
    /// Maximum screenshot dimension (for resizing)
    pub max_screenshot_dimension: u32,
    /// Confidence threshold for action execution
    pub confidence_threshold: f64,
    /// Whether to enable thought generation
    pub enable_thought: bool,
    /// Target platform
    pub platform: Platform,
}

impl Default for GuiAgentConfig {
    fn default() -> Self {
        Self {
            screen_stabilize_timeout_ms: 2000,
            default_swipe_duration_ms: 300,
            default_long_press_duration_ms: 1000,
            capture_screenshots: true,
            max_screenshot_dimension: 1120,
            confidence_threshold: 0.7,
            enable_thought: true,
            platform: Platform::Android,
        }
    }
}

/// Result of a GUI agent inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiAgentResult {
    /// The predicted action
    pub action: GuiAction,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Raw model output
    pub raw_output: Option<String>,
    /// Processing time (ms)
    pub latency_ms: u32,
    /// Model used for inference
    pub model: String,
}

/// Trait for platform-specific GUI automation
pub trait GuiPlatform: Send + Sync {
    /// Get the platform type
    fn platform(&self) -> Platform;

    /// Check if a device is connected
    fn is_connected(&self) -> bool;

    /// Capture a screenshot
    fn capture_screenshot(&self) -> Result<Screenshot, GuiError>;

    /// Execute a GUI action
    fn execute_action(&self, action: &GuiAction) -> Result<(), GuiError>;

    /// Get current app/window context
    fn get_context(&self) -> Result<String, GuiError>;

    /// List available devices
    fn list_devices(&self) -> Result<Vec<DeviceInfo>, GuiError>;

    /// Connect to a specific device
    fn connect(&mut self, device_id: &str) -> Result<(), GuiError>;

    /// Disconnect from current device
    fn disconnect(&mut self) -> Result<(), GuiError>;
}

/// Information about a connected device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device identifier
    pub id: String,
    /// Device name/model
    pub name: String,
    /// Platform type
    pub platform: Platform,
    /// OS version
    pub os_version: Option<String>,
    /// Screen resolution
    pub resolution: Option<(u32, u32)>,
    /// Connection type
    pub connection: ConnectionType,
    /// Whether device is currently selected
    pub active: bool,
}

/// How the device is connected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Usb,
    Wifi,
    Emulator,
    Simulator,
    Remote,
}

/// Errors that can occur in GUI operations
#[derive(Debug, Clone)]
pub enum GuiError {
    /// No device connected
    NoDevice,
    /// Device disconnected during operation
    DeviceDisconnected,
    /// Failed to capture screenshot
    ScreenshotFailed(String),
    /// Failed to execute action
    ActionFailed(String),
    /// Timeout waiting for operation
    Timeout,
    /// Invalid coordinates
    InvalidCoordinates,
    /// Platform not supported
    PlatformNotSupported,
    /// Connection failed
    ConnectionFailed(String),
    /// Permission denied
    PermissionDenied,
    /// Internal error
    Internal(String),
}

impl fmt::Display for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuiError::NoDevice => write!(f, "No device connected"),
            GuiError::DeviceDisconnected => write!(f, "Device disconnected"),
            GuiError::ScreenshotFailed(msg) => write!(f, "Screenshot failed: {}", msg),
            GuiError::ActionFailed(msg) => write!(f, "Action failed: {}", msg),
            GuiError::Timeout => write!(f, "Operation timed out"),
            GuiError::InvalidCoordinates => write!(f, "Invalid coordinates"),
            GuiError::PlatformNotSupported => write!(f, "Platform not supported"),
            GuiError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            GuiError::PermissionDenied => write!(f, "Permission denied"),
            GuiError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for GuiError {}

/// A sequence of GUI actions forming a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiTaskSequence {
    /// Task description/instruction
    pub instruction: String,
    /// Sequence of actions
    pub actions: Vec<GuiAction>,
    /// Whether the sequence completed successfully
    pub completed: bool,
    /// Total execution time (ms)
    pub total_time_ms: u32,
    /// Platform where executed
    pub platform: Platform,
    /// App context
    pub app_context: Option<String>,
}

impl GuiTaskSequence {
    /// Create a new empty task sequence
    pub fn new(instruction: impl Into<String>, platform: Platform) -> Self {
        Self {
            instruction: instruction.into(),
            actions: Vec::new(),
            completed: false,
            total_time_ms: 0,
            platform,
            app_context: None,
        }
    }

    /// Add an action to the sequence
    pub fn add_action(&mut self, action: GuiAction) {
        self.actions.push(action);
    }

    /// Mark as completed
    pub fn complete(&mut self, total_time_ms: u32) {
        self.completed = true;
        self.total_time_ms = total_time_ms;
    }

    /// Get the number of actions
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Get action types summary
    pub fn action_summary(&self) -> Vec<(&'static str, usize)> {
        let mut counts = std::collections::HashMap::new();
        for action in &self.actions {
            *counts.entry(action.action_type()).or_insert(0) += 1;
        }
        let mut summary: Vec<_> = counts.into_iter().collect();
        summary.sort_by(|a, b| b.1.cmp(&a.1));
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_point_creation() {
        let point = NormalizedPoint::new(500, 500);
        assert_eq!(point.x, 500);
        assert_eq!(point.y, 500);

        // Test clamping
        let clamped = NormalizedPoint::new(1500, 2000);
        assert_eq!(clamped.x, 1000);
        assert_eq!(clamped.y, 1000);
    }

    #[test]
    fn test_normalized_point_conversion() {
        let point = NormalizedPoint::from_absolute(540, 960, 1080, 1920);
        assert_eq!(point.x, 500);
        assert_eq!(point.y, 500);

        let (abs_x, abs_y) = point.to_absolute(1080, 1920);
        assert_eq!(abs_x, 540);
        assert_eq!(abs_y, 960);
    }

    #[test]
    fn test_swipe_direction_end_point() {
        let start = NormalizedPoint::new(500, 500);

        let up = SwipeDirection::Up.end_point(&start, 200);
        assert_eq!(up.y, 300);

        let down = SwipeDirection::Down.end_point(&start, 200);
        assert_eq!(down.y, 700);

        let left = SwipeDirection::Left.end_point(&start, 200);
        assert_eq!(left.x, 300);

        let right = SwipeDirection::Right.end_point(&start, 200);
        assert_eq!(right.x, 700);
    }

    #[test]
    fn test_gui_action_click() {
        let action = GuiAction::click(100, 200);
        assert_eq!(action.action_type(), "click");
        assert!(action.is_primitive());
        assert_eq!(action.point.unwrap().x, 100);
        assert_eq!(action.point.unwrap().y, 200);
    }

    #[test]
    fn test_gui_action_long_press() {
        let action = GuiAction::long_press(100, 200, 1000);
        assert_eq!(action.action_type(), "long_press");
        assert_eq!(action.duration.unwrap(), 1000);
    }

    #[test]
    fn test_gui_action_swipe() {
        let action = GuiAction::swipe_direction(500, 500, SwipeDirection::Down);
        assert_eq!(action.action_type(), "swipe");
        assert!(matches!(
            action.to,
            Some(SwipeTarget::Direction(SwipeDirection::Down))
        ));
    }

    #[test]
    fn test_gui_action_type_text() {
        let action = GuiAction::type_text("Hello, World!");
        assert_eq!(action.action_type(), "type");
        assert_eq!(action.type_text.as_deref(), Some("Hello, World!"));
    }

    #[test]
    fn test_gui_action_serialization() {
        let action = GuiAction::click_with_thought(500, 300, "Clicking the submit button");
        let json = action.to_compact_json();
        assert!(json.contains("POINT"));
        assert!(json.contains("thought"));
        // Check it's compact (no unnecessary whitespace between keys/values)
        assert!(json.contains("\"POINT\":"));
        assert!(json.contains("\"thought\":"));

        // Deserialize back
        let parsed: GuiAction = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.point.unwrap().x, 500);
        assert_eq!(
            parsed.thought.as_deref(),
            Some("Clicking the submit button")
        );
    }

    #[test]
    fn test_gui_task_sequence() {
        let mut sequence = GuiTaskSequence::new("Click the login button", Platform::Android);
        sequence.add_action(GuiAction::click(500, 300));
        sequence.add_action(GuiAction::type_text("username"));
        sequence.add_action(GuiAction::click(500, 400));
        sequence.add_action(GuiAction::type_text("password"));
        sequence.add_action(GuiAction::click(500, 600));

        assert_eq!(sequence.len(), 5);
        let summary = sequence.action_summary();
        assert!(summary.iter().any(|(t, c)| *t == "click" && *c == 3));
        assert!(summary.iter().any(|(t, c)| *t == "type" && *c == 2));
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(format!("{}", Platform::Android), "android");
        assert_eq!(format!("{}", Platform::Ios), "ios");
        assert_eq!(format!("{}", Platform::MacOs), "macos");
    }

    #[test]
    fn test_action_chaining() {
        let action = GuiAction::click(500, 500)
            .with_thought("Clicking button")
            .with_status(TaskStatus::Continue);

        assert!(action.thought.is_some());
        assert!(action.status.is_some());
    }
}
