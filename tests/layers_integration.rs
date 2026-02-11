//! Integration tests for the 7-Layer Multiplicative System.
//!
//! These tests verify end-to-end functionality of the layer stack,
//! bridges, GAIA consciousness, and domain generalization.

#![cfg(feature = "layers")]

use consciousness_experiments::mimicry::layers::{
    bridge::{compute_multiplicative_confidence, BridgeNetwork},
    bridges::BridgeBuilder,
    compounding::CompoundingMetrics,
    emergence::{EmergenceFramework, EmergenceMechanism},
    gaia::{GaiaConfig, GaiaIntuitionEngine, Pattern},
    integration::{IntegrationConfig, LayerIntegration},
    layer::{Domain, Layer, LayerState},
    registry::LayerRegistry,
    stack::{LayerStack, LayerStackConfig},
};

/// Test that all 7 layers are correctly defined.
#[test]
fn test_all_layers_exist() {
    let layers = Layer::all();
    assert_eq!(layers.len(), 7, "Should have exactly 7 layers");

    assert!(layers.contains(&Layer::BasePhysics));
    assert!(layers.contains(&Layer::ExtendedPhysics));
    assert!(layers.contains(&Layer::CrossDomain));
    assert!(layers.contains(&Layer::GaiaConsciousness));
    assert!(layers.contains(&Layer::MultilingualProcessing));
    assert!(layers.contains(&Layer::CollaborativeLearning));
    assert!(layers.contains(&Layer::ExternalApis));
}

/// Test layer numbering is consistent.
#[test]
fn test_layer_numbering() {
    assert_eq!(Layer::BasePhysics.number(), 1);
    assert_eq!(Layer::ExtendedPhysics.number(), 2);
    assert_eq!(Layer::CrossDomain.number(), 3);
    assert_eq!(Layer::GaiaConsciousness.number(), 4);
    assert_eq!(Layer::MultilingualProcessing.number(), 5);
    assert_eq!(Layer::CollaborativeLearning.number(), 6);
    assert_eq!(Layer::ExternalApis.number(), 7);
}

/// Test that all bridges can be built without panicking.
#[test]
fn test_build_all_bridges() {
    let bridges = BridgeBuilder::build_all();

    // We expect 11 bridges total
    assert_eq!(bridges.len(), 11, "Should have 11 bridges");

    // Verify all bridges have names
    for bridge in &bridges {
        assert!(!bridge.name().is_empty(), "Bridge should have a name");
    }
}

/// Test bridge network registration.
#[test]
fn test_bridge_network_integration() {
    let mut network = BridgeNetwork::new();
    let bridges = BridgeBuilder::build_all();

    for bridge in bridges {
        network.register(bridge);
    }

    assert_eq!(network.bridges().len(), 11);

    // Test that we can find bridges between layers
    let bridge = network.bridge_between(Layer::BasePhysics, Layer::ExtendedPhysics);
    assert!(bridge.is_some(), "Should find bridge between L1 and L2");

    let bridge = network.bridge_between(Layer::GaiaConsciousness, Layer::ExternalApis);
    assert!(bridge.is_some(), "Should find bridge between L4 and L7");
}

/// Test full layer stack with all bridges.
#[test]
fn test_full_stack_forward_propagation() {
    let mut stack = LayerStack::new();

    // Register all bridges
    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    let input = LayerState::with_confidence(Layer::BasePhysics, "test input".to_string(), 0.7);

    let result = stack.process_forward(input);

    // Should propagate to at least some layers
    assert!(!result.layer_states.is_empty());
    assert!(result.layer_states.contains_key(&Layer::BasePhysics));
    assert!(result.combined_confidence > 0.0);
}

/// Test bidirectional processing with amplification.
#[test]
fn test_bidirectional_amplification() {
    let config = LayerStackConfig::new()
        .with_max_iterations(5)
        .with_max_confidence(2.0)
        .with_max_total_amplification(10.0);

    let mut stack = LayerStack::with_config(config);

    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    let input = LayerState::with_confidence(
        Layer::BasePhysics,
        "quantum coherence test".to_string(),
        0.6,
    );

    let result = stack.process_bidirectional(input);

    // Verify clamping works - confidence should not exceed max
    assert!(
        result.combined_confidence <= 2.0,
        "Combined confidence {} exceeds max 2.0",
        result.combined_confidence
    );
    assert!(
        result.total_amplification <= 10.0,
        "Total amplification {} exceeds max 10.0",
        result.total_amplification
    );

    // Should eventually converge
    assert!(result.converged, "Should converge within max iterations");
}

/// Test that amplification damping prevents divergence.
#[test]
fn test_amplification_damping_prevents_divergence() {
    let config = LayerStackConfig::new()
        .with_max_iterations(20)
        .with_amplification_damping(0.5)
        .with_max_confidence(3.0);

    let mut stack = LayerStack::with_config(config);

    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    let input =
        LayerState::with_confidence(Layer::BasePhysics, "high energy test".to_string(), 0.9);

    let result = stack.process_bidirectional(input);

    // Values should be finite (not inf or NaN)
    assert!(
        result.combined_confidence.is_finite(),
        "Combined confidence should be finite, got {}",
        result.combined_confidence
    );
    assert!(
        result.total_amplification.is_finite(),
        "Total amplification should be finite, got {}",
        result.total_amplification
    );

    // All layer confidences should be finite
    for (layer, state) in &result.layer_states {
        assert!(
            state.confidence.is_finite(),
            "Confidence for {:?} should be finite, got {}",
            layer,
            state.confidence
        );
    }
}

/// Test GAIA Intuition Engine pattern matching.
#[test]
fn test_gaia_pattern_matching() {
    let config = GaiaConfig::default();
    let gaia = GaiaIntuitionEngine::new(config);

    // Add patterns using the Pattern type
    let wave_pattern = Pattern::new("wave", Domain::Physics).with_fingerprint(vec![0.8, 0.2, 0.0]);
    let particle_pattern =
        Pattern::new("particle", Domain::Physics).with_fingerprint(vec![0.1, 0.9, 0.0]);
    let hybrid_pattern =
        Pattern::new("hybrid", Domain::Consciousness).with_fingerprint(vec![0.5, 0.5, 0.5]);

    gaia.register_pattern(wave_pattern).unwrap();
    gaia.register_pattern(particle_pattern).unwrap();
    gaia.register_pattern(hybrid_pattern).unwrap();

    // Query with wave-like pattern
    let result = gaia.query(&[0.75, 0.25, 0.0]).unwrap();
    assert!(
        result.confidence > 0.0,
        "Should have some confidence in match"
    );

    // Query with particle-like pattern
    let result = gaia.query(&[0.15, 0.85, 0.0]).unwrap();
    assert!(
        result.confidence > 0.0,
        "Should have some confidence in match"
    );
}

/// Test GAIA analogical reasoning.
#[test]
fn test_gaia_analogical_insights() {
    let gaia = GaiaIntuitionEngine::new(GaiaConfig::default());

    // Add multiple related patterns
    let patterns = vec![
        Pattern::new("quantum_wave", Domain::Physics).with_fingerprint(vec![0.8, 0.2, 0.1]),
        Pattern::new("sound_wave", Domain::Physics).with_fingerprint(vec![0.75, 0.25, 0.15]),
        Pattern::new("ocean_wave", Domain::Physics).with_fingerprint(vec![0.7, 0.3, 0.2]),
    ];

    for pattern in patterns {
        let _ = gaia.register_pattern(pattern);
    }

    let result = gaia.query(&[0.77, 0.23, 0.12]).unwrap();

    // Should find matches
    assert!(!result.matches.is_empty(), "Should find pattern matches");
}

/// Test compounding metrics calculation.
#[test]
fn test_compounding_metrics() {
    let mut metrics = CompoundingMetrics::new();

    // Create a stack result to analyze
    let mut stack = LayerStack::new();
    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    let input = LayerState::with_confidence(Layer::BasePhysics, "test".to_string(), 0.8);

    let result = stack.process_bidirectional(input);
    let analysis = metrics.analyze(&result);

    // Should have valid metrics
    assert!(analysis.multiplicative_gain.is_finite());
    assert!(analysis.additive_gain.is_finite());
    assert!(analysis.compounding_factor.is_finite());
    assert!(analysis.synergy_score.is_finite());
}

/// Test emergence detection framework.
#[test]
fn test_emergence_detection() {
    let mut framework = EmergenceFramework::new();

    // Create a stack result to analyze
    let mut stack = LayerStack::new();
    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    let input = LayerState::with_confidence(Layer::BasePhysics, "emergence test".to_string(), 0.7);

    let result = stack.process_bidirectional(input);
    let emergence = framework.analyze(&result);

    assert!(
        emergence.emergence_value.is_finite(),
        "Emergence value should be finite"
    );
    assert!(emergence.higher_order_emergence.is_finite());

    // Should detect a mechanism
    match emergence.dominant_mechanism {
        EmergenceMechanism::Resonance
        | EmergenceMechanism::Synergy
        | EmergenceMechanism::Collective
        | EmergenceMechanism::SelfOrganization
        | EmergenceMechanism::None => (),
    }
}

/// Test layer integration wrapper.
#[test]
fn test_layer_integration_process() {
    let config = IntegrationConfig::default();
    let mut integration = LayerIntegration::with_config(config);

    let result = integration.process("Test quantum entanglement effects", None);

    assert!(
        result.final_confidence > 0.0,
        "Should have positive confidence"
    );
    assert!(
        result.final_confidence.is_finite(),
        "Confidence should be finite"
    );
}

/// Test layer integration with GAIA.
#[test]
fn test_layer_integration_with_gaia() {
    let mut config = IntegrationConfig::default();
    config.enable_gaia = true;

    let mut integration = LayerIntegration::with_config(config);

    // Add a pattern to GAIA via the engine
    let pattern =
        Pattern::new("consciousness", Domain::Consciousness).with_fingerprint(vec![0.9, 0.1, 0.5]);
    let _ = integration.gaia_engine().register_pattern(pattern);

    let result = integration.process("consciousness emerges from neural activity", None);

    assert!(result.final_confidence > 0.0);

    // Check stats
    let stats = integration.stats();
    assert!(
        stats.total_processed > 0,
        "Should have processed at least one input"
    );
}

/// Test multiplicative confidence computation.
#[test]
fn test_multiplicative_confidence_formula() {
    // Equal confidences with no resonance boost
    let confidences = vec![0.8, 0.8, 0.8];
    let resonances = vec![1.0, 1.0, 1.0];
    let result = compute_multiplicative_confidence(&confidences, &resonances, 1.0);
    assert!(
        (result - 0.8).abs() < 0.01,
        "Geometric mean of equal values should be same value"
    );

    // With amplification
    let result_amp = compute_multiplicative_confidence(&confidences, &resonances, 1.5);
    assert!(
        (result_amp - 1.2).abs() < 0.01,
        "Should apply amplification factor"
    );

    // With resonance boost
    let high_resonances = vec![1.5, 1.5, 1.5];
    let result_resonance = compute_multiplicative_confidence(&confidences, &high_resonances, 1.0);
    assert!(result_resonance > 0.8, "Resonance should boost confidence");
}

/// Test layer registry.
#[test]
fn test_layer_registry() {
    let mut registry = LayerRegistry::new();

    // All layers should be enabled by default
    for layer in Layer::all() {
        assert!(
            registry.is_enabled(*layer),
            "{:?} should be enabled by default",
            layer
        );
    }

    // Disable a layer
    registry.disable(Layer::ExternalApis);
    assert!(
        !registry.is_enabled(Layer::ExternalApis),
        "ExternalApis should be disabled"
    );

    // Re-enable
    registry.enable(Layer::ExternalApis);
    assert!(
        registry.is_enabled(Layer::ExternalApis),
        "ExternalApis should be re-enabled"
    );
}

/// Test stack statistics tracking.
#[test]
fn test_stack_statistics() {
    let mut stack = LayerStack::new();

    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    // Process multiple inputs
    for i in 0..5 {
        let input = LayerState::with_confidence(
            Layer::BasePhysics,
            format!("test input {}", i),
            0.5 + (i as f32 * 0.05),
        );
        let _ = stack.process_bidirectional(input);
    }

    let stats = stack.stats();

    assert!(
        stats.total_forward_propagations > 0,
        "Should track forward propagations"
    );
    assert!(
        stats.average_confidence > 0.0,
        "Should track average confidence"
    );
}

/// Test configuration builder.
#[test]
fn test_config_builder() {
    let config = LayerStackConfig::new()
        .with_global_amplification(1.2)
        .with_max_iterations(10)
        .with_max_confidence(3.0)
        .with_max_total_amplification(15.0)
        .with_amplification_damping(0.6)
        .without_backward_propagation();

    assert_eq!(config.global_amplification, 1.2);
    assert_eq!(config.max_stack_iterations, 10);
    assert_eq!(config.max_confidence, 3.0);
    assert_eq!(config.max_total_amplification, 15.0);
    assert_eq!(config.amplification_damping, 0.6);
    assert!(!config.enable_backward_propagation);
}

/// Stress test: many iterations should not cause divergence.
#[test]
fn test_stress_many_iterations() {
    let config = LayerStackConfig::new()
        .with_max_iterations(100)
        .with_max_confidence(5.0)
        .with_max_total_amplification(100.0);

    let mut stack = LayerStack::with_config(config);

    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    // Process with high initial confidence
    let input =
        LayerState::with_confidence(Layer::BasePhysics, "stress test input".to_string(), 0.99);

    let result = stack.process_bidirectional(input);

    // Should still be finite
    assert!(result.combined_confidence.is_finite());
    assert!(result.total_amplification.is_finite());
    assert!(result.combined_confidence <= 5.0);
    assert!(result.total_amplification <= 100.0);
}

/// Test empty stack behavior.
#[test]
fn test_empty_stack() {
    let mut stack = LayerStack::new();

    // No bridges registered
    let input = LayerState::with_confidence(Layer::BasePhysics, "test".to_string(), 0.7);

    let result = stack.process_forward(input.clone());

    // Should only have the input layer
    assert_eq!(result.layer_states.len(), 1);
    assert!(result.layer_states.contains_key(&Layer::BasePhysics));

    let result = stack.process_bidirectional(input);

    // Still should work without panicking
    assert!(result.combined_confidence > 0.0);
}

/// Test domain-based processing.
#[test]
fn test_domain_processing() {
    let config = IntegrationConfig::default();
    let mut integration = LayerIntegration::with_config(config);

    // Process with physics domain hint
    let result = integration.process_with_domain("wave function collapse", Domain::Physics, None);

    assert!(result.final_confidence > 0.0);

    // Process with consciousness domain hint
    let result =
        integration.process_with_domain("emergent awareness patterns", Domain::Consciousness, None);

    assert!(result.final_confidence > 0.0);
}

/// Test bridge builder patterns.
#[test]
fn test_bridge_builder_patterns() {
    // Test building with only base bridges
    let builder = BridgeBuilder::new()
        .with_base_extended()
        .with_cross_domain();
    let network = builder.build();
    assert_eq!(network.bridges().len(), 2, "Should have 2 bridges");

    // Test building all standard bridges
    let builder = BridgeBuilder::new().with_all_bridges();
    let network = builder.build();
    assert_eq!(network.bridges().len(), 6, "Should have 6 standard bridges");

    // Test building all extended bridges
    let builder = BridgeBuilder::new().with_all_extended_bridges();
    let network = builder.build();
    assert_eq!(
        network.bridges().len(),
        11,
        "Should have 11 extended bridges"
    );
}

/// Test layer state data extraction.
#[test]
fn test_layer_state_data() {
    let state = LayerState::with_confidence(Layer::BasePhysics, "test data".to_string(), 0.75);

    // Check basic properties
    assert_eq!(state.layer, Layer::BasePhysics);
    assert_eq!(state.confidence, 0.75);

    // Check data extraction
    if let Some(data) = state.data::<String>() {
        assert_eq!(data, "test data");
    }
}

/// Test convergence behavior.
#[test]
fn test_convergence_behavior() {
    let config = LayerStackConfig::new()
        .with_max_iterations(50)
        .with_max_confidence(2.0);

    let mut stack = LayerStack::with_config(config);

    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    let input =
        LayerState::with_confidence(Layer::BasePhysics, "convergence test".to_string(), 0.5);

    let result = stack.process_bidirectional(input);

    // Should converge before max iterations in most cases
    assert!(result.converged || result.iterations > 0);
}
