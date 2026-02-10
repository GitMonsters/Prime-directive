//! Analogical Transfer for cross-domain pattern mapping.
//!
//! Enables insights from one domain to be applied in another domain
//! through structural similarity mapping.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::pattern::{Pattern, PatternId, PatternMatch, PatternMemory};
use super::{GaiaError, GaiaResult};
use crate::mimicry::layers::layer::Domain;

/// Strength of an analogical transfer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransferStrength {
    /// Very strong analogy (>0.8 structural match).
    Strong,
    /// Moderate analogy (0.5-0.8).
    Moderate,
    /// Weak analogy (0.3-0.5).
    Weak,
    /// Tenuous analogy (<0.3).
    Tenuous,
}

impl TransferStrength {
    /// Create from a numeric value.
    pub fn from_value(value: f32) -> Self {
        if value >= 0.8 {
            TransferStrength::Strong
        } else if value >= 0.5 {
            TransferStrength::Moderate
        } else if value >= 0.3 {
            TransferStrength::Weak
        } else {
            TransferStrength::Tenuous
        }
    }

    /// Get the numeric value.
    pub fn value(&self) -> f32 {
        match self {
            TransferStrength::Strong => 0.9,
            TransferStrength::Moderate => 0.65,
            TransferStrength::Weak => 0.4,
            TransferStrength::Tenuous => 0.2,
        }
    }

    /// Check if transfer is reliable enough for use.
    pub fn is_usable(&self) -> bool {
        matches!(self, TransferStrength::Strong | TransferStrength::Moderate)
    }
}

/// A mapping between patterns in different domains.
#[derive(Debug, Clone)]
pub struct AnalogicalMapping {
    /// Source pattern ID.
    pub source_id: PatternId,
    /// Source domain.
    pub source_domain: Domain,
    /// Target pattern ID (the analogous pattern).
    pub target_id: PatternId,
    /// Target domain.
    pub target_domain: Domain,
    /// Mapping strength.
    pub strength: TransferStrength,
    /// Structural correspondence score.
    pub correspondence: f32,
    /// Mapping rationale (for explainability).
    pub rationale: String,
}

impl AnalogicalMapping {
    /// Create a new mapping.
    pub fn new(source: &Pattern, target: &Pattern, correspondence: f32) -> Self {
        Self {
            source_id: source.id().to_string(),
            source_domain: source.domain(),
            target_id: target.id().to_string(),
            target_domain: target.domain(),
            strength: TransferStrength::from_value(correspondence),
            correspondence,
            rationale: format!(
                "Structural similarity between {:?} pattern '{}' and {:?} pattern '{}'",
                source.domain(),
                source.id(),
                target.domain(),
                target.id()
            ),
        }
    }
}

/// A bridge between domains for analogical reasoning.
#[derive(Debug, Clone)]
pub struct DomainBridge {
    /// Source domain.
    pub source: Domain,
    /// Target domain.
    pub target: Domain,
    /// Affinity score (how well these domains map to each other).
    pub affinity: f32,
    /// Known mappings.
    pub mappings: Vec<AnalogicalMapping>,
}

impl DomainBridge {
    /// Create a new domain bridge.
    pub fn new(source: Domain, target: Domain) -> Self {
        // Base affinity depends on domain relationships
        let affinity = Self::compute_base_affinity(source, target);

        Self {
            source,
            target,
            affinity,
            mappings: Vec::new(),
        }
    }

    fn compute_base_affinity(source: Domain, target: Domain) -> f32 {
        // Some domains have natural affinities
        match (source, target) {
            // Physics and consciousness share structure/process duality
            (Domain::Physics, Domain::Consciousness) | (Domain::Consciousness, Domain::Physics) => {
                0.6
            }

            // Language and social domains share communication
            (Domain::Language, Domain::Social) | (Domain::Social, Domain::Language) => 0.7,

            // Emergent relates to everything
            (Domain::Emergent, _) | (_, Domain::Emergent) => 0.5,

            // Same domain (shouldn't happen but handle it)
            _ if source == target => 1.0,

            // Default affinity
            _ => 0.3,
        }
    }

    /// Add a mapping to this bridge.
    pub fn add_mapping(&mut self, mapping: AnalogicalMapping) {
        self.mappings.push(mapping);
    }

    /// Get mappings above a certain strength.
    pub fn strong_mappings(&self) -> Vec<&AnalogicalMapping> {
        self.mappings
            .iter()
            .filter(|m| m.strength.is_usable())
            .collect()
    }
}

/// Result of an analogical transfer.
#[derive(Debug, Clone)]
pub struct TransferResult {
    /// Source pattern used for analogy.
    pub source_pattern: PatternId,
    /// Source domain.
    pub source_domain: Domain,
    /// Target domain.
    pub target_domain: Domain,
    /// Matched patterns in target domain.
    pub target_matches: Vec<PatternMatch>,
    /// Transfer strength.
    pub strength: TransferStrength,
    /// Insight generated from the transfer.
    pub insight: Option<String>,
    /// Transformed feature vector for target domain.
    pub transformed_features: Vec<f32>,
}

/// The analogical transfer engine.
pub struct AnalogicalTransfer {
    /// Domain bridges.
    bridges: HashMap<(Domain, Domain), DomainBridge>,
    /// Transfer history for learning.
    history: Vec<TransferResult>,
    /// Maximum history size.
    max_history: usize,
}

impl AnalogicalTransfer {
    /// Create a new analogical transfer engine.
    pub fn new() -> Self {
        let mut engine = Self {
            bridges: HashMap::new(),
            history: Vec::new(),
            max_history: 1000,
        };

        // Initialize domain bridges
        engine.initialize_bridges();

        engine
    }

    fn initialize_bridges(&mut self) {
        let domains = [
            Domain::Physics,
            Domain::Language,
            Domain::Consciousness,
            Domain::Social,
            Domain::External,
            Domain::Emergent,
        ];

        for &source in &domains {
            for &target in &domains {
                if source != target {
                    self.bridges
                        .insert((source, target), DomainBridge::new(source, target));
                }
            }
        }
    }

    /// Get a domain bridge.
    pub fn bridge(&self, source: Domain, target: Domain) -> Option<&DomainBridge> {
        self.bridges.get(&(source, target))
    }

    /// Get a mutable domain bridge.
    pub fn bridge_mut(&mut self, source: Domain, target: Domain) -> Option<&mut DomainBridge> {
        self.bridges.get_mut(&(source, target))
    }

    /// Perform an analogical transfer.
    pub fn transfer(
        &self,
        source_pattern: &Pattern,
        target_domain: Domain,
        query_features: &[f32],
        pattern_memory: &PatternMemory,
    ) -> GaiaResult<TransferResult> {
        if source_pattern.domain() == target_domain {
            return Err(GaiaError::TransferFailed(
                "Cannot transfer to same domain".into(),
            ));
        }

        // Get bridge affinity
        let bridge = self
            .bridges
            .get(&(source_pattern.domain(), target_domain))
            .ok_or_else(|| GaiaError::TransferFailed("No bridge exists".into()))?;

        // Transform features for target domain
        let transformed = self.transform_features(
            query_features,
            source_pattern.domain(),
            target_domain,
            bridge.affinity,
        );

        // Find matching patterns in target domain
        let target_matches = pattern_memory.find_matches_in_domain(
            &transformed,
            target_domain,
            0.3, // Lower threshold for analogical matches
            5,
        );

        // Calculate transfer strength
        let best_match_sim = target_matches.first().map(|m| m.similarity).unwrap_or(0.0);
        let combined_strength = best_match_sim * bridge.affinity;
        let strength = TransferStrength::from_value(combined_strength);

        // Generate insight
        let insight = self.generate_insight(source_pattern, &target_matches, target_domain);

        Ok(TransferResult {
            source_pattern: source_pattern.id().to_string(),
            source_domain: source_pattern.domain(),
            target_domain,
            target_matches,
            strength,
            insight,
            transformed_features: transformed,
        })
    }

    /// Transform features from source to target domain.
    fn transform_features(
        &self,
        features: &[f32],
        _source: Domain,
        _target: Domain,
        affinity: f32,
    ) -> Vec<f32> {
        // Simple transformation: scale by affinity and rotate slightly
        // In a real system, this would use learned transformations
        features
            .iter()
            .enumerate()
            .map(|(i, &f)| {
                // Apply rotation effect based on position
                let rotation = (i as f32 * 0.1).sin() * 0.1;
                (f * affinity) + rotation
            })
            .collect()
    }

    /// Generate an insight from the transfer.
    fn generate_insight(
        &self,
        source: &Pattern,
        target_matches: &[PatternMatch],
        target_domain: Domain,
    ) -> Option<String> {
        if target_matches.is_empty() {
            return None;
        }

        let best_match = &target_matches[0];

        Some(format!(
            "Pattern '{}' from {:?} domain analogous to '{}' in {:?} domain \
             (similarity: {:.1}%)",
            source.id(),
            source.domain(),
            best_match.pattern_id,
            target_domain,
            best_match.similarity * 100.0
        ))
    }

    /// Record a transfer result for learning.
    pub fn record_transfer(&mut self, result: TransferResult) {
        self.history.push(result);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Record feedback for a transfer (for learning).
    pub fn record_feedback(
        &mut self,
        source_pattern: &str,
        target_pattern: &str,
        success: bool,
        pattern_memory: &PatternMemory,
    ) -> GaiaResult<()> {
        // Get patterns to determine domains
        let source = pattern_memory
            .get(source_pattern)
            .ok_or_else(|| GaiaError::PatternNotFound(source_pattern.into()))?;
        let target = pattern_memory
            .get(target_pattern)
            .ok_or_else(|| GaiaError::PatternNotFound(target_pattern.into()))?;

        // Update bridge affinity
        if let Some(bridge) = self.bridges.get_mut(&(source.domain(), target.domain())) {
            let delta = if success { 0.01 } else { -0.01 };
            bridge.affinity = (bridge.affinity + delta).clamp(0.1, 1.0);
        }

        Ok(())
    }

    /// Get transfer history.
    pub fn history(&self) -> &[TransferResult] {
        &self.history
    }

    /// Get statistics about transfers.
    pub fn stats(&self) -> TransferStats {
        let mut by_domain: HashMap<Domain, usize> = HashMap::new();
        let mut strong_count = 0;
        let mut weak_count = 0;

        for result in &self.history {
            *by_domain.entry(result.target_domain).or_insert(0) += 1;
            if result.strength.is_usable() {
                strong_count += 1;
            } else {
                weak_count += 1;
            }
        }

        TransferStats {
            total_transfers: self.history.len(),
            transfers_by_domain: by_domain,
            strong_transfers: strong_count,
            weak_transfers: weak_count,
            success_rate: if self.history.is_empty() {
                0.0
            } else {
                strong_count as f32 / self.history.len() as f32
            },
        }
    }

    /// Find analogies between patterns across all domains.
    pub fn find_analogies(
        &self,
        pattern: &Pattern,
        pattern_memory: &PatternMemory,
        min_strength: f32,
    ) -> Vec<(Domain, Vec<PatternMatch>)> {
        let mut results = Vec::new();

        for &domain in &[
            Domain::Physics,
            Domain::Language,
            Domain::Consciousness,
            Domain::Social,
            Domain::Emergent,
        ] {
            if domain != pattern.domain() {
                if let Ok(transfer) =
                    self.transfer(pattern, domain, pattern.fingerprint(), pattern_memory)
                {
                    let strong_matches: Vec<_> = transfer
                        .target_matches
                        .into_iter()
                        .filter(|m| m.similarity >= min_strength)
                        .collect();

                    if !strong_matches.is_empty() {
                        results.push((domain, strong_matches));
                    }
                }
            }
        }

        results
    }
}

impl Default for AnalogicalTransfer {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about analogical transfers.
#[derive(Debug, Clone)]
pub struct TransferStats {
    /// Total number of transfers.
    pub total_transfers: usize,
    /// Transfers by target domain.
    pub transfers_by_domain: HashMap<Domain, usize>,
    /// Number of strong transfers.
    pub strong_transfers: usize,
    /// Number of weak transfers.
    pub weak_transfers: usize,
    /// Success rate (strong/total).
    pub success_rate: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_strength() {
        assert!(matches!(
            TransferStrength::from_value(0.9),
            TransferStrength::Strong
        ));
        assert!(matches!(
            TransferStrength::from_value(0.6),
            TransferStrength::Moderate
        ));
        assert!(matches!(
            TransferStrength::from_value(0.35),
            TransferStrength::Weak
        ));
        assert!(matches!(
            TransferStrength::from_value(0.1),
            TransferStrength::Tenuous
        ));
    }

    #[test]
    fn test_domain_bridge_creation() {
        let bridge = DomainBridge::new(Domain::Physics, Domain::Consciousness);
        assert_eq!(bridge.source, Domain::Physics);
        assert_eq!(bridge.target, Domain::Consciousness);
        assert!(bridge.affinity > 0.0);
    }

    #[test]
    fn test_analogical_transfer_creation() {
        let engine = AnalogicalTransfer::new();

        // Should have bridges for all domain pairs
        let bridge = engine.bridge(Domain::Physics, Domain::Language);
        assert!(bridge.is_some());
    }

    #[test]
    fn test_transfer_same_domain_fails() {
        let engine = AnalogicalTransfer::new();
        let memory = PatternMemory::new();
        let pattern = Pattern::new("test", Domain::Physics).with_fingerprint(vec![1.0, 0.0, 0.0]);

        let result = engine.transfer(&pattern, Domain::Physics, &[1.0, 0.0, 0.0], &memory);
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_to_different_domain() {
        let engine = AnalogicalTransfer::new();
        let memory = PatternMemory::new();

        // Add target pattern
        memory
            .register(
                Pattern::new("target", Domain::Language).with_fingerprint(vec![0.8, 0.1, 0.1]),
            )
            .unwrap();

        let source = Pattern::new("source", Domain::Physics).with_fingerprint(vec![1.0, 0.0, 0.0]);

        let result = engine.transfer(&source, Domain::Language, &[1.0, 0.0, 0.0], &memory);
        assert!(result.is_ok());

        let transfer = result.unwrap();
        assert_eq!(transfer.source_domain, Domain::Physics);
        assert_eq!(transfer.target_domain, Domain::Language);
    }

    #[test]
    fn test_mapping_creation() {
        let source = Pattern::new("src", Domain::Physics);
        let target = Pattern::new("tgt", Domain::Language);

        let mapping = AnalogicalMapping::new(&source, &target, 0.7);
        assert_eq!(mapping.source_id, "src");
        assert_eq!(mapping.target_id, "tgt");
        assert!(matches!(mapping.strength, TransferStrength::Moderate));
    }

    #[test]
    fn test_stats() {
        let engine = AnalogicalTransfer::new();
        let stats = engine.stats();

        assert_eq!(stats.total_transfers, 0);
        assert_eq!(stats.success_rate, 0.0);
    }
}
