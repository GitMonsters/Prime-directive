//! Compounding Metrics System
//!
//! This module provides metrics for measuring multiplicative vs additive value
//! in the layer integration system. It quantifies the compounding effect of
//! cross-layer amplification.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::layer::Layer;
use super::stack::StackProcessResult;

/// Metrics for measuring compounding effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompoundingMetrics {
    /// Total samples analyzed.
    pub total_samples: u64,
    /// Running metrics.
    metrics: MetricAccumulator,
    /// Per-layer metrics.
    layer_metrics: HashMap<Layer, LayerMetrics>,
    /// Per-bridge metrics.
    bridge_metrics: HashMap<String, BridgeMetrics>,
    /// Historical compounding factors.
    compounding_history: Vec<f32>,
}

/// Accumulated metric values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct MetricAccumulator {
    /// Sum of multiplicative gains.
    multiplicative_sum: f64,
    /// Sum of additive gains.
    additive_sum: f64,
    /// Sum of compounding factors.
    compounding_factor_sum: f64,
    /// Maximum compounding factor observed.
    max_compounding_factor: f32,
    /// Sum of emergent value (value beyond sum of parts).
    emergent_value_sum: f64,
}

/// Metrics for a single layer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LayerMetrics {
    /// Number of activations.
    pub activations: u64,
    /// Total confidence contribution.
    pub confidence_contribution: f64,
    /// Average amplification received.
    pub avg_amplification: f32,
    /// Peak confidence achieved.
    pub peak_confidence: f32,
}

/// Metrics for a single bridge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BridgeMetrics {
    /// Number of forward traversals.
    pub forward_count: u64,
    /// Number of backward traversals.
    pub backward_count: u64,
    /// Total amplification factor.
    pub total_amplification: f64,
    /// Average resonance.
    pub avg_resonance: f32,
}

/// Result of a compounding analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompoundingAnalysis {
    /// The multiplicative gain (product of layer contributions).
    pub multiplicative_gain: f32,
    /// The additive gain (sum of layer contributions).
    pub additive_gain: f32,
    /// The compounding factor (multiplicative / additive).
    pub compounding_factor: f32,
    /// Emergent value (multiplicative - additive).
    pub emergent_value: f32,
    /// Whether compounding is beneficial (factor > 1.0).
    pub is_beneficial: bool,
    /// Synergy score (0-1, how well layers work together).
    pub synergy_score: f32,
    /// Per-layer analysis.
    pub layer_analysis: HashMap<Layer, LayerAnalysis>,
}

/// Analysis for a single layer's contribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerAnalysis {
    /// Raw confidence from this layer.
    pub raw_confidence: f32,
    /// Confidence after amplification.
    pub amplified_confidence: f32,
    /// Contribution to multiplicative product.
    pub multiplicative_contribution: f32,
    /// Contribution to additive sum.
    pub additive_contribution: f32,
}

impl CompoundingMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        Self {
            total_samples: 0,
            metrics: MetricAccumulator::default(),
            layer_metrics: HashMap::new(),
            bridge_metrics: HashMap::new(),
            compounding_history: Vec::new(),
        }
    }

    /// Analyze a stack processing result.
    pub fn analyze(&mut self, result: &StackProcessResult) -> CompoundingAnalysis {
        self.total_samples += 1;

        // Calculate multiplicative and additive gains
        let confidences: Vec<f32> = result.layer_confidences.values().copied().collect();

        let multiplicative_gain = if confidences.is_empty() {
            1.0
        } else {
            confidences
                .iter()
                .product::<f32>()
                .powf(1.0 / confidences.len() as f32)
        };

        let additive_gain = if confidences.is_empty() {
            0.0
        } else {
            confidences.iter().sum::<f32>() / confidences.len() as f32
        };

        let compounding_factor = if additive_gain > 0.0 {
            multiplicative_gain / additive_gain
        } else {
            1.0
        };

        let emergent_value = multiplicative_gain - additive_gain;
        let is_beneficial = compounding_factor > 1.0;

        // Calculate synergy score
        let synergy_score = self.calculate_synergy(&confidences, result.total_amplification);

        // Update accumulators
        self.metrics.multiplicative_sum += multiplicative_gain as f64;
        self.metrics.additive_sum += additive_gain as f64;
        self.metrics.compounding_factor_sum += compounding_factor as f64;
        self.metrics.emergent_value_sum += emergent_value as f64;

        if compounding_factor > self.metrics.max_compounding_factor {
            self.metrics.max_compounding_factor = compounding_factor;
        }

        self.compounding_history.push(compounding_factor);
        if self.compounding_history.len() > 1000 {
            self.compounding_history.remove(0);
        }

        // Update layer metrics
        let mut layer_analysis = HashMap::new();
        for (layer, &confidence) in &result.layer_confidences {
            let layer_metric = self.layer_metrics.entry(*layer).or_default();
            layer_metric.activations += 1;
            layer_metric.confidence_contribution += confidence as f64;

            if confidence > layer_metric.peak_confidence {
                layer_metric.peak_confidence = confidence;
            }

            // Estimate amplification from raw to final
            let amplified_confidence = confidence * result.total_amplification;

            layer_analysis.insert(
                *layer,
                LayerAnalysis {
                    raw_confidence: confidence,
                    amplified_confidence,
                    multiplicative_contribution: confidence.ln().exp(),
                    additive_contribution: confidence / confidences.len() as f32,
                },
            );
        }

        CompoundingAnalysis {
            multiplicative_gain,
            additive_gain,
            compounding_factor,
            emergent_value,
            is_beneficial,
            synergy_score,
            layer_analysis,
        }
    }

    /// Calculate synergy score based on how well layers work together.
    fn calculate_synergy(&self, confidences: &[f32], amplification: f32) -> f32 {
        if confidences.is_empty() {
            return 0.0;
        }

        // Synergy is high when:
        // 1. Variance is low (layers agree)
        // 2. Amplification is high (layers boost each other)
        // 3. No layer is significantly weaker

        let mean = confidences.iter().sum::<f32>() / confidences.len() as f32;
        let variance =
            confidences.iter().map(|c| (c - mean).powi(2)).sum::<f32>() / confidences.len() as f32;

        let min_conf = confidences.iter().copied().fold(f32::MAX, f32::min);
        let balance_score = min_conf / mean.max(0.001);

        // Combine factors
        let variance_factor = 1.0 / (1.0 + variance * 10.0);
        let amplification_factor = (amplification - 1.0).max(0.0).min(1.0);

        (variance_factor * 0.3 + balance_score * 0.3 + amplification_factor * 0.4).clamp(0.0, 1.0)
    }

    /// Update bridge metrics.
    pub fn record_bridge_activity(
        &mut self,
        bridge_name: &str,
        forward: bool,
        amplification: f32,
        resonance: f32,
    ) {
        let metrics = self
            .bridge_metrics
            .entry(bridge_name.to_string())
            .or_default();

        if forward {
            metrics.forward_count += 1;
        } else {
            metrics.backward_count += 1;
        }

        metrics.total_amplification += amplification as f64;

        let total_ops = metrics.forward_count + metrics.backward_count;
        metrics.avg_resonance =
            (metrics.avg_resonance * (total_ops - 1) as f32 + resonance) / total_ops as f32;
    }

    /// Get average compounding factor.
    pub fn average_compounding_factor(&self) -> f32 {
        if self.total_samples == 0 {
            return 1.0;
        }
        (self.metrics.compounding_factor_sum / self.total_samples as f64) as f32
    }

    /// Get average multiplicative gain.
    pub fn average_multiplicative_gain(&self) -> f32 {
        if self.total_samples == 0 {
            return 1.0;
        }
        (self.metrics.multiplicative_sum / self.total_samples as f64) as f32
    }

    /// Get average additive gain.
    pub fn average_additive_gain(&self) -> f32 {
        if self.total_samples == 0 {
            return 0.0;
        }
        (self.metrics.additive_sum / self.total_samples as f64) as f32
    }

    /// Get total emergent value.
    pub fn total_emergent_value(&self) -> f32 {
        self.metrics.emergent_value_sum as f32
    }

    /// Get maximum compounding factor observed.
    pub fn max_compounding_factor(&self) -> f32 {
        self.metrics.max_compounding_factor
    }

    /// Get layer metrics.
    pub fn layer_metrics(&self) -> &HashMap<Layer, LayerMetrics> {
        &self.layer_metrics
    }

    /// Get bridge metrics.
    pub fn bridge_metrics(&self) -> &HashMap<String, BridgeMetrics> {
        &self.bridge_metrics
    }

    /// Generate a summary report.
    pub fn summary(&self) -> String {
        format!(
            "=== COMPOUNDING METRICS SUMMARY ===\n\
             Total Samples: {}\n\
             \n\
             Multiplicative vs Additive:\n\
             Avg Multiplicative Gain: {:.4}\n\
             Avg Additive Gain: {:.4}\n\
             Avg Compounding Factor: {:.4}x\n\
             Max Compounding Factor: {:.4}x\n\
             Total Emergent Value: {:.4}\n\
             \n\
             Layer Activity:\n{}\n\
             Bridge Activity:\n{}",
            self.total_samples,
            self.average_multiplicative_gain(),
            self.average_additive_gain(),
            self.average_compounding_factor(),
            self.max_compounding_factor(),
            self.total_emergent_value(),
            self.format_layer_metrics(),
            self.format_bridge_metrics(),
        )
    }

    /// Format layer metrics for display.
    fn format_layer_metrics(&self) -> String {
        let mut lines = Vec::new();
        for layer in Layer::all() {
            if let Some(metrics) = self.layer_metrics.get(layer) {
                lines.push(format!(
                    "  {}: {} activations, peak {:.2}",
                    layer.name(),
                    metrics.activations,
                    metrics.peak_confidence
                ));
            }
        }
        if lines.is_empty() {
            "  No layer activity recorded.".to_string()
        } else {
            lines.join("\n")
        }
    }

    /// Format bridge metrics for display.
    fn format_bridge_metrics(&self) -> String {
        let mut lines = Vec::new();
        for (name, metrics) in &self.bridge_metrics {
            lines.push(format!(
                "  {}: {} fwd, {} bwd, avg resonance {:.2}",
                name, metrics.forward_count, metrics.backward_count, metrics.avg_resonance
            ));
        }
        if lines.is_empty() {
            "  No bridge activity recorded.".to_string()
        } else {
            lines.join("\n")
        }
    }

    /// Reset all metrics.
    pub fn reset(&mut self) {
        self.total_samples = 0;
        self.metrics = MetricAccumulator::default();
        self.layer_metrics.clear();
        self.bridge_metrics.clear();
        self.compounding_history.clear();
    }
}

impl Default for CompoundingMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = CompoundingMetrics::new();
        assert_eq!(metrics.total_samples, 0);
        assert_eq!(metrics.average_compounding_factor(), 1.0);
    }

    #[test]
    fn test_compounding_analysis() {
        let mut metrics = CompoundingMetrics::new();

        let mut result = StackProcessResult::empty();
        result.layer_confidences.insert(Layer::BasePhysics, 0.8);
        result.layer_confidences.insert(Layer::ExtendedPhysics, 0.8);
        result
            .layer_confidences
            .insert(Layer::GaiaConsciousness, 0.9);
        result.total_amplification = 1.2;

        let analysis = metrics.analyze(&result);

        assert!(analysis.multiplicative_gain > 0.0);
        assert!(analysis.additive_gain > 0.0);
        assert_eq!(metrics.total_samples, 1);
    }

    #[test]
    fn test_bridge_activity_recording() {
        let mut metrics = CompoundingMetrics::new();

        metrics.record_bridge_activity("test_bridge", true, 1.1, 0.8);
        metrics.record_bridge_activity("test_bridge", false, 1.05, 0.85);

        let bridge = metrics.bridge_metrics().get("test_bridge").unwrap();
        assert_eq!(bridge.forward_count, 1);
        assert_eq!(bridge.backward_count, 1);
    }

    #[test]
    fn test_synergy_calculation() {
        let metrics = CompoundingMetrics::new();

        // High synergy: similar confidences
        let balanced = vec![0.8, 0.82, 0.78, 0.81];
        let synergy = metrics.calculate_synergy(&balanced, 1.5);
        assert!(synergy > 0.5);

        // Low synergy: very different confidences
        let imbalanced = vec![0.9, 0.1, 0.5, 0.3];
        let low_synergy = metrics.calculate_synergy(&imbalanced, 1.0);
        assert!(low_synergy < synergy);
    }
}
