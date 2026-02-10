//! External API Provider trait and common types.
//!
//! Defines the interface that all external API providers must implement.

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::mimicry::layers::layer::Domain;

/// Error types for API operations.
#[derive(Debug, Clone)]
pub enum ApiError {
    /// Network or connection error.
    NetworkError(String),
    /// Authentication failed.
    AuthenticationError(String),
    /// Rate limit exceeded.
    RateLimitExceeded { retry_after: Option<Duration> },
    /// Invalid request.
    InvalidRequest(String),
    /// Provider not available.
    ProviderUnavailable(String),
    /// Timeout.
    Timeout(Duration),
    /// Unknown error.
    Unknown(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            ApiError::RateLimitExceeded { retry_after } => {
                if let Some(d) = retry_after {
                    write!(f, "Rate limit exceeded, retry after {:?}", d)
                } else {
                    write!(f, "Rate limit exceeded")
                }
            }
            ApiError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiError::ProviderUnavailable(msg) => write!(f, "Provider unavailable: {}", msg),
            ApiError::Timeout(d) => write!(f, "Timeout after {:?}", d),
            ApiError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

/// Result type for API operations.
pub type ApiResult<T> = Result<T, ApiError>;

/// Query to send to an external API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiQuery {
    /// The query text or payload.
    pub query: String,
    /// Domain hint for routing.
    pub domain: Option<Domain>,
    /// Additional parameters.
    pub params: HashMap<String, String>,
    /// Maximum response tokens/items.
    pub max_results: Option<usize>,
    /// Timeout for this query.
    pub timeout: Option<Duration>,
    /// Request priority (0-10, higher = more important).
    pub priority: u8,
}

impl ApiQuery {
    /// Create a new query.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            domain: None,
            params: HashMap::new(),
            max_results: None,
            timeout: None,
            priority: 5,
        }
    }

    /// Set the domain hint.
    pub fn with_domain(mut self, domain: Domain) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Add a parameter.
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Set max results.
    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = Some(max);
        self
    }

    /// Set timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set priority.
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }
}

/// Response from an external API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    /// Response content.
    pub content: String,
    /// Confidence score (0.0-1.0).
    pub confidence: f32,
    /// Source attribution.
    pub source: String,
    /// Response metadata.
    pub metadata: HashMap<String, String>,
    /// Processing time.
    pub processing_time_ms: u64,
    /// Whether this is a cached response.
    pub cached: bool,
    /// Domain of the response.
    pub domain: Domain,
}

impl ApiResponse {
    /// Create a new response.
    pub fn new(content: impl Into<String>, confidence: f32, source: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            confidence,
            source: source.into(),
            metadata: HashMap::new(),
            processing_time_ms: 0,
            cached: false,
            domain: Domain::External,
        }
    }

    /// Create a stub response.
    pub fn stub(content: impl Into<String>) -> Self {
        Self::new(content, 0.5, "stub").with_metadata("is_stub", "true")
    }

    /// Set metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set processing time.
    pub fn with_processing_time(mut self, ms: u64) -> Self {
        self.processing_time_ms = ms;
        self
    }

    /// Mark as cached.
    pub fn with_cached(mut self, cached: bool) -> Self {
        self.cached = cached;
        self
    }

    /// Set domain.
    pub fn with_domain(mut self, domain: Domain) -> Self {
        self.domain = domain;
        self
    }
}

/// Status of an external provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderStatus {
    /// Provider is operational.
    Healthy,
    /// Provider is degraded (slow or partial failures).
    Degraded,
    /// Provider is unavailable.
    Unavailable,
    /// Provider is a stub (not real).
    Stub,
    /// Status unknown.
    Unknown,
}

/// Information about a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider name.
    pub name: String,
    /// Provider version.
    pub version: String,
    /// Supported domains.
    pub domains: Vec<Domain>,
    /// Current status.
    pub status: ProviderStatus,
    /// Whether this is a stub.
    pub is_stub: bool,
    /// Rate limit info.
    pub rate_limit: Option<RateLimitInfo>,
}

/// Rate limit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Maximum requests per period.
    pub max_requests: u32,
    /// Period duration.
    pub period: Duration,
    /// Remaining requests in current period.
    pub remaining: u32,
    /// Time until reset.
    pub reset_in: Duration,
}

/// Configuration for a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API endpoint URL.
    pub endpoint: Option<String>,
    /// API key or token.
    pub api_key: Option<String>,
    /// Request timeout.
    pub timeout: Duration,
    /// Maximum retries.
    pub max_retries: u32,
    /// Enable caching.
    pub enable_cache: bool,
    /// Cache TTL.
    pub cache_ttl: Duration,
    /// Custom headers.
    pub headers: HashMap<String, String>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            endpoint: None,
            api_key: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            enable_cache: true,
            cache_ttl: Duration::from_secs(3600),
            headers: HashMap::new(),
        }
    }
}

impl ProviderConfig {
    /// Create a stub configuration.
    pub fn stub() -> Self {
        Self {
            timeout: Duration::from_millis(100),
            max_retries: 0,
            enable_cache: false,
            ..Default::default()
        }
    }
}

/// Trait for external API providers.
///
/// All external providers must implement this trait to integrate
/// with the layer system.
pub trait ExternalApiProvider: Send + Sync {
    /// Get provider information.
    fn info(&self) -> ProviderInfo;

    /// Get current status.
    fn status(&self) -> ProviderStatus;

    /// Check if the provider is available.
    fn is_available(&self) -> bool {
        matches!(
            self.status(),
            ProviderStatus::Healthy | ProviderStatus::Degraded | ProviderStatus::Stub
        )
    }

    /// Query the provider (synchronous version).
    fn query_sync(&self, query: &ApiQuery) -> ApiResult<ApiResponse>;

    /// Get supported domains.
    fn supported_domains(&self) -> Vec<Domain> {
        self.info().domains
    }

    /// Check if a domain is supported.
    fn supports_domain(&self, domain: Domain) -> bool {
        self.supported_domains().contains(&domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_query_creation() {
        let query = ApiQuery::new("test query")
            .with_domain(Domain::Physics)
            .with_param("format", "json")
            .with_max_results(10);

        assert_eq!(query.query, "test query");
        assert_eq!(query.domain, Some(Domain::Physics));
        assert_eq!(query.params.get("format"), Some(&"json".to_string()));
        assert_eq!(query.max_results, Some(10));
    }

    #[test]
    fn test_api_response_creation() {
        let response = ApiResponse::new("result", 0.9, "test_source")
            .with_metadata("key", "value")
            .with_processing_time(100);

        assert_eq!(response.content, "result");
        assert_eq!(response.confidence, 0.9);
        assert_eq!(response.metadata.get("key"), Some(&"value".to_string()));
        assert_eq!(response.processing_time_ms, 100);
    }

    #[test]
    fn test_stub_response() {
        let response = ApiResponse::stub("stub content");
        assert_eq!(response.confidence, 0.5);
        assert_eq!(response.source, "stub");
    }

    #[test]
    fn test_provider_config_defaults() {
        let config = ProviderConfig::default();
        assert_eq!(config.max_retries, 3);
        assert!(config.enable_cache);
    }
}
