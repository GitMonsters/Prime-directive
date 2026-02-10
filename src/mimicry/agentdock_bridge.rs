// =================================================================
// AGENTDOCK BRIDGE: MCP Container Orchestration Integration
// =================================================================
// Integrates AgentDock's MCP (Model Context Protocol) platform into
// RustyWorm for multi-model observation and container lifecycle
// management.
//
// This module manages:
// - Container creation and lifecycle (create, close, release)
// - MCP tool calling via JSON-RPC
// - Multi-model session management
// - Long-horizon observation (100+ turns)
// - Health monitoring and fallback strategies
//
// AGENTDOCK API:
// - POST /node/create?image_name=agentdock-node-explore -> {container_id}
// - POST /container/{id}/mcp -> MCP JSON-RPC response
// - POST /node/close -> close container
// - POST /node/release -> release container
// - GET /node/details -> container info
// - GET /alive -> health check
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::mimicry::rl_optimizer::BehaviorObservation;

// =================================================================
// CONFIGURATION
// =================================================================

/// Configuration for AgentDock connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDockConfig {
    /// AgentDock manager URL (e.g., http://localhost:8080)
    pub manager_url: String,
    
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    
    /// Maximum containers to keep in pool
    pub max_pool_size: usize,
    
    /// Container idle timeout before release (seconds)
    pub container_idle_timeout_secs: u64,
    
    /// Default image for container creation
    pub default_image: String,
    
    /// Enable health monitoring
    pub enable_health_monitoring: bool,
    
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
}

impl Default for AgentDockConfig {
    fn default() -> Self {
        AgentDockConfig {
            manager_url: "http://localhost:8080".to_string(),
            request_timeout_ms: 60000,
            max_pool_size: 10,
            container_idle_timeout_secs: 300,
            default_image: "agentdock-node-explore".to_string(),
            enable_health_monitoring: true,
            health_check_interval_secs: 30,
        }
    }
}

// =================================================================
// MODEL CONFIGURATION
// =================================================================

/// Configuration for a specific model provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Human-readable model name
    pub name: String,
    
    /// API key for the model provider (optional for local models)
    pub api_key: Option<String>,
    
    /// Base URL for the model API
    pub base_url: String,
    
    /// Priority for scheduling (higher = preferred)
    pub priority: u8,
    
    /// Maximum tokens for this model
    pub max_tokens: u32,
    
    /// Temperature setting for generation
    pub temperature: f64,
    
    /// Model capabilities (e.g., "chat", "tools", "vision")
    pub capabilities: Vec<String>,
    
    /// Whether this model is currently healthy/available
    pub is_healthy: bool,
    
    /// Last health check timestamp
    pub last_health_check: Option<DateTime<Utc>>,
}

impl ModelConfig {
    /// Create a new model configuration
    pub fn new(name: &str, base_url: &str) -> Self {
        ModelConfig {
            name: name.to_string(),
            api_key: None,
            base_url: base_url.to_string(),
            priority: 1,
            max_tokens: 2048,
            temperature: 0.7,
            capabilities: vec!["chat".to_string()],
            is_healthy: true,
            last_health_check: None,
        }
    }
    
    /// Set API key
    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }
    
    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
    
    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
    
    /// Set temperature
    pub fn with_temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }
    
    /// Add capability
    pub fn with_capability(mut self, capability: &str) -> Self {
        self.capabilities.push(capability.to_string());
        self
    }
}

// =================================================================
// CONTAINER SESSION
// =================================================================

/// Represents an active container session with conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSession {
    /// Unique session identifier
    pub session_id: String,
    
    /// Container ID from AgentDock
    pub container_id: String,
    
    /// Model configuration for this session
    pub model_config: ModelConfig,
    
    /// Conversation history (role, content)
    pub conversation_history: Vec<Message>,
    
    /// Number of turns in this session
    pub turn_count: usize,
    
    /// Maximum turns allowed
    pub max_turns: usize,
    
    /// Session creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    
    /// Whether the session is still active
    pub is_active: bool,
    
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

impl ContainerSession {
    /// Create a new container session
    pub fn new(session_id: &str, container_id: &str, model_config: ModelConfig) -> Self {
        let now = Utc::now();
        ContainerSession {
            session_id: session_id.to_string(),
            container_id: container_id.to_string(),
            model_config,
            conversation_history: Vec::new(),
            turn_count: 0,
            max_turns: 100,
            created_at: now,
            last_activity: now,
            is_active: true,
            metadata: HashMap::new(),
        }
    }
    
    /// Add a message to conversation history
    pub fn add_message(&mut self, role: &str, content: &str) {
        self.conversation_history.push(Message {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        });
        self.turn_count += 1;
        self.last_activity = Utc::now();
    }
    
    /// Check if session has reached max turns
    pub fn is_at_max_turns(&self) -> bool {
        self.turn_count >= self.max_turns
    }
    
    /// Get session duration in seconds
    pub fn duration_secs(&self) -> i64 {
        (Utc::now() - self.created_at).num_seconds()
    }
    
    /// Check if session is idle (no activity for given seconds)
    pub fn is_idle(&self, idle_threshold_secs: u64) -> bool {
        let idle_duration = (Utc::now() - self.last_activity).num_seconds();
        idle_duration > idle_threshold_secs as i64
    }
}

/// A message in conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role (user, assistant, system)
    pub role: String,
    
    /// Message content
    pub content: String,
    
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
}

// =================================================================
// MCP PROTOCOL TYPES
// =================================================================

/// MCP JSON-RPC request
#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: McpParams,
    pub id: u64,
}

impl McpRequest {
    /// Create a new MCP tool call request
    pub fn tool_call(tool_name: &str, arguments: serde_json::Value, id: u64) -> Self {
        McpRequest {
            jsonrpc: "2.0".to_string(),
            method: "tools/call".to_string(),
            params: McpParams {
                name: tool_name.to_string(),
                arguments,
            },
            id,
        }
    }
    
    /// Create a tool list request
    pub fn list_tools(id: u64) -> Self {
        McpRequest {
            jsonrpc: "2.0".to_string(),
            method: "tools/list".to_string(),
            params: McpParams {
                name: String::new(),
                arguments: serde_json::Value::Null,
            },
            id,
        }
    }
}

/// MCP request parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct McpParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub arguments: serde_json::Value,
}

/// MCP JSON-RPC response
#[derive(Debug, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
    pub id: u64,
}

/// MCP error response
#[derive(Debug, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Result of an MCP tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Tool name that was called
    pub tool_name: String,
    
    /// Result content
    pub result: String,
    
    /// Whether the call was successful
    pub success: bool,
    
    /// Error message if call failed
    pub error: Option<String>,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// =================================================================
// AGENTDOCK API RESPONSE TYPES
// =================================================================

/// Response from container creation
#[derive(Debug, Deserialize)]
pub struct CreateContainerResponse {
    pub container_id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Container details response
#[derive(Debug, Deserialize)]
pub struct ContainerDetails {
    pub container_id: String,
    pub image: String,
    pub status: String,
    pub created_at: Option<String>,
    pub ports: Option<HashMap<String, String>>,
}

/// Health check response
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: Option<String>,
}

// =================================================================
// AGENTDOCK BRIDGE
// =================================================================

/// Main bridge for AgentDock MCP integration
pub struct AgentDockBridge {
    /// HTTP client for API calls
    client: reqwest::Client,
    
    /// AgentDock configuration
    config: AgentDockConfig,
    
    /// Registered model configurations
    models: HashMap<String, ModelConfig>,
    
    /// Active container sessions
    sessions: HashMap<String, ContainerSession>,
    
    /// Request ID counter
    request_id: u64,
    
    /// Whether AgentDock is available
    is_available: bool,
    
    /// Last health check result
    last_health_check: Option<DateTime<Utc>>,
    
    /// Statistics
    stats: BridgeStatistics,
}

/// Bridge statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BridgeStatistics {
    /// Total containers created
    pub containers_created: usize,
    
    /// Total containers released
    pub containers_released: usize,
    
    /// Total MCP calls made
    pub mcp_calls: usize,
    
    /// Failed MCP calls
    pub mcp_failures: usize,
    
    /// Total observations collected
    pub observations_collected: usize,
    
    /// Average response time in ms
    pub avg_response_time_ms: f64,
}

impl AgentDockBridge {
    /// Create a new AgentDock bridge
    pub fn new(config: AgentDockConfig) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(config.request_timeout_ms))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        
        Ok(AgentDockBridge {
            client,
            config,
            models: HashMap::new(),
            sessions: HashMap::new(),
            request_id: 1,
            is_available: false,
            last_health_check: None,
            stats: BridgeStatistics::default(),
        })
    }
    
    /// Create with default configuration
    pub fn with_defaults() -> Result<Self, String> {
        Self::new(AgentDockConfig::default())
    }
    
    /// Get the manager URL
    pub fn manager_url(&self) -> &str {
        &self.config.manager_url
    }
    
    /// Register a model configuration
    pub fn register_model(&mut self, model_id: &str, config: ModelConfig) {
        self.models.insert(model_id.to_string(), config);
    }
    
    /// Get a registered model configuration
    pub fn get_model(&self, model_id: &str) -> Option<&ModelConfig> {
        self.models.get(model_id)
    }
    
    /// List all registered model IDs
    pub fn list_models(&self) -> Vec<String> {
        self.models.keys().cloned().collect()
    }
    
    /// List available models (sorted by priority)
    pub fn list_available_models(&self) -> Vec<&ModelConfig> {
        let mut models: Vec<_> = self.models.values()
            .filter(|m| m.is_healthy)
            .collect();
        models.sort_by(|a, b| b.priority.cmp(&a.priority));
        models
    }
    
    /// Get next request ID
    fn next_request_id(&mut self) -> u64 {
        let id = self.request_id;
        self.request_id += 1;
        id
    }
    
    // -----------------------------------------------------------------
    // HEALTH CHECK
    // -----------------------------------------------------------------
    
    /// Check if AgentDock is available
    pub async fn health_check(&mut self) -> Result<bool, String> {
        let url = format!("{}/alive", self.config.manager_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Health check failed: {}", e))?;
        
        let is_healthy = response.status().is_success();
        self.is_available = is_healthy;
        self.last_health_check = Some(Utc::now());
        
        Ok(is_healthy)
    }
    
    /// Check if AgentDock is currently available
    pub fn is_available(&self) -> bool {
        self.is_available
    }
    
    // -----------------------------------------------------------------
    // CONTAINER LIFECYCLE
    // -----------------------------------------------------------------
    
    /// Create a new container session
    pub async fn create_session(
        &mut self,
        model_id: &str,
    ) -> Result<ContainerSession, String> {
        // Get model config
        let model_config = self.models.get(model_id)
            .cloned()
            .ok_or_else(|| format!("Model '{}' not registered", model_id))?;
        
        // Create container via AgentDock API
        let container_id = self.create_container(&self.config.default_image.clone()).await?;
        
        // Generate session ID
        let session_id = format!("session_{}_{}", model_id, chrono::Utc::now().timestamp_millis());
        
        // Create session
        let session = ContainerSession::new(&session_id, &container_id, model_config);
        
        // Store session
        self.sessions.insert(session_id.clone(), session.clone());
        self.stats.containers_created += 1;
        
        Ok(session)
    }
    
    /// Create a container via AgentDock API
    async fn create_container(&self, image_name: &str) -> Result<String, String> {
        let url = format!("{}/node/create?image_name={}", self.config.manager_url, image_name);
        
        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| format!("Container creation failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Container creation failed ({}): {}", status, body));
        }
        
        let result: CreateContainerResponse = response.json()
            .await
            .map_err(|e| format!("Failed to parse container response: {}", e))?;
        
        Ok(result.container_id)
    }
    
    /// Get session by ID
    pub fn get_session(&self, session_id: &str) -> Option<&ContainerSession> {
        self.sessions.get(session_id)
    }
    
    /// Get mutable session by ID
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut ContainerSession> {
        self.sessions.get_mut(session_id)
    }
    
    /// Release a session (close and remove container)
    pub async fn release_session(&mut self, session_id: &str) -> Result<(), String> {
        let session = self.sessions.remove(session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;
        
        // Release container via AgentDock API
        self.release_container(&session.container_id).await?;
        
        self.stats.containers_released += 1;
        
        Ok(())
    }
    
    /// Release a container via AgentDock API
    async fn release_container(&self, container_id: &str) -> Result<(), String> {
        let url = format!("{}/node/release", self.config.manager_url);
        
        let body = serde_json::json!({
            "container_id": container_id
        });
        
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Container release failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Container release failed ({}): {}", status, body));
        }
        
        Ok(())
    }
    
    /// Close a container (stop but don't remove)
    pub async fn close_session(&mut self, session_id: &str) -> Result<(), String> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;
        
        // Close container via AgentDock API
        let url = format!("{}/node/close", self.config.manager_url);
        
        let body = serde_json::json!({
            "container_id": session.container_id
        });
        
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Container close failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Container close failed ({}): {}", status, body));
        }
        
        session.is_active = false;
        
        Ok(())
    }
    
    /// Clean up idle sessions
    pub async fn cleanup_idle_sessions(&mut self) -> Result<usize, String> {
        let idle_threshold = self.config.container_idle_timeout_secs;
        
        let idle_sessions: Vec<String> = self.sessions.iter()
            .filter(|(_, s)| s.is_idle(idle_threshold))
            .map(|(id, _)| id.clone())
            .collect();
        
        let count = idle_sessions.len();
        
        for session_id in idle_sessions {
            if let Err(e) = self.release_session(&session_id).await {
                eprintln!("[AgentDock] Failed to release idle session {}: {}", session_id, e);
            }
        }
        
        Ok(count)
    }
    
    // -----------------------------------------------------------------
    // MCP TOOL CALLING
    // -----------------------------------------------------------------
    
    /// Call an MCP tool on a container
    pub async fn call_mcp_tool(
        &mut self,
        container_id: &str,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        
        let request_id = self.next_request_id();
        let mcp_request = McpRequest::tool_call(tool_name, arguments, request_id);
        
        let url = format!("{}/container/{}/mcp", self.config.manager_url, container_id);
        
        let response = self.client
            .post(&url)
            .json(&mcp_request)
            .send()
            .await
            .map_err(|e| {
                self.stats.mcp_failures += 1;
                format!("MCP call failed: {}", e)
            })?;
        
        let elapsed = start.elapsed().as_millis() as u64;
        self.stats.mcp_calls += 1;
        
        // Update average response time
        let total_calls = self.stats.mcp_calls as f64;
        self.stats.avg_response_time_ms = 
            (self.stats.avg_response_time_ms * (total_calls - 1.0) + elapsed as f64) / total_calls;
        
        if !response.status().is_success() {
            self.stats.mcp_failures += 1;
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Ok(ToolResult {
                tool_name: tool_name.to_string(),
                result: String::new(),
                success: false,
                error: Some(format!("HTTP error ({}): {}", status, body)),
                execution_time_ms: elapsed,
            });
        }
        
        let mcp_response: McpResponse = response.json()
            .await
            .map_err(|e| {
                self.stats.mcp_failures += 1;
                format!("Failed to parse MCP response: {}", e)
            })?;
        
        if let Some(error) = mcp_response.error {
            self.stats.mcp_failures += 1;
            return Ok(ToolResult {
                tool_name: tool_name.to_string(),
                result: String::new(),
                success: false,
                error: Some(format!("MCP error ({}): {}", error.code, error.message)),
                execution_time_ms: elapsed,
            });
        }
        
        let result_str = mcp_response.result
            .map(|v| {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    serde_json::to_string(&v).unwrap_or_default()
                }
            })
            .unwrap_or_default();
        
        Ok(ToolResult {
            tool_name: tool_name.to_string(),
            result: result_str,
            success: true,
            error: None,
            execution_time_ms: elapsed,
        })
    }
    
    /// List available MCP tools for a container
    pub async fn list_mcp_tools(&mut self, container_id: &str) -> Result<Vec<String>, String> {
        let request_id = self.next_request_id();
        let mcp_request = McpRequest::list_tools(request_id);
        
        let url = format!("{}/container/{}/mcp", self.config.manager_url, container_id);
        
        let response = self.client
            .post(&url)
            .json(&mcp_request)
            .send()
            .await
            .map_err(|e| format!("MCP list tools failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("MCP list tools failed ({}): {}", status, body));
        }
        
        let mcp_response: McpResponse = response.json()
            .await
            .map_err(|e| format!("Failed to parse MCP response: {}", e))?;
        
        if let Some(result) = mcp_response.result {
            if let Some(tools) = result.get("tools") {
                if let Some(arr) = tools.as_array() {
                    return Ok(arr.iter()
                        .filter_map(|t| t.get("name").and_then(|n| n.as_str()))
                        .map(|s| s.to_string())
                        .collect());
                }
            }
        }
        
        Ok(vec![])
    }
    
    // -----------------------------------------------------------------
    // MODEL OBSERVATION
    // -----------------------------------------------------------------
    
    /// Observe a model's behavior for a single query
    pub async fn observe_model(
        &mut self,
        session_id: &str,
        query: &str,
    ) -> Result<BehaviorObservation, String> {
        let start = std::time::Instant::now();
        
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;
        
        if !session.is_active {
            return Err("Session is not active".to_string());
        }
        
        if session.is_at_max_turns() {
            return Err(format!(
                "Session has reached max turns ({})", 
                session.max_turns
            ));
        }
        
        // Add user message to history
        session.add_message("user", query);
        
        // Call the model via MCP
        let tool_result = self.call_mcp_tool(
            &session.container_id.clone(),
            "chat",
            serde_json::json!({
                "messages": session.conversation_history.iter()
                    .map(|m| serde_json::json!({
                        "role": m.role,
                        "content": m.content
                    }))
                    .collect::<Vec<_>>(),
                "model": session.model_config.name,
                "max_tokens": session.model_config.max_tokens,
                "temperature": session.model_config.temperature,
            }),
        ).await?;
        
        if !tool_result.success {
            return Err(tool_result.error.unwrap_or("Unknown error".to_string()));
        }
        
        // Add assistant response to history
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.add_message("assistant", &tool_result.result);
        }
        
        let elapsed = start.elapsed().as_millis() as u64;
        self.stats.observations_collected += 1;
        
        // Extract behavior patterns from response
        let patterns = self.extract_patterns(&tool_result.result);
        
        Ok(BehaviorObservation {
            query: query.to_string(),
            response: tool_result.result,
            patterns,
            similarity_to_target: 0.0,  // To be computed later
            confidence: 0.8,  // Default confidence
        })
    }
    
    /// Observe multiple models with the same query
    pub async fn observe_multi_model(
        &mut self,
        query: &str,
        model_ids: &[&str],
    ) -> Result<HashMap<String, Vec<BehaviorObservation>>, String> {
        let mut results = HashMap::new();
        
        for model_id in model_ids {
            // Create a session for this model
            match self.create_session(model_id).await {
                Ok(session) => {
                    let session_id = session.session_id.clone();
                    
                    // Observe the model
                    match self.observe_model(&session_id, query).await {
                        Ok(observation) => {
                            results.insert(model_id.to_string(), vec![observation]);
                        }
                        Err(e) => {
                            eprintln!("[AgentDock] Observation failed for {}: {}", model_id, e);
                        }
                    }
                    
                    // Release the session
                    if let Err(e) = self.release_session(&session_id).await {
                        eprintln!("[AgentDock] Failed to release session: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("[AgentDock] Failed to create session for {}: {}", model_id, e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Extract behavior patterns from a response
    fn extract_patterns(&self, response: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Check for common patterns
        if response.contains("I think") || response.contains("In my opinion") {
            patterns.push("expresses_opinion".to_string());
        }
        if response.contains("However,") || response.contains("On the other hand") {
            patterns.push("considers_alternatives".to_string());
        }
        if response.contains("```") {
            patterns.push("uses_code_blocks".to_string());
        }
        if response.contains("1.") || response.contains("2.") {
            patterns.push("uses_numbered_lists".to_string());
        }
        if response.contains("*") || response.contains("-") {
            patterns.push("uses_bullet_points".to_string());
        }
        if response.contains("?") && !response.ends_with("?") {
            patterns.push("asks_clarifying_questions".to_string());
        }
        if response.len() > 500 {
            patterns.push("verbose_response".to_string());
        } else if response.len() < 100 {
            patterns.push("concise_response".to_string());
        }
        
        // Check formatting
        if response.starts_with("Certainly") || response.starts_with("Of course") {
            patterns.push("affirmative_opener".to_string());
        }
        if response.contains("I apologize") || response.contains("I'm sorry") {
            patterns.push("apologetic".to_string());
        }
        if response.contains("Let me") || response.contains("I'll") {
            patterns.push("action_oriented".to_string());
        }
        
        patterns
    }
    
    // -----------------------------------------------------------------
    // STATISTICS & STATUS
    // -----------------------------------------------------------------
    
    /// Get bridge statistics
    pub fn get_statistics(&self) -> &BridgeStatistics {
        &self.stats
    }
    
    /// Get count of active sessions
    pub fn active_session_count(&self) -> usize {
        self.sessions.values().filter(|s| s.is_active).count()
    }
    
    /// Get status summary
    pub fn status_summary(&self) -> String {
        let mut lines = vec!["=== AGENTDOCK BRIDGE STATUS ===".to_string()];
        
        lines.push(format!(
            "Manager URL: {}",
            self.config.manager_url
        ));
        lines.push(format!(
            "Available: {}",
            if self.is_available { "YES" } else { "NO" }
        ));
        lines.push(format!(
            "Last health check: {}",
            self.last_health_check
                .map(|t| t.to_rfc3339())
                .unwrap_or_else(|| "Never".to_string())
        ));
        
        lines.push(String::new());
        lines.push("Registered Models:".to_string());
        for (id, config) in &self.models {
            lines.push(format!(
                "  {}: {} (priority: {}, healthy: {})",
                id, config.name, config.priority, config.is_healthy
            ));
        }
        
        lines.push(String::new());
        lines.push(format!("Active sessions: {}", self.active_session_count()));
        lines.push(format!("Total sessions: {}", self.sessions.len()));
        
        lines.push(String::new());
        lines.push("Statistics:".to_string());
        lines.push(format!("  Containers created: {}", self.stats.containers_created));
        lines.push(format!("  Containers released: {}", self.stats.containers_released));
        lines.push(format!("  MCP calls: {}", self.stats.mcp_calls));
        lines.push(format!("  MCP failures: {}", self.stats.mcp_failures));
        lines.push(format!("  Observations: {}", self.stats.observations_collected));
        lines.push(format!("  Avg response time: {:.1}ms", self.stats.avg_response_time_ms));
        
        lines.join("\n")
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agentdock_config_default() {
        let config = AgentDockConfig::default();
        assert_eq!(config.manager_url, "http://localhost:8080");
        assert_eq!(config.max_pool_size, 10);
        assert!(config.enable_health_monitoring);
    }
    
    #[test]
    fn test_model_config_builder() {
        let config = ModelConfig::new("gpt-4", "https://api.openai.com/v1")
            .with_api_key("sk-test")
            .with_priority(10)
            .with_max_tokens(4096)
            .with_temperature(0.9)
            .with_capability("tools");
        
        assert_eq!(config.name, "gpt-4");
        assert_eq!(config.api_key, Some("sk-test".to_string()));
        assert_eq!(config.priority, 10);
        assert_eq!(config.max_tokens, 4096);
        assert_eq!(config.temperature, 0.9);
        assert!(config.capabilities.contains(&"tools".to_string()));
    }
    
    #[test]
    fn test_container_session_creation() {
        let model_config = ModelConfig::new("test-model", "http://localhost:8080");
        let session = ContainerSession::new("session-1", "container-abc", model_config);
        
        assert_eq!(session.session_id, "session-1");
        assert_eq!(session.container_id, "container-abc");
        assert_eq!(session.turn_count, 0);
        assert!(session.is_active);
        assert!(session.conversation_history.is_empty());
    }
    
    #[test]
    fn test_container_session_add_message() {
        let model_config = ModelConfig::new("test-model", "http://localhost:8080");
        let mut session = ContainerSession::new("session-1", "container-abc", model_config);
        
        session.add_message("user", "Hello!");
        session.add_message("assistant", "Hi there!");
        
        assert_eq!(session.turn_count, 2);
        assert_eq!(session.conversation_history.len(), 2);
        assert_eq!(session.conversation_history[0].role, "user");
        assert_eq!(session.conversation_history[1].role, "assistant");
    }
    
    #[test]
    fn test_session_max_turns() {
        let model_config = ModelConfig::new("test-model", "http://localhost:8080");
        let mut session = ContainerSession::new("session-1", "container-abc", model_config);
        session.max_turns = 5;
        
        for i in 0..5 {
            session.add_message("user", &format!("Message {}", i));
        }
        
        assert!(session.is_at_max_turns());
    }
    
    #[test]
    fn test_mcp_request_tool_call() {
        let request = McpRequest::tool_call(
            "search",
            serde_json::json!({"query": "test"}),
            42,
        );
        
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "tools/call");
        assert_eq!(request.id, 42);
        assert_eq!(request.params.name, "search");
    }
    
    #[test]
    fn test_mcp_request_list_tools() {
        let request = McpRequest::list_tools(1);
        
        assert_eq!(request.method, "tools/list");
        assert_eq!(request.id, 1);
    }
    
    #[test]
    fn test_tool_result() {
        let result = ToolResult {
            tool_name: "search".to_string(),
            result: "Found 5 results".to_string(),
            success: true,
            error: None,
            execution_time_ms: 150,
        };
        
        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.execution_time_ms, 150);
    }
    
    #[test]
    fn test_bridge_creation() {
        let bridge = AgentDockBridge::with_defaults();
        assert!(bridge.is_ok());
        
        let bridge = bridge.unwrap();
        assert!(!bridge.is_available());
        assert_eq!(bridge.active_session_count(), 0);
    }
    
    #[test]
    fn test_bridge_register_model() {
        let mut bridge = AgentDockBridge::with_defaults().unwrap();
        
        let config = ModelConfig::new("gpt-4", "https://api.openai.com/v1")
            .with_priority(10);
        bridge.register_model("gpt4", config);
        
        assert!(bridge.get_model("gpt4").is_some());
        assert!(bridge.get_model("unknown").is_none());
        
        let models = bridge.list_models();
        assert!(models.contains(&"gpt4".to_string()));
    }
    
    #[test]
    fn test_bridge_list_available_models() {
        let mut bridge = AgentDockBridge::with_defaults().unwrap();
        
        let mut config1 = ModelConfig::new("model-a", "http://localhost:8080")
            .with_priority(5);
        config1.is_healthy = true;
        
        let mut config2 = ModelConfig::new("model-b", "http://localhost:8081")
            .with_priority(10);
        config2.is_healthy = true;
        
        let mut config3 = ModelConfig::new("model-c", "http://localhost:8082")
            .with_priority(1);
        config3.is_healthy = false;  // Unhealthy
        
        bridge.register_model("a", config1);
        bridge.register_model("b", config2);
        bridge.register_model("c", config3);
        
        let available = bridge.list_available_models();
        assert_eq!(available.len(), 2);  // Only healthy models
        assert_eq!(available[0].name, "model-b");  // Highest priority first
        assert_eq!(available[1].name, "model-a");
    }
    
    #[test]
    fn test_bridge_extract_patterns() {
        let bridge = AgentDockBridge::with_defaults().unwrap();
        
        // Test verbose response
        let long_response = "a".repeat(600);
        let patterns = bridge.extract_patterns(&long_response);
        assert!(patterns.contains(&"verbose_response".to_string()));
        
        // Test concise response
        let short_response = "Yes.";
        let patterns = bridge.extract_patterns(short_response);
        assert!(patterns.contains(&"concise_response".to_string()));
        
        // Test code blocks
        let code_response = "Here's the code:\n```python\nprint('hello')\n```";
        let patterns = bridge.extract_patterns(code_response);
        assert!(patterns.contains(&"uses_code_blocks".to_string()));
        
        // Test affirmative opener
        let affirmative = "Certainly! I can help with that.";
        let patterns = bridge.extract_patterns(affirmative);
        assert!(patterns.contains(&"affirmative_opener".to_string()));
    }
    
    #[test]
    fn test_bridge_statistics() {
        let bridge = AgentDockBridge::with_defaults().unwrap();
        let stats = bridge.get_statistics();
        
        assert_eq!(stats.containers_created, 0);
        assert_eq!(stats.mcp_calls, 0);
        assert_eq!(stats.observations_collected, 0);
    }
    
    #[test]
    fn test_bridge_status_summary() {
        let mut bridge = AgentDockBridge::with_defaults().unwrap();
        bridge.register_model("test", ModelConfig::new("test", "http://localhost"));
        
        let summary = bridge.status_summary();
        assert!(summary.contains("AGENTDOCK BRIDGE STATUS"));
        assert!(summary.contains("Manager URL"));
        assert!(summary.contains("Registered Models"));
    }
    
    #[test]
    fn test_message_serialization() {
        let msg = Message {
            role: "user".to_string(),
            content: "Hello!".to_string(),
            timestamp: Utc::now(),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        let restored: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.role, "user");
        assert_eq!(restored.content, "Hello!");
    }
    
    #[test]
    fn test_container_session_serialization() {
        let model_config = ModelConfig::new("test", "http://localhost");
        let session = ContainerSession::new("sess-1", "cont-abc", model_config);
        
        let json = serde_json::to_string(&session).unwrap();
        let restored: ContainerSession = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.session_id, "sess-1");
        assert_eq!(restored.container_id, "cont-abc");
    }
}
