//! Physics-Language Bridge (Layer 1 ↔ Layer 5).
//!
//! Connects base physics with multilingual processing through conceptual
//! structure mapping. Enables translation of physical concepts into linguistic
//! representations across languages.

use crate::mimicry::layers::bridge::{
    AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult,
};
use crate::mimicry::layers::layer::{Layer, LayerState};

/// Bridge between Base Physics (L1) and Multilingual Processing (L5).
pub struct PhysicsLanguageBridge {
    resonance: f32,
    amplification_factor: f32,
    /// Conceptual mapping strength.
    conceptual_strength: f32,
}

impl PhysicsLanguageBridge {
    pub fn new() -> Self {
        Self {
            resonance: 0.85,
            amplification_factor: 1.2,
            conceptual_strength: 0.75,
        }
    }

    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    fn transform_forward(&self, physics_state: &LayerState) -> LayerState {
        let mut new_state =
            LayerState::new(Layer::MultilingualProcessing, physics_state.data_arc());
        new_state.confidence = physics_state.confidence * self.resonance * self.conceptual_strength;
        new_state.add_upstream(physics_state.id.clone());
        new_state.set_metadata("source_bridge", "physics_language");
        new_state.set_metadata("mapping_type", "conceptual_structure");
        new_state
    }

    fn transform_backward(&self, language_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::BasePhysics, language_state.data_arc());
        new_state.confidence = language_state.confidence * self.resonance;
        new_state.add_upstream(language_state.id.clone());
        new_state.set_metadata("source_bridge", "physics_language");
        new_state.set_metadata("mapping_type", "linguistic_grounding");
        new_state
    }
}

impl Default for PhysicsLanguageBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for PhysicsLanguageBridge {
    fn name(&self) -> &str {
        "PhysicsLanguageBridge"
    }

    fn source_layer(&self) -> Layer {
        Layer::BasePhysics
    }

    fn target_layer(&self) -> Layer {
        Layer::MultilingualProcessing
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
        if feedback.layer != Layer::MultilingualProcessing {
            return Err(BridgeError::InvalidInput(format!(
                "Expected MultilingualProcessing layer, got {:?}",
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
            let conceptual_boost = up_state.confidence * self.conceptual_strength * 0.15;
            down_state.confidence = (down_state.confidence + conceptual_boost).min(2.0);

            let grounding_boost = down_state.confidence * 0.12;
            up_state.confidence = (up_state.confidence + grounding_boost).min(2.0);

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
    fn test_physics_language_bridge() {
        let bridge = PhysicsLanguageBridge::new();
        assert_eq!(bridge.source_layer(), Layer::BasePhysics);
        assert_eq!(bridge.target_layer(), Layer::MultilingualProcessing);
    }

    #[test]
    fn test_conceptual_mapping() {
        let bridge = PhysicsLanguageBridge::new();
        let physics = LayerState::with_confidence(Layer::BasePhysics, (), 0.9);

        let result = bridge.forward(&physics).unwrap();
        assert_eq!(result.layer, Layer::MultilingualProcessing);
        assert!(result.confidence > 0.0);
    }
}
