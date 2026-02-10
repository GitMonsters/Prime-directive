//! GUI Bridge - HTTP Client to AgentCPM-GUI Service
//!
//! This module provides the bridge between RustyWorm and the AgentCPM-GUI
//! vision-language model for screenshot analysis and action prediction.

use super::gui_agent::{
    GuiAction, GuiAgentConfig, GuiAgentResult, GuiError, ImageFormat, NormalizedPoint, Platform,
    Screenshot, TaskStatus,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "gui")]
use reqwest::blocking::Client;

/// Configuration for the AgentCPM-GUI bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiBridgeConfig {
    /// Service endpoint URL
    pub endpoint: String,
    /// Model name being served
    pub model_name: String,
    /// Request timeout in milliseconds
    pub timeout_ms: u32,
    /// Maximum tokens for response
    pub max_tokens: u32,
    /// Temperature for sampling
    pub temperature: f32,
    /// Top-p for nucleus sampling
    pub top_p: f32,
    /// Whether to enable thought/reasoning output
    pub enable_thought: bool,
    /// System prompt language
    pub language: Language,
}

/// Language for system prompts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Default for GuiBridgeConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8000/v1/chat/completions".to_string(),
            model_name: "AgentCPM-GUI".to_string(),
            timeout_ms: 30000,
            max_tokens: 2048,
            temperature: 0.1,
            top_p: 0.3,
            enable_thought: true,
            language: Language::English,
        }
    }
}

/// The GUI Bridge client for AgentCPM-GUI
pub struct GuiBridge {
    config: GuiBridgeConfig,
    #[cfg(feature = "gui")]
    client: Client,
    /// Action schema for the model
    action_schema: String,
    /// System prompt
    system_prompt: String,
}

impl GuiBridge {
    /// Create a new GUI bridge with the given configuration
    pub fn new(config: GuiBridgeConfig) -> Self {
        let action_schema = Self::build_action_schema(config.enable_thought);
        let system_prompt = Self::build_system_prompt(&action_schema, config.language);

        #[cfg(feature = "gui")]
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms as u64))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            #[cfg(feature = "gui")]
            client,
            action_schema,
            system_prompt,
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(GuiBridgeConfig::default())
    }

    /// Build the action schema JSON
    fn build_action_schema(enable_thought: bool) -> String {
        let thought_requirement = if enable_thought {
            "required"
        } else {
            "optional"
        };

        serde_json::json!({
            "type": "object",
            "description": "Execute action and determine task status",
            "additionalProperties": false,
            "properties": {
                "thought": {
                    "type": "string",
                    "description": "Agent's reasoning process"
                },
                "POINT": {
                    "$ref": "#/$defs/Location",
                    "description": "Click at specified screen position"
                },
                "to": {
                    "description": "Movement gesture parameter",
                    "oneOf": [
                        {
                            "enum": ["up", "down", "left", "right"],
                            "description": "Swipe direction from current POINT"
                        },
                        {
                            "$ref": "#/$defs/Location",
                            "description": "Swipe to specific location"
                        }
                    ]
                },
                "duration": {
                    "type": "integer",
                    "description": "Action duration or wait time in milliseconds",
                    "minimum": 0,
                    "default": 200
                },
                "PRESS": {
                    "type": "string",
                    "description": "Hardware key: HOME (home), BACK (back), ENTER (enter)",
                    "enum": ["HOME", "BACK", "ENTER"]
                },
                "TYPE": {
                    "type": "string",
                    "description": "Text input"
                },
                "STATUS": {
                    "type": "string",
                    "description": "Task status",
                    "enum": ["continue", "finish", "satisfied", "impossible", "interrupt", "need_feedback"],
                    "default": "continue"
                }
            },
            "required": [thought_requirement],
            "$defs": {
                "Location": {
                    "type": "array",
                    "description": "Coordinates relative to top-left origin, scaled 0-1000",
                    "items": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 1000
                    },
                    "minItems": 2,
                    "maxItems": 2
                }
            }
        }).to_string()
    }

    /// Build the system prompt
    fn build_system_prompt(schema: &str, language: Language) -> String {
        match language {
            Language::Chinese => format!(
                r#"# Role
你是一名熟悉移动设备触屏GUI操作的智能体，将根据用户的问题，分析当前界面的GUI元素和布局，生成相应的操作。

# Task
针对用户问题，根据输入的当前屏幕截图，输出下一步的操作。

# Rule
- 以紧凑JSON格式输出
- 输出操作必须遵循Schema约束

# Schema
{}"#,
                schema
            ),
            Language::English => format!(
                r#"# Role
You are an intelligent agent familiar with mobile device touchscreen GUI operations. Based on the user's question, you analyze the current interface's GUI elements and layout to generate appropriate actions.

# Task
For the user's question, based on the current screenshot input, output the next action to take.

# Rule
- Output in compact JSON format
- Output must follow the Schema constraints

# Schema
{}"#,
                schema
            ),
        }
    }

    /// Predict the next action given a screenshot and instruction
    #[cfg(feature = "gui")]
    pub fn predict(
        &self,
        screenshot: &Screenshot,
        instruction: &str,
    ) -> Result<GuiAgentResult, GuiError> {
        use std::time::Instant;

        let start = Instant::now();

        // Resize screenshot if needed
        let image_data = self.prepare_image(screenshot)?;
        let base64_image = base64::encode(&image_data);

        // Build the request
        let request = self.build_request(instruction, &base64_image);

        // Send request
        let response = self
            .client
            .post(&self.config.endpoint)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| GuiError::Internal(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(GuiError::Internal(format!(
                "API returned error status: {}",
                response.status()
            )));
        }

        let response_body: ChatCompletionResponse = response
            .json()
            .map_err(|e| GuiError::Internal(format!("Failed to parse response: {}", e)))?;

        let raw_output = response_body
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        // Parse the action from the response
        let action = self.parse_action(&raw_output)?;

        let latency_ms = start.elapsed().as_millis() as u32;

        Ok(GuiAgentResult {
            action,
            confidence: 1.0, // AgentCPM-GUI doesn't provide confidence scores
            raw_output: Some(raw_output),
            latency_ms,
            model: self.config.model_name.clone(),
        })
    }

    /// Predict action (stub for non-gui builds)
    #[cfg(not(feature = "gui"))]
    pub fn predict(
        &self,
        _screenshot: &Screenshot,
        _instruction: &str,
    ) -> Result<GuiAgentResult, GuiError> {
        Err(GuiError::PlatformNotSupported)
    }

    /// Prepare image data (resize if needed)
    fn prepare_image(&self, screenshot: &Screenshot) -> Result<Vec<u8>, GuiError> {
        // For now, just return the original data
        // In production, you'd resize to max_screenshot_dimension
        Ok(screenshot.data.clone())
    }

    /// Build the chat completion request
    #[cfg(feature = "gui")]
    fn build_request(&self, instruction: &str, base64_image: &str) -> ChatCompletionRequest {
        let user_content = match self.config.language {
            Language::Chinese => format!("<Question>{}</Question>\n当前屏幕截图：", instruction),
            Language::English => {
                format!("<Question>{}</Question>\nCurrent screenshot:", instruction)
            }
        };

        ChatCompletionRequest {
            model: self.config.model_name.clone(),
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: MessageContent::Text(self.system_prompt.clone()),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: MessageContent::MultiPart(vec![
                        ContentPart::Text { text: user_content },
                        ContentPart::ImageUrl {
                            image_url: ImageUrl {
                                url: format!("data:image/jpeg;base64,{}", base64_image),
                            },
                        },
                    ]),
                },
            ],
        }
    }

    /// Parse the action from model output
    fn parse_action(&self, output: &str) -> Result<GuiAction, GuiError> {
        // Try to parse as JSON directly
        if let Ok(action) = serde_json::from_str::<GuiAction>(output) {
            return Ok(action);
        }

        // Try to extract JSON from the output
        if let Some(json_start) = output.find('{') {
            if let Some(json_end) = output.rfind('}') {
                let json_str = &output[json_start..=json_end];
                if let Ok(action) = serde_json::from_str::<GuiAction>(json_str) {
                    return Ok(action);
                }
            }
        }

        Err(GuiError::Internal(format!(
            "Failed to parse action from output: {}",
            output
        )))
    }

    /// Get the current configuration
    pub fn config(&self) -> &GuiBridgeConfig {
        &self.config
    }

    /// Update the endpoint
    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) {
        self.config.endpoint = endpoint.into();
    }

    /// Check if the service is healthy
    #[cfg(feature = "gui")]
    pub fn health_check(&self) -> Result<bool, GuiError> {
        // Try to hit the models endpoint
        let models_url = self.config.endpoint.replace("/chat/completions", "/models");

        let response = self
            .client
            .get(&models_url)
            .send()
            .map_err(|e| GuiError::ConnectionFailed(e.to_string()))?;

        Ok(response.status().is_success())
    }

    #[cfg(not(feature = "gui"))]
    pub fn health_check(&self) -> Result<bool, GuiError> {
        Err(GuiError::PlatformNotSupported)
    }
}

// ============================================================================
// Request/Response types for OpenAI-compatible API
// ============================================================================

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    temperature: f32,
    max_tokens: u32,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: MessageContent,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MessageContent {
    Text(String),
    MultiPart(Vec<ContentPart>),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

// ============================================================================
// Batch processing support
// ============================================================================

/// Batch prediction request
#[derive(Debug, Clone)]
pub struct BatchPredictRequest {
    /// Screenshot for analysis
    pub screenshot: Screenshot,
    /// Task instruction
    pub instruction: String,
    /// Request ID for tracking
    pub request_id: String,
}

/// Batch prediction result
#[derive(Debug, Clone)]
pub struct BatchPredictResult {
    /// Request ID
    pub request_id: String,
    /// Prediction result
    pub result: Result<GuiAgentResult, String>,
}

impl GuiBridge {
    /// Predict actions for multiple screenshots in batch
    #[cfg(feature = "gui")]
    pub fn predict_batch(&self, requests: Vec<BatchPredictRequest>) -> Vec<BatchPredictResult> {
        requests
            .into_iter()
            .map(|req| {
                let result = self
                    .predict(&req.screenshot, &req.instruction)
                    .map_err(|e| e.to_string());
                BatchPredictResult {
                    request_id: req.request_id,
                    result,
                }
            })
            .collect()
    }

    #[cfg(not(feature = "gui"))]
    pub fn predict_batch(&self, requests: Vec<BatchPredictRequest>) -> Vec<BatchPredictResult> {
        requests
            .into_iter()
            .map(|req| BatchPredictResult {
                request_id: req.request_id,
                result: Err("GUI feature not enabled".to_string()),
            })
            .collect()
    }
}

// ============================================================================
// Multi-turn conversation support
// ============================================================================

/// A conversation context for multi-turn GUI interactions
#[derive(Debug, Clone)]
pub struct GuiConversation {
    /// Conversation history
    history: Vec<(String, GuiAction)>,
    /// Maximum history length
    max_history: usize,
}

impl GuiConversation {
    /// Create a new conversation
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history,
        }
    }

    /// Add an interaction to history
    pub fn add_turn(&mut self, instruction: &str, action: GuiAction) {
        self.history.push((instruction.to_string(), action));
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Get history as formatted context
    pub fn get_context(&self) -> String {
        self.history
            .iter()
            .enumerate()
            .map(|(i, (instr, action))| {
                format!(
                    "Turn {}: Instruction: {} -> Action: {}",
                    i + 1,
                    instr,
                    action.to_compact_json()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.history.clear();
    }

    /// Get history length
    pub fn len(&self) -> usize {
        self.history.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_config_default() {
        let config = GuiBridgeConfig::default();
        assert_eq!(config.model_name, "AgentCPM-GUI");
        assert_eq!(config.temperature, 0.1);
        assert!(config.enable_thought);
    }

    #[test]
    fn test_action_schema_generation() {
        let schema = GuiBridge::build_action_schema(true);
        assert!(schema.contains("thought"));
        assert!(schema.contains("POINT"));
        assert!(schema.contains("required"));
    }

    #[test]
    fn test_system_prompt_languages() {
        let schema = "{}";

        let chinese = GuiBridge::build_system_prompt(schema, Language::Chinese);
        assert!(chinese.contains("智能体"));

        let english = GuiBridge::build_system_prompt(schema, Language::English);
        assert!(english.contains("intelligent agent"));
    }

    #[test]
    fn test_parse_action() {
        let bridge = GuiBridge::with_defaults();

        // Direct JSON
        let json = r#"{"POINT":[500,300],"thought":"Clicking button"}"#;
        let action = bridge.parse_action(json).unwrap();
        assert_eq!(action.point.unwrap().x, 500);
        assert!(action.thought.is_some());

        // JSON embedded in text
        let text = r#"Here's the action: {"POINT":[100,200]} and that's it."#;
        let action = bridge.parse_action(text).unwrap();
        assert_eq!(action.point.unwrap().x, 100);
    }

    #[test]
    fn test_gui_conversation() {
        let mut conv = GuiConversation::new(5);
        assert!(conv.is_empty());

        conv.add_turn("Click login", GuiAction::click(500, 300));
        conv.add_turn("Type username", GuiAction::type_text("user123"));

        assert_eq!(conv.len(), 2);

        let context = conv.get_context();
        assert!(context.contains("Turn 1"));
        assert!(context.contains("Click login"));
    }

    #[test]
    fn test_conversation_max_history() {
        let mut conv = GuiConversation::new(3);

        for i in 0..5 {
            conv.add_turn(&format!("Action {}", i), GuiAction::click(i * 100, i * 100));
        }

        assert_eq!(conv.len(), 3);
        let context = conv.get_context();
        assert!(!context.contains("Action 0"));
        assert!(context.contains("Action 4"));
    }
}
