//! Amplification Engine for multiplicative confidence boosting.
//!
//! This module implements the core mechanism for multiplicative confidence
//! amplification across layers. Unlike traditional additive systems where
//! confidence is bounded by the weakest input, this system allows confidence
//! to compound through resonance between layers.
//!
//! # Key Concepts
//!
//! ## Multiplicative Confidence
//!
//! ```text
//! Traditional: confidence = min(c1, c2, c3)  // Bounded by weakest
//! Multiplicative: confidence = c1 × c2 × c3 × amplification  // Can exceed 1.0
//! ```
//!
//! ## Convergence Detection
//!
//! The amplification process iterates until:
//! - Confidence stabilizes (delta below threshold)
//! - Maximum iterations reached
//! - Divergence detected (unbounded growth)
//!
//! ## Damping
//!
//! To prevent runaway amplification, damping factors are applied:
//! - Per-iteration damping
//! - Maximum confidence caps (configurable)
//! - Saturation detection
//!
//! # Example
//!
//! ```rust,ignore
//! use rustyworm::mimicry::layers::amplification::{AmplificationEngine, AmplificationConfig};
//!
//! let engine = AmplificationEngine::new(AmplificationConfig::default());
//! let result = engine.amplify_bidirectional(&forward_state, &backward_state);
//!
//! println!("Amplified confidence: {}", result.final_confidence);
//! println!("Iterations: {}", result.iterations);
//! ```

pub mod engine;
pub mod convergence;
pub mod metrics;

// Re-export primary types
pub use engine::{AmplificationEngine, AmplificationConfig, AmplificationInput};
pub use convergence::{ConvergenceDetector, ConvergenceConfig, ConvergenceStatus};
pub use metrics::{AmplificationMetrics, IterationMetrics, MetricsCollector};

/// Error types for amplification operations.
#[derive(Debug, Clone)]
pub enum AmplificationError {
    /// Amplification diverged (unbounded growth).
    Divergence { iterations: u32, final_value: f32 },
    /// Maximum iterations reached without convergence.
    MaxIterationsReached { iterations: u32, delta: f32 },
    /// Invalid input (NaN, negative confidence, etc.).
    InvalidInput(String),
    /// Configuration error.
    ConfigError(String),
}

impl std::fmt::Display for AmplificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmplificationError::Divergence { iterations, final_value } => {
                write!(f, "Amplification diverged after {} iterations (value: {})", iterations, final_value)
            }
            AmplificationError::MaxIterationsReached { iterations, delta } => {
                write!(f, "Max iterations ({}) reached, delta still {}", iterations, delta)
            }
            AmplificationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AmplificationError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl std::error::Error for AmplificationError {}

/// Result type for amplification operations.
pub type AmplificationResult<T> = Result<T, AmplificationError>;

/// Result of an amplification operation.
#[derive(Debug, Clone)]
pub struct AmplifiedResult {
    /// Final confidence value.
    pub final_confidence: f32,
    /// Number of iterations performed.
    pub iterations: u32,
    /// Whether convergence was achieved.
    pub converged: bool,
    /// Final delta (change in last iteration).
    pub final_delta: f32,
    /// Peak confidence during amplification.
    pub peak_confidence: f32,
    /// Amplification factor achieved.
    pub amplification_factor: f32,
    /// Processing time in microseconds.
    pub processing_time_us: u64,
    /// Metrics from the amplification process.
    pub metrics: AmplificationMetrics,
}

impl AmplifiedResult {
    /// Create a simple result without metrics.
    pub fn simple(confidence: f32, iterations: u32, converged: bool) -> Self {
        Self {
            final_confidence: confidence,
            iterations,
            converged,
            final_delta: 0.0,
            peak_confidence: confidence,
            amplification_factor: 1.0,
            processing_time_us: 0,
            metrics: AmplificationMetrics::default(),
        }
    }

    /// Check if amplification was successful.
    pub fn is_successful(&self) -> bool {
        self.converged && self.final_confidence > 0.0 && !self.final_confidence.is_nan()
    }
}

/// Prelude for convenient imports.
pub mod prelude {
    pub use super::{
        AmplificationConfig, AmplificationEngine, AmplificationError, AmplificationInput,
        AmplificationMetrics, AmplificationResult, AmplifiedResult, ConvergenceConfig,
        ConvergenceDetector, ConvergenceStatus, IterationMetrics, MetricsCollector,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplified_result() {
        let result = AmplifiedResult::simple(0.85, 5, true);
        assert!(result.is_successful());
        assert_eq!(result.iterations, 5);
    }

    #[test]
    fn test_amplification_error_display() {
        let err = AmplificationError::Divergence {
            iterations: 10,
            final_value: f32::INFINITY,
        };
        assert!(err.to_string().contains("diverged"));
    }
}
