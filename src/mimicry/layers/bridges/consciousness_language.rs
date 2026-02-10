//! GAIA Consciousness to Multilingual Processing bridge (L4↔L5).
//!
//! This bridge connects intuitive pattern recognition with linguistic
//! perspective and translation capabilities.

use super::super::bridge::{AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult};
use super::super::layer::{Layer, LayerState};

/// Bridge between GAIA Consciousness (L4) and Multilingual Processing (L5).
///
/// This bridge enables:
/// - Forward: Intuitive insights inform linguistic perspective
/// - Backward: Linguistic patterns refine intuition
/// - Amplification: Cross-cultural intuition emerges
pub struct ConsciousnessLanguageBridge {
    /// Base resonance for this bridge.
    base_resonance: f32,
}

impl ConsciousnessLanguageBridge {
    pub fn new() -> Self {
        Self {
            base_resonance: 0.85,
        }
    }
}

impl Default for ConsciousnessLanguageBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for ConsciousnessLanguageBridge {
    fn name(&self) -> &str {
        "Consciousness-Language"
    }

    fn source_layer(&self) -> Layer {
        Layer::GaiaConsciousness
    }

    fn target_layer(&self) -> Layer {
        Layer::MultilingualProcessing
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::GaiaConsciousness {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::GaiaConsciousness,
                input.layer
            )));
        }

        // Transform intuitive insights into linguistic perspective
        let mut output = LayerState::with_confidence(
            Layer::MultilingualProcessing,
            input.data_arc(),
            input.confidence * 0.92,
        );

        output.set_metadata("transform", "intuition_to_perspective");
        output.set_metadata("source_bridge", self.name());
        output.add_upstream(&input.id);

        Ok(output)
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::MultilingualProcessing {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::MultilingualProcessing,
                feedback.layer
            )));
        }

        // Linguistic patterns refine intuitive understanding
        let mut refined = LayerState::with_confidence(
            Layer::GaiaConsciousness,
            feedback.data_arc(),
            feedback.confidence * 0.90,
        );

        refined.set_metadata("refinement", "linguistic_intuition");
        refined.add_upstream(&feedback.id);

        Ok(refined)
    }

    fn amplify(
        &self,
        up: &LayerState,
        down: &LayerState,
        max_iterations: u32,
    ) -> BridgeResult<AmplificationResult> {
        let mut up_conf = up.confidence;
        let mut down_conf = down.confidence;
        let mut total_factor = 1.0f32;

        for _ in 0..max_iterations.min(8) {
            // Cross-cultural intuition emerges from the interaction
            let cultural_resonance = (up_conf * down_conf).sqrt();
            let boost = 1.0 + (cultural_resonance * 0.12);

            up_conf *= boost;
            down_conf *= boost;
            total_factor *= boost;

            // Check convergence
            if boost < 1.01 {
                break;
            }
        }

        Ok(AmplificationResult {
            up_state: LayerState::with_confidence(up.layer, up.data_arc(), up_conf),
            down_state: LayerState::with_confidence(down.layer, down.data_arc(), down_conf),
            combined_confidence: up_conf * down_conf,
            amplification_factor: total_factor,
            iterations: max_iterations.min(8),
            converged: true,
            resonance: self.base_resonance,
        })
    }

    fn resonance(&self) -> f32 {
        self.base_resonance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_language_bridge() {
        let bridge = ConsciousnessLanguageBridge::new();

        assert_eq!(bridge.source_layer(), Layer::GaiaConsciousness);
        assert_eq!(bridge.target_layer(), Layer::MultilingualProcessing);
        assert!(bridge.resonance() > 0.8);
    }

    #[test]
    fn test_forward_transform() {
        let bridge = ConsciousnessLanguageBridge::new();
        let input = LayerState::with_confidence(
            Layer::GaiaConsciousness,
            "intuitive insight".to_string(),
            0.9,
        );

        let result = bridge.forward(&input).unwrap();
        assert_eq!(result.layer, Layer::MultilingualProcessing);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_backward_refinement() {
        let bridge = ConsciousnessLanguageBridge::new();
        let feedback = LayerState::with_confidence(
            Layer::MultilingualProcessing,
            "linguistic pattern".to_string(),
            0.85,
        );

        let result = bridge.backward(&feedback).unwrap();
        assert_eq!(result.layer, Layer::GaiaConsciousness);
    }

    #[test]
    fn test_amplification() {
        let bridge = ConsciousnessLanguageBridge::new();

        let up = LayerState::with_confidence(Layer::GaiaConsciousness, (), 0.8);
        let down = LayerState::with_confidence(Layer::MultilingualProcessing, (), 0.8);

        let result = bridge.amplify(&up, &down, 10).unwrap();
        assert!(result.amplification_factor > 1.0);
    }
}
