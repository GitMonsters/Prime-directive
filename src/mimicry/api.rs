// =================================================================
// API MODULE: HTTP Client for Real AI Model Observation
// =================================================================
// Feature-gated behind `api = ["reqwest"]`. This module provides
// HTTP client support for observing real AI models and feeding
// their responses into the RustyWorm mimicry pipeline.
//
// COMPOUND INTEGRATIONS:
//   - API responses → BehaviorAnalyzer::build_signature()
//   - Responses stored → EvolutionTracker::training_data.store()
//   - Profiles refined → analyzer.refine_profile()
//   - Templates updated → TemplateLibrary::apply_feedback()
//   - Cache compiled → SignatureCache::compile_from()
//
// SUPPORTED PROVIDERS:
//   - OpenAI (GPT-4o, o1) via chat completions API
//   - Anthropic (Claude) via messages API
//   - Google (Gemini) via generativelanguage API
//   - Ollama (local models) via generate API
//   - Custom (any OpenAI-compatible endpoint)
//
// All types have serde derives for persistence.
// Tests work without actual API keys (mock/stub patterns).
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

// =================================================================
// API PROVIDER ENUM
// =================================================================

/// Supported API providers for model observation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApiProvider {
    OpenAI,
    Anthropic,
    Google,
    Ollama,
    Custom(String), // custom label
}

impl ApiProvider {
    /// Default base URL for the provider
    pub fn default_base_url(&self) -> &str {
        match self {
            ApiProvider::OpenAI => "https://api.openai.com/v1",
            ApiProvider::Anthropic => "https://api.anthropic.com/v1",
            ApiProvider::Google => "https://generativelanguage.googleapis.com/v1beta",
            ApiProvider::Ollama => "http://localhost:11434",
            ApiProvider::Custom(_) => "",
        }
    }

    /// Default model name for the provider
    pub fn default_model(&self) -> &str {
        match self {
            ApiProvider::OpenAI => "gpt-4o",
            ApiProvider::Anthropic => "claude-sonnet-4-20250514",
            ApiProvider::Google => "gemini-1.5-pro",
            ApiProvider::Ollama => "llama3",
            ApiProvider::Custom(_) => "default",
        }
    }

    /// Environment variable name for the API key
    pub fn env_key_name(&self) -> &str {
        match self {
            ApiProvider::OpenAI => "OPENAI_API_KEY",
            ApiProvider::Anthropic => "ANTHROPIC_API_KEY",
            ApiProvider::Google => "GOOGLE_API_KEY",
            ApiProvider::Ollama => "", // no key needed
            ApiProvider::Custom(_) => "CUSTOM_API_KEY",
        }
    }

    /// Map to RustyWorm profile ID for compound integration
    pub fn profile_id(&self) -> &str {
        match self {
            ApiProvider::OpenAI => "gpt4o",
            ApiProvider::Anthropic => "claude",
            ApiProvider::Google => "gemini",
            ApiProvider::Ollama => "llama",
            ApiProvider::Custom(label) => {
                if label.is_empty() {
                    "custom"
                } else {
                    // Return label as-is; callers use to_string() anyway
                    label.as_str()
                }
            }
        }
    }

    /// Parse provider from a string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" | "gpt4o" | "gpt-4o" | "o1" => Some(ApiProvider::OpenAI),
            "anthropic" | "claude" => Some(ApiProvider::Anthropic),
            "google" | "gemini" => Some(ApiProvider::Google),
            "ollama" | "llama" | "local" => Some(ApiProvider::Ollama),
            _ => Some(ApiProvider::Custom(s.to_string())),
        }
    }
}

impl std::fmt::Display for ApiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiProvider::OpenAI => write!(f, "OpenAI"),
            ApiProvider::Anthropic => write!(f, "Anthropic"),
            ApiProvider::Google => write!(f, "Google"),
            ApiProvider::Ollama => write!(f, "Ollama"),
            ApiProvider::Custom(label) => write!(f, "Custom({})", label),
        }
    }
}

// =================================================================
// API CONFIG
// =================================================================

/// Configuration for an API provider connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub provider: ApiProvider,
    pub base_url: String,
    pub api_key: Option<String>,
    pub model: String,
    pub timeout_secs: u64,
    pub max_tokens: u32,
    pub temperature: f64,
}

impl ApiConfig {
    /// Create a new config for a provider, pulling API key from environment
    pub fn new(provider: ApiProvider) -> Self {
        let base_url = provider.default_base_url().to_string();
        let model = provider.default_model().to_string();
        let env_key = provider.env_key_name();
        let api_key = if env_key.is_empty() {
            None
        } else {
            std::env::var(env_key).ok()
        };

        ApiConfig {
            provider,
            base_url,
            api_key,
            model,
            timeout_secs: 60,
            max_tokens: 1024,
            temperature: 0.7,
        }
    }

    /// Create config with explicit API key
    pub fn with_key(provider: ApiProvider, key: &str) -> Self {
        let mut config = Self::new(provider);
        config.api_key = Some(key.to_string());
        config
    }

    /// Set the model
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    /// Set the base URL
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = tokens;
        self
    }

    /// Check if the config has a valid API key (or doesn't need one)
    pub fn has_credentials(&self) -> bool {
        match self.provider {
            ApiProvider::Ollama => true, // no key needed
            _ => self.api_key.is_some() && !self.api_key.as_ref().unwrap().is_empty(),
        }
    }
}

// =================================================================
// API REQUEST/RESPONSE TYPES
// =================================================================

/// A prompt sent to an API provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPrompt {
    pub system: Option<String>,
    pub user: String,
}

impl ApiPrompt {
    pub fn new(user: &str) -> Self {
        ApiPrompt {
            system: None,
            user: user.to_string(),
        }
    }

    pub fn with_system(mut self, system: &str) -> Self {
        self.system = Some(system.to_string());
        self
    }
}

/// A response from an API provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub provider: ApiProvider,
    pub model: String,
    pub content: String,
    pub prompt: String,
    pub tokens_used: Option<u64>,
    pub latency_ms: u64,
    pub raw_json: Option<String>,
}

/// Result of comparing multiple providers on the same prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub prompt: String,
    pub responses: Vec<ApiResponse>,
    pub similarity_matrix: Vec<Vec<f64>>,
}

// =================================================================
// API CLIENT
// =================================================================

/// HTTP client for making requests to AI model APIs.
/// Uses reqwest::blocking for synchronous operation consistent
/// with the rest of the engine.
pub struct ApiClient {
    client: Client,
    config: ApiConfig,
}

impl ApiClient {
    /// Create a new API client with the given config
    pub fn new(config: ApiConfig) -> Result<Self, String> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(ApiClient { client, config })
    }

    /// Get the current config
    pub fn config(&self) -> &ApiConfig {
        &self.config
    }

    /// Send a prompt to the configured API provider
    pub fn send(&self, prompt: &ApiPrompt) -> Result<ApiResponse, String> {
        if !self.config.has_credentials() {
            return Err(format!(
                "No API key configured for {}. Set {} environment variable or use /api-config.",
                self.config.provider,
                self.config.provider.env_key_name()
            ));
        }

        let start = std::time::Instant::now();

        let result = match self.config.provider {
            ApiProvider::OpenAI => self.send_openai(prompt),
            ApiProvider::Anthropic => self.send_anthropic(prompt),
            ApiProvider::Google => self.send_google(prompt),
            ApiProvider::Ollama => self.send_ollama(prompt),
            ApiProvider::Custom(_) => self.send_openai_compatible(prompt),
        };

        let latency = start.elapsed().as_millis() as u64;

        result.map(|(content, tokens, raw)| ApiResponse {
            provider: self.config.provider.clone(),
            model: self.config.model.clone(),
            content,
            prompt: prompt.user.clone(),
            tokens_used: tokens,
            latency_ms: latency,
            raw_json: raw,
        })
    }

    /// Send to OpenAI chat completions API
    fn send_openai(
        &self,
        prompt: &ApiPrompt,
    ) -> Result<(String, Option<u64>, Option<String>), String> {
        let url = format!("{}/chat/completions", self.config.base_url);

        let mut messages = Vec::new();
        if let Some(ref system) = prompt.system {
            messages.push(serde_json::json!({
                "role": "system",
                "content": system
            }));
        }
        messages.push(serde_json::json!({
            "role": "user",
            "content": &prompt.user
        }));

        let body = serde_json::json!({
            "model": &self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(ref key) = self.config.api_key {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", key))
                    .map_err(|e| format!("Invalid API key format: {}", e))?,
            );
        }

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .map_err(|e| format!("OpenAI request failed: {}", e))?;

        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read OpenAI response: {}", e))?;

        if !status.is_success() {
            return Err(format!("OpenAI API error ({}): {}", status, text));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tokens = json["usage"]["total_tokens"].as_u64();

        Ok((content, tokens, Some(text)))
    }

    /// Send to Anthropic messages API
    fn send_anthropic(
        &self,
        prompt: &ApiPrompt,
    ) -> Result<(String, Option<u64>, Option<String>), String> {
        let url = format!("{}/messages", self.config.base_url);

        let messages = vec![serde_json::json!({
            "role": "user",
            "content": &prompt.user
        })];

        let mut body = serde_json::json!({
            "model": &self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens
        });

        if let Some(ref system) = prompt.system {
            body["system"] = serde_json::json!(system);
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        if let Some(ref key) = self.config.api_key {
            headers.insert(
                "x-api-key",
                HeaderValue::from_str(key).map_err(|e| format!("Invalid API key format: {}", e))?,
            );
        }

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .map_err(|e| format!("Anthropic request failed: {}", e))?;

        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read Anthropic response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Anthropic API error ({}): {}", status, text));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

        let content = json["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let input_tokens = json["usage"]["input_tokens"].as_u64().unwrap_or(0);
        let output_tokens = json["usage"]["output_tokens"].as_u64().unwrap_or(0);
        let tokens = Some(input_tokens + output_tokens);

        Ok((content, tokens, Some(text)))
    }

    /// Send to Google Gemini generateContent API
    fn send_google(
        &self,
        prompt: &ApiPrompt,
    ) -> Result<(String, Option<u64>, Option<String>), String> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or("No Google API key configured")?;

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.config.base_url, self.config.model, api_key
        );

        let mut parts = Vec::new();
        if let Some(ref system) = prompt.system {
            parts.push(serde_json::json!({"text": system}));
        }
        parts.push(serde_json::json!({"text": &prompt.user}));

        let body = serde_json::json!({
            "contents": [{
                "parts": parts
            }],
            "generationConfig": {
                "maxOutputTokens": self.config.max_tokens,
                "temperature": self.config.temperature
            }
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .map_err(|e| format!("Google request failed: {}", e))?;

        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read Google response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Google API error ({}): {}", status, text));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse Google response: {}", e))?;

        let content = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tokens = json["usageMetadata"]["totalTokenCount"].as_u64();

        Ok((content, tokens, Some(text)))
    }

    /// Send to Ollama local API
    fn send_ollama(
        &self,
        prompt: &ApiPrompt,
    ) -> Result<(String, Option<u64>, Option<String>), String> {
        let url = format!("{}/api/generate", self.config.base_url);

        let mut full_prompt = String::new();
        if let Some(ref system) = prompt.system {
            full_prompt.push_str(system);
            full_prompt.push_str("\n\n");
        }
        full_prompt.push_str(&prompt.user);

        let body = serde_json::json!({
            "model": &self.config.model,
            "prompt": full_prompt,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "num_predict": self.config.max_tokens
            }
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .map_err(|e| format!("Ollama request failed: {}", e))?;

        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read Ollama response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Ollama API error ({}): {}", status, text));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

        let content = json["response"].as_str().unwrap_or("").to_string();
        let eval_count = json["eval_count"].as_u64();

        Ok((content, eval_count, Some(text)))
    }

    /// Send to any OpenAI-compatible endpoint (for Custom provider)
    fn send_openai_compatible(
        &self,
        prompt: &ApiPrompt,
    ) -> Result<(String, Option<u64>, Option<String>), String> {
        // Reuse OpenAI format since most custom endpoints are OpenAI-compatible
        self.send_openai(prompt)
    }
}

// =================================================================
// OBSERVATION SESSION
// =================================================================

/// Manages a multi-turn observation session with a specific provider.
/// Stores prompts and responses for systematic behavior analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSession {
    pub provider: ApiProvider,
    pub model: String,
    pub observations: Vec<ApiResponse>,
    pub prompts_sent: Vec<String>,
    pub total_tokens: u64,
    pub total_latency_ms: u64,
}

impl ObservationSession {
    pub fn new(provider: ApiProvider, model: &str) -> Self {
        ObservationSession {
            provider,
            model: model.to_string(),
            observations: Vec::new(),
            prompts_sent: Vec::new(),
            total_tokens: 0,
            total_latency_ms: 0,
        }
    }

    /// Record an observation
    pub fn record(&mut self, response: ApiResponse) {
        self.prompts_sent.push(response.prompt.clone());
        self.total_tokens += response.tokens_used.unwrap_or(0);
        self.total_latency_ms += response.latency_ms;
        self.observations.push(response);
    }

    /// Get all response texts for signature building
    pub fn response_texts(&self) -> Vec<String> {
        self.observations
            .iter()
            .map(|r| r.content.clone())
            .collect()
    }

    /// Summary of this observation session
    pub fn summary(&self) -> String {
        let avg_latency = if self.observations.is_empty() {
            0
        } else {
            self.total_latency_ms / self.observations.len() as u64
        };

        format!(
            "Observation Session: {} ({})\n\
             Observations: {}\n\
             Total tokens: {}\n\
             Avg latency: {}ms",
            self.provider,
            self.model,
            self.observations.len(),
            self.total_tokens,
            avg_latency
        )
    }
}

// =================================================================
// STUDY PROMPTS - Diverse prompts for comprehensive observation
// =================================================================

/// A curated set of diverse prompts designed to elicit characteristic
/// behavior from AI models. Covers different categories to build
/// comprehensive behavioral signatures.
pub fn study_prompts() -> Vec<ApiPrompt> {
    vec![
        // Factual knowledge
        ApiPrompt::new("Explain how photosynthesis works in simple terms."),
        // Reasoning/analysis
        ApiPrompt::new(
            "What are the trade-offs between microservices and monolithic architecture?",
        ),
        // Creative
        ApiPrompt::new("Write a short haiku about artificial intelligence."),
        // Code generation
        ApiPrompt::new("Write a Python function that checks if a string is a palindrome."),
        // Ethics/safety boundary
        ApiPrompt::new("What are the ethical considerations of facial recognition technology?"),
        // Uncertainty handling
        ApiPrompt::new("What will be the most important technology in 2050?"),
        // Multi-step reasoning
        ApiPrompt::new(
            "A farmer has 17 sheep. All but 9 run away. How many does the farmer have left?",
        ),
        // Meta/self-awareness
        ApiPrompt::new("How do you handle questions you're not sure about?"),
        // Long-form explanation
        ApiPrompt::new(
            "Explain the concept of recursion to someone who has never programmed before.",
        ),
        // Instruction following
        ApiPrompt::new("List exactly 5 benefits of regular exercise. Number each one."),
    ]
}

// =================================================================
// API OBSERVER - Compound integration point
// =================================================================

/// The compound integration hub for API observations.
/// Orchestrates: API call → parse → analyze → store → refine.
///
/// This is the high-level interface that the MimicryEngine uses
/// to interact with real AI models and feed observations into
/// the mimicry pipeline.
pub struct ApiObserver {
    configs: HashMap<String, ApiConfig>,
    sessions: HashMap<String, ObservationSession>,
}

impl ApiObserver {
    pub fn new() -> Self {
        ApiObserver {
            configs: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    /// Configure a provider. If no explicit key is given, tries env var.
    pub fn configure(&mut self, provider: ApiProvider, key: Option<&str>) {
        let config = if let Some(k) = key {
            ApiConfig::with_key(provider.clone(), k)
        } else {
            ApiConfig::new(provider.clone())
        };
        let provider_id = provider.profile_id().to_string();
        self.configs.insert(provider_id, config);
    }

    /// Configure with a full ApiConfig
    pub fn configure_with(&mut self, config: ApiConfig) {
        let provider_id = config.provider.profile_id().to_string();
        self.configs.insert(provider_id, config);
    }

    /// Get the config for a provider
    pub fn get_config(&self, provider_id: &str) -> Option<&ApiConfig> {
        self.configs.get(provider_id)
    }

    /// Check if a provider is configured and has credentials
    pub fn is_ready(&self, provider_id: &str) -> bool {
        self.configs
            .get(provider_id)
            .map(|c| c.has_credentials())
            .unwrap_or(false)
    }

    /// Send a prompt to a provider and return the response.
    /// Does NOT automatically integrate — call `observe_and_integrate()`
    /// on the engine for full compound pipeline.
    pub fn send(&mut self, provider_id: &str, prompt: &ApiPrompt) -> Result<ApiResponse, String> {
        let config = self
            .configs
            .get(provider_id)
            .ok_or_else(|| {
                format!(
                    "Provider '{}' not configured. Use /api-config to set up.",
                    provider_id
                )
            })?
            .clone();

        let client = ApiClient::new(config.clone())?;
        let response = client.send(prompt)?;

        // Record in session
        let session = self
            .sessions
            .entry(provider_id.to_string())
            .or_insert_with(|| ObservationSession::new(config.provider.clone(), &config.model));
        session.record(response.clone());

        Ok(response)
    }

    /// Send a prompt to all configured providers for comparison
    pub fn send_to_all(&mut self, prompt: &ApiPrompt) -> Vec<Result<ApiResponse, String>> {
        let provider_ids: Vec<String> = self
            .configs
            .keys()
            .filter(|id| {
                self.configs
                    .get(*id)
                    .map(|c| c.has_credentials())
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        let mut results = Vec::new();
        for id in provider_ids {
            results.push(self.send(&id, prompt));
        }
        results
    }

    /// Run a comprehensive study: send diverse prompts to a provider
    /// to build a thorough behavioral signature.
    /// Returns the responses and a summary.
    pub fn study(
        &mut self,
        provider_id: &str,
        count: usize,
    ) -> Result<(Vec<ApiResponse>, String), String> {
        let prompts = study_prompts();
        let n = count.min(prompts.len());
        let mut responses = Vec::new();

        for prompt in prompts.iter().take(n) {
            match self.send(provider_id, prompt) {
                Ok(resp) => responses.push(resp),
                Err(e) => {
                    // Continue on individual failures, report at end
                    responses.push(ApiResponse {
                        provider: self
                            .configs
                            .get(provider_id)
                            .map(|c| c.provider.clone())
                            .unwrap_or(ApiProvider::Custom("unknown".to_string())),
                        model: self
                            .configs
                            .get(provider_id)
                            .map(|c| c.model.clone())
                            .unwrap_or_default(),
                        content: format!("[ERROR: {}]", e),
                        prompt: prompt.user.clone(),
                        tokens_used: None,
                        latency_ms: 0,
                        raw_json: None,
                    });
                }
            }
        }

        let successful = responses
            .iter()
            .filter(|r| !r.content.starts_with("[ERROR"))
            .count();
        let summary = format!(
            "Study complete: {}/{} prompts successful for '{}'",
            successful, n, provider_id
        );

        Ok((responses, summary))
    }

    /// Get the observation session for a provider
    pub fn get_session(&self, provider_id: &str) -> Option<&ObservationSession> {
        self.sessions.get(provider_id)
    }

    /// Get all response texts from a provider's session (for signature building)
    pub fn collected_responses(&self, provider_id: &str) -> Vec<String> {
        self.sessions
            .get(provider_id)
            .map(|s| s.response_texts())
            .unwrap_or_default()
    }

    /// Summary of all configured providers and sessions
    pub fn summary(&self) -> String {
        let mut lines = vec!["=== API OBSERVER STATUS ===".to_string()];

        if self.configs.is_empty() {
            lines.push("No providers configured.".to_string());
            lines.push("Use /api-config <provider> [key] to set up a provider.".to_string());
        } else {
            lines.push("Configured providers:".to_string());
            for (id, config) in &self.configs {
                let status = if config.has_credentials() {
                    "ready"
                } else {
                    "no key"
                };
                let obs_count = self
                    .sessions
                    .get(id)
                    .map(|s| s.observations.len())
                    .unwrap_or(0);
                lines.push(format!(
                    "  {:<12} {} ({}) [{}] {} observations",
                    id, config.provider, config.model, status, obs_count
                ));
            }
        }

        lines.join("\n")
    }

    /// List configured provider IDs
    pub fn configured_providers(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }
}

impl Default for ApiObserver {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================
// COMPARISON UTILITIES
// =================================================================

/// Simple text similarity using Jaccard coefficient on word sets.
/// Used for comparing responses from different providers.
pub fn text_similarity(a: &str, b: &str) -> f64 {
    let words_a: std::collections::HashSet<&str> = a
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .collect();
    let words_b: std::collections::HashSet<&str> = b
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .collect();

    if words_a.is_empty() && words_b.is_empty() {
        return 1.0;
    }
    if words_a.is_empty() || words_b.is_empty() {
        return 0.0;
    }

    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Build a similarity matrix for a set of responses
pub fn build_similarity_matrix(responses: &[&str]) -> Vec<Vec<f64>> {
    let n = responses.len();
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = if i == j {
                1.0
            } else {
                text_similarity(responses[i], responses[j])
            };
        }
    }
    matrix
}

/// Format a comparison result into a readable string
pub fn format_comparison(result: &ComparisonResult) -> String {
    let mut lines = vec![format!("=== COMPARISON: \"{}\" ===", result.prompt)];

    for (_i, resp) in result.responses.iter().enumerate() {
        lines.push(format!(
            "\n--- {} ({}) [{}ms, {} tokens] ---",
            resp.provider,
            resp.model,
            resp.latency_ms,
            resp.tokens_used
                .map(|t| t.to_string())
                .unwrap_or_else(|| "?".to_string())
        ));
        // Truncate long responses for display
        let preview = if resp.content.len() > 200 {
            format!("{}...", &resp.content[..200])
        } else {
            resp.content.clone()
        };
        lines.push(preview);
    }

    // Similarity matrix
    if result.responses.len() > 1 {
        lines.push("\nSimilarity Matrix:".to_string());
        let labels: Vec<String> = result
            .responses
            .iter()
            .map(|r| format!("{}", r.provider))
            .collect();

        // Header
        let header = format!(
            "  {:<12} {}",
            "",
            labels
                .iter()
                .map(|l| format!("{:<12}", l))
                .collect::<String>()
        );
        lines.push(header);

        for (i, row) in result.similarity_matrix.iter().enumerate() {
            let cells: String = row.iter().map(|v| format!("{:<12.2}", v)).collect();
            lines.push(format!("  {:<12} {}", labels[i], cells));
        }
    }

    lines.join("\n")
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_provider_defaults() {
        let openai = ApiProvider::OpenAI;
        assert!(openai.default_base_url().contains("openai.com"));
        assert_eq!(openai.default_model(), "gpt-4o");
        assert_eq!(openai.env_key_name(), "OPENAI_API_KEY");
        assert_eq!(openai.profile_id(), "gpt4o");

        let anthropic = ApiProvider::Anthropic;
        assert!(anthropic.default_base_url().contains("anthropic.com"));
        assert_eq!(anthropic.profile_id(), "claude");

        let ollama = ApiProvider::Ollama;
        assert!(ollama.default_base_url().contains("localhost"));
        assert_eq!(ollama.env_key_name(), "");
    }

    #[test]
    fn test_api_provider_from_str() {
        assert_eq!(ApiProvider::from_str("openai"), Some(ApiProvider::OpenAI));
        assert_eq!(ApiProvider::from_str("gpt4o"), Some(ApiProvider::OpenAI));
        assert_eq!(
            ApiProvider::from_str("claude"),
            Some(ApiProvider::Anthropic)
        );
        assert_eq!(ApiProvider::from_str("gemini"), Some(ApiProvider::Google));
        assert_eq!(ApiProvider::from_str("ollama"), Some(ApiProvider::Ollama));
        assert_eq!(ApiProvider::from_str("local"), Some(ApiProvider::Ollama));

        match ApiProvider::from_str("custom-thing") {
            Some(ApiProvider::Custom(label)) => assert_eq!(label, "custom-thing"),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn test_api_provider_display() {
        assert_eq!(format!("{}", ApiProvider::OpenAI), "OpenAI");
        assert_eq!(format!("{}", ApiProvider::Anthropic), "Anthropic");
        assert_eq!(format!("{}", ApiProvider::Google), "Google");
        assert_eq!(format!("{}", ApiProvider::Ollama), "Ollama");
        assert_eq!(
            format!("{}", ApiProvider::Custom("myapi".to_string())),
            "Custom(myapi)"
        );
    }

    #[test]
    fn test_api_config_new() {
        let config = ApiConfig::new(ApiProvider::Ollama);
        assert!(config.has_credentials()); // ollama needs no key
        assert_eq!(config.model, "llama3");
        assert_eq!(config.timeout_secs, 60);
        assert_eq!(config.max_tokens, 1024);
    }

    #[test]
    fn test_api_config_with_key() {
        let config = ApiConfig::with_key(ApiProvider::OpenAI, "sk-test-123");
        assert!(config.has_credentials());
        assert_eq!(config.api_key, Some("sk-test-123".to_string()));
    }

    #[test]
    fn test_api_config_builder_pattern() {
        let config = ApiConfig::new(ApiProvider::OpenAI)
            .with_model("gpt-4-turbo")
            .with_temperature(0.5)
            .with_max_tokens(2048)
            .with_base_url("https://custom.endpoint.com/v1");

        assert_eq!(config.model, "gpt-4-turbo");
        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.max_tokens, 2048);
        assert_eq!(config.base_url, "https://custom.endpoint.com/v1");
    }

    #[test]
    fn test_api_config_no_key() {
        // Without setting env var, OpenAI config should not have credentials
        // (unless the user actually has OPENAI_API_KEY set in their env)
        let config = ApiConfig::new(ApiProvider::OpenAI);
        // We can't assert has_credentials() == false because the env var
        // might actually be set. Just verify the config is valid.
        assert_eq!(config.provider, ApiProvider::OpenAI);
    }

    #[test]
    fn test_api_prompt() {
        let prompt = ApiPrompt::new("Hello, world!");
        assert_eq!(prompt.user, "Hello, world!");
        assert!(prompt.system.is_none());

        let prompt_with_system = prompt.with_system("You are a helpful assistant.");
        assert_eq!(
            prompt_with_system.system,
            Some("You are a helpful assistant.".to_string())
        );
        assert_eq!(prompt_with_system.user, "Hello, world!");
    }

    #[test]
    fn test_api_config_serialization() {
        let config = ApiConfig::with_key(ApiProvider::OpenAI, "sk-test");
        let json = serde_json::to_string(&config).unwrap();
        let restored: ApiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.provider, ApiProvider::OpenAI);
        assert_eq!(restored.api_key, Some("sk-test".to_string()));
        assert_eq!(restored.model, "gpt-4o");
    }

    #[test]
    fn test_api_response_serialization() {
        let response = ApiResponse {
            provider: ApiProvider::OpenAI,
            model: "gpt-4o".to_string(),
            content: "Test response".to_string(),
            prompt: "Test prompt".to_string(),
            tokens_used: Some(42),
            latency_ms: 150,
            raw_json: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        let restored: ApiResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.content, "Test response");
        assert_eq!(restored.tokens_used, Some(42));
        assert_eq!(restored.latency_ms, 150);
    }

    #[test]
    fn test_observation_session() {
        let mut session = ObservationSession::new(ApiProvider::OpenAI, "gpt-4o");

        session.record(ApiResponse {
            provider: ApiProvider::OpenAI,
            model: "gpt-4o".to_string(),
            content: "Response one".to_string(),
            prompt: "Prompt one".to_string(),
            tokens_used: Some(50),
            latency_ms: 200,
            raw_json: None,
        });

        session.record(ApiResponse {
            provider: ApiProvider::OpenAI,
            model: "gpt-4o".to_string(),
            content: "Response two".to_string(),
            prompt: "Prompt two".to_string(),
            tokens_used: Some(30),
            latency_ms: 150,
            raw_json: None,
        });

        assert_eq!(session.observations.len(), 2);
        assert_eq!(session.total_tokens, 80);
        assert_eq!(session.total_latency_ms, 350);

        let texts = session.response_texts();
        assert_eq!(texts, vec!["Response one", "Response two"]);

        let summary = session.summary();
        assert!(summary.contains("OpenAI"));
        assert!(summary.contains("gpt-4o"));
        assert!(summary.contains("2")); // observation count
    }

    #[test]
    fn test_observation_session_serialization() {
        let mut session =
            ObservationSession::new(ApiProvider::Anthropic, "claude-sonnet-4-20250514");
        session.record(ApiResponse {
            provider: ApiProvider::Anthropic,
            model: "claude-sonnet-4-20250514".to_string(),
            content: "Test".to_string(),
            prompt: "Hello".to_string(),
            tokens_used: Some(10),
            latency_ms: 100,
            raw_json: None,
        });

        let json = serde_json::to_string(&session).unwrap();
        let restored: ObservationSession = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.observations.len(), 1);
        assert_eq!(restored.provider, ApiProvider::Anthropic);
    }

    #[test]
    fn test_study_prompts() {
        let prompts = study_prompts();
        assert_eq!(prompts.len(), 10);

        // Check diversity - no two prompts should be the same
        for i in 0..prompts.len() {
            for j in (i + 1)..prompts.len() {
                assert_ne!(prompts[i].user, prompts[j].user);
            }
        }
    }

    #[test]
    fn test_api_observer_configure() {
        let mut observer = ApiObserver::new();

        observer.configure(ApiProvider::Ollama, None);
        assert!(observer.is_ready("llama"));
        assert!(observer.get_config("llama").is_some());

        observer.configure(ApiProvider::OpenAI, Some("sk-test-key"));
        assert!(observer.is_ready("gpt4o"));

        assert!(!observer.is_ready("nonexistent"));
    }

    #[test]
    fn test_api_observer_configure_with() {
        let mut observer = ApiObserver::new();

        let config = ApiConfig::with_key(ApiProvider::Anthropic, "test-key")
            .with_model("claude-haiku-4-20250514");
        observer.configure_with(config);

        let stored = observer.get_config("claude").unwrap();
        assert_eq!(stored.model, "claude-haiku-4-20250514");
        assert!(observer.is_ready("claude"));
    }

    #[test]
    fn test_api_observer_summary_empty() {
        let observer = ApiObserver::new();
        let summary = observer.summary();
        assert!(summary.contains("No providers configured"));
    }

    #[test]
    fn test_api_observer_summary_configured() {
        let mut observer = ApiObserver::new();
        observer.configure(ApiProvider::Ollama, None);
        observer.configure(ApiProvider::OpenAI, Some("sk-test"));

        let summary = observer.summary();
        assert!(summary.contains("API OBSERVER STATUS"));
        assert!(summary.contains("ready"));
        assert!(summary.contains("Ollama") || summary.contains("llama"));
    }

    #[test]
    fn test_api_observer_configured_providers() {
        let mut observer = ApiObserver::new();
        observer.configure(ApiProvider::Ollama, None);
        observer.configure(ApiProvider::OpenAI, Some("sk-test"));

        let providers = observer.configured_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"llama".to_string()));
        assert!(providers.contains(&"gpt4o".to_string()));
    }

    #[test]
    fn test_api_observer_send_no_config() {
        let mut observer = ApiObserver::new();
        let result = observer.send("gpt4o", &ApiPrompt::new("test"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not configured"));
    }

    #[test]
    fn test_text_similarity_identical() {
        assert_eq!(text_similarity("hello world", "hello world"), 1.0);
    }

    #[test]
    fn test_text_similarity_empty() {
        assert_eq!(text_similarity("", ""), 1.0);
        assert_eq!(text_similarity("hello", ""), 0.0);
        assert_eq!(text_similarity("", "hello"), 0.0);
    }

    #[test]
    fn test_text_similarity_partial() {
        let sim = text_similarity("the quick brown fox", "the slow brown dog");
        assert!(sim > 0.0);
        assert!(sim < 1.0);
        // "the" and "brown" overlap = 2, union = 6
        // sim = 2/6 ≈ 0.33
        assert!(sim > 0.2 && sim < 0.5);
    }

    #[test]
    fn test_text_similarity_disjoint() {
        let sim = text_similarity("hello world", "foo bar");
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_build_similarity_matrix() {
        let responses = vec!["hello world", "hello there", "goodbye world"];
        let matrix = build_similarity_matrix(&responses);

        assert_eq!(matrix.len(), 3);
        // Diagonal should be 1.0
        for i in 0..3 {
            assert_eq!(matrix[i][i], 1.0);
        }
        // Should be symmetric
        for i in 0..3 {
            for j in 0..3 {
                assert!((matrix[i][j] - matrix[j][i]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_comparison_result_serialization() {
        let result = ComparisonResult {
            prompt: "Test prompt".to_string(),
            responses: vec![ApiResponse {
                provider: ApiProvider::OpenAI,
                model: "gpt-4o".to_string(),
                content: "A response".to_string(),
                prompt: "Test prompt".to_string(),
                tokens_used: Some(20),
                latency_ms: 100,
                raw_json: None,
            }],
            similarity_matrix: vec![vec![1.0]],
        };

        let json = serde_json::to_string(&result).unwrap();
        let restored: ComparisonResult = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.prompt, "Test prompt");
        assert_eq!(restored.responses.len(), 1);
    }

    #[test]
    fn test_format_comparison() {
        let result = ComparisonResult {
            prompt: "What is Rust?".to_string(),
            responses: vec![
                ApiResponse {
                    provider: ApiProvider::OpenAI,
                    model: "gpt-4o".to_string(),
                    content: "Rust is a systems programming language.".to_string(),
                    prompt: "What is Rust?".to_string(),
                    tokens_used: Some(20),
                    latency_ms: 150,
                    raw_json: None,
                },
                ApiResponse {
                    provider: ApiProvider::Anthropic,
                    model: "claude-sonnet-4-20250514".to_string(),
                    content: "Rust is a language focused on safety and performance.".to_string(),
                    prompt: "What is Rust?".to_string(),
                    tokens_used: Some(25),
                    latency_ms: 200,
                    raw_json: None,
                },
            ],
            similarity_matrix: vec![vec![1.0, 0.3], vec![0.3, 1.0]],
        };

        let formatted = format_comparison(&result);
        assert!(formatted.contains("COMPARISON"));
        assert!(formatted.contains("What is Rust?"));
        assert!(formatted.contains("OpenAI"));
        assert!(formatted.contains("Anthropic"));
        assert!(formatted.contains("Similarity Matrix"));
    }

    #[test]
    fn test_api_client_no_credentials() {
        // Create a client with no API key for a provider that needs one
        let config = ApiConfig::new(ApiProvider::OpenAI);
        // If OPENAI_API_KEY is not set, this should fail on send
        if !config.has_credentials() {
            let client = ApiClient::new(config).unwrap();
            let result = client.send(&ApiPrompt::new("test"));
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("No API key"));
        }
    }

    #[test]
    fn test_api_client_creation() {
        let config = ApiConfig::with_key(ApiProvider::OpenAI, "sk-test");
        let client = ApiClient::new(config);
        assert!(client.is_ok());
        assert_eq!(client.unwrap().config().model, "gpt-4o");
    }
}
