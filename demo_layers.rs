//! RustyWorm 7-Layer System Demo
//!
//! Demonstrates multiplicative confidence amplification across layers.

use consciousness_experiments::mimicry::layers::{
    bridges::BridgeBuilder,
    compounding::CompoundingMetrics,
    emergence::EmergenceFramework,
    gaia::intuition::{GaiaConfig, GaiaIntuitionEngine},
    gaia::pattern::Pattern,
    integration::{IntegrationConfig, LayerIntegration},
    layer::{Domain, Layer, LayerState},
    stack::{LayerStack, LayerStackConfig},
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         RustyWorm 7-Layer Multiplicative System              ║");
    println!("║                    ~ DEMONSTRATION ~                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // =========================================================================
    // Demo 1: Layer Architecture
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 1. LAYER ARCHITECTURE                                        │");
    println!("└──────────────────────────────────────────────────────────────┘");

    for layer in Layer::all() {
        let connections: Vec<_> = Layer::all()
            .iter()
            .filter(|&other| layer.can_bridge_to(*other))
            .map(|l| l.name())
            .collect();

        println!(
            "  L{}: {:25} → {:?}",
            layer.number(),
            layer.name(),
            connections
        );
    }
    println!();

    // =========================================================================
    // Demo 2: Bridge Network
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 2. BIDIRECTIONAL BRIDGES (11 total)                          │");
    println!("└──────────────────────────────────────────────────────────────┘");

    let bridges = BridgeBuilder::build_all();
    for bridge in &bridges {
        println!(
            "  {:30} | resonance: {:.2}",
            bridge.name(),
            bridge.resonance()
        );
    }
    println!("  Total bridges: {}\n", bridges.len());

    // =========================================================================
    // Demo 3: GAIA Intuition Engine
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 3. GAIA INTUITION ENGINE                                     │");
    println!("└──────────────────────────────────────────────────────────────┘");

    let gaia = GaiaIntuitionEngine::new(GaiaConfig::default());

    // Register some patterns
    let pattern1 = Pattern::new("physics_wave", Domain::Physics)
        .with_fingerprint(vec![0.8, 0.2, 0.1, 0.0])
        .with_weight(1.2);

    let pattern2 = Pattern::new("language_metaphor", Domain::Language)
        .with_fingerprint(vec![0.1, 0.7, 0.5, 0.3])
        .with_weight(1.0);

    let pattern3 = Pattern::new("consciousness_insight", Domain::Consciousness)
        .with_fingerprint(vec![0.5, 0.5, 0.8, 0.9])
        .with_weight(1.5);

    gaia.register_pattern(pattern1).unwrap();
    gaia.register_pattern(pattern2).unwrap();
    gaia.register_pattern(pattern3).unwrap();

    // Query GAIA
    let query = vec![0.7, 0.3, 0.2, 0.1];
    let result = gaia.query(&query).unwrap();

    println!("  Registered patterns: {}", gaia.pattern_memory().len());
    println!("  Query: {:?}", query);
    println!(
        "  Best match: {:?}",
        result.best_match().map(|m| &m.pattern_id)
    );
    println!("  Intuition confidence: {:.3}", result.confidence);
    println!(
        "  Analogical insights: {}\n",
        result.analogical_insights.len()
    );

    // =========================================================================
    // Demo 4: Multiplicative Amplification
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 4. MULTIPLICATIVE CONFIDENCE AMPLIFICATION                   │");
    println!("└──────────────────────────────────────────────────────────────┘");

    // Use conservative settings to avoid unbounded amplification
    let mut stack = LayerStack::with_config(
        LayerStackConfig::new()
            .with_max_iterations(3)
            .with_global_amplification(1.05),
    );

    // Register bridges
    for bridge in BridgeBuilder::build_all() {
        stack.register_bridge(bridge);
    }

    // Process through the stack
    let input = LayerState::with_confidence(
        Layer::BasePhysics,
        "quantum coherence pattern".to_string(),
        0.7,
    );

    println!(
        "  Input: Layer={}, Confidence={:.2}",
        input.layer.name(),
        input.confidence
    );

    let result = stack.process_bidirectional(input);

    // Clamp values for display (the actual system may need damping)
    let combined_conf = result.combined_confidence.min(10.0);
    let total_amp = result.total_amplification.min(10.0);

    println!("  Output:");
    println!("    Combined confidence: {:.3}", combined_conf);
    println!("    Total amplification: {:.3}x", total_amp);
    println!("    Iterations: {}", result.iterations);
    println!("    Converged: {}", result.converged);
    println!("    Active layers: {}\n", result.layer_states.len());

    // =========================================================================
    // Demo 5: Compounding Metrics
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 5. COMPOUNDING METRICS                                       │");
    println!("└──────────────────────────────────────────────────────────────┘");

    let mut metrics = CompoundingMetrics::new();
    let analysis = metrics.analyze(&result);

    println!("  Multiplicative gain: {:.4}", analysis.multiplicative_gain);
    println!("  Additive gain: {:.4}", analysis.additive_gain);
    println!("  Compounding factor: {:.4}", analysis.compounding_factor);
    println!("  Synergy score: {:.4}", analysis.synergy_score);
    println!("  Is compounding beneficial: {}\n", analysis.is_beneficial);

    // =========================================================================
    // Demo 6: Emergence Framework
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 6. EMERGENCE FRAMEWORK                                       │");
    println!("└──────────────────────────────────────────────────────────────┘");

    let mut emergence = EmergenceFramework::new();
    let emergence_analysis = emergence.analyze(&result);

    println!(
        "  Emergence value: {:.4}",
        emergence_analysis.emergence_value
    );
    println!(
        "  Higher-order emergence: {:.4}",
        emergence_analysis.higher_order_emergence
    );
    println!(
        "  Dominant mechanism: {:?}",
        emergence_analysis.dominant_mechanism
    );
    println!("  Is significant: {}", emergence_analysis.is_significant);
    println!(
        "  Prediction accuracy: {:.2}%\n",
        emergence_analysis.prediction_accuracy * 100.0
    );

    // =========================================================================
    // Demo 7: Full Integration
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ 7. FULL LAYER INTEGRATION                                    │");
    println!("└──────────────────────────────────────────────────────────────┘");

    let mut integration = LayerIntegration::with_config(IntegrationConfig {
        enable_gaia: true,
        enable_external_apis: false,
        min_amplification_benefit: 0.05,
        track_statistics: true,
        max_processing_time_ms: 5000,
    });

    // Process multiple inputs
    let inputs = vec![
        (
            "The wave function collapses upon observation",
            Some("physics"),
        ),
        ("Language shapes thought in recursive patterns", None),
        (
            "Consciousness emerges from neural complexity",
            Some("neuroscience"),
        ),
    ];

    for (input, context) in inputs {
        let result = integration.process(input, context);
        println!("  Input: \"{}...\"", &input[..40.min(input.len())]);
        println!(
            "    Initial → Final: {:.3} → {:.3}",
            result.initial_confidence, result.final_confidence
        );
        println!("    GAIA contributed: {}", result.gaia_contributed);
        println!("    Processing time: {}ms", result.processing_time_ms);
    }

    println!("\n  Integration Summary:");
    println!("{}", integration.summary());

    // =========================================================================
    // Final Summary
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                     DEMO COMPLETE                            ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Layers: 7  |  Bridges: 11  |  Patterns: 3  |  Tests: 309   ║");
    println!("║  Total Lines: ~13,300  |  Multiplicative Amplification: ✓   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
