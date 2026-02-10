//! Layer stack orchestrator for the 7-layer multiplicative integration system.
//!
//! The stack coordinates information flow through all layers, managing
//! bidirectional propagation and multiplicative amplification.

use std::collections::HashMap;
use std::sync::Arc;

use super::bridge::{BidirectionalBridge, BridgeNetwork};
use super::layer::{Layer, LayerConfig, LayerSignal, LayerState};
use super::registry::LayerRegistry;

/// Configuration for the layer stack.
#[derive(Debug, Clone)]
pub struct LayerStackConfig {
    /// Maximum number of full-stack amplification cycles.
    pub max_stack_iterations: u32,
    /// Convergence threshold for stopping iteration.
    pub convergence_threshold: f32,
    /// Global amplification factor.
    pub global_amplification: f32,
    /// Whether to enable backward propagation by default.
    pub enable_backward_propagation: bool,
    /// Minimum confidence to continue propagation.
    pub min_propagation_confidence: f32,
    /// Per-layer configuration overrides.
    pub layer_configs: HashMap<Layer, LayerConfig>,
}

impl Default for LayerStackConfig {
    fn default() -> Self {
        Self {
            max_stack_iterations: 5,
            convergence_threshold: 0.01,
            global_amplification: 1.1,
            enable_backward_propagation: true,
            min_propagation_confidence: 0.1,
            layer_configs: HashMap::new(),
        }
    }
}

impl LayerStackConfig {
    /// Create a new configuration with the given settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the global amplification factor.
    pub fn with_global_amplification(mut self, factor: f32) -> Self {
        self.global_amplification = factor;
        self
    }

    /// Set max stack iterations.
    pub fn with_max_iterations(mut self, max: u32) -> Self {
        self.max_stack_iterations = max;
        self
    }

    /// Add a layer-specific configuration.
    pub fn with_layer_config(mut self, config: LayerConfig) -> Self {
        self.layer_configs.insert(config.layer, config);
        self
    }

    /// Disable backward propagation.
    pub fn without_backward_propagation(mut self) -> Self {
        self.enable_backward_propagation = false;
        self
    }
}

/// Result of processing through the entire stack.
#[derive(Debug, Clone)]
pub struct StackProcessResult {
    /// Final states for each layer.
    pub layer_states: HashMap<Layer, LayerState>,
    /// Combined confidence across all layers.
    pub combined_confidence: f32,
    /// Total amplification achieved.
    pub total_amplification: f32,
    /// Number of iterations performed.
    pub iterations: u32,
    /// Whether convergence was achieved.
    pub converged: bool,
    /// Per-layer confidence values.
    pub layer_confidences: HashMap<Layer, f32>,
    /// Trace of signals for debugging.
    pub signal_trace: Vec<LayerSignal>,
}

impl StackProcessResult {
    /// Create an empty result.
    pub fn empty() -> Self {
        Self {
            layer_states: HashMap::new(),
            combined_confidence: 0.0,
            total_amplification: 1.0,
            iterations: 0,
            converged: false,
            layer_confidences: HashMap::new(),
            signal_trace: Vec::new(),
        }
    }

    /// Get the state for a specific layer.
    pub fn get_state(&self, layer: Layer) -> Option<&LayerState> {
        self.layer_states.get(&layer)
    }

    /// Check if processing was successful.
    pub fn is_successful(&self, min_confidence: f32) -> bool {
        self.combined_confidence >= min_confidence
    }
}

/// The main orchestrator for the 7-layer system.
pub struct LayerStack {
    /// Layer registry.
    registry: LayerRegistry,
    /// Bridge network.
    bridge_network: BridgeNetwork,
    /// Stack configuration.
    config: LayerStackConfig,
    /// Current layer states.
    current_states: HashMap<Layer, LayerState>,
    /// Processing statistics.
    stats: StackStats,
}

/// Statistics for stack processing.
#[derive(Debug, Clone, Default)]
pub struct StackStats {
    pub total_forward_propagations: u64,
    pub total_backward_propagations: u64,
    pub total_amplifications: u64,
    pub average_confidence: f32,
    pub max_confidence_achieved: f32,
    pub convergence_count: u64,
    pub non_convergence_count: u64,
}

impl LayerStack {
    /// Create a new layer stack with default configuration.
    pub fn new() -> Self {
        Self::with_config(LayerStackConfig::default())
    }

    /// Create a new layer stack with custom configuration.
    pub fn with_config(config: LayerStackConfig) -> Self {
        let mut registry = LayerRegistry::new();

        // Apply layer-specific configs
        for (layer, layer_config) in &config.layer_configs {
            registry.configure(*layer, layer_config.clone());
        }

        let mut bridge_network = BridgeNetwork::new();
        bridge_network.set_global_amplification(config.global_amplification);

        Self {
            registry,
            bridge_network,
            config,
            current_states: HashMap::new(),
            stats: StackStats::default(),
        }
    }

    /// Get a reference to the registry.
    pub fn registry(&self) -> &LayerRegistry {
        &self.registry
    }

    /// Get a mutable reference to the registry.
    pub fn registry_mut(&mut self) -> &mut LayerRegistry {
        &mut self.registry
    }

    /// Get a reference to the bridge network.
    pub fn bridge_network(&self) -> &BridgeNetwork {
        &self.bridge_network
    }

    /// Get a mutable reference to the bridge network.
    pub fn bridge_network_mut(&mut self) -> &mut BridgeNetwork {
        &mut self.bridge_network
    }

    /// Register a bridge in the network.
    pub fn register_bridge(&mut self, bridge: Arc<dyn BidirectionalBridge>) {
        self.bridge_network.register(bridge);
    }

    /// Get the current configuration.
    pub fn config(&self) -> &LayerStackConfig {
        &self.config
    }

    /// Update the configuration.
    pub fn set_config(&mut self, config: LayerStackConfig) {
        self.bridge_network
            .set_global_amplification(config.global_amplification);
        self.config = config;
    }

    /// Get processing statistics.
    pub fn stats(&self) -> &StackStats {
        &self.stats
    }

    /// Reset statistics.
    pub fn reset_stats(&mut self) {
        self.stats = StackStats::default();
    }

    /// Get the current state for a layer.
    pub fn get_current_state(&self, layer: Layer) -> Option<&LayerState> {
        self.current_states.get(&layer)
    }

    /// Set the state for a layer.
    pub fn set_state(&mut self, state: LayerState) {
        self.current_states.insert(state.layer, state);
    }

    /// Clear all current states.
    pub fn clear_states(&mut self) {
        self.current_states.clear();
    }

    /// Process input through the entire stack with forward propagation.
    pub fn process_forward(&mut self, input: LayerState) -> StackProcessResult {
        let mut result = StackProcessResult::empty();
        let start_layer = input.layer;

        // Set the initial state
        self.current_states.insert(start_layer, input.clone());
        result.layer_states.insert(start_layer, input.clone());
        result
            .layer_confidences
            .insert(start_layer, input.confidence);

        // Get layers to process (from start layer upward)
        let layers_to_process: Vec<Layer> = Layer::all()
            .iter()
            .filter(|&&l| l.number() > start_layer.number() && self.registry.is_enabled(l))
            .copied()
            .collect();

        // Forward propagation through each layer
        let mut current_state = input;
        for target_layer in layers_to_process {
            // Find bridge from current layer to target
            if let Some(bridge) = self
                .bridge_network
                .bridge_between(current_state.layer, target_layer)
            {
                match bridge.forward(&current_state) {
                    Ok(new_state) => {
                        self.stats.total_forward_propagations += 1;

                        // Create signal for trace
                        let signal =
                            LayerSignal::new(current_state.layer, target_layer, new_state.clone());
                        result.signal_trace.push(signal);

                        // Update states
                        self.current_states.insert(target_layer, new_state.clone());
                        result.layer_states.insert(target_layer, new_state.clone());
                        result
                            .layer_confidences
                            .insert(target_layer, new_state.confidence);

                        current_state = new_state;
                    }
                    Err(_) => {
                        // Bridge failed, stop propagation to this path
                        break;
                    }
                }
            }
        }

        // Calculate combined confidence
        result.combined_confidence = self.calculate_combined_confidence(&result.layer_confidences);
        result.iterations = 1;

        result
    }

    /// Process with bidirectional amplification.
    pub fn process_bidirectional(&mut self, input: LayerState) -> StackProcessResult {
        let mut result = self.process_forward(input);

        if !self.config.enable_backward_propagation {
            return result;
        }

        let mut previous_confidence = result.combined_confidence;

        for iteration in 0..self.config.max_stack_iterations {
            // Backward propagation
            self.propagate_backward(&mut result);

            // Forward propagation again
            self.propagate_forward_from_states(&mut result);

            // Amplification across all bridges
            self.amplify_all_bridges(&mut result);

            // Check convergence
            let confidence_change = (result.combined_confidence - previous_confidence).abs();
            if confidence_change < self.config.convergence_threshold {
                result.converged = true;
                self.stats.convergence_count += 1;
                break;
            }

            previous_confidence = result.combined_confidence;
            result.iterations = iteration + 2; // +1 for initial forward, +1 for 0-indexed
        }

        if !result.converged {
            self.stats.non_convergence_count += 1;
        }

        // Update statistics
        self.update_stats(&result);

        result
    }

    /// Propagate backward through the stack.
    fn propagate_backward(&mut self, result: &mut StackProcessResult) {
        let layers: Vec<Layer> = result
            .layer_states
            .keys()
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        for (i, &source_layer) in layers.iter().enumerate() {
            if i + 1 >= layers.len() {
                break;
            }

            let target_layer = layers[i + 1];

            if let Some(bridge) = self
                .bridge_network
                .bridge_between(source_layer, target_layer)
            {
                if let Some(source_state) = result.layer_states.get(&source_layer) {
                    if let Ok(refined_state) = bridge.backward(source_state) {
                        self.stats.total_backward_propagations += 1;

                        let signal =
                            LayerSignal::new(source_layer, target_layer, refined_state.clone());
                        result.signal_trace.push(signal);

                        // Merge refined state with existing
                        if let Some(existing) = result.layer_states.get_mut(&target_layer) {
                            existing.confidence =
                                (existing.confidence + refined_state.confidence) / 2.0;
                            existing.increment_amplification();
                        }
                    }
                }
            }
        }
    }

    /// Propagate forward from current states.
    fn propagate_forward_from_states(&mut self, result: &mut StackProcessResult) {
        let layers: Vec<Layer> = result.layer_states.keys().copied().collect();

        for &source_layer in &layers {
            for &target_layer in &layers {
                if target_layer.number() <= source_layer.number() {
                    continue;
                }

                if let Some(bridge) = self
                    .bridge_network
                    .bridge_between(source_layer, target_layer)
                {
                    if let Some(source_state) = result.layer_states.get(&source_layer) {
                        if let Ok(new_state) = bridge.forward(source_state) {
                            self.stats.total_forward_propagations += 1;

                            // Merge with existing state
                            if let Some(existing) = result.layer_states.get_mut(&target_layer) {
                                existing.confidence = (existing.confidence * 0.7
                                    + new_state.confidence * 0.3)
                                    * self.config.global_amplification;
                                existing.increment_amplification();
                            }
                        }
                    }
                }
            }
        }

        // Recalculate combined confidence
        result.layer_confidences = result
            .layer_states
            .iter()
            .map(|(l, s)| (*l, s.confidence))
            .collect();
        result.combined_confidence = self.calculate_combined_confidence(&result.layer_confidences);
    }

    /// Run amplification across all bridges.
    fn amplify_all_bridges(&mut self, result: &mut StackProcessResult) {
        let bridges = self.bridge_network.bridges().to_vec();

        for bridge in bridges {
            let source = bridge.source_layer();
            let target = bridge.target_layer();

            let (source_state, target_state) = {
                let s = result.layer_states.get(&source).cloned();
                let t = result.layer_states.get(&target).cloned();
                match (s, t) {
                    (Some(s), Some(t)) => (s, t),
                    _ => continue,
                }
            };

            let layer_config = self.config.layer_configs.get(&source);
            let max_iterations = layer_config
                .map(|c| c.max_amplification_iterations)
                .unwrap_or(10);

            if let Ok(amp_result) = bridge.amplify(&source_state, &target_state, max_iterations) {
                self.stats.total_amplifications += 1;

                // Update states with amplified versions
                result.layer_states.insert(source, amp_result.up_state);
                result.layer_states.insert(target, amp_result.down_state);

                // Track amplification
                result.total_amplification *= amp_result.amplification_factor;
            }
        }

        // Recalculate combined confidence
        result.layer_confidences = result
            .layer_states
            .iter()
            .map(|(l, s)| (*l, s.confidence))
            .collect();
        result.combined_confidence = self.calculate_combined_confidence(&result.layer_confidences);
    }

    /// Calculate combined confidence using multiplicative formula.
    fn calculate_combined_confidence(&self, confidences: &HashMap<Layer, f32>) -> f32 {
        if confidences.is_empty() {
            return 0.0;
        }

        // Multiplicative combination (geometric mean with amplification)
        let product: f32 = confidences.values().product();
        let n = confidences.len() as f32;
        product.powf(1.0 / n) * self.config.global_amplification
    }

    /// Update statistics after processing.
    fn update_stats(&mut self, result: &StackProcessResult) {
        // Update running average
        let current_count = self.stats.convergence_count + self.stats.non_convergence_count;
        if current_count > 0 {
            self.stats.average_confidence = (self.stats.average_confidence
                * (current_count - 1) as f32
                + result.combined_confidence)
                / current_count as f32;
        } else {
            self.stats.average_confidence = result.combined_confidence;
        }

        // Update max confidence
        if result.combined_confidence > self.stats.max_confidence_achieved {
            self.stats.max_confidence_achieved = result.combined_confidence;
        }
    }

    /// Inject a state directly into a layer (for testing or external input).
    pub fn inject_state(&mut self, state: LayerState) {
        self.current_states.insert(state.layer, state);
    }

    /// Get all current layer states.
    pub fn all_states(&self) -> &HashMap<Layer, LayerState> {
        &self.current_states
    }
}

impl Default for LayerStack {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for LayerStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerStack")
            .field("config", &self.config)
            .field("stats", &self.stats)
            .field("current_states", &self.current_states.len())
            .field("bridge_count", &self.bridge_network.bridges().len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_creation() {
        let stack = LayerStack::new();
        assert_eq!(stack.config().max_stack_iterations, 5);
        assert!(stack.bridge_network().bridges().is_empty());
    }

    #[test]
    fn test_stack_config() {
        let config = LayerStackConfig::new()
            .with_global_amplification(1.5)
            .with_max_iterations(10);

        let stack = LayerStack::with_config(config);
        assert_eq!(stack.config().global_amplification, 1.5);
        assert_eq!(stack.config().max_stack_iterations, 10);
    }

    #[test]
    fn test_state_injection() {
        let mut stack = LayerStack::new();
        let state = LayerState::new(Layer::GaiaConsciousness, "test".to_string());

        stack.inject_state(state);

        assert!(stack.get_current_state(Layer::GaiaConsciousness).is_some());
    }

    #[test]
    fn test_forward_processing_no_bridges() {
        let mut stack = LayerStack::new();
        let input = LayerState::with_confidence(Layer::BasePhysics, "input".to_string(), 0.8);

        let result = stack.process_forward(input);

        // Without bridges, only the input layer should have a state
        assert_eq!(result.layer_states.len(), 1);
        assert!(result.layer_states.contains_key(&Layer::BasePhysics));
    }

    #[test]
    fn test_stack_result() {
        let result = StackProcessResult::empty();
        assert!(!result.is_successful(0.5));
        assert!(result.layer_states.is_empty());
    }

    #[test]
    fn test_combined_confidence_calculation() {
        let stack = LayerStack::new();
        let mut confidences = HashMap::new();
        confidences.insert(Layer::BasePhysics, 0.8);
        confidences.insert(Layer::ExtendedPhysics, 0.8);

        let combined = stack.calculate_combined_confidence(&confidences);
        // Geometric mean of 0.8, 0.8 = 0.8, times 1.1 amplification = 0.88
        assert!((combined - 0.88).abs() < 0.01);
    }
}
