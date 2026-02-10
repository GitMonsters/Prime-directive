//! Internal-External Bridge (Layer 2 ↔ Layer 7).
//!
//! Connects extended physics domains with external API validation.
//! Enables real-time external verification and feedback loops.

use crate::mimicry::layers::bridge::{
    AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult,
};
use crate::mimicry::layers::layer::{Layer, LayerState};

/// Bridge between Extended Physics (L2) and External APIs (L7).
pub struct InternalExternalBridge {
    resonance: f32,
    amplification_factor: f32,
    /// Validation threshold for external confirmation.
    validation_threshold: f32,
}

impl InternalExternalBridge {
    pub fn new() -> Self {
        Self {
            resonance: 0.95,
            amplification_factor: 1.15,
            validation_threshold: 0.6,
        }
    }

    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    pub fn with_validation_threshold(mut self, threshold: f32) -> Self {
        self.validation_threshold = threshold;
        self
    }

    fn transform_forward(&self, internal_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::ExternalApis, internal_state.data_arc());
        new_state.confidence = internal_state.confidence * self.resonance;
        new_state.add_upstream(internal_state.id.clone());
        new_state.set_metadata("source_bridge", "internal_external");
        new_state.set_metadata("requires_validation", "true");
        new_state
    }

    fn transform_backward(&self, external_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::ExtendedPhysics, external_state.data_arc());

        // External validation can significantly boost or reduce confidence
        let validation_factor = if external_state.confidence > self.validation_threshold {
            1.25 // Strong external validation
        } else if external_state.confidence > 0.3 {
            1.0 // Neutral
        } else {
            0.7 // External disagreement reduces confidence
        };

        new_state.confidence = external_state.confidence * self.resonance * validation_factor;
        new_state.add_upstream(external_state.id.clone());
        new_state.set_metadata("source_bridge", "internal_external");
        new_state.set_metadata("validation_factor", &validation_factor.to_string());
        new_state.set_metadata("externally_validated", "true");
        new_state
    }
}

impl Default for InternalExternalBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for InternalExternalBridge {
    fn name(&self) -> &str {
        "InternalExternalBridge"
    }

    fn source_layer(&self) -> Layer {
        Layer::ExtendedPhysics
    }

    fn target_layer(&self) -> Layer {
        Layer::ExternalApis
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::ExtendedPhysics {
            return Err(BridgeError::InvalidInput(format!(
                "Expected ExtendedPhysics layer, got {:?}",
                input.layer
            )));
        }
        Ok(self.transform_forward(input))
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::ExternalApis {
            return Err(BridgeError::InvalidInput(format!(
                "Expected ExternalApis layer, got {:?}",
                feedback.layer
            )));
        }
        Ok(self.transform_backward(feedback))
    }

    fn amplify(
        &self,
        up: &LayerState,
        down: &LayerState,
        max_iterations: u32,
    ) -> BridgeResult<AmplificationResult> {
        let mut up_state = up.clone();
        let mut down_state = down.clone();
        let mut previous_combined = 0.0f32;

        for i in 0..max_iterations {
            // Internal → External: request validation
            let internal_influence = up_state.confidence * 0.1;
            down_state.confidence = (down_state.confidence + internal_influence).min(2.0);

            // External → Internal: validation feedback
            let validation_boost = if down_state.confidence > self.validation_threshold {
                down_state.confidence * 0.2
            } else {
                down_state.confidence * 0.05
            };
            up_state.confidence = (up_state.confidence + validation_boost).min(2.0);

            let combined = up_state.confidence * down_state.confidence * self.amplification_factor;

            if (combined - previous_combined).abs() < 0.001 {
                return Ok(AmplificationResult {
                    up_state,
                    down_state,
                    combined_confidence: combined,
                    amplification_factor: self.amplification_factor,
                    iterations: i + 1,
                    converged: true,
                    resonance: self.resonance,
                });
            }

            previous_combined = combined;
            up_state.increment_amplification();
            down_state.increment_amplification();
        }

        let combined = up_state.confidence * down_state.confidence * self.amplification_factor;
        Ok(AmplificationResult {
            up_state,
            down_state,
            combined_confidence: combined,
            amplification_factor: self.amplification_factor,
            iterations: max_iterations,
            converged: false,
            resonance: self.resonance,
        })
    }

    fn resonance(&self) -> f32 {
        self.resonance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_external_bridge() {
        let bridge = InternalExternalBridge::new();
        assert_eq!(bridge.source_layer(), Layer::ExtendedPhysics);
        assert_eq!(bridge.target_layer(), Layer::ExternalApis);
    }

    #[test]
    fn test_validation_boost() {
        let bridge = InternalExternalBridge::new();

        // High external validation should boost
        let high_validation = LayerState::with_confidence(Layer::ExternalApis, (), 0.9);
        let result = bridge.backward(&high_validation).unwrap();
        assert!(result.confidence > high_validation.confidence);

        // Low external validation should reduce
        let low_validation = LayerState::with_confidence(Layer::ExternalApis, (), 0.2);
        let result = bridge.backward(&low_validation).unwrap();
        assert!(result.confidence < low_validation.confidence);
    }
}
