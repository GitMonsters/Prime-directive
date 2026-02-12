//! RNA Editing Bridge
//!
//! Connects RustyWorm to OCTO's RNA Editing Layer via PyO3.
//! Provides intelligent routing decisions based on input characteristics.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Once;

static INIT: Once = Once::new();
static mut BRIDGE_INITIALIZED: bool = false;

/// Configuration for OCTO bridge
#[derive(Debug, Clone)]
pub struct OctoConfig {
    /// Minimum confidence for System 1 routing (default: 0.65)
    pub system1_threshold: f32,
    /// Maximum temperature for System 1 routing (default: 1.8)
    pub temperature_threshold: f32,
    /// Hidden dimension for embeddings (default: 256)
    pub hidden_dim: usize,
}

impl Default for OctoConfig {
    fn default() -> Self {
        Self {
            system1_threshold: 0.65,
            temperature_threshold: 1.8,
            hidden_dim: 256,
        }
    }
}

/// Result from RNA editing analysis
#[derive(Debug, Clone)]
pub struct RNAEditingResult {
    /// Temperature: uncertainty measure (0.1 - 5.0)
    /// High = uncertain, Low = confident
    pub temperature: f32,

    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,

    /// Head gates for each attention head [num_heads]
    /// Values 0-1, controls which "aspects" of persona are active
    pub head_gates: Vec<f32>,

    /// Pathway weights [num_pathways] summing to 1.0
    /// Index 0 = perception, 1 = reasoning, 2 = action
    pub pathway_weights: Vec<f32>,
}

impl RNAEditingResult {
    /// Get the primary pathway index (highest weight)
    pub fn primary_pathway(&self) -> usize {
        self.pathway_weights
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get pathway name from index
    pub fn pathway_name(index: usize) -> &'static str {
        match index {
            0 => "perception",
            1 => "reasoning",
            2 => "action",
            _ => "unknown",
        }
    }
}

/// Routing decision from RNA analysis
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingDecision {
    /// Fast path: high confidence, use cached patterns
    System1 {
        head_gates: Vec<f32>,
        primary_pathway: usize,
    },
    /// Deep path: low confidence, use deliberate reasoning
    System2 {
        temperature: f32,
        primary_pathway: usize,
    },
}

impl RoutingDecision {
    /// Check if this is System 1 routing
    pub fn is_system1(&self) -> bool {
        matches!(self, RoutingDecision::System1 { .. })
    }

    /// Get the primary pathway
    pub fn primary_pathway(&self) -> usize {
        match self {
            RoutingDecision::System1 {
                primary_pathway, ..
            } => *primary_pathway,
            RoutingDecision::System2 {
                primary_pathway, ..
            } => *primary_pathway,
        }
    }
}

/// Bridge to OCTO's RNA Editing Layer
#[derive(Debug)]
pub struct OctoRNABridge {
    config: OctoConfig,
    initialized: bool,
}

impl OctoRNABridge {
    /// Create a new RNA bridge with default config
    pub fn new() -> Result<Self, String> {
        Self::with_config(OctoConfig::default())
    }

    /// Create a new RNA bridge with custom config
    pub fn with_config(config: OctoConfig) -> Result<Self, String> {
        Self::initialize_python()?;
        Ok(Self {
            config,
            initialized: true,
        })
    }

    /// Initialize Python interpreter and import bridge module
    fn initialize_python() -> Result<(), String> {
        let mut init_error: Option<String> = None;

        INIT.call_once(|| {
            let result = Python::with_gil(|py| {
                // Add OCTO path to Python path
                let sys = py
                    .import("sys")
                    .map_err(|e| format!("Failed to import sys: {}", e))?;
                let path = sys
                    .getattr("path")
                    .map_err(|e| format!("Failed to get sys.path: {}", e))?;
                path.call_method1("insert", (0, "/home/worm/octotetrahedral-agi"))
                    .map_err(|e| format!("Failed to insert path: {}", e))?;

                // Import and verify bridge module
                py.import("rustyworm_bridge")
                    .map_err(|e| format!("Failed to import rustyworm_bridge: {}", e))?;

                unsafe {
                    BRIDGE_INITIALIZED = true;
                }
                Ok::<(), String>(())
            });

            if let Err(e) = result {
                init_error = Some(e);
            }
        });

        if let Some(e) = init_error {
            return Err(e);
        }

        if unsafe { BRIDGE_INITIALIZED } {
            Ok(())
        } else {
            Err("Failed to initialize OCTO bridge".to_string())
        }
    }

    /// Check if bridge is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized && unsafe { BRIDGE_INITIALIZED }
    }

    /// Get current config
    pub fn config(&self) -> &OctoConfig {
        &self.config
    }

    /// Update config
    pub fn set_config(&mut self, config: OctoConfig) {
        self.config = config;
    }

    /// Set System 1 confidence threshold
    pub fn set_system1_threshold(&mut self, threshold: f32) {
        self.config.system1_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set temperature threshold
    pub fn set_temperature_threshold(&mut self, threshold: f32) {
        self.config.temperature_threshold = threshold.clamp(0.1, 5.0);
    }

    /// Analyze input and get RNA editing parameters
    pub fn analyze(&self, embedding: &[f32]) -> Result<RNAEditingResult, String> {
        if !self.is_initialized() {
            return Err("OCTO bridge not initialized".to_string());
        }

        Python::with_gil(|py| {
            let bridge = py
                .import("rustyworm_bridge")
                .map_err(|e| format!("Import error: {}", e))?;

            let result = bridge
                .call_method1("analyze", (embedding.to_vec(),))
                .map_err(|e| format!("Call error: {}", e))?;

            let dict: &PyDict = result
                .downcast()
                .map_err(|e| format!("Type error: {}", e))?;

            Ok(RNAEditingResult {
                temperature: dict
                    .get_item("temperature")
                    .map_err(|e| format!("Key error: {}", e))?
                    .ok_or("Missing temperature")?
                    .extract()
                    .map_err(|e| format!("Extract error: {}", e))?,
                confidence: dict
                    .get_item("confidence")
                    .map_err(|e| format!("Key error: {}", e))?
                    .ok_or("Missing confidence")?
                    .extract()
                    .map_err(|e| format!("Extract error: {}", e))?,
                head_gates: dict
                    .get_item("head_gates")
                    .map_err(|e| format!("Key error: {}", e))?
                    .ok_or("Missing head_gates")?
                    .extract()
                    .map_err(|e| format!("Extract error: {}", e))?,
                pathway_weights: dict
                    .get_item("pathway_weights")
                    .map_err(|e| format!("Key error: {}", e))?
                    .ok_or("Missing pathway_weights")?
                    .extract()
                    .map_err(|e| format!("Extract error: {}", e))?,
            })
        })
    }

    /// Get routing decision based on RNA analysis
    pub fn get_routing(&self, embedding: &[f32]) -> Result<RoutingDecision, String> {
        let rna = self.analyze(embedding)?;

        let primary_pathway = rna.primary_pathway();

        if rna.confidence > self.config.system1_threshold
            && rna.temperature < self.config.temperature_threshold
        {
            Ok(RoutingDecision::System1 {
                head_gates: rna.head_gates,
                primary_pathway,
            })
        } else {
            Ok(RoutingDecision::System2 {
                temperature: rna.temperature,
                primary_pathway,
            })
        }
    }

    /// Format RNA result for display
    pub fn format_stats(&self, rna: &RNAEditingResult) -> String {
        let primary = rna.primary_pathway();
        let route = if rna.confidence > self.config.system1_threshold
            && rna.temperature < self.config.temperature_threshold
        {
            "System 1 (Fast)"
        } else {
            "System 2 (Deep)"
        };

        format!(
            "OCTO RNA Analysis:\n\
             ├─ Temperature: {:.2} ({})\n\
             ├─ Confidence: {:.1}%\n\
             ├─ Route: {}\n\
             ├─ Primary Pathway: {} ({:.1}%)\n\
             ├─ Head Gates: [{}]\n\
             └─ Config: S1 threshold={:.2}, temp threshold={:.1}",
            rna.temperature,
            if rna.temperature > 2.0 {
                "high uncertainty"
            } else if rna.temperature > 1.0 {
                "moderate"
            } else {
                "confident"
            },
            rna.confidence * 100.0,
            route,
            RNAEditingResult::pathway_name(primary),
            rna.pathway_weights[primary] * 100.0,
            rna.head_gates
                .iter()
                .map(|g| format!("{:.2}", g))
                .collect::<Vec<_>>()
                .join(", "),
            self.config.system1_threshold,
            self.config.temperature_threshold
        )
    }

    /// Format config for display
    pub fn format_config(&self) -> String {
        format!(
            "OCTO Configuration:\n\
             ├─ System 1 Threshold: {:.2} (confidence must exceed)\n\
             ├─ Temperature Threshold: {:.1} (temp must be below)\n\
             └─ Hidden Dimension: {}",
            self.config.system1_threshold,
            self.config.temperature_threshold,
            self.config.hidden_dim
        )
    }
}

impl Default for OctoRNABridge {
    fn default() -> Self {
        Self::new().expect("Failed to create OctoRNABridge")
    }
}

impl Clone for OctoRNABridge {
    fn clone(&self) -> Self {
        // Create a new bridge with the same config
        // The Python state is shared globally via Once, so cloning just copies config
        Self {
            config: self.config.clone(),
            initialized: self.initialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octo_config_default() {
        let config = OctoConfig::default();
        assert_eq!(config.system1_threshold, 0.65);
        assert_eq!(config.temperature_threshold, 1.8);
        assert_eq!(config.hidden_dim, 256);
    }

    #[test]
    fn test_rna_result_primary_pathway() {
        let result = RNAEditingResult {
            temperature: 1.0,
            confidence: 0.8,
            head_gates: vec![0.5; 8],
            pathway_weights: vec![0.2, 0.6, 0.2], // reasoning is highest
        };
        assert_eq!(result.primary_pathway(), 1);
        assert_eq!(RNAEditingResult::pathway_name(1), "reasoning");
    }

    #[test]
    fn test_routing_decision_is_system1() {
        let s1 = RoutingDecision::System1 {
            head_gates: vec![1.0; 8],
            primary_pathway: 0,
        };
        let s2 = RoutingDecision::System2 {
            temperature: 2.0,
            primary_pathway: 1,
        };
        assert!(s1.is_system1());
        assert!(!s2.is_system1());
    }
}
