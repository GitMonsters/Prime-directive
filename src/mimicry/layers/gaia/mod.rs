//! GAIA Intuition Engine - Layer 4 Implementation.
//!
//! GAIA (General Analogical Intuition Architecture) provides pattern recognition
//! beyond explicit rules, enabling cross-domain analogical reasoning with
//! weighted pattern matching and reinforcement learning.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                  GAIA Intuition Engine                   │
//! ├─────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐ │
//! │  │   Pattern   │───▶│  Resonance  │───▶│  Analogical │ │
//! │  │   Memory    │    │    Field    │    │   Transfer  │ │
//! │  └─────────────┘    └─────────────┘    └─────────────┘ │
//! │         ▲                  │                  │        │
//! │         │                  ▼                  │        │
//! │         │           ┌─────────────┐          │        │
//! │         └───────────│ Reinforcement│◀─────────┘        │
//! │                     │   Learning  │                    │
//! │                     └─────────────┘                    │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Key Components
//!
//! - **Pattern Memory**: Stores learned patterns with weights and domain associations
//! - **Resonance Field**: Activates related patterns through spreading activation
//! - **Analogical Transfer**: Maps patterns across domains
//! - **Reinforcement**: Updates pattern weights based on outcome feedback
//!
//! # Example
//!
//! ```rust,ignore
//! use rustyworm::mimicry::layers::gaia::{GaiaIntuitionEngine, Pattern, GaiaConfig};
//!
//! let mut gaia = GaiaIntuitionEngine::new(GaiaConfig::default());
//!
//! // Register a pattern
//! let pattern = Pattern::new("conservation", Domain::Physics)
//!     .with_fingerprint(vec![0.8, 0.2, 0.5, 0.9])
//!     .with_weight(1.0);
//! gaia.register_pattern(pattern);
//!
//! // Find matching patterns
//! let query = vec![0.75, 0.25, 0.48, 0.85];
//! let matches = gaia.match_patterns(&query, 0.7);
//! ```

pub mod analogical;
pub mod intuition;
pub mod pattern;
pub mod persistence;
pub mod resonance;

// Re-export primary types
pub use analogical::{
    AnalogicalMapping, AnalogicalTransfer, DomainBridge, TransferResult, TransferStrength,
};
pub use intuition::{GaiaConfig, GaiaIntuitionEngine, IntuitionResult};
pub use pattern::{Pattern, PatternId, PatternMatch, PatternMemory, PatternStats};
pub use persistence::{GaiaPersistence, GaiaSnapshot, PatternData, SnapshotStats};
pub use resonance::{ActivationState, ResonanceConfig, ResonanceField, ResonanceResult};

/// Error types for GAIA operations.
#[derive(Debug, Clone)]
pub enum GaiaError {
    /// Pattern not found in memory.
    PatternNotFound(String),
    /// Invalid pattern fingerprint (wrong dimension, NaN, etc.).
    InvalidFingerprint(String),
    /// Resonance field saturation (too many active patterns).
    ResonanceSaturation,
    /// Analogical transfer failed.
    TransferFailed(String),
    /// Configuration error.
    ConfigError(String),
}

impl std::fmt::Display for GaiaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GaiaError::PatternNotFound(id) => write!(f, "Pattern not found: {}", id),
            GaiaError::InvalidFingerprint(msg) => write!(f, "Invalid fingerprint: {}", msg),
            GaiaError::ResonanceSaturation => write!(f, "Resonance field saturated"),
            GaiaError::TransferFailed(msg) => write!(f, "Analogical transfer failed: {}", msg),
            GaiaError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for GaiaError {}

/// Result type for GAIA operations.
pub type GaiaResult<T> = Result<T, GaiaError>;

/// Prelude for convenient imports.
pub mod prelude {
    pub use super::{
        ActivationState, AnalogicalMapping, AnalogicalTransfer, DomainBridge, GaiaConfig,
        GaiaError, GaiaIntuitionEngine, GaiaResult, IntuitionResult, Pattern, PatternId,
        PatternMatch, PatternMemory, PatternStats, ResonanceConfig, ResonanceField,
        ResonanceResult, TransferResult, TransferStrength,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::layers::layer::Domain;

    #[test]
    fn test_gaia_error_display() {
        let err = GaiaError::PatternNotFound("test_pattern".into());
        assert!(err.to_string().contains("test_pattern"));
    }

    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern::new("test", Domain::Physics);
        assert_eq!(pattern.id(), "test");
        assert_eq!(pattern.domain(), Domain::Physics);
    }

    #[test]
    fn test_resonance_field_creation() {
        let field = ResonanceField::new(ResonanceConfig::default());
        assert!(field.is_empty());
    }

    #[test]
    fn test_pattern_memory_creation() {
        let memory = PatternMemory::new();
        assert!(memory.is_empty());
    }
}
