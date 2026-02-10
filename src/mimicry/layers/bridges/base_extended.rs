//! Base-Extended Physics Bridge (Layer 1 ↔ Layer 2).
//!
//! This bridge connects the base physics layer (perception, memory,
//! compression, reasoning) with the extended physics layer (planning,
//! tool use, execution, learning).

use crate::mimicry::layers::bridge::{
    AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult,
};
use crate::mimicry::layers::layer::{Layer, LayerState};

/// Bridge between Base Physics (L1) and Extended Physics (L2).
///
/// This is the foundational bridge that enables the 8-phase cognitive
/// pipeline to operate as a unified system with feedback loops.
pub struct BaseExtendedBridge {
    /// Current resonance factor.
    resonance: f32,
    /// Amplification factor for this bridge.
    amplification_factor: f32,
    /// Learning rate for reinforcement.
    learning_rate: f32,
    /// Total successful transfers.
    successful_transfers: u64,
}

impl BaseExtendedBridge {
    /// Create a new base-extended bridge.
    pub fn new() -> Self {
        Self {
            resonance: 1.0,
            amplification_factor: 1.15,
            learning_rate: 0.01,
            successful_transfers: 0,
        }
    }

    /// Create with custom resonance.
    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    /// Create with custom amplification factor.
    pub fn with_amplification_factor(mut self, factor: f32) -> Self {
        self.amplification_factor = factor;
        self
    }

    /// Transform base physics output into extended physics input.
    fn transform_forward(&self, base_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::ExtendedPhysics, base_state.data_arc());
        new_state.confidence = base_state.confidence * self.resonance;
        new_state.add_upstream(base_state.id.clone());
        new_state.set_metadata("source_bridge", "base_extended");
        new_state.set_metadata("direction", "forward");
        new_state
    }

    /// Transform extended physics feedback into base physics refinement.
    fn transform_backward(&self, extended_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::BasePhysics, extended_state.data_arc());
        // Backward refinement gets a slight confidence boost from learning
        new_state.confidence = extended_state.confidence * self.resonance * 1.05;
        new_state.add_upstream(extended_state.id.clone());
        new_state.set_metadata("source_bridge", "base_extended");
        new_state.set_metadata("direction", "backward");
        new_state.set_metadata("refinement", "true");
        new_state
    }
}

impl Default for BaseExtendedBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for BaseExtendedBridge {
    fn name(&self) -> &str {
        "BaseExtendedBridge"
    }

    fn source_layer(&self) -> Layer {
        Layer::BasePhysics
    }

    fn target_layer(&self) -> Layer {
        Layer::ExtendedPhysics
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
        if feedback.layer != Layer::ExtendedPhysics {
            return Err(BridgeError::InvalidInput(format!(
                "Expected ExtendedPhysics layer, got {:?}",
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
        let mut iterations = 0;
        let convergence_threshold = 0.001;
        let mut previous_combined = 0.0f32;

        for i in 0..max_iterations {
            // Forward pass: up influences down
            let forward_influence = up_state.confidence * self.resonance * 0.3;
            down_state.confidence = (down_state.confidence + forward_influence).min(2.0);

            // Backward pass: down influences up
            let backward_influence = down_state.confidence * self.resonance * 0.3;
            up_state.confidence = (up_state.confidence + backward_influence).min(2.0);

            // Apply amplification
            let combined = up_state.confidence * down_state.confidence * self.amplification_factor;

            // Check convergence
            if (combined - previous_combined).abs() < convergence_threshold {
                iterations = i + 1;
                break;
            }

            previous_combined = combined;
            iterations = i + 1;

            up_state.increment_amplification();
            down_state.increment_amplification();
        }

        let combined_confidence =
            up_state.confidence * down_state.confidence * self.amplification_factor;

        Ok(AmplificationResult {
            up_state,
            down_state,
            combined_confidence,
            amplification_factor: self.amplification_factor,
            iterations,
            converged: iterations < max_iterations,
            resonance: self.resonance,
        })
    }

    fn resonance(&self) -> f32 {
        self.resonance
    }

    fn reinforce(&mut self, result: &AmplificationResult) {
        if result.converged && result.combined_confidence > 1.0 {
            // Increase resonance for successful amplification
            self.resonance = (self.resonance + self.learning_rate).min(2.0);
            self.successful_transfers += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_creation() {
        let bridge = BaseExtendedBridge::new();
        assert_eq!(bridge.source_layer(), Layer::BasePhysics);
        assert_eq!(bridge.target_layer(), Layer::ExtendedPhysics);
        assert_eq!(bridge.resonance(), 1.0);
    }

    #[test]
    fn test_forward_propagation() {
        let bridge = BaseExtendedBridge::new();
        let input = LayerState::with_confidence(Layer::BasePhysics, "test".to_string(), 0.8);

        let output = bridge.forward(&input).unwrap();
        assert_eq!(output.layer, Layer::ExtendedPhysics);
        assert!((output.confidence - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_backward_propagation() {
        let bridge = BaseExtendedBridge::new();
        let feedback =
            LayerState::with_confidence(Layer::ExtendedPhysics, "feedback".to_string(), 0.9);

        let output = bridge.backward(&feedback).unwrap();
        assert_eq!(output.layer, Layer::BasePhysics);
        // Backward gets 5% boost
        assert!((output.confidence - 0.945).abs() < 0.001);
    }

    #[test]
    fn test_invalid_layer() {
        let bridge = BaseExtendedBridge::new();
        let wrong_layer = LayerState::new(Layer::GaiaConsciousness, "test".to_string());

        assert!(bridge.forward(&wrong_layer).is_err());
    }

    #[test]
    fn test_amplification() {
        let bridge = BaseExtendedBridge::new();
        let up = LayerState::with_confidence(Layer::BasePhysics, (), 0.8);
        let down = LayerState::with_confidence(Layer::ExtendedPhysics, (), 0.9);

        let result = bridge.amplify(&up, &down, 10).unwrap();
        assert!(result.combined_confidence > 0.72); // Should be amplified
        assert!(result.iterations > 0);
    }
}
