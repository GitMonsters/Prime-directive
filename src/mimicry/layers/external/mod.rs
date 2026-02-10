//! External API Providers for Layer 7.
//!
//! This module provides stubs and interfaces for external API integrations
//! that can enhance the layer system with real-time external validation
//! and specialized knowledge.
//!
//! # Available Providers
//!
//! - **Physics**: Computational physics (Wolfram Alpha, etc.)
//! - **LLM**: Large Language Model providers
//! - **Knowledge**: Knowledge graph queries (Wikidata, etc.)
//! - **CLARIN**: Natural language processing services
//! - **Collaborative**: Multi-agent coordination
//!
//! # Usage
//!
//! All providers implement the `ExternalApiProvider` trait, which provides
//! a unified interface for querying external services.
//!
//! ```rust,ignore
//! use rustyworm::mimicry::layers::external::{ExternalApiProvider, PhysicsProvider};
//!
//! let provider = PhysicsProvider::new_stub();
//! let query = ApiQuery::new("energy conservation").with_domain(Domain::Physics);
//! let response = provider.query(&query).await?;
//! ```
//!
//! # Stubs vs Real Implementations
//!
//! By default, all providers are stubs that return simulated responses.
//! Real implementations can be enabled through feature flags and
//! environment configuration.

pub mod provider;
pub mod physics;
pub mod llm;
pub mod knowledge;
pub mod clarin;
pub mod collaborative;

// Re-export primary types
pub use provider::{
    ApiError, ApiQuery, ApiResponse, ApiResult, ExternalApiProvider, ProviderConfig, ProviderInfo,
    ProviderStatus,
};
pub use physics::PhysicsProvider;
pub use llm::LlmProvider;
pub use knowledge::KnowledgeProvider;
pub use clarin::ClarinProvider;
pub use collaborative::CollaborativeProvider;

use std::collections::HashMap;
use std::sync::RwLock;

/// Registry of all available external providers.
pub struct ProviderRegistry {
    providers: RwLock<HashMap<String, Box<dyn ExternalApiProvider>>>,
}

impl ProviderRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
        }
    }

    /// Create a registry with default stub providers.
    pub fn with_stubs() -> Self {
        let registry = Self::new();

        // Register default stub providers
        registry.register("physics", Box::new(PhysicsProvider::new_stub()));
        registry.register("llm", Box::new(LlmProvider::new_stub()));
        registry.register("knowledge", Box::new(KnowledgeProvider::new_stub()));
        registry.register("clarin", Box::new(ClarinProvider::new_stub()));
        registry.register("collaborative", Box::new(CollaborativeProvider::new_stub()));

        registry
    }

    /// Register a provider.
    pub fn register(&self, name: &str, provider: Box<dyn ExternalApiProvider>) {
        self.providers
            .write()
            .unwrap()
            .insert(name.to_string(), provider);
    }

    /// Get a provider by name.
    pub fn get(&self, name: &str) -> Option<ProviderInfo> {
        self.providers
            .read()
            .unwrap()
            .get(name)
            .map(|p| p.info())
    }

    /// List all registered providers.
    pub fn list(&self) -> Vec<String> {
        self.providers.read().unwrap().keys().cloned().collect()
    }

    /// Get the status of all providers.
    pub fn status(&self) -> HashMap<String, ProviderStatus> {
        self.providers
            .read()
            .unwrap()
            .iter()
            .map(|(name, p)| (name.clone(), p.status()))
            .collect()
    }

    /// Check if a provider is available and healthy.
    pub fn is_healthy(&self, name: &str) -> bool {
        self.providers
            .read()
            .unwrap()
            .get(name)
            .map(|p| p.status() == ProviderStatus::Healthy)
            .unwrap_or(false)
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::with_stubs()
    }
}

/// Prelude for convenient imports.
pub mod prelude {
    pub use super::{
        ApiError, ApiQuery, ApiResponse, ApiResult, ClarinProvider, CollaborativeProvider,
        ExternalApiProvider, KnowledgeProvider, LlmProvider, PhysicsProvider, ProviderConfig,
        ProviderInfo, ProviderRegistry, ProviderStatus,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ProviderRegistry::new();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_registry_with_stubs() {
        let registry = ProviderRegistry::with_stubs();
        let providers = registry.list();

        assert!(providers.contains(&"physics".to_string()));
        assert!(providers.contains(&"llm".to_string()));
        assert!(providers.contains(&"knowledge".to_string()));
    }

    #[test]
    fn test_provider_registration() {
        let registry = ProviderRegistry::new();
        registry.register("test", Box::new(PhysicsProvider::new_stub()));

        assert!(registry.get("test").is_some());
        assert!(registry.get("nonexistent").is_none());
    }
}
