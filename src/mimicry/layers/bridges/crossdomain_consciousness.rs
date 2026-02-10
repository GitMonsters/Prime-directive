//! Cross-Domain to GAIA Consciousness bridge (L3↔L4).
//!
//! This bridge connects emergent cross-domain patterns with
//! intuitive consciousness processing.

use super::super::bridge::{AmplificationResult, BidirectionalBridge, BridgeError, BridgeResult};
use super::super::layer::{Layer, LayerState};

/// Bridge between Cross-Domain (L3) and GAIA Consciousness (L4).
///
/// This bridge enables:
/// - Forward: Emergent patterns inform intuition
/// - Backward: Intuitive insights guide pattern discovery
/// - Amplification: Deep emergent intuition
pub struct CrossDomainConsciousnessBridge {
    /// Base resonance for this bridge.
    base_resonance: f32,
}

impl CrossDomainConsciousnessBridge {
    pub fn new() -> Self {
        Self {
            base_resonance: 0.90,
        }
    }
}

impl Default for CrossDomainConsciousnessBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalBridge for CrossDomainConsciousnessBridge {
    fn name(&self) -> &str {
        "CrossDomain-Consciousness"
    }

    fn source_layer(&self) -> Layer {
        Layer::CrossDomain
    }

    fn target_layer(&self) -> Layer {
        Layer::GaiaConsciousness
    }

    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState> {
        if input.layer != Layer::CrossDomain {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::CrossDomain,
                input.layer
            )));
        }

        // Emergent patterns inform intuitive processing
        let mut output = LayerState::with_confidence(
            Layer::GaiaConsciousness,
            input.data_arc(),
            input.confidence * 0.95, // High transfer rate for this conceptual bridge
        );

        output.set_metadata("transform", "emergence_to_intuition");
        output.set_metadata("source_bridge", self.name());
        output.add_upstream(&input.id);

        Ok(output)
    }

    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState> {
        if feedback.layer != Layer::GaiaConsciousness {
            return Err(BridgeError::InvalidInput(format!(
                "Invalid source layer: expected {:?}, got {:?}",
                Layer::GaiaConsciousness,
                feedback.layer
            )));
        }

        // Intuitive insights guide cross-domain pattern discovery
        let mut refined = LayerState::with_confidence(
            Layer::CrossDomain,
            feedback.data_arc(),
            feedback.confidence * 0.93,
        );

        refined.set_metadata("refinement", "intuition_guided_emergence");
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

        for _ in 0..max_iterations.min(10) {
            // Deep emergent intuition develops through resonance
            let emergent_resonance = (up_conf * down_conf).sqrt();
            let intuition_boost = 1.0 + (emergent_resonance * 0.15);

            up_conf *= intuition_boost;
            down_conf *= intuition_boost;
            total_factor *= intuition_boost;

            if intuition_boost < 1.01 {
                break;
            }
        }

        Ok(AmplificationResult {
            up_state: LayerState::with_confidence(up.layer, up.data_arc(), up_conf),
            down_state: LayerState::with_confidence(down.layer, down.data_arc(), down_conf),
            combined_confidence: up_conf * down_conf,
            amplification_factor: total_factor,
            iterations: max_iterations.min(10),
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
        let bridge = CrossDomainConsciousnessBridge::new();
        assert_eq!(bridge.source_layer(), Layer::CrossDomain);
        assert_eq!(bridge.target_layer(), Layer::GaiaConsciousness);
        assert!(bridge.resonance() >= 0.9);
    }

    #[test]
    fn test_high_transfer_rate() {
        let bridge = CrossDomainConsciousnessBridge::new();
        let input =
            LayerState::with_confidence(Layer::CrossDomain, "emergent pattern".to_string(), 0.9);

        let result = bridge.forward(&input).unwrap();
        // Should maintain high confidence due to conceptual alignment
        assert!(result.confidence > 0.85);
    }

    #[test]
    fn test_strong_amplification() {
        let bridge = CrossDomainConsciousnessBridge::new();

        let up = LayerState::with_confidence(Layer::CrossDomain, (), 0.8);
        let down = LayerState::with_confidence(Layer::GaiaConsciousness, (), 0.8);

        let result = bridge.amplify(&up, &down, 10).unwrap();
        // This bridge should provide strong amplification
        assert!(result.amplification_factor > 1.3);
    }
}
