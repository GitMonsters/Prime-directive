//! Domain Generalization Framework
//!
//! This module enables the compounding integration pattern to be applied
//! to domains beyond AI mimicry, including medical AI, climate modeling,
//! and financial systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::layer::{Layer, LayerState};

/// Domain-specific configuration for generalized layer systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    /// Name of the domain.
    pub name: String,
    /// Description of the domain application.
    pub description: String,
    /// Domain-specific layers.
    pub layers: Vec<DomainLayer>,
    /// Domain-specific bridges.
    pub bridges: Vec<DomainBridgeConfig>,
    /// Domain-specific amplification settings.
    pub amplification: AmplificationConfig,
}

/// A domain-specific layer definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainLayer {
    /// Layer identifier.
    pub id: String,
    /// Layer name.
    pub name: String,
    /// Maps to base layer.
    pub base_layer: Layer,
    /// Domain-specific function.
    pub function: String,
    /// Confidence threshold.
    pub min_confidence: f32,
}

/// Domain-specific bridge configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBridgeConfig {
    /// Source layer ID.
    pub source: String,
    /// Target layer ID.
    pub target: String,
    /// Base resonance value.
    pub resonance: f32,
    /// Transfer function type.
    pub transfer_type: TransferType,
}

/// Types of transfer functions between layers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransferType {
    /// Direct transfer with scaling.
    Linear,
    /// Sigmoid-based transfer.
    Sigmoid,
    /// Exponential amplification.
    Exponential,
    /// Domain-specific custom transfer.
    Custom,
}

/// Amplification configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplificationConfig {
    /// Global amplification factor.
    pub global_factor: f32,
    /// Maximum iterations.
    pub max_iterations: u32,
    /// Convergence threshold.
    pub convergence_threshold: f32,
}

impl Default for AmplificationConfig {
    fn default() -> Self {
        Self {
            global_factor: 1.1,
            max_iterations: 10,
            convergence_threshold: 0.01,
        }
    }
}

/// Factory for creating domain-specific configurations.
pub struct DomainFactory;

impl DomainFactory {
    /// Create a medical AI configuration.
    pub fn medical_ai() -> DomainConfig {
        DomainConfig {
            name: "MedicalAI".to_string(),
            description: "Medical diagnosis and treatment recommendation system".to_string(),
            layers: vec![
                DomainLayer {
                    id: "symptoms".to_string(),
                    name: "Symptom Analysis".to_string(),
                    base_layer: Layer::BasePhysics,
                    function: "Pattern matching on symptom presentations".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "diagnostics".to_string(),
                    name: "Diagnostic Reasoning".to_string(),
                    base_layer: Layer::ExtendedPhysics,
                    function: "Differential diagnosis generation".to_string(),
                    min_confidence: 0.8,
                },
                DomainLayer {
                    id: "pathways".to_string(),
                    name: "Care Pathways".to_string(),
                    base_layer: Layer::CrossDomain,
                    function: "Treatment pathway selection".to_string(),
                    min_confidence: 0.75,
                },
                DomainLayer {
                    id: "intuition".to_string(),
                    name: "Clinical Intuition".to_string(),
                    base_layer: Layer::GaiaConsciousness,
                    function: "Pattern recognition from experience".to_string(),
                    min_confidence: 0.6,
                },
                DomainLayer {
                    id: "communication".to_string(),
                    name: "Patient Communication".to_string(),
                    base_layer: Layer::MultilingualProcessing,
                    function: "Clear explanation of findings".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "consensus".to_string(),
                    name: "Multi-Specialist Consensus".to_string(),
                    base_layer: Layer::CollaborativeLearning,
                    function: "Integration of specialist opinions".to_string(),
                    min_confidence: 0.85,
                },
                DomainLayer {
                    id: "validation".to_string(),
                    name: "Evidence Validation".to_string(),
                    base_layer: Layer::ExternalApis,
                    function: "Clinical trial and guideline validation".to_string(),
                    min_confidence: 0.9,
                },
            ],
            bridges: vec![
                DomainBridgeConfig {
                    source: "symptoms".to_string(),
                    target: "diagnostics".to_string(),
                    resonance: 0.9,
                    transfer_type: TransferType::Linear,
                },
                DomainBridgeConfig {
                    source: "diagnostics".to_string(),
                    target: "pathways".to_string(),
                    resonance: 0.85,
                    transfer_type: TransferType::Sigmoid,
                },
                DomainBridgeConfig {
                    source: "intuition".to_string(),
                    target: "diagnostics".to_string(),
                    resonance: 0.7,
                    transfer_type: TransferType::Custom,
                },
            ],
            amplification: AmplificationConfig {
                global_factor: 1.15,
                max_iterations: 5,
                convergence_threshold: 0.02,
            },
        }
    }

    /// Create a climate modeling configuration.
    pub fn climate_modeling() -> DomainConfig {
        DomainConfig {
            name: "ClimateModeling".to_string(),
            description: "Multi-scale climate prediction and analysis".to_string(),
            layers: vec![
                DomainLayer {
                    id: "physics".to_string(),
                    name: "Atmospheric Physics".to_string(),
                    base_layer: Layer::BasePhysics,
                    function: "Thermodynamic and fluid dynamics".to_string(),
                    min_confidence: 0.8,
                },
                DomainLayer {
                    id: "oceans".to_string(),
                    name: "Ocean Dynamics".to_string(),
                    base_layer: Layer::ExtendedPhysics,
                    function: "Ocean circulation and heat transport".to_string(),
                    min_confidence: 0.75,
                },
                DomainLayer {
                    id: "coupling".to_string(),
                    name: "Earth System Coupling".to_string(),
                    base_layer: Layer::CrossDomain,
                    function: "Atmosphere-ocean-land interactions".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "teleconnections".to_string(),
                    name: "Teleconnection Patterns".to_string(),
                    base_layer: Layer::GaiaConsciousness,
                    function: "Long-range pattern recognition".to_string(),
                    min_confidence: 0.6,
                },
                DomainLayer {
                    id: "regional".to_string(),
                    name: "Regional Downscaling".to_string(),
                    base_layer: Layer::MultilingualProcessing,
                    function: "Global to local translation".to_string(),
                    min_confidence: 0.65,
                },
                DomainLayer {
                    id: "ensemble".to_string(),
                    name: "Ensemble Integration".to_string(),
                    base_layer: Layer::CollaborativeLearning,
                    function: "Multi-model ensemble synthesis".to_string(),
                    min_confidence: 0.8,
                },
                DomainLayer {
                    id: "observations".to_string(),
                    name: "Observational Validation".to_string(),
                    base_layer: Layer::ExternalApis,
                    function: "Satellite and station data integration".to_string(),
                    min_confidence: 0.85,
                },
            ],
            bridges: vec![
                DomainBridgeConfig {
                    source: "physics".to_string(),
                    target: "oceans".to_string(),
                    resonance: 0.9,
                    transfer_type: TransferType::Linear,
                },
                DomainBridgeConfig {
                    source: "coupling".to_string(),
                    target: "teleconnections".to_string(),
                    resonance: 0.75,
                    transfer_type: TransferType::Exponential,
                },
                DomainBridgeConfig {
                    source: "ensemble".to_string(),
                    target: "observations".to_string(),
                    resonance: 0.85,
                    transfer_type: TransferType::Sigmoid,
                },
            ],
            amplification: AmplificationConfig {
                global_factor: 1.08,
                max_iterations: 15,
                convergence_threshold: 0.005,
            },
        }
    }

    /// Create a financial systems configuration.
    pub fn financial_systems() -> DomainConfig {
        DomainConfig {
            name: "FinancialSystems".to_string(),
            description: "Multi-factor financial analysis and risk assessment".to_string(),
            layers: vec![
                DomainLayer {
                    id: "fundamentals".to_string(),
                    name: "Fundamental Analysis".to_string(),
                    base_layer: Layer::BasePhysics,
                    function: "Balance sheet and income analysis".to_string(),
                    min_confidence: 0.8,
                },
                DomainLayer {
                    id: "technicals".to_string(),
                    name: "Technical Analysis".to_string(),
                    base_layer: Layer::ExtendedPhysics,
                    function: "Price pattern and momentum analysis".to_string(),
                    min_confidence: 0.65,
                },
                DomainLayer {
                    id: "correlations".to_string(),
                    name: "Cross-Asset Correlations".to_string(),
                    base_layer: Layer::CrossDomain,
                    function: "Inter-market relationship analysis".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "sentiment".to_string(),
                    name: "Market Sentiment".to_string(),
                    base_layer: Layer::GaiaConsciousness,
                    function: "Intuitive market psychology reading".to_string(),
                    min_confidence: 0.55,
                },
                DomainLayer {
                    id: "narrative".to_string(),
                    name: "Narrative Analysis".to_string(),
                    base_layer: Layer::MultilingualProcessing,
                    function: "News and social media interpretation".to_string(),
                    min_confidence: 0.6,
                },
                DomainLayer {
                    id: "consensus".to_string(),
                    name: "Analyst Consensus".to_string(),
                    base_layer: Layer::CollaborativeLearning,
                    function: "Aggregated analyst opinions".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "realtime".to_string(),
                    name: "Real-Time Data".to_string(),
                    base_layer: Layer::ExternalApis,
                    function: "Live market data and news feeds".to_string(),
                    min_confidence: 0.95,
                },
            ],
            bridges: vec![
                DomainBridgeConfig {
                    source: "fundamentals".to_string(),
                    target: "technicals".to_string(),
                    resonance: 0.6,
                    transfer_type: TransferType::Sigmoid,
                },
                DomainBridgeConfig {
                    source: "sentiment".to_string(),
                    target: "technicals".to_string(),
                    resonance: 0.7,
                    transfer_type: TransferType::Custom,
                },
                DomainBridgeConfig {
                    source: "narrative".to_string(),
                    target: "sentiment".to_string(),
                    resonance: 0.8,
                    transfer_type: TransferType::Linear,
                },
            ],
            amplification: AmplificationConfig {
                global_factor: 1.05,
                max_iterations: 8,
                convergence_threshold: 0.015,
            },
        }
    }

    /// Create a neuroscience configuration.
    pub fn neuroscience() -> DomainConfig {
        DomainConfig {
            name: "Neuroscience".to_string(),
            description: "Multi-scale brain analysis from neurons to behavior".to_string(),
            layers: vec![
                DomainLayer {
                    id: "cellular".to_string(),
                    name: "Cellular/Molecular".to_string(),
                    base_layer: Layer::BasePhysics,
                    function: "Neuron and synapse dynamics".to_string(),
                    min_confidence: 0.75,
                },
                DomainLayer {
                    id: "circuits".to_string(),
                    name: "Neural Circuits".to_string(),
                    base_layer: Layer::ExtendedPhysics,
                    function: "Circuit-level computation".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "systems".to_string(),
                    name: "Brain Systems".to_string(),
                    base_layer: Layer::CrossDomain,
                    function: "Large-scale network dynamics".to_string(),
                    min_confidence: 0.65,
                },
                DomainLayer {
                    id: "cognition".to_string(),
                    name: "Cognitive Processes".to_string(),
                    base_layer: Layer::GaiaConsciousness,
                    function: "Higher cognitive function modeling".to_string(),
                    min_confidence: 0.6,
                },
                DomainLayer {
                    id: "behavior".to_string(),
                    name: "Behavioral Output".to_string(),
                    base_layer: Layer::MultilingualProcessing,
                    function: "Behavior generation and interpretation".to_string(),
                    min_confidence: 0.7,
                },
                DomainLayer {
                    id: "social".to_string(),
                    name: "Social Cognition".to_string(),
                    base_layer: Layer::CollaborativeLearning,
                    function: "Social interaction modeling".to_string(),
                    min_confidence: 0.55,
                },
                DomainLayer {
                    id: "imaging".to_string(),
                    name: "Neuroimaging Data".to_string(),
                    base_layer: Layer::ExternalApis,
                    function: "fMRI, EEG, and other measurements".to_string(),
                    min_confidence: 0.8,
                },
            ],
            bridges: vec![
                DomainBridgeConfig {
                    source: "cellular".to_string(),
                    target: "circuits".to_string(),
                    resonance: 0.85,
                    transfer_type: TransferType::Exponential,
                },
                DomainBridgeConfig {
                    source: "circuits".to_string(),
                    target: "systems".to_string(),
                    resonance: 0.8,
                    transfer_type: TransferType::Linear,
                },
                DomainBridgeConfig {
                    source: "cognition".to_string(),
                    target: "behavior".to_string(),
                    resonance: 0.75,
                    transfer_type: TransferType::Sigmoid,
                },
            ],
            amplification: AmplificationConfig {
                global_factor: 1.12,
                max_iterations: 12,
                convergence_threshold: 0.008,
            },
        }
    }

    /// List all available domain templates.
    pub fn available_domains() -> Vec<&'static str> {
        vec![
            "medical_ai",
            "climate_modeling",
            "financial_systems",
            "neuroscience",
        ]
    }

    /// Create a domain config by name.
    pub fn create(name: &str) -> Option<DomainConfig> {
        match name.to_lowercase().as_str() {
            "medical_ai" | "medical" | "healthcare" => Some(Self::medical_ai()),
            "climate_modeling" | "climate" | "weather" => Some(Self::climate_modeling()),
            "financial_systems" | "financial" | "finance" => Some(Self::financial_systems()),
            "neuroscience" | "neuro" | "brain" => Some(Self::neuroscience()),
            _ => None,
        }
    }
}

/// Domain-specific processor that adapts layer system to a domain.
#[derive(Debug)]
pub struct DomainProcessor {
    /// Domain configuration.
    config: DomainConfig,
    /// Layer ID to base layer mapping.
    layer_mapping: HashMap<String, Layer>,
}

impl DomainProcessor {
    /// Create a new domain processor.
    pub fn new(config: DomainConfig) -> Self {
        let mut layer_mapping = HashMap::new();
        for layer in &config.layers {
            layer_mapping.insert(layer.id.clone(), layer.base_layer);
        }

        Self {
            config,
            layer_mapping,
        }
    }

    /// Get the domain configuration.
    pub fn config(&self) -> &DomainConfig {
        &self.config
    }

    /// Map a domain layer ID to its base layer.
    pub fn get_base_layer(&self, layer_id: &str) -> Option<Layer> {
        self.layer_mapping.get(layer_id).copied()
    }

    /// Create a layer state for this domain.
    pub fn create_state<T: std::any::Any + Send + Sync + 'static>(
        &self,
        layer_id: &str,
        data: T,
    ) -> Option<LayerState> {
        self.get_base_layer(layer_id)
            .map(|layer| LayerState::new(layer, data))
    }

    /// Get summary of domain.
    pub fn summary(&self) -> String {
        let mut lines = vec![
            format!("=== {} ===", self.config.name),
            self.config.description.clone(),
            format!("Layers: {}", self.config.layers.len()),
            format!("Bridges: {}", self.config.bridges.len()),
        ];

        lines.push("\nLayer Mapping:".to_string());
        for layer in &self.config.layers {
            lines.push(format!(
                "  {} -> {} (min conf: {:.0}%)",
                layer.name,
                layer.base_layer.name(),
                layer.min_confidence * 100.0
            ));
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_factory() {
        let medical = DomainFactory::medical_ai();
        assert_eq!(medical.name, "MedicalAI");
        assert_eq!(medical.layers.len(), 7);

        let climate = DomainFactory::climate_modeling();
        assert_eq!(climate.name, "ClimateModeling");

        let finance = DomainFactory::financial_systems();
        assert_eq!(finance.name, "FinancialSystems");

        let neuro = DomainFactory::neuroscience();
        assert_eq!(neuro.name, "Neuroscience");
    }

    #[test]
    fn test_domain_create() {
        assert!(DomainFactory::create("medical").is_some());
        assert!(DomainFactory::create("finance").is_some());
        assert!(DomainFactory::create("unknown").is_none());
    }

    #[test]
    fn test_domain_processor() {
        let config = DomainFactory::medical_ai();
        let processor = DomainProcessor::new(config);

        assert!(processor.get_base_layer("symptoms").is_some());
        assert!(processor.get_base_layer("diagnostics").is_some());
        assert!(processor.get_base_layer("unknown").is_none());
    }

    #[test]
    fn test_create_domain_state() {
        let config = DomainFactory::financial_systems();
        let processor = DomainProcessor::new(config);

        let state = processor.create_state("fundamentals", "balance sheet data".to_string());
        assert!(state.is_some());
        assert_eq!(state.unwrap().layer, Layer::BasePhysics);
    }
}
