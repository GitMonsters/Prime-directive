//! Pattern types and memory management for GAIA.
//!
//! Patterns are the core data structure for GAIA's intuition system.
//! Each pattern consists of a fingerprint (feature vector), weight
//! (learned importance), and metadata for cross-domain linking.

use std::collections::HashMap;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use super::{GaiaError, GaiaResult};
use crate::mimicry::layers::layer::Domain;

/// Unique identifier for a pattern.
pub type PatternId = String;

/// A pattern recognized by GAIA.
///
/// Patterns represent learned associations that can be matched against
/// input features and transferred across domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Unique identifier.
    id: PatternId,

    /// Human-readable name.
    name: String,

    /// Feature vector representation.
    fingerprint: Vec<f32>,

    /// Learned importance weight (adjusted through reinforcement).
    weight: f32,

    /// Number of times this pattern has been activated.
    activation_count: u64,

    /// Success rate when this pattern was the primary match.
    success_rate: f32,

    /// Total successes (for calculating success_rate).
    successes: u64,

    /// Total attempts (for calculating success_rate).
    attempts: u64,

    /// Primary domain for this pattern.
    domain: Domain,

    /// Cross-links to related patterns in other domains.
    cross_links: Vec<PatternId>,

    /// Metadata tags for categorization.
    tags: Vec<String>,

    /// Creation timestamp.
    created_at: u64,

    /// Last activation timestamp.
    last_activated: u64,
}

impl Pattern {
    /// Create a new pattern with the given ID and domain.
    pub fn new(id: impl Into<String>, domain: Domain) -> Self {
        let id = id.into();
        Self {
            name: id.clone(),
            id,
            fingerprint: Vec::new(),
            weight: 1.0,
            activation_count: 0,
            success_rate: 0.5, // Start at neutral
            successes: 0,
            attempts: 0,
            domain,
            cross_links: Vec::new(),
            tags: Vec::new(),
            created_at: Self::current_time(),
            last_activated: 0,
        }
    }

    /// Set the pattern name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the feature fingerprint.
    pub fn with_fingerprint(mut self, fingerprint: Vec<f32>) -> Self {
        self.fingerprint = fingerprint;
        self
    }

    /// Set the initial weight.
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }

    /// Add tags for categorization.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add a cross-domain link.
    pub fn with_cross_link(mut self, pattern_id: PatternId) -> Self {
        if !self.cross_links.contains(&pattern_id) {
            self.cross_links.push(pattern_id);
        }
        self
    }

    /// Get the pattern ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the pattern name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the fingerprint.
    pub fn fingerprint(&self) -> &[f32] {
        &self.fingerprint
    }

    /// Get the fingerprint dimension.
    pub fn dimension(&self) -> usize {
        self.fingerprint.len()
    }

    /// Get the current weight.
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Get the domain.
    pub fn domain(&self) -> Domain {
        self.domain
    }

    /// Get activation count.
    pub fn activation_count(&self) -> u64 {
        self.activation_count
    }

    /// Get success rate.
    pub fn success_rate(&self) -> f32 {
        self.success_rate
    }

    /// Get cross-links.
    pub fn cross_links(&self) -> &[PatternId] {
        &self.cross_links
    }

    /// Get tags.
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Calculate similarity to a query fingerprint using cosine similarity.
    pub fn similarity(&self, query: &[f32]) -> f32 {
        if self.fingerprint.is_empty() || query.is_empty() {
            return 0.0;
        }

        // Use the minimum dimension
        let dim = self.fingerprint.len().min(query.len());

        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for i in 0..dim {
            dot += self.fingerprint[i] * query[i];
            norm_a += self.fingerprint[i] * self.fingerprint[i];
            norm_b += query[i] * query[i];
        }

        let denominator = (norm_a.sqrt() * norm_b.sqrt()).max(1e-10);
        (dot / denominator).clamp(-1.0, 1.0)
    }

    /// Calculate weighted similarity (combines cosine similarity with weight).
    pub fn weighted_similarity(&self, query: &[f32]) -> f32 {
        self.similarity(query) * self.weight * self.success_rate.max(0.1)
    }

    /// Record an activation of this pattern.
    pub fn activate(&mut self) {
        self.activation_count += 1;
        self.last_activated = Self::current_time();
    }

    /// Record a success outcome (used for reinforcement).
    pub fn record_success(&mut self) {
        self.successes += 1;
        self.attempts += 1;
        self.update_success_rate();
        // Increase weight on success (bounded)
        self.weight = (self.weight * 1.05).min(10.0);
    }

    /// Record a failure outcome (used for reinforcement).
    pub fn record_failure(&mut self) {
        self.attempts += 1;
        self.update_success_rate();
        // Decrease weight on failure (bounded)
        self.weight = (self.weight * 0.95).max(0.1);
    }

    /// Apply direct weight adjustment.
    pub fn adjust_weight(&mut self, delta: f32) {
        self.weight = (self.weight + delta).clamp(0.1, 10.0);
    }

    fn update_success_rate(&mut self) {
        if self.attempts > 0 {
            self.success_rate = self.successes as f32 / self.attempts as f32;
        }
    }

    fn current_time() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

/// Result of matching a pattern against a query.
#[derive(Debug, Clone)]
pub struct PatternMatch {
    /// The matched pattern ID.
    pub pattern_id: PatternId,
    /// Raw similarity score (0.0 to 1.0).
    pub similarity: f32,
    /// Weighted similarity (includes weight and success rate).
    pub weighted_similarity: f32,
    /// The domain of the matched pattern.
    pub domain: Domain,
    /// Cross-links from this pattern.
    pub cross_links: Vec<PatternId>,
}

impl PatternMatch {
    /// Create a new pattern match result.
    pub fn new(pattern: &Pattern, similarity: f32, weighted_similarity: f32) -> Self {
        Self {
            pattern_id: pattern.id().to_string(),
            similarity,
            weighted_similarity,
            domain: pattern.domain(),
            cross_links: pattern.cross_links().to_vec(),
        }
    }
}

/// Statistics about pattern memory.
#[derive(Debug, Clone, Default)]
pub struct PatternStats {
    /// Total number of patterns.
    pub total_patterns: usize,
    /// Patterns per domain.
    pub patterns_by_domain: HashMap<Domain, usize>,
    /// Average weight across all patterns.
    pub average_weight: f32,
    /// Average success rate across all patterns.
    pub average_success_rate: f32,
    /// Total activation count.
    pub total_activations: u64,
    /// Number of cross-domain links.
    pub cross_link_count: usize,
}

/// Thread-safe storage for patterns.
pub struct PatternMemory {
    /// Pattern storage by ID.
    patterns: RwLock<HashMap<PatternId, Pattern>>,
    /// Index by domain for fast lookups.
    domain_index: RwLock<HashMap<Domain, Vec<PatternId>>>,
    /// Index by tag.
    tag_index: RwLock<HashMap<String, Vec<PatternId>>>,
}

impl PatternMemory {
    /// Create a new empty pattern memory.
    pub fn new() -> Self {
        Self {
            patterns: RwLock::new(HashMap::new()),
            domain_index: RwLock::new(HashMap::new()),
            tag_index: RwLock::new(HashMap::new()),
        }
    }

    /// Check if memory is empty.
    pub fn is_empty(&self) -> bool {
        self.patterns.read().unwrap().is_empty()
    }

    /// Get the number of stored patterns.
    pub fn len(&self) -> usize {
        self.patterns.read().unwrap().len()
    }

    /// Register a new pattern.
    pub fn register(&self, pattern: Pattern) -> GaiaResult<()> {
        // Validate fingerprint
        for &val in pattern.fingerprint() {
            if val.is_nan() || val.is_infinite() {
                return Err(GaiaError::InvalidFingerprint(
                    "Fingerprint contains NaN or infinite values".into(),
                ));
            }
        }

        let id = pattern.id().to_string();
        let domain = pattern.domain();
        let tags = pattern.tags().to_vec();

        // Store pattern
        {
            let mut patterns = self.patterns.write().unwrap();
            patterns.insert(id.clone(), pattern);
        }

        // Update domain index
        {
            let mut domain_idx = self.domain_index.write().unwrap();
            domain_idx
                .entry(domain)
                .or_insert_with(Vec::new)
                .push(id.clone());
        }

        // Update tag index
        {
            let mut tag_idx = self.tag_index.write().unwrap();
            for tag in tags {
                tag_idx.entry(tag).or_insert_with(Vec::new).push(id.clone());
            }
        }

        Ok(())
    }

    /// Get a pattern by ID.
    pub fn get(&self, id: &str) -> Option<Pattern> {
        self.patterns.read().unwrap().get(id).cloned()
    }

    /// Check if a pattern exists.
    pub fn contains(&self, id: &str) -> bool {
        self.patterns.read().unwrap().contains_key(id)
    }

    /// Get all patterns in a domain.
    pub fn get_by_domain(&self, domain: Domain) -> Vec<Pattern> {
        let domain_idx = self.domain_index.read().unwrap();
        let patterns = self.patterns.read().unwrap();

        domain_idx
            .get(&domain)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| patterns.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all patterns with a specific tag.
    pub fn get_by_tag(&self, tag: &str) -> Vec<Pattern> {
        let tag_idx = self.tag_index.read().unwrap();
        let patterns = self.patterns.read().unwrap();

        tag_idx
            .get(tag)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| patterns.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Find patterns matching a query fingerprint.
    pub fn find_matches(
        &self,
        query: &[f32],
        min_similarity: f32,
        max_results: usize,
    ) -> Vec<PatternMatch> {
        let patterns = self.patterns.read().unwrap();

        let mut matches: Vec<PatternMatch> = patterns
            .values()
            .filter_map(|p| {
                let sim = p.similarity(query);
                if sim >= min_similarity {
                    Some(PatternMatch::new(p, sim, p.weighted_similarity(query)))
                } else {
                    None
                }
            })
            .collect();

        // Sort by weighted similarity (highest first)
        matches.sort_by(|a, b| {
            b.weighted_similarity
                .partial_cmp(&a.weighted_similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches.truncate(max_results);
        matches
    }

    /// Find patterns matching within a specific domain.
    pub fn find_matches_in_domain(
        &self,
        query: &[f32],
        domain: Domain,
        min_similarity: f32,
        max_results: usize,
    ) -> Vec<PatternMatch> {
        let domain_patterns = self.get_by_domain(domain);

        let mut matches: Vec<PatternMatch> = domain_patterns
            .iter()
            .filter_map(|p| {
                let sim = p.similarity(query);
                if sim >= min_similarity {
                    Some(PatternMatch::new(p, sim, p.weighted_similarity(query)))
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by(|a, b| {
            b.weighted_similarity
                .partial_cmp(&a.weighted_similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches.truncate(max_results);
        matches
    }

    /// Activate a pattern and record it.
    pub fn activate(&self, id: &str) -> GaiaResult<()> {
        let mut patterns = self.patterns.write().unwrap();
        let pattern = patterns
            .get_mut(id)
            .ok_or_else(|| GaiaError::PatternNotFound(id.into()))?;
        pattern.activate();
        Ok(())
    }

    /// Record success for a pattern.
    pub fn record_success(&self, id: &str) -> GaiaResult<()> {
        let mut patterns = self.patterns.write().unwrap();
        let pattern = patterns
            .get_mut(id)
            .ok_or_else(|| GaiaError::PatternNotFound(id.into()))?;
        pattern.record_success();
        Ok(())
    }

    /// Record failure for a pattern.
    pub fn record_failure(&self, id: &str) -> GaiaResult<()> {
        let mut patterns = self.patterns.write().unwrap();
        let pattern = patterns
            .get_mut(id)
            .ok_or_else(|| GaiaError::PatternNotFound(id.into()))?;
        pattern.record_failure();
        Ok(())
    }

    /// Adjust weight for a pattern.
    pub fn adjust_weight(&self, id: &str, delta: f32) -> GaiaResult<()> {
        let mut patterns = self.patterns.write().unwrap();
        let pattern = patterns
            .get_mut(id)
            .ok_or_else(|| GaiaError::PatternNotFound(id.into()))?;
        pattern.adjust_weight(delta);
        Ok(())
    }

    /// Add a cross-link between patterns.
    pub fn add_cross_link(&self, from_id: &str, to_id: &str) -> GaiaResult<()> {
        // Verify both patterns exist
        {
            let patterns = self.patterns.read().unwrap();
            if !patterns.contains_key(from_id) {
                return Err(GaiaError::PatternNotFound(from_id.into()));
            }
            if !patterns.contains_key(to_id) {
                return Err(GaiaError::PatternNotFound(to_id.into()));
            }
        }

        // Add the link
        let mut patterns = self.patterns.write().unwrap();
        if let Some(pattern) = patterns.get_mut(from_id) {
            if !pattern.cross_links.contains(&to_id.to_string()) {
                pattern.cross_links.push(to_id.to_string());
            }
        }

        Ok(())
    }

    /// Get statistics about the pattern memory.
    pub fn stats(&self) -> PatternStats {
        let patterns = self.patterns.read().unwrap();

        let mut stats = PatternStats {
            total_patterns: patterns.len(),
            ..Default::default()
        };

        if patterns.is_empty() {
            return stats;
        }

        let mut total_weight = 0.0;
        let mut total_success_rate = 0.0;

        for pattern in patterns.values() {
            *stats
                .patterns_by_domain
                .entry(pattern.domain())
                .or_insert(0) += 1;
            total_weight += pattern.weight();
            total_success_rate += pattern.success_rate();
            stats.total_activations += pattern.activation_count();
            stats.cross_link_count += pattern.cross_links().len();
        }

        stats.average_weight = total_weight / patterns.len() as f32;
        stats.average_success_rate = total_success_rate / patterns.len() as f32;

        stats
    }

    /// Remove a pattern from memory.
    pub fn remove(&self, id: &str) -> GaiaResult<Pattern> {
        let pattern = {
            let mut patterns = self.patterns.write().unwrap();
            patterns
                .remove(id)
                .ok_or_else(|| GaiaError::PatternNotFound(id.into()))?
        };

        // Remove from domain index
        {
            let mut domain_idx = self.domain_index.write().unwrap();
            if let Some(ids) = domain_idx.get_mut(&pattern.domain()) {
                ids.retain(|i| i != id);
            }
        }

        // Remove from tag index
        {
            let mut tag_idx = self.tag_index.write().unwrap();
            for tag in pattern.tags() {
                if let Some(ids) = tag_idx.get_mut(tag) {
                    ids.retain(|i| i != id);
                }
            }
        }

        Ok(pattern)
    }

    /// Clear all patterns.
    pub fn clear(&self) {
        self.patterns.write().unwrap().clear();
        self.domain_index.write().unwrap().clear();
        self.tag_index.write().unwrap().clear();
    }

    /// Get all pattern IDs.
    pub fn all_ids(&self) -> Vec<PatternId> {
        self.patterns.read().unwrap().keys().cloned().collect()
    }

    /// Get all patterns.
    pub fn all_patterns(&self) -> Vec<Pattern> {
        self.patterns.read().unwrap().values().cloned().collect()
    }
}

impl Default for PatternMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern::new("test", Domain::Physics)
            .with_name("Test Pattern")
            .with_fingerprint(vec![0.5, 0.5, 0.5])
            .with_weight(1.5);

        assert_eq!(pattern.id(), "test");
        assert_eq!(pattern.name(), "Test Pattern");
        assert_eq!(pattern.dimension(), 3);
        assert_eq!(pattern.weight(), 1.5);
    }

    #[test]
    fn test_pattern_similarity() {
        let pattern = Pattern::new("test", Domain::Physics).with_fingerprint(vec![1.0, 0.0, 0.0]);

        // Identical direction
        assert!((pattern.similarity(&[1.0, 0.0, 0.0]) - 1.0).abs() < 0.001);

        // Orthogonal
        assert!(pattern.similarity(&[0.0, 1.0, 0.0]).abs() < 0.001);

        // Opposite
        assert!((pattern.similarity(&[-1.0, 0.0, 0.0]) + 1.0).abs() < 0.001);
    }

    #[test]
    fn test_pattern_reinforcement() {
        let mut pattern = Pattern::new("test", Domain::Physics).with_weight(1.0);

        let initial_weight = pattern.weight();
        pattern.record_success();
        assert!(pattern.weight() > initial_weight);

        let after_success = pattern.weight();
        pattern.record_failure();
        assert!(pattern.weight() < after_success);
    }

    #[test]
    fn test_pattern_memory_registration() {
        let memory = PatternMemory::new();
        let pattern = Pattern::new("test", Domain::Physics).with_fingerprint(vec![1.0, 0.0, 0.0]);

        memory.register(pattern).unwrap();
        assert_eq!(memory.len(), 1);
        assert!(memory.contains("test"));
    }

    #[test]
    fn test_pattern_memory_find_matches() {
        let memory = PatternMemory::new();

        memory
            .register(Pattern::new("p1", Domain::Physics).with_fingerprint(vec![1.0, 0.0, 0.0]))
            .unwrap();
        memory
            .register(Pattern::new("p2", Domain::Physics).with_fingerprint(vec![0.9, 0.1, 0.0]))
            .unwrap();
        memory
            .register(Pattern::new("p3", Domain::Physics).with_fingerprint(vec![0.0, 1.0, 0.0]))
            .unwrap();

        let matches = memory.find_matches(&[1.0, 0.0, 0.0], 0.5, 10);

        // Should find p1 and p2, not p3
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|m| m.pattern_id == "p1"));
        assert!(matches.iter().any(|m| m.pattern_id == "p2"));
    }

    #[test]
    fn test_pattern_memory_domain_lookup() {
        let memory = PatternMemory::new();

        memory
            .register(Pattern::new("physics1", Domain::Physics))
            .unwrap();
        memory
            .register(Pattern::new("language1", Domain::Language))
            .unwrap();

        let physics = memory.get_by_domain(Domain::Physics);
        assert_eq!(physics.len(), 1);
        assert_eq!(physics[0].id(), "physics1");
    }

    #[test]
    fn test_invalid_fingerprint() {
        let memory = PatternMemory::new();
        let pattern = Pattern::new("bad", Domain::Physics).with_fingerprint(vec![f32::NAN]);

        assert!(memory.register(pattern).is_err());
    }

    #[test]
    fn test_cross_links() {
        let memory = PatternMemory::new();

        memory.register(Pattern::new("a", Domain::Physics)).unwrap();
        memory
            .register(Pattern::new("b", Domain::Language))
            .unwrap();

        memory.add_cross_link("a", "b").unwrap();

        let pattern_a = memory.get("a").unwrap();
        assert!(pattern_a.cross_links().contains(&"b".to_string()));
    }
}
