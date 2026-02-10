//! Collaborative Learning to External APIs bridge (L6↔L7).
//!
//! This bridge connects multi-agent collaborative learning with
//! real-time external API validation and feedback.

use super::super::bridge::{AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult};
use super::super::layer::{Layer, LayerState};

/// Bridge between Collaborative Learning (L6) and External APIs (L7).
///
/// This bridge enables:
/// - Forward: Collective hypotheses are validated by external sources
/// - Backward: External validation refines collective knowledge
/// - Amplification: Externally-validated collective intelligence
pub struct CollaborativeExternalBridge {
    /// Base resonance for this bridge.
    base_resonance: f32,
}

impl CollaborativeExternalBridge {
    pub fn new() -> Self {
        Self {
            base_resonance: 0.78,
        }
    }
}

impl Default for CollaborativeExternalBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for CollaborativeExternalBridge {
    fn name(&self) -> &str {
        "Collaborative-External"
    }

    fn source_layer(&self) -> Layer {
        Layer::CollaborativeLearning
    }

    fn target_layer(&self) -> Layer {
        Layer::ExternalApis
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::CollaborativeLearning {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::CollaborativeLearning,
                input.layer
            )));
        }

        // Collective hypotheses sent for external validation
        let mut output = LayerState::with_confidence(
            Layer::ExternalApis,
            input.data_arc(),
            input.confidence * 0.85,
        );

        output.set_metadata("transform", "collective_to_external");
        output.set_metadata("validation_pending", "true");
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

        // External validation refines collective knowledge
        // External validation typically provides high-quality signal
        let validation_boost = if feedback.get_metadata("validated").is_some() {
            1.1
        } else {
            1.0
        };

        let mut refined = LayerState::with_confidence(
            Layer::CollaborativeLearning,
            feedback.data_arc(),
            feedback.confidence * 0.95 * validation_boost,
        );

        refined.set_metadata("refinement", "externally_validated");
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
            // External validation amplifies collective confidence
            let validation_resonance = (up_conf * down_conf).sqrt();
            let external_boost = 1.0 + (validation_resonance * 0.08);

            up_conf *= external_boost;
            down_conf *= external_boost;
            total_factor *= external_boost;

            if external_boost < 1.005 {
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
        let bridge = CollaborativeExternalBridge::new();
        assert_eq!(bridge.source_layer(), Layer::CollaborativeLearning);
        assert_eq!(bridge.target_layer(), Layer::ExternalApis);
    }

    #[test]
    fn test_forward() {
        let bridge = CollaborativeExternalBridge::new();
        let input = LayerState::with_confidence(
            Layer::CollaborativeLearning,
            "collective hypothesis".to_string(),
            0.8,
        );

        let result = bridge.forward(&input).unwrap();
        assert_eq!(result.layer, Layer::ExternalApis);
        assert_eq!(result.get_metadata("validation_pending"), Some("true"));
    }

    #[test]
    fn test_backward_with_validation() {
        let bridge = CollaborativeExternalBridge::new();
        let mut feedback =
            LayerState::with_confidence(Layer::ExternalApis, "validated result".to_string(), 0.9);
        feedback.set_metadata("validated", "true");

        let result = bridge.backward(&feedback).unwrap();
        assert_eq!(result.layer, Layer::CollaborativeLearning);
        // Should get validation boost
        assert!(result.confidence > 0.9);
    }
}
