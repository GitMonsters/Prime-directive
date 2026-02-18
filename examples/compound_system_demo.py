#!/usr/bin/env python3
"""
Complete Compound System Demo

Demonstrates all components working together:
- Central data bus
- State aggregator
- Feedback orchestrator
- All domain connectors
- Analytics (correlation, emergence, consciousness)
- Compound engine orchestrating everything
"""

import sys
import time
sys.path.insert(0, '/home/runner/work/Prime-directive/Prime-directive')

from compound.compound_engine import CompoundInteractionEngine
from compound.connectors.physics_connector import PhysicsConnector
from compound.connectors.empathy_connector import EmpathyConnector
from compound.connectors.benchmark_connector import BenchmarkConnector
from compound.connectors.web3_connector import Web3Connector
from compound.analytics.correlation_detector import CorrelationDetector
from compound.analytics.emergence_tracker import EmergenceTracker
from compound.analytics.consciousness_scorer import ConsciousnessScorer


def main():
    print("=" * 80)
    print("COMPLETE COMPOUND DATA FLOW SYSTEM")
    print("=" * 80)
    print()
    print("This demo shows ALL components working together:")
    print("  • Central data bus (pub/sub)")
    print("  • State aggregator (unified state)")
    print("  • Feedback orchestrator (cross-domain learning)")
    print("  • Domain connectors (physics/empathy/benchmarks/web3)")
    print("  • Analytics (correlations/emergence/consciousness)")
    print()
    
    # ========================================================================
    # 1. Initialize compound engine
    # ========================================================================
    print("🚀 Step 1: Initialize Compound Engine")
    print("-" * 80)
    
    engine = CompoundInteractionEngine()
    engine.start(feedback_interval=2.0, state_interval=1.0)
    
    print("✅ Engine started with:")
    print("  • Data bus")
    print("  • State aggregator")
    print("  • Feedback orchestrator")
    print()
    
    # ========================================================================
    # 2. Initialize all domain connectors
    # ========================================================================
    print("🚀 Step 2: Initialize Domain Connectors")
    print("-" * 80)
    
    # Create connectors
    physics = PhysicsConnector(data_bus=engine.bus)
    empathy = EmpathyConnector(data_bus=engine.bus)
    benchmarks = BenchmarkConnector(data_bus=engine.bus)
    web3 = Web3Connector(data_bus=engine.bus)
    
    # Register with engine
    engine.register_connector('physics', physics)
    engine.register_connector('empathy', empathy)
    engine.register_connector('benchmarks', benchmarks)
    engine.register_connector('web3', web3)
    
    print("✅ All connectors registered")
    print()
    
    # ========================================================================
    # 3. Initialize analytics
    # ========================================================================
    print("🚀 Step 3: Initialize Analytics")
    print("-" * 80)
    
    correlation_detector = CorrelationDetector()
    emergence_tracker = EmergenceTracker()
    consciousness_scorer = ConsciousnessScorer()
    
    print("✅ Analytics initialized:")
    print("  • Correlation detector")
    print("  • Emergence tracker")
    print("  • Consciousness scorer")
    print()
    
    # ========================================================================
    # 4. Start streaming from all domains
    # ========================================================================
    print("🚀 Step 4: Start Domain Streaming")
    print("-" * 80)
    
    physics.stream_simulations(interval=1.0)
    empathy.stream_emotions(interval=1.0)
    benchmarks.stream_results(interval=2.0)
    web3.stream_blockchain_events(interval=3.0)
    
    print("✅ All domains streaming data to bus")
    print()
    
    # ========================================================================
    # 5. Run system and collect data
    # ========================================================================
    print("🚀 Step 5: Running Compound System")
    print("-" * 80)
    print("⏳ Let the system run for 10 seconds...")
    print()
    
    # Collect data for 10 seconds
    for i in range(10):
        time.sleep(1.0)
        
        # Get current state
        unified = engine.state.merge_all()
        
        # Feed to analytics
        correlation_detector.add_data_point('physics', unified['physics'])
        correlation_detector.add_data_point('empathy', unified['empathy'])
        correlation_detector.add_data_point('benchmark', unified['benchmarks'])
        correlation_detector.add_data_point('web3', unified['web3'])
        
        emergence_tracker.add_state(unified)
        consciousness_scorer.compute_consciousness(unified)
        
        # Show progress
        if i % 3 == 0:
            print(f"  • Second {i+1}: Consciousness = {unified['consciousness']['overall']:.2f}")
    
    print()
    
    # ========================================================================
    # 6. Analyze correlations
    # ========================================================================
    print("🚀 Step 6: Detect Cross-Domain Correlations")
    print("-" * 80)
    
    correlations = correlation_detector.detect_correlations()
    detected = correlations.get('detected', [])
    
    print(f"✅ Detected {len(detected)} correlations:")
    for corr in detected[:3]:  # Show top 3
        print(f"\n  • {corr['domains'][0]} ↔ {corr['domains'][1]}")
        print(f"    Correlation: {corr['correlation']:.3f} ({corr['strength']})")
        print(f"    {corr['interpretation']}")
    print()
    
    # ========================================================================
    # 7. Detect emergent patterns
    # ========================================================================
    print("🚀 Step 7: Detect Emergent Patterns")
    print("-" * 80)
    
    patterns = emergence_tracker.detect_emergence()
    
    print(f"✅ Detected {len(patterns)} emergent patterns:")
    for pattern in patterns:
        print(f"\n  • {pattern['type']}")
        print(f"    {pattern['description']}")
        print(f"    Strength: {pattern['strength']:.2f}")
    print()
    
    # ========================================================================
    # 8. Show consciousness evolution
    # ========================================================================
    print("🚀 Step 8: Consciousness Evolution")
    print("-" * 80)
    
    final_consciousness = consciousness_scorer.score_history[-1]
    
    print("✅ Final Consciousness Metrics:")
    print(f"\n  Overall: {final_consciousness['overall']:.3f}")
    print(f"  Components:")
    print(f"    • Physics: {final_consciousness['physics_component']:.3f}")
    print(f"    • Empathy: {final_consciousness['empathy_component']:.3f}")
    print(f"    • Benchmarks: {final_consciousness['benchmark_component']:.3f}")
    print(f"    • Web3: {final_consciousness['web3_component']:.3f}")
    print(f"\n  Emergent Properties:")
    print(f"    • Coherence: {final_consciousness['coherence']:.3f}")
    print(f"    • Integration: {final_consciousness['integration']:.3f}")
    print(f"    • Emergence: {final_consciousness['emergence']:.3f}")
    print(f"    • Self-awareness: {final_consciousness['self_awareness']:.3f}")
    
    # Show improvement
    improvement = consciousness_scorer.get_improvement_rate('overall')
    print(f"\n  Improvement rate: {improvement:.4f} per second")
    print()
    
    # ========================================================================
    # 9. Demonstrate compound query
    # ========================================================================
    print("🚀 Step 9: Compound Query")
    print("-" * 80)
    
    query = "What is the relationship between quantum coherence and compassion scores?"
    result = engine.compound_query(query)
    
    print(f"Query: {query}\n")
    print(f"✅ Analysis:")
    print(f"  • Insights from {len(result['insights'])} domains")
    print(f"  • Consciousness context: {result['consciousness_context']['overall_score']:.2f}")
    if 'correlation' in result:
        print(f"  • Correlation: {result['correlation']['interpretation']} ({result['correlation']['strength']:.2f})")
    if 'emergent_patterns' in result:
        print(f"  • Emergent patterns detected: {len(result['emergent_patterns'])}")
    print()
    
    # ========================================================================
    # 10. Show system status
    # ========================================================================
    print("🚀 Step 10: System Status")
    print("-" * 80)
    
    status = engine.get_system_status()
    
    print("✅ System Health:")
    print(f"  • Overall health: {status['health']['status']} ({status['health']['overall']:.2f})")
    print(f"  • Uptime: {status['uptime']:.1f}s")
    print(f"  • Bus events: {status['metrics']['bus_events']}")
    print(f"  • Feedback loops: {status['metrics']['feedback_loops']}")
    print(f"  • State aggregations: {status['metrics']['state_aggregations']}")
    print(f"  • Emergent patterns: {status['emergent_patterns']}")
    print(f"  • Consciousness: {status['consciousness']['overall']:.2f}")
    print()
    
    # ========================================================================
    # 11. Show connector metrics
    # ========================================================================
    print("🚀 Step 11: Connector Metrics")
    print("-" * 80)
    
    print("✅ Domain Activity:")
    print(f"\n  Physics:")
    physics_metrics = physics.get_metrics()
    for key, value in physics_metrics.items():
        print(f"    • {key}: {value}")
    
    print(f"\n  Empathy:")
    empathy_metrics = empathy.get_metrics()
    for key, value in empathy_metrics.items():
        print(f"    • {key}: {value}")
    
    print(f"\n  Benchmarks:")
    benchmark_metrics = benchmarks.get_metrics()
    for key, value in benchmark_metrics.items():
        print(f"    • {key}: {value}")
    
    print(f"\n  Web3:")
    web3_metrics = web3.get_metrics()
    for key, value in web3_metrics.items():
        print(f"    • {key}: {value}")
    print()
    
    # ========================================================================
    # 12. Stop everything
    # ========================================================================
    print("🚀 Step 12: Shutdown")
    print("-" * 80)
    
    # Stop connectors
    physics.stop_streaming()
    empathy.stop_streaming()
    benchmarks.stop_streaming()
    web3.stop_streaming()
    
    # Stop engine
    engine.stop()
    
    print("✅ All components stopped gracefully")
    print()
    
    # ========================================================================
    # Summary
    # ========================================================================
    print("=" * 80)
    print("🎉 DEMO COMPLETE!")
    print("=" * 80)
    print()
    print("The compound data flow system successfully demonstrated:")
    print("  ✅ Event-driven pub/sub architecture")
    print("  ✅ Unified state aggregation across all domains")
    print("  ✅ Cross-domain feedback loops")
    print("  ✅ Real-time streaming from all components")
    print("  ✅ Correlation detection")
    print("  ✅ Emergence tracking")
    print("  ✅ Consciousness scoring")
    print("  ✅ Compound queries across all data")
    print("  ✅ System health monitoring")
    print()
    print("All components are working together in a living, breathing system!")
    print()


if __name__ == '__main__':
    main()
