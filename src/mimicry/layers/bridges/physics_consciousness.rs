//! Physics-Consciousness Bridge (Layer 1 ↔ Layer 4).
//!
//! Connects base physics with GAIA consciousness through analogical reasoning.
//! Enables intuitive pattern recognition based on physical principles.

use crate::mimicry::layers::bridge::{
    AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult,
};
use crate::mimicry::layers::layer::{Layer, LayerState};

/// Bridge between Base Physics (L1) and GAIA Consciousness (L4).
///
/// This bridge enables analogical reasoning between physical patterns
/// and consciousness-level intuitions.
pub struct PhysicsConsciousnessBridge {
    resonance: f32,
    amplification_factor: f32,
    /// Weight for analogical mapping strength.
    analogy_strength: f32,
    /// Intuition threshold for pattern activation.
    intuition_threshold: f32,
}

impl PhysicsConsciousnessBridge {
    pub fn new() -> Self {
        Self {
            resonance: 0.9,             // Slightly lower base resonance - these are distant layers
            amplification_factor: 1.25, // Higher amplification for cross-cutting insights
            analogy_strength: 0.7,
            intuition_threshold: 0.5,
        }
    }

    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    pub fn with_analogy_strength(mut self, strength: f32) -> Self {
        self.analogy_strength = strength;
        self
    }

    fn compute_analogy_factor(&self, confidence: f32) -> f32 {
        // Analogical mapping is non-linear - stronger for clearer patterns
        if confidence > 0.8 {
            self.analogy_strength * 1.3
        } else if confidence > 0.5 {
            self.analogy_strength
        } else {
            self.analogy_strength * 0.7
        }
    }

    fn transform_forward(&self, physics_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::GaiaConsciousness, physics_state.data_arc());

        let analogy_factor = self.compute_analogy_factor(physics_state.confidence);
        new_state.confidence = physics_state.confidence * self.resonance * analogy_factor;
        new_state.add_upstream(physics_state.id.clone());
        new_state.set_metadata("source_bridge", "physics_consciousness");
        new_state.set_metadata("analogy_factor", &analogy_factor.to_string());
        new_state.set_metadata("mapping_type", "physics_to_intuition");
        new_state
    }

    fn transform_backward(&self, consciousness_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::BasePhysics, consciousness_state.data_arc());

        // Intuitions refine physics understanding
        let intuition_factor = if consciousness_state.confidence > self.intuition_threshold {
            1.2 // Strong intuition provides significant guidance
        } else {
            1.0 // Weak intuition is passthrough
        };

        new_state.confidence = consciousness_state.confidence * self.resonance * intuition_factor;
        new_state.add_upstream(consciousness_state.id.clone());
        new_state.set_metadata("source_bridge", "physics_consciousness");
        new_state.set_metadata("intuition_factor", &intuition_factor.to_string());
        new_state.set_metadata("mapping_type", "intuition_to_physics");
        new_state
    }
}

impl Default for PhysicsConsciousnessBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for PhysicsConsciousnessBridge {
    fn name(&self) -> &str {
        "PhysicsConsciousnessBridge"
    }

    fn source_layer(&self) -> Layer {
        Layer::BasePhysics
    }

    fn target_layer(&self) -> Layer {
        Layer::GaiaConsciousness
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::BasePhysics {
            return Err(BridgeError::InvalidInput(format!(
                "Expected BasePhysics layer, got {:?}",
                input.layer
            )));
        }
        Ok(self.transform_forward(input))
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::GaiaConsciousness {
            return Err(BridgeError::InvalidInput(format!(
                "Expected GaiaConsciousness layer, got {:?}",
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
            // Physics informs intuition
            let analogy_factor = self.compute_analogy_factor(up_state.confidence);
            let physics_influence = up_state.confidence * analogy_factor * 0.2;
            down_state.confidence = (down_state.confidence + physics_influence).min(2.0);

            // Intuition refines physics
            let intuition_influence = if down_state.confidence > self.intuition_threshold {
                down_state.confidence * 0.25
            } else {
                down_state.confidence * 0.1
            };
            up_state.confidence = (up_state.confidence + intuition_influence).min(2.0);

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
    fn test_physics_consciousness_bridge() {
        let bridge = PhysicsConsciousnessBridge::new();
        assert_eq!(bridge.source_layer(), Layer::BasePhysics);
        assert_eq!(bridge.target_layer(), Layer::GaiaConsciousness);
    }

    #[test]
    fn test_analogical_mapping() {
        let bridge = PhysicsConsciousnessBridge::new();

        let physics = LayerState::with_confidence(Layer::BasePhysics, (), 0.9);
        let result = bridge.forward(&physics).unwrap();

        assert_eq!(result.layer, Layer::GaiaConsciousness);
        assert!(result.get_metadata("analogy_factor").is_some());
    }

    #[test]
    fn test_intuition_feedback() {
        let bridge = PhysicsConsciousnessBridge::new();

        let consciousness = LayerState::with_confidence(Layer::GaiaConsciousness, (), 0.7);
        let result = bridge.backward(&consciousness).unwrap();

        assert_eq!(result.layer, Layer::BasePhysics);
        // High intuition should boost confidence
        assert!(result.confidence > consciousness.confidence * 0.9);
    }
}
