//! Multilingual Processing to Collaborative Learning bridge (L5↔L6).
//!
//! This bridge connects linguistic processing with multi-agent learning,
//! enabling cross-lingual collaborative knowledge transfer.

use super::super::bridge::{AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult};
use super::super::layer::{Layer, LayerState};

/// Bridge between Multilingual Processing (L5) and Collaborative Learning (L6).
///
/// This bridge enables:
/// - Forward: Linguistic perspectives inform collective learning
/// - Backward: Collective knowledge enriches linguistic understanding
/// - Amplification: Multi-lingual collective intelligence emerges
pub struct LanguageCollaborativeBridge {
    /// Base resonance for this bridge.
    base_resonance: f32,
}

impl LanguageCollaborativeBridge {
    pub fn new() -> Self {
        Self {
            base_resonance: 0.82,
        }
    }
}

impl Default for LanguageCollaborativeBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for LanguageCollaborativeBridge {
    fn name(&self) -> &str {
        "Language-Collaborative"
    }

    fn source_layer(&self) -> Layer {
        Layer::MultilingualProcessing
    }

    fn target_layer(&self) -> Layer {
        Layer::CollaborativeLearning
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::MultilingualProcessing {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::MultilingualProcessing,
                input.layer
            )));
        }

        // Linguistic perspectives inform collective learning
        let mut output = LayerState::with_confidence(
            Layer::CollaborativeLearning,
            input.data_arc(),
            input.confidence * 0.90,
        );

        output.set_metadata("transform", "perspective_to_collective");
        output.set_metadata("source_bridge", self.name());
        output.add_upstream(&input.id);

        Ok(output)
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::CollaborativeLearning {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::CollaborativeLearning,
                feedback.layer
            )));
        }

        // Collective knowledge enriches linguistic understanding
        let mut refined = LayerState::with_confidence(
            Layer::MultilingualProcessing,
            feedback.data_arc(),
            feedback.confidence * 0.88,
        );

        refined.set_metadata("refinement", "collective_linguistic");
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
            // Multi-lingual collective intelligence emerges
            let collective_resonance = (up_conf * down_conf).sqrt();
            let diversity_bonus = 1.0 + (collective_resonance * 0.10);

            up_conf *= diversity_bonus;
            down_conf *= diversity_bonus;
            total_factor *= diversity_bonus;

            if diversity_bonus < 1.01 {
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
    fn test_bridge_creation() {
        let bridge = LanguageCollaborativeBridge::new();
        assert_eq!(bridge.source_layer(), Layer::MultilingualProcessing);
        assert_eq!(bridge.target_layer(), Layer::CollaborativeLearning);
    }

    #[test]
    fn test_forward() {
        let bridge = LanguageCollaborativeBridge::new();
        let input = LayerState::with_confidence(
            Layer::MultilingualProcessing,
            "multi-lingual input".to_string(),
            0.85,
        );

        let result = bridge.forward(&input).unwrap();
        assert_eq!(result.layer, Layer::CollaborativeLearning);
    }

    #[test]
    fn test_backward() {
        let bridge = LanguageCollaborativeBridge::new();
        let feedback = LayerState::with_confidence(
            Layer::CollaborativeLearning,
            "collective knowledge".to_string(),
            0.80,
        );

        let result = bridge.backward(&feedback).unwrap();
        assert_eq!(result.layer, Layer::MultilingualProcessing);
    }
}
