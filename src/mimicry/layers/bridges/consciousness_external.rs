//! GAIA Consciousness to External APIs bridge (L4↔L7).
//!
//! This bridge enables intuitive insights to be validated by external
//! sources and for external data to inform intuition.

use super::super::bridge::{AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult};
use super::super::layer::{Layer, LayerState};

/// Bridge between GAIA Consciousness (L4) and External APIs (L7).
///
/// This bridge enables:
/// - Forward: Intuitive hypotheses are externally validated
/// - Backward: External knowledge feeds intuition
/// - Amplification: Grounded intuition (intuition with evidence)
pub struct ConsciousnessExternalBridge {
    /// Base resonance for this bridge.
    base_resonance: f32,
}

impl ConsciousnessExternalBridge {
    pub fn new() -> Self {
        Self {
            base_resonance: 0.75,
        }
    }
}

impl Default for ConsciousnessExternalBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for ConsciousnessExternalBridge {
    fn name(&self) -> &str {
        "Consciousness-External"
    }

    fn source_layer(&self) -> Layer {
        Layer::GaiaConsciousness
    }

    fn target_layer(&self) -> Layer {
        Layer::ExternalApis
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::GaiaConsciousness {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::GaiaConsciousness,
                input.layer
            )));
        }

        // Intuitive hypotheses sent for external validation
        let mut output = LayerState::with_confidence(
            Layer::ExternalApis,
            input.data_arc(),
            input.confidence * 0.80, // Some loss in translation
        );

        output.set_metadata("transform", "intuition_to_external");
        output.set_metadata("hypothesis_type", "intuitive");
        output.add_upstream(&input.id);

        Ok(output)
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::ExternalApis {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::ExternalApis,
                feedback.layer
            )));
        }

        // External knowledge feeds and grounds intuition
        let grounding_factor = if feedback.get_metadata("verified").is_some() {
            1.15 // Verified external data strongly grounds intuition
        } else {
            1.0
        };

        let mut refined = LayerState::with_confidence(
            Layer::GaiaConsciousness,
            feedback.data_arc(),
            feedback.confidence * 0.88 * grounding_factor,
        );

        refined.set_metadata("refinement", "externally_grounded_intuition");
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

        for _ in 0..max_iterations.min(6) {
            // Grounded intuition develops
            let grounding_resonance = (up_conf * down_conf).sqrt();
            let grounding_boost = 1.0 + (grounding_resonance * 0.09);

            up_conf *= grounding_boost;
            down_conf *= grounding_boost;
            total_factor *= grounding_boost;

            if grounding_boost < 1.005 {
                break;
            }
        }

        Ok(AmplificationResult {
            up_state: LayerState::with_confidence(up.layer, up.data_arc(), up_conf),
            down_state: LayerState::with_confidence(down.layer, down.data_arc(), down_conf),
            combined_confidence: up_conf * down_conf,
            amplification_factor: total_factor,
            iterations: max_iterations.min(6),
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
    fn test_bridge_creation() {
        let bridge = ConsciousnessExternalBridge::new();
        assert_eq!(bridge.source_layer(), Layer::GaiaConsciousness);
        assert_eq!(bridge.target_layer(), Layer::ExternalApis);
    }

    #[test]
    fn test_forward() {
        let bridge = ConsciousnessExternalBridge::new();
        let input = LayerState::with_confidence(
            Layer::GaiaConsciousness,
            "intuitive hypothesis".to_string(),
            0.9,
        );

        let result = bridge.forward(&input).unwrap();
        assert_eq!(result.layer, Layer::ExternalApis);
        assert_eq!(result.get_metadata("hypothesis_type"), Some("intuitive"));
    }

    #[test]
    fn test_backward_with_verification() {
        let bridge = ConsciousnessExternalBridge::new();
        let mut feedback =
            LayerState::with_confidence(Layer::ExternalApis, "verified data".to_string(), 0.85);
        feedback.set_metadata("verified", "true");

        let result = bridge.backward(&feedback).unwrap();
        assert_eq!(result.layer, Layer::GaiaConsciousness);
        // Should get grounding boost
        assert!(result.confidence > 0.85);
    }
}
