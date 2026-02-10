//! Individual-Collective Bridge (Layer 3 ↔ Layer 6).
//!
//! Connects cross-domain relationships with collaborative learning.
//! Enables social amplification of patterns and collective intelligence.

use crate::mimicry::layers::bridge::{
    AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult,
};
use crate::mimicry::layers::layer::{Layer, LayerState};

/// Bridge between Cross-Domain (L3) and Collaborative Learning (L6).
pub struct IndividualCollectiveBridge {
    resonance: f32,
    amplification_factor: f32,
    /// Social amplification factor for collective insights.
    social_amplification: f32,
}

impl IndividualCollectiveBridge {
    pub fn new() -> Self {
        Self {
            resonance: 1.1,            // Higher resonance - social learning is powerful
            amplification_factor: 1.3, // Strong amplification for collective wisdom
            social_amplification: 0.8,
        }
    }

    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    pub fn with_social_amplification(mut self, factor: f32) -> Self {
        self.social_amplification = factor;
        self
    }

    fn transform_forward(&self, individual_state: &LayerState) -> LayerState {
        let mut new_state =
            LayerState::new(Layer::CollaborativeLearning, individual_state.data_arc());
        // Individual insights get social boost when shared
        new_state.confidence =
            individual_state.confidence * self.resonance * self.social_amplification;
        new_state.add_upstream(individual_state.id.clone());
        new_state.set_metadata("source_bridge", "individual_collective");
        new_state.set_metadata("social_factor", &self.social_amplification.to_string());
        new_state
    }

    fn transform_backward(&self, collective_state: &LayerState) -> LayerState {
        let mut new_state = LayerState::new(Layer::CrossDomain, collective_state.data_arc());
        // Collective wisdom refines individual understanding with extra boost
        new_state.confidence = collective_state.confidence * self.resonance * 1.1;
        new_state.add_upstream(collective_state.id.clone());
        new_state.set_metadata("source_bridge", "individual_collective");
        new_state.set_metadata("collective_wisdom", "true");
        new_state
    }
}

impl Default for IndividualCollectiveBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for IndividualCollectiveBridge {
    fn name(&self) -> &str {
        "IndividualCollectiveBridge"
    }

    fn source_layer(&self) -> Layer {
        Layer::CrossDomain
    }

    fn target_layer(&self) -> Layer {
        Layer::CollaborativeLearning
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::CrossDomain {
            return Err(BridgeError::InvalidInput(format!(
                "Expected CrossDomain layer, got {:?}",
                input.layer
            )));
        }
        Ok(self.transform_forward(input))
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::CollaborativeLearning {
            return Err(BridgeError::InvalidInput(format!(
                "Expected CollaborativeLearning layer, got {:?}",
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
            // Individual → Collective: social amplification
            let social_boost = up_state.confidence * self.social_amplification * 0.2;
            down_state.confidence = (down_state.confidence + social_boost).min(2.5); // Higher cap for social

            // Collective → Individual: wisdom feedback
            let wisdom_boost = down_state.confidence * 0.25;
            up_state.confidence = (up_state.confidence + wisdom_boost).min(2.0);

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
    fn test_individual_collective_bridge() {
        let bridge = IndividualCollectiveBridge::new();
        assert_eq!(bridge.source_layer(), Layer::CrossDomain);
        assert_eq!(bridge.target_layer(), Layer::CollaborativeLearning);
        assert!(bridge.resonance() > 1.0); // Higher resonance for social
    }

    #[test]
    fn test_social_amplification() {
        let bridge = IndividualCollectiveBridge::new();
        let individual = LayerState::with_confidence(Layer::CrossDomain, (), 0.7);

        let collective = bridge.forward(&individual).unwrap();
        // Social boost should increase confidence
        assert!(collective.confidence > individual.confidence * 0.5);
    }

    #[test]
    fn test_collective_wisdom_feedback() {
        let bridge = IndividualCollectiveBridge::new();
        let collective = LayerState::with_confidence(Layer::CollaborativeLearning, (), 0.8);

        let refined = bridge.backward(&collective).unwrap();
        // Collective wisdom should boost individual
        assert!(refined.confidence > collective.confidence);
    }
}
