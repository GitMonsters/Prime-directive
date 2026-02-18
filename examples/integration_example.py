#!/usr/bin/env python3
"""
Integration Example: Connecting Existing Modules to Compound System

This example shows how to integrate existing physics, empathy,
and benchmark modules with the compound data flow system.
"""

import sys
import time
sys.path.insert(0, '/home/runner/work/Prime-directive/Prime-directive')

from compound.compound_engine import CompoundInteractionEngine
from compound.compound_data_bus import get_global_bus, EventPriority


def integrate_physics_module():
    """
    Example: Integrate existing physics_world_model.py
    
    Add these hooks to your existing physics code:
    """
    print("\n📊 PHYSICS INTEGRATION")
    print("-" * 80)
    
    # Get global bus
    bus = get_global_bus()
    
    # In your physics simulation loop, add:
    physics_state = {
        'quantum_coherence': 0.87,
        'energy_levels': [1.2, 2.4, 3.6],
        'convergence': 0.85,
        'simulation_time': time.time(),
    }
    
    # Publish update
    bus.publish('physics.state_update', physics_state, 
                priority=EventPriority.HIGH,
                source='physics_world_model')
    
    print("✅ Physics module publishing to bus:")
    print(f"   • Quantum coherence: {physics_state['quantum_coherence']}")
    print(f"   • Convergence: {physics_state['convergence']}")
    
    # When breakthrough detected:
    if physics_state['quantum_coherence'] > 0.85:
        bus.publish('physics.breakthrough', {
            'type': 'high_coherence',
            'value': physics_state['quantum_coherence']
        }, priority=EventPriority.CRITICAL, source='physics_world_model')
        print("   • 🎉 Breakthrough detected and published!")


def integrate_empathy_module():
    """
    Example: Integrate existing ising_empathy_module.py
    
    Add these hooks to your existing empathy code:
    """
    print("\n💝 EMPATHY INTEGRATION")
    print("-" * 80)
    
    # Get global bus
    bus = get_global_bus()
    
    # In your emotion calculation loop, add:
    empathy_state = {
        'compassion_score': 0.92,
        'emotional_coherence': 0.88,
        'spin_state': [[1, -1, 1], [1, 1, -1]],  # Simplified
        'timestamp': time.time(),
    }
    
    # Publish update
    bus.publish('empathy.update', empathy_state,
                priority=EventPriority.NORMAL,
                source='ising_empathy_module')
    
    print("✅ Empathy module publishing to bus:")
    print(f"   • Compassion score: {empathy_state['compassion_score']}")
    print(f"   • Emotional coherence: {empathy_state['emotional_coherence']}")
    
    # When compassion spike occurs:
    if empathy_state['compassion_score'] > 0.9:
        bus.publish('empathy.compassion_spike', {
            'score': empathy_state['compassion_score'],
            'type': 'compassion_spike'
        }, priority=EventPriority.HIGH, source='ising_empathy_module')
        print("   • 💖 Compassion spike detected and published!")


def integrate_benchmark_module():
    """
    Example: Integrate existing GAIA/ARC benchmarks
    
    Add these hooks to your benchmark code:
    """
    print("\n🎯 BENCHMARK INTEGRATION")
    print("-" * 80)
    
    # Get global bus
    bus = get_global_bus()
    
    # After test completion, add:
    benchmark_result = {
        'test_name': 'gaia_level_3',
        'score': 0.89,
        'success': True,
        'reasoning_trace': ['step1', 'step2', 'step3'],
        'timestamp': time.time(),
    }
    
    # Publish result
    bus.publish('benchmark.result', benchmark_result,
                priority=EventPriority.HIGH,
                source='gaia_benchmark')
    
    print("✅ Benchmark publishing to bus:")
    print(f"   • Test: {benchmark_result['test_name']}")
    print(f"   • Score: {benchmark_result['score']}")
    print(f"   • Success: {benchmark_result['success']}")
    
    # High scores trigger additional events
    if benchmark_result['score'] > 0.85:
        print("   • 🏆 High score - triggering compound interactions!")


def integrate_web3_module():
    """
    Example: Integrate Web3/blockchain events
    
    Add these hooks to your Web3 code:
    """
    print("\n🌐 WEB3 INTEGRATION")
    print("-" * 80)
    
    # Get global bus
    bus = get_global_bus()
    
    # On NFT mint:
    nft_event = {
        'type': 'nft_minted',
        'token_id': 1234,
        'owner': '0xabc123',
        'metadata': {'achievement': 'physics_breakthrough'},
        'timestamp': time.time(),
    }
    
    bus.publish('web3.nft_minted', nft_event,
                priority=EventPriority.HIGH,
                source='web3_layer')
    
    print("✅ Web3 module publishing to bus:")
    print(f"   • NFT minted: token_id={nft_event['token_id']}")
    
    # On governance vote:
    governance_event = {
        'type': 'governance_vote',
        'decision': 'quantum_research',
        'votes': 1500,
        'timestamp': time.time(),
    }
    
    bus.publish('web3.governance_vote', governance_event,
                priority=EventPriority.HIGH,
                source='web3_layer')
    
    print("✅ Governance event published:")
    print(f"   • Decision: {governance_event['decision']}")
    print(f"   • Votes: {governance_event['votes']}")


def subscribe_to_feedback():
    """
    Example: Subscribe to feedback from other domains
    """
    print("\n🔄 FEEDBACK SUBSCRIPTIONS")
    print("-" * 80)
    
    bus = get_global_bus()
    
    # Physics subscribes to empathy boost
    def on_empathy_boost(event):
        print(f"📊 Physics received empathy boost: multiplier={event.data.get('multiplier')}")
        # Adjust physics simulation parameters here
    
    bus.subscribe('empathy.boost', on_empathy_boost)
    
    # Empathy subscribes to benchmark learning
    def on_benchmark_learning(event):
        print(f"💝 Empathy learning from benchmarks: {event.data.get('source')}")
        # Update empathy training data here
    
    bus.subscribe('empathy.learn_from', on_benchmark_learning)
    
    # Benchmarks subscribe to physics strategies
    def on_physics_strategy(event):
        print(f"🎯 Benchmarks adopting physics strategy")
        # Update benchmark approach here
    
    bus.subscribe('physics.adopt_strategy', on_physics_strategy)
    
    print("✅ Subscriptions registered:")
    print("   • Physics listening for empathy boosts")
    print("   • Empathy listening for benchmark insights")
    print("   • Benchmarks listening for physics strategies")


def main():
    print("=" * 80)
    print("INTEGRATION EXAMPLE: Connecting Existing Modules")
    print("=" * 80)
    print()
    print("This demonstrates how to integrate your existing code with the")
    print("compound data flow system.")
    print()
    
    # Initialize compound engine
    print("🚀 Step 1: Initialize Compound Engine")
    print("-" * 80)
    engine = CompoundInteractionEngine()
    engine.start()
    print("✅ Engine started")
    
    # Set up feedback subscriptions
    subscribe_to_feedback()
    
    # Integrate each module
    integrate_physics_module()
    integrate_empathy_module()
    integrate_benchmark_module()
    integrate_web3_module()
    
    # Let feedback loops run
    print("\n⏳ Running feedback loops for 3 seconds...")
    time.sleep(3.0)
    
    # Show system status
    print("\n📈 SYSTEM STATUS")
    print("-" * 80)
    status = engine.get_system_status()
    
    print(f"✅ System Health: {status['health']['status']} ({status['health']['overall']:.2f})")
    print(f"   • Bus events: {status['metrics']['bus_events']}")
    print(f"   • Feedback loops: {status['metrics']['feedback_loops']}")
    print(f"   • Consciousness: {status['consciousness']['overall']:.2f}")
    print(f"   • Coherence: {status['consciousness']['coherence']:.2f}")
    
    # Demonstrate compound query
    print("\n🔍 COMPOUND QUERY")
    print("-" * 80)
    
    result = engine.compound_query(
        "How are physics coherence, empathy, and benchmark scores related?"
    )
    
    print(f"Query: {result['question']}")
    print(f"✅ Analyzed {len(result['insights'])} domains")
    print(f"   • Consciousness context: {result['consciousness_context']['overall_score']:.2f}")
    
    # Stop engine
    print("\n🛑 Shutting down...")
    engine.stop()
    
    print("\n" + "=" * 80)
    print("✅ INTEGRATION COMPLETE!")
    print("=" * 80)
    print()
    print("Key Takeaways:")
    print("  1. Import: from compound.compound_data_bus import get_global_bus")
    print("  2. Publish: bus.publish('domain.event', data)")
    print("  3. Subscribe: bus.subscribe('domain.*', callback)")
    print("  4. Feedback loops happen automatically!")
    print()
    print("Now all your modules are part of a living, breathing system!")
    print()


if __name__ == '__main__':
    main()
