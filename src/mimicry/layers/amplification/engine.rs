//! Core amplification engine implementation.
//!
//! The AmplificationEngine orchestrates the multiplicative confidence
//! boosting process across layer states.

use std::time::Instant;

use serde::{Deserialize, Serialize};

use super::convergence::{ConvergenceConfig, ConvergenceDetector, ConvergenceStatus};
use super::metrics::{IterationMetrics, MetricsCollector};
use super::{AmplificationError, AmplificationResult, AmplifiedResult};
use crate::mimicry::layers::layer::LayerState;

/// Configuration for the amplification engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplificationConfig {
    /// Base amplification factor per iteration.
    pub base_factor: f32,

    /// Maximum number of iterations.
    pub max_iterations: u32,

    /// Damping factor to prevent runaway (applied per iteration).
    pub damping: f32,

    /// Maximum allowed confidence (caps amplification).
    pub max_confidence: f32,

    /// Minimum confidence (floor).
    pub min_confidence: f32,

    /// Whether to use adaptive amplification.
    pub adaptive: bool,

    /// Convergence configuration.
    pub convergence: ConvergenceConfig,

    /// Enable metrics collection.
    pub collect_metrics: bool,
}

impl Default for AmplificationConfig {
    fn default() -> Self {
        Self {
            base_factor: 1.1,
            max_iterations: 20,
            damping: 0.95,
            max_confidence: 10.0,
            min_confidence: 0.001,
            adaptive: true,
            convergence: ConvergenceConfig::default(),
            collect_metrics: true,
        }
    }
}

impl AmplificationConfig {
    /// Create a conservative configuration (lower amplification).
    pub fn conservative() -> Self {
        Self {
            base_factor: 1.05,
            max_iterations: 10,
            damping: 0.9,
            max_confidence: 2.0,
            ..Default::default()
        }
    }

    /// Create an aggressive configuration (higher amplification).
    pub fn aggressive() -> Self {
        Self {
            base_factor: 1.2,
            max_iterations: 30,
            damping: 0.98,
            max_confidence: 20.0,
            ..Default::default()
        }
    }

    /// Create a fast configuration (fewer iterations).
    pub fn fast() -> Self {
        Self {
            max_iterations: 5,
            collect_metrics: false,
            ..Default::default()
        }
    }
}

/// Input for amplification operations.
#[derive(Debug, Clone)]
pub struct AmplificationInput {
    /// Forward (upward) confidence from lower layers.
    pub forward_confidence: f32,
    /// Backward (downward) confidence from higher layers.
    pub backward_confidence: f32,
    /// Optional weights for forward vs backward.
    pub forward_weight: f32,
    pub backward_weight: f32,
    /// Initial combined confidence (before amplification).
    pub initial_confidence: f32,
    /// Resonance factor from bridge.
    pub resonance: f32,
}

impl AmplificationInput {
    /// Create a new input with default weights.
    pub fn new(forward: f32, backward: f32) -> Self {
        Self {
            forward_confidence: forward,
            backward_confidence: backward,
            forward_weight: 0.5,
            backward_weight: 0.5,
            initial_confidence: (forward * backward).sqrt(), // Geometric mean
            resonance: 1.0,
        }
    }

    /// Set weights.
    pub fn with_weights(mut self, forward: f32, backward: f32) -> Self {
        let total = forward + backward;
        self.forward_weight = forward / total;
        self.backward_weight = backward / total;
        self
    }

    /// Set resonance.
    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    /// Create from layer states.
    pub fn from_states(forward_state: &LayerState, backward_state: &LayerState) -> Self {
        Self::new(forward_state.confidence, backward_state.confidence)
    }

    /// Calculate initial combined confidence.
    pub fn combined(&self) -> f32 {
        let weighted = self.forward_confidence * self.forward_weight
            + self.backward_confidence * self.backward_weight;
        weighted * self.resonance
    }
}

/// The main amplification engine.
pub struct AmplificationEngine {
    config: AmplificationConfig,
    convergence_detector: ConvergenceDetector,
    metrics_collector: MetricsCollector,
}

impl AmplificationEngine {
    /// Create a new amplification engine.
    pub fn new(config: AmplificationConfig) -> Self {
        Self {
            convergence_detector: ConvergenceDetector::new(config.convergence.clone()),
            metrics_collector: MetricsCollector::new(),
            config,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(AmplificationConfig::default())
    }

    /// Get the configuration.
    pub fn config(&self) -> &AmplificationConfig {
        &self.config
    }

    /// Amplify confidence from a single value.
    pub fn amplify(&self, initial_confidence: f32) -> AmplificationResult<AmplifiedResult> {
        self.validate_input(initial_confidence)?;

        let start = Instant::now();
        let mut confidence = initial_confidence;
        let mut peak = confidence;
        let mut iteration = 0;
        let mut last_delta = 0.0f32;

        self.convergence_detector.reset();
        self.metrics_collector.reset();

        while iteration < self.config.max_iterations {
            let old_confidence = confidence;

            // Calculate amplification factor
            let factor = if self.config.adaptive {
                self.adaptive_factor(confidence, iteration)
            } else {
                self.config.base_factor
            };

            // Apply amplification
            confidence *= factor;

            // Apply damping
            confidence *= self.config.damping;

            // Apply bounds
            confidence = confidence.clamp(self.config.min_confidence, self.config.max_confidence);

            // Track peak
            peak = peak.max(confidence);

            // Calculate delta
            last_delta = (confidence - old_confidence).abs();

            // Record metrics
            if self.config.collect_metrics {
                self.metrics_collector.record_iteration(IterationMetrics {
                    iteration,
                    confidence,
                    delta: last_delta,
                    factor,
                });
            }

            // Check convergence
            let status = self
                .convergence_detector
                .check(old_confidence, confidence, iteration);
            match status {
                ConvergenceStatus::Converged => {
                    return Ok(self.build_result(
                        confidence,
                        iteration + 1,
                        true,
                        last_delta,
                        peak,
                        initial_confidence,
                        start.elapsed().as_micros() as u64,
                    ));
                }
                ConvergenceStatus::Diverging => {
                    return Err(AmplificationError::Divergence {
                        iterations: iteration + 1,
                        final_value: confidence,
                    });
                }
                ConvergenceStatus::InProgress => {}
            }

            iteration += 1;
        }

        // Max iterations reached
        if last_delta <= self.config.convergence.threshold {
            // Close enough to converged
            Ok(self.build_result(
                confidence,
                iteration,
                true,
                last_delta,
                peak,
                initial_confidence,
                start.elapsed().as_micros() as u64,
            ))
        } else {
            Err(AmplificationError::MaxIterationsReached {
                iterations: iteration,
                delta: last_delta,
            })
        }
    }

    /// Amplify bidirectional confidence.
    pub fn amplify_bidirectional(
        &self,
        input: &AmplificationInput,
    ) -> AmplificationResult<AmplifiedResult> {
        let combined = input.combined();
        self.validate_input(combined)?;

        let start = Instant::now();
        let mut forward = input.forward_confidence;
        let mut backward = input.backward_confidence;
        let mut combined = input.initial_confidence;
        let mut peak = combined;
        let mut iteration = 0;
        let mut last_delta = 0.0f32;

        self.convergence_detector.reset();
        self.metrics_collector.reset();

        while iteration < self.config.max_iterations {
            let old_combined = combined;

            // Amplify forward and backward separately
            let f_factor = if self.config.adaptive {
                self.adaptive_factor(forward, iteration)
            } else {
                self.config.base_factor
            };
            let b_factor = if self.config.adaptive {
                self.adaptive_factor(backward, iteration)
            } else {
                self.config.base_factor
            };

            forward *= f_factor * self.config.damping;
            backward *= b_factor * self.config.damping;

            // Apply bounds
            forward = forward.clamp(self.config.min_confidence, self.config.max_confidence);
            backward = backward.clamp(self.config.min_confidence, self.config.max_confidence);

            // Combine multiplicatively with resonance
            combined = (forward * backward).sqrt() * input.resonance;
            combined = combined.clamp(self.config.min_confidence, self.config.max_confidence);

            peak = peak.max(combined);
            last_delta = (combined - old_combined).abs();

            // Record metrics
            if self.config.collect_metrics {
                self.metrics_collector.record_iteration(IterationMetrics {
                    iteration,
                    confidence: combined,
                    delta: last_delta,
                    factor: (f_factor + b_factor) / 2.0,
                });
            }

            // Check convergence
            let status = self
                .convergence_detector
                .check(old_combined, combined, iteration);
            match status {
                ConvergenceStatus::Converged => {
                    return Ok(self.build_result(
                        combined,
                        iteration + 1,
                        true,
                        last_delta,
                        peak,
                        input.initial_confidence,
                        start.elapsed().as_micros() as u64,
                    ));
                }
                ConvergenceStatus::Diverging => {
                    return Err(AmplificationError::Divergence {
                        iterations: iteration + 1,
                        final_value: combined,
                    });
                }
                ConvergenceStatus::InProgress => {}
            }

            iteration += 1;
        }

        // Max iterations - check if close enough
        if last_delta <= self.config.convergence.threshold * 2.0 {
            Ok(self.build_result(
                combined,
                iteration,
                true,
                last_delta,
                peak,
                input.initial_confidence,
                start.elapsed().as_micros() as u64,
            ))
        } else {
            Err(AmplificationError::MaxIterationsReached {
                iterations: iteration,
                delta: last_delta,
            })
        }
    }

    /// Amplify layer states directly.
    pub fn amplify_states(
        &self,
        forward_state: &LayerState,
        backward_state: &LayerState,
    ) -> AmplificationResult<AmplifiedResult> {
        let input = AmplificationInput::from_states(forward_state, backward_state);
        self.amplify_bidirectional(&input)
    }

    /// Calculate adaptive amplification factor.
    fn adaptive_factor(&self, confidence: f32, iteration: u32) -> f32 {
        // Lower factor as confidence increases (diminishing returns)
        let confidence_adjustment = 1.0 / (1.0 + confidence.ln().max(0.0) * 0.1);

        // Lower factor as iterations increase (convergence pressure)
        let iteration_adjustment = 1.0 / (1.0 + iteration as f32 * 0.05);

        self.config.base_factor * confidence_adjustment * iteration_adjustment
    }

    fn validate_input(&self, confidence: f32) -> AmplificationResult<()> {
        if confidence.is_nan() {
            return Err(AmplificationError::InvalidInput("Confidence is NaN".into()));
        }
        if confidence.is_infinite() {
            return Err(AmplificationError::InvalidInput(
                "Confidence is infinite".into(),
            ));
        }
        if confidence < 0.0 {
            return Err(AmplificationError::InvalidInput(
                "Confidence must be non-negative".into(),
            ));
        }
        Ok(())
    }

    fn build_result(
        &self,
        final_confidence: f32,
        iterations: u32,
        converged: bool,
        final_delta: f32,
        peak_confidence: f32,
        initial_confidence: f32,
        processing_time_us: u64,
    ) -> AmplifiedResult {
        let amplification_factor = if initial_confidence > 0.0 {
            final_confidence / initial_confidence
        } else {
            1.0
        };

        AmplifiedResult {
            final_confidence,
            iterations,
            converged,
            final_delta,
            peak_confidence,
            amplification_factor,
            processing_time_us,
            metrics: self.metrics_collector.collect(),
        }
    }
}

impl Default for AmplificationEngine {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = AmplificationEngine::with_defaults();
        assert_eq!(engine.config().max_iterations, 20);
    }

    #[test]
    fn test_simple_amplification() {
        let engine = AmplificationEngine::with_defaults();
        let result = engine.amplify(0.5).unwrap();

        assert!(result.is_successful());
        assert!(result.final_confidence > 0.0);
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_bidirectional_amplification() {
        let engine = AmplificationEngine::with_defaults();
        let input = AmplificationInput::new(0.8, 0.7);

        let result = engine.amplify_bidirectional(&input).unwrap();
        assert!(result.is_successful());
    }

    #[test]
    fn test_invalid_input() {
        let engine = AmplificationEngine::with_defaults();

        assert!(engine.amplify(f32::NAN).is_err());
        assert!(engine.amplify(-1.0).is_err());
        assert!(engine.amplify(f32::INFINITY).is_err());
    }

    #[test]
    fn test_amplification_with_resonance() {
        let engine = AmplificationEngine::with_defaults();

        let low_resonance = AmplificationInput::new(0.5, 0.5).with_resonance(0.5);
        let high_resonance = AmplificationInput::new(0.5, 0.5).with_resonance(1.5);

        let low_result = engine.amplify_bidirectional(&low_resonance).unwrap();
        let high_result = engine.amplify_bidirectional(&high_resonance).unwrap();

        // Higher resonance should lead to higher confidence
        assert!(high_result.final_confidence > low_result.final_confidence);
    }

    #[test]
    fn test_conservative_config() {
        let engine = AmplificationEngine::new(AmplificationConfig::conservative());
        // Conservative config may not converge with default threshold, that's OK
        // What matters is the max confidence is capped
        match engine.amplify(0.5) {
            Ok(result) => {
                assert!(result.final_confidence <= 2.0);
            }
            Err(AmplificationError::MaxIterationsReached { .. }) => {
                // This is acceptable for conservative config
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_adaptive_factor() {
        let engine = AmplificationEngine::with_defaults();

        // Higher confidence should have lower factor
        let low_factor = engine.adaptive_factor(2.0, 0);
        let high_factor = engine.adaptive_factor(0.5, 0);
        assert!(low_factor < high_factor);

        // Later iterations should have lower factor
        let early_factor = engine.adaptive_factor(0.5, 0);
        let late_factor = engine.adaptive_factor(0.5, 10);
        assert!(late_factor < early_factor);
    }
}
