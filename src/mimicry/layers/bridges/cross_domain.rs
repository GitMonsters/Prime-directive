//! Cross-Domain Bridge (Layer 2 ↔ Layer 3).
//!
//! Connects extended physics domains with cross-domain relationship detection.
//! Enables emergence and composition pattern recognition.

use crate::mimicry::layers::bridge::{
    AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult,
};
use crate::mimicry::layers::layer::{Layer, LayerState};

/// Bridge between Extended Physics (L2) and Cross-Domain (L3).
///
/// This bridge enables the detection of emergent properties and
/// compositional patterns that arise from lower-level physics.
pub struct CrossDomainBridge {
    resonance: f32,
    amplification_factor: f32,
    /// Emergence detection sensitivity.
    emergence_threshold: f32,
    /// Composition detection sensitivity.
    composition_threshold: f32,
}

impl CrossDomainBridge {
    pub fn new() -> Self {
        Self {
            resonance: 1.0,
            amplification_factor: 1.2,
            emergence_threshold: 0.6,
            composition_threshold: 0.5,
        }
    }

    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    pub fn with_emergence_threshold(mut self, threshold: f32) -> Self {
        self.emergence_threshold = threshold;
        self
    }

    fn transform_forward(&self, extended_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::CrossDomain, extended_state.data_arc());

        // Cross-domain detection may boost or reduce confidence based on
        // whether patterns are recognized
        let emergence_factor = if extended_state.confidence > self.emergence_threshold {
            1.1 // Boost for high-confidence inputs
        } else {
            0.95 // Slight reduction for uncertain inputs
        };

        new_state.confidence = extended_state.confidence * self.resonance * emergence_factor;
        new_state.add_upstream(extended_state.id.clone());
        new_state.set_metadata("source_bridge", "cross_domain");
        new_state.set_metadata("emergence_factor", &emergence_factor.to_string());
        new_state
    }

    fn transform_backward(&self, cross_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::ExtendedPhysics, cross_state.data_arc());

        // Backward pass applies compositional refinement
        let composition_factor = if cross_state.confidence > self.composition_threshold {
            1.15 // Strong composition detected
        } else {
            1.0
        };

        new_state.confidence = cross_state.confidence * self.resonance * composition_factor;
        new_state.add_upstream(cross_state.id.clone());
        new_state.set_metadata("source_bridge", "cross_domain");
        new_state.set_metadata("composition_factor", &composition_factor.to_string());
        new_state
    }
}

impl Default for CrossDomainBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for CrossDomainBridge {
    fn name(&self) -> &str {
        "CrossDomainBridge"
    }

    fn source_layer(&self) -> Layer {
        Layer::ExtendedPhysics
    }

    fn target_layer(&self) -> Layer {
        Layer::CrossDomain
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
        if feedback.layer != Layer::CrossDomain {
            return Err(BridgeError::InvalidInput(format!(
                "Expected CrossDomain layer, got {:?}",
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
            // Emergence amplification
            let emergence_boost = if up_state.confidence > self.emergence_threshold {
                0.1 * self.resonance
            } else {
                0.05 * self.resonance
            };
            down_state.confidence = (down_state.confidence + emergence_boost).min(2.0);

            // Composition feedback
            let composition_boost = if down_state.confidence > self.composition_threshold {
                0.1 * self.resonance
            } else {
                0.05 * self.resonance
            };
            up_state.confidence = (up_state.confidence + composition_boost).min(2.0);

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
    fn test_cross_domain_bridge() {
        let bridge = CrossDomainBridge::new();
        assert_eq!(bridge.source_layer(), Layer::ExtendedPhysics);
        assert_eq!(bridge.target_layer(), Layer::CrossDomain);
    }

    #[test]
    fn test_emergence_detection() {
        let bridge = CrossDomainBridge::new();

        // High confidence should get emergence boost
        let high_conf = LayerState::with_confidence(Layer::ExtendedPhysics, (), 0.8);
        let result = bridge.forward(&high_conf).unwrap();
        assert!(result.confidence > 0.8); // Should be boosted

        // Low confidence should not get boost
        let low_conf = LayerState::with_confidence(Layer::ExtendedPhysics, (), 0.4);
        let result = bridge.forward(&low_conf).unwrap();
        assert!(result.confidence < 0.4); // Slight reduction
    }
}
