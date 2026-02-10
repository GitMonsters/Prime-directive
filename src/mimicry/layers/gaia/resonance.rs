//! Resonance Field for spreading activation across patterns.
//!
//! The resonance field enables patterns to activate related patterns,
//! creating a spreading activation effect similar to neural networks.

use std::collections::HashMap;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use super::pattern::{PatternId, PatternMemory};

/// Configuration for the resonance field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceConfig {
    /// Maximum number of activation steps.
    pub max_steps: u32,

    /// Activation decay per step (0.0-1.0).
    pub decay_rate: f32,

    /// Minimum activation to continue spreading.
    pub activation_threshold: f32,

    /// Maximum number of patterns that can be active.
    pub max_active_patterns: usize,

    /// Whether to follow cross-domain links.
    pub enable_cross_domain: bool,

    /// Weight multiplier for cross-domain activation.
    pub cross_domain_weight: f32,
}

impl Default for ResonanceConfig {
    fn default() -> Self {
        Self {
            max_steps: 5,
            decay_rate: 0.7,
            activation_threshold: 0.1,
            max_active_patterns: 100,
            enable_cross_domain: true,
            cross_domain_weight: 0.5,
        }
    }
}

impl ResonanceConfig {
    /// Create a fast configuration with fewer steps.
    pub fn fast() -> Self {
        Self {
            max_steps: 2,
            max_active_patterns: 20,
            ..Default::default()
        }
    }

    /// Create a deep configuration with more spreading.
    pub fn deep() -> Self {
        Self {
            max_steps: 10,
            decay_rate: 0.8,
            max_active_patterns: 200,
            ..Default::default()
        }
    }
}

/// Activation state of a single pattern.
#[derive(Debug, Clone)]
pub struct ActivationState {
    /// Current activation level.
    pub activation: f32,
    /// Step at which this pattern was activated.
    pub activated_at_step: u32,
    /// Source pattern that activated this one.
    pub activated_by: Option<PatternId>,
    /// Whether this was an initial seed pattern.
    pub is_seed: bool,
}

impl ActivationState {
    /// Create a seed activation.
    pub fn seed(activation: f32) -> Self {
        Self {
            activation,
            activated_at_step: 0,
            activated_by: None,
            is_seed: true,
        }
    }

    /// Create a derived activation.
    pub fn derived(activation: f32, step: u32, source: PatternId) -> Self {
        Self {
            activation,
            activated_at_step: step,
            activated_by: Some(source),
            is_seed: false,
        }
    }
}

/// Result of a resonance activation.
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// All activated patterns with their states.
    pub activations: HashMap<PatternId, ActivationState>,

    /// Number of spreading steps taken.
    pub steps_taken: u32,

    /// Total activation across all patterns.
    pub total_activation: f32,

    /// Peak activation value.
    pub peak_activation: f32,

    /// Number of cross-domain activations.
    pub cross_domain_activations: usize,
}

impl ResonanceResult {
    /// Create an empty result.
    pub fn empty() -> Self {
        Self {
            activations: HashMap::new(),
            steps_taken: 0,
            total_activation: 0.0,
            peak_activation: 0.0,
            cross_domain_activations: 0,
        }
    }

    /// Get sorted activations (highest first).
    pub fn sorted_activations(&self) -> Vec<(&PatternId, &ActivationState)> {
        let mut sorted: Vec<_> = self.activations.iter().collect();
        sorted.sort_by(|a, b| {
            b.1.activation
                .partial_cmp(&a.1.activation)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    /// Get patterns above a threshold.
    pub fn patterns_above(&self, threshold: f32) -> Vec<&PatternId> {
        self.activations
            .iter()
            .filter(|(_, state)| state.activation >= threshold)
            .map(|(id, _)| id)
            .collect()
    }

    /// Check if a pattern is active.
    pub fn is_active(&self, pattern_id: &str) -> bool {
        self.activations.contains_key(pattern_id)
    }

    /// Get activation for a specific pattern.
    pub fn activation_of(&self, pattern_id: &str) -> f32 {
        self.activations
            .get(pattern_id)
            .map(|s| s.activation)
            .unwrap_or(0.0)
    }
}

/// The resonance field for spreading activation.
pub struct ResonanceField {
    /// Configuration.
    config: ResonanceConfig,

    /// Current activation state.
    current_activations: RwLock<HashMap<PatternId, ActivationState>>,

    /// Activation history for analysis.
    history: RwLock<Vec<ResonanceResult>>,
}

impl ResonanceField {
    /// Create a new resonance field.
    pub fn new(config: ResonanceConfig) -> Self {
        Self {
            config,
            current_activations: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(ResonanceConfig::default())
    }

    /// Check if the field has no activations.
    pub fn is_empty(&self) -> bool {
        self.current_activations.read().unwrap().is_empty()
    }

    /// Get current number of active patterns.
    pub fn active_count(&self) -> usize {
        self.current_activations.read().unwrap().len()
    }

    /// Get the current configuration.
    pub fn config(&self) -> &ResonanceConfig {
        &self.config
    }

    /// Activate the field with seed patterns.
    pub fn activate(
        &self,
        seed_patterns: &[&str],
        pattern_memory: &PatternMemory,
    ) -> ResonanceResult {
        let mut activations: HashMap<PatternId, ActivationState> = HashMap::new();
        let mut cross_domain_count = 0;

        // Initialize seed patterns
        for &pattern_id in seed_patterns {
            if let Some(pattern) = pattern_memory.get(pattern_id) {
                let activation = pattern.weight() * pattern.success_rate().max(0.1);
                activations.insert(pattern_id.to_string(), ActivationState::seed(activation));
            }
        }

        // Spreading activation
        let mut step = 0;
        while step < self.config.max_steps {
            step += 1;

            let mut new_activations: Vec<(PatternId, ActivationState)> = Vec::new();

            // For each active pattern, spread to linked patterns
            for (pattern_id, state) in &activations {
                if state.activation < self.config.activation_threshold {
                    continue;
                }

                if let Some(pattern) = pattern_memory.get(pattern_id) {
                    // Spread to cross-linked patterns
                    if self.config.enable_cross_domain {
                        for link_id in pattern.cross_links() {
                            if activations.len() + new_activations.len()
                                >= self.config.max_active_patterns
                            {
                                break;
                            }

                            if !activations.contains_key(link_id) {
                                let spread_activation = state.activation
                                    * self.config.decay_rate
                                    * self.config.cross_domain_weight;

                                if spread_activation >= self.config.activation_threshold {
                                    new_activations.push((
                                        link_id.clone(),
                                        ActivationState::derived(
                                            spread_activation,
                                            step,
                                            pattern_id.clone(),
                                        ),
                                    ));
                                    cross_domain_count += 1;
                                }
                            }
                        }
                    }
                }
            }

            // Add new activations
            if new_activations.is_empty() {
                break; // No more spreading possible
            }

            for (id, state) in new_activations {
                if !activations.contains_key(&id) {
                    activations.insert(id, state);
                }
            }
        }

        // Calculate result metrics
        let total_activation: f32 = activations.values().map(|s| s.activation).sum();
        let peak_activation = activations
            .values()
            .map(|s| s.activation)
            .fold(0.0f32, |a, b| a.max(b));

        let result = ResonanceResult {
            activations: activations.clone(),
            steps_taken: step,
            total_activation,
            peak_activation,
            cross_domain_activations: cross_domain_count,
        };

        // Update current state
        *self.current_activations.write().unwrap() = activations;

        // Store in history
        {
            let mut history = self.history.write().unwrap();
            history.push(result.clone());
            if history.len() > 100 {
                history.remove(0);
            }
        }

        result
    }

    /// Get the current activation state.
    pub fn current_state(&self) -> HashMap<PatternId, ActivationState> {
        self.current_activations.read().unwrap().clone()
    }

    /// Decay all activations by a factor.
    pub fn decay(&self, factor: f32) {
        let mut activations = self.current_activations.write().unwrap();
        activations.retain(|_, state| {
            state.activation *= factor;
            state.activation >= self.config.activation_threshold
        });
    }

    /// Reset the field, clearing all activations.
    pub fn reset(&self) {
        self.current_activations.write().unwrap().clear();
    }

    /// Get activation history.
    pub fn history(&self) -> Vec<ResonanceResult> {
        self.history.read().unwrap().clone()
    }

    /// Inject external activation for a pattern.
    pub fn inject_activation(&self, pattern_id: &str, activation: f32) {
        let mut activations = self.current_activations.write().unwrap();

        if let Some(state) = activations.get_mut(pattern_id) {
            state.activation += activation;
        } else {
            activations.insert(pattern_id.to_string(), ActivationState::seed(activation));
        }
    }

    /// Compute resonance between two pattern sets.
    pub fn compute_resonance(
        &self,
        set_a: &[&str],
        set_b: &[&str],
        pattern_memory: &PatternMemory,
    ) -> f32 {
        if set_a.is_empty() || set_b.is_empty() {
            return 0.0;
        }

        let mut total_resonance = 0.0;
        let mut count = 0;

        for &a_id in set_a {
            if let Some(pattern_a) = pattern_memory.get(a_id) {
                for &b_id in set_b {
                    if a_id != b_id {
                        if let Some(pattern_b) = pattern_memory.get(b_id) {
                            // Compute similarity as resonance measure
                            let sim = pattern_a.similarity(pattern_b.fingerprint());
                            total_resonance += sim.abs();
                            count += 1;
                        }
                    }
                }
            }
        }

        if count > 0 {
            total_resonance / count as f32
        } else {
            0.0
        }
    }
}

impl Default for ResonanceField {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::layers::gaia::pattern::Pattern;
    use crate::mimicry::layers::layer::Domain;

    fn setup_memory() -> PatternMemory {
        let memory = PatternMemory::new();

        // Pattern A with link to B
        memory
            .register(
                Pattern::new("a", Domain::Physics)
                    .with_fingerprint(vec![1.0, 0.0, 0.0])
                    .with_cross_link("b".into()),
            )
            .unwrap();

        // Pattern B with link to C
        memory
            .register(
                Pattern::new("b", Domain::Language)
                    .with_fingerprint(vec![0.0, 1.0, 0.0])
                    .with_cross_link("c".into()),
            )
            .unwrap();

        // Pattern C (no links)
        memory
            .register(
                Pattern::new("c", Domain::Consciousness).with_fingerprint(vec![0.0, 0.0, 1.0]),
            )
            .unwrap();

        memory
    }

    #[test]
    fn test_resonance_creation() {
        let field = ResonanceField::new(ResonanceConfig::default());
        assert!(field.is_empty());
    }

    #[test]
    fn test_seed_activation() {
        let field = ResonanceField::new(ResonanceConfig::default());
        let memory = setup_memory();

        let result = field.activate(&["a"], &memory);

        assert!(result.is_active("a"));
        assert!(result.activations.get("a").unwrap().is_seed);
    }

    #[test]
    fn test_spreading_activation() {
        let config = ResonanceConfig {
            enable_cross_domain: true,
            cross_domain_weight: 1.0,
            decay_rate: 0.9,
            activation_threshold: 0.01,
            ..Default::default()
        };
        let field = ResonanceField::new(config);
        let memory = setup_memory();

        let result = field.activate(&["a"], &memory);

        // A should activate B through cross-link
        assert!(result.is_active("a"));
        assert!(
            result.is_active("b"),
            "B should be activated via cross-link from A"
        );

        // Check that B was activated by A
        let b_state = result.activations.get("b").unwrap();
        assert!(!b_state.is_seed);
        assert_eq!(b_state.activated_by.as_deref(), Some("a"));
    }

    #[test]
    fn test_activation_decay() {
        let field = ResonanceField::new(ResonanceConfig::default());
        let memory = setup_memory();

        field.activate(&["a"], &memory);
        assert!(!field.is_empty());

        field.decay(0.01); // Strong decay
        assert!(field.is_empty()); // Should clear all below threshold
    }

    #[test]
    fn test_inject_activation() {
        let field = ResonanceField::new(ResonanceConfig::default());

        field.inject_activation("test", 0.5);

        let state = field.current_state();
        assert!(state.contains_key("test"));
        assert_eq!(state.get("test").unwrap().activation, 0.5);
    }

    #[test]
    fn test_resonance_computation() {
        let field = ResonanceField::new(ResonanceConfig::default());
        let memory = setup_memory();

        // Patterns A and B are orthogonal, so resonance should be ~0
        let resonance = field.compute_resonance(&["a"], &["b"], &memory);
        assert!(resonance.abs() < 0.1);
    }

    #[test]
    fn test_sorted_activations() {
        let field = ResonanceField::new(ResonanceConfig::default());

        field.inject_activation("low", 0.2);
        field.inject_activation("high", 0.8);
        field.inject_activation("mid", 0.5);

        let state = field.current_state();
        let result = ResonanceResult {
            activations: state,
            steps_taken: 0,
            total_activation: 1.5,
            peak_activation: 0.8,
            cross_domain_activations: 0,
        };

        let sorted = result.sorted_activations();
        assert_eq!(sorted[0].0, "high");
        assert_eq!(sorted[1].0, "mid");
        assert_eq!(sorted[2].0, "low");
    }
}
