#!/usr/bin/env python3
"""
Feedback Orchestrator

Circular feedback system where insights from one component improve others.

Examples:
- High empathy scores → adjust physics simulation parameters
- Physics breakthroughs → trigger new benchmark tests
- Benchmark wins → update empathy training data
- Web3 governance votes → change system priorities
"""

import time
from typing import Dict, Optional, Any

# Handle both package and standalone imports
try:
    from .compound_data_bus import CompoundDataBus, Event, EventPriority, get_global_bus
    from .compound_state import CompoundState
except ImportError:
    from compound_data_bus import CompoundDataBus, Event, EventPriority, get_global_bus
    from compound_state import CompoundState


class FeedbackOrchestrator:
    """
    Orchestrates cross-domain feedback loops.
    
    Analyzes system state and creates interventions that use
    learnings from one domain to improve others.
    """

    def __init__(self, data_bus: Optional[CompoundDataBus] = None,
                 state_aggregator: Optional[CompoundState] = None):
        """
        Initialize feedback orchestrator.
        
        Args:
            data_bus: Data bus for publishing events (uses global if None)
            state_aggregator: State aggregator for reading system state
        """
        self.bus = data_bus or get_global_bus()
        self.state = state_aggregator or CompoundState()
        
        # Feedback history
        self.feedback_history = []
        
        # Feedback rules and thresholds
        self.rules = self._initialize_rules()
        
        # Subscribe to key events
        self._setup_subscriptions()

    def _initialize_rules(self) -> Dict:
        """Initialize feedback rules and thresholds"""
        return {
            'physics_to_empathy': {
                'enabled': True,
                'threshold': 0.8,
                'multiplier': 1.5,
            },
            'empathy_to_benchmarks': {
                'enabled': True,
                'threshold': 0.9,
                'difficulty': 'advanced',
            },
            'benchmarks_to_web3': {
                'enabled': True,
                'threshold': 0.85,
                'reward_multiplier': 2.0,
            },
            'web3_to_physics': {
                'enabled': True,
                'resource_multiplier': 2.0,
            },
        }

    def _setup_subscriptions(self) -> None:
        """Subscribe to events that trigger feedback"""
        # Listen to all domain updates
        self.bus.subscribe('physics.*', self._on_physics_event)
        self.bus.subscribe('empathy.*', self._on_empathy_event)
        self.bus.subscribe('benchmark.*', self._on_benchmark_event)
        self.bus.subscribe('web3.*', self._on_web3_event)

    def _on_physics_event(self, event: Event) -> None:
        """Handle physics events"""
        self.state.update_physics(event.data)

    def _on_empathy_event(self, event: Event) -> None:
        """Handle empathy events"""
        self.state.update_empathy(event.data)

    def _on_benchmark_event(self, event: Event) -> None:
        """Handle benchmark events"""
        self.state.update_benchmark(event.data)

    def _on_web3_event(self, event: Event) -> None:
        """Handle web3 events"""
        self.state.update_web3(event.data)

    def apply_cross_domain_learning(self) -> Dict[str, Any]:
        """
        Use learnings from one domain to improve others.
        
        Returns:
            Summary of feedback actions taken
        """
        actions_taken = []
        
        # Get current state
        unified = self.state.merge_all()
        
        # Physics → Empathy feedback
        if self.rules['physics_to_empathy']['enabled']:
            physics_coherence = unified['physics'].get('quantum_coherence', 0)
            threshold = self.rules['physics_to_empathy']['threshold']
            
            if physics_coherence > threshold:
                multiplier = self.rules['physics_to_empathy']['multiplier']
                self.bus.publish('empathy.boost', {
                    'reason': 'High physics coherence detected',
                    'multiplier': multiplier,
                    'physics_coherence': physics_coherence
                }, priority=EventPriority.HIGH, source='feedback_orchestrator')
                
                actions_taken.append({
                    'type': 'physics_to_empathy',
                    'action': 'boost_empathy',
                    'multiplier': multiplier
                })
        
        # Empathy → Benchmarks feedback
        if self.rules['empathy_to_benchmarks']['enabled']:
            compassion = unified['empathy'].get('compassion_score', 0)
            threshold = self.rules['empathy_to_benchmarks']['threshold']
            
            if compassion > threshold:
                difficulty = self.rules['empathy_to_benchmarks']['difficulty']
                self.bus.publish('benchmark.enable_empathy_tests', {
                    'difficulty': difficulty,
                    'compassion_score': compassion
                }, priority=EventPriority.NORMAL, source='feedback_orchestrator')
                
                actions_taken.append({
                    'type': 'empathy_to_benchmarks',
                    'action': 'enable_empathy_tests',
                    'difficulty': difficulty
                })
        
        # Benchmarks → Web3 feedback
        if self.rules['benchmarks_to_web3']['enabled']:
            gaia_score = unified['benchmarks'].get('gaia_score', 0)
            threshold = self.rules['benchmarks_to_web3']['threshold']
            
            if gaia_score > threshold:
                self.bus.publish('web3.mint_nft', {
                    'type': 'benchmark_milestone',
                    'score': gaia_score,
                    'benchmark': 'gaia'
                }, priority=EventPriority.HIGH, source='feedback_orchestrator')
                
                actions_taken.append({
                    'type': 'benchmarks_to_web3',
                    'action': 'mint_nft',
                    'score': gaia_score
                })
        
        # Benchmarks → Empathy learning
        if unified['benchmarks'].get('success_rate', 0) > 0.8:
            self.bus.publish('empathy.learn_from', {
                'source': 'successful_benchmarks',
                'success_rate': unified['benchmarks']['success_rate']
            }, priority=EventPriority.NORMAL, source='feedback_orchestrator')
            
            actions_taken.append({
                'type': 'benchmarks_to_empathy',
                'action': 'transfer_learning',
                'success_rate': unified['benchmarks']['success_rate']
            })
        
        # Web3 → Physics/Empathy/Benchmarks (governance)
        if self.rules['web3_to_physics']['enabled']:
            governance_priority = unified['web3'].get('dao_priority')
            
            if governance_priority and governance_priority != 'balanced':
                # Redirect resources based on DAO decision
                resources = self.rules['web3_to_physics']['resource_multiplier']
                
                for domain in ['physics', 'empathy', 'benchmark']:
                    multiplier = resources if governance_priority in domain else 1.0
                    self.bus.publish(f'{domain}.adjust_focus', {
                        'priority': governance_priority,
                        'resources': multiplier
                    }, priority=EventPriority.HIGH, source='feedback_orchestrator')
                
                actions_taken.append({
                    'type': 'web3_governance',
                    'action': 'adjust_priorities',
                    'priority': governance_priority
                })
        
        # Record feedback
        feedback_record = {
            'timestamp': time.time(),
            'actions': actions_taken,
            'state_snapshot': unified
        }
        self.feedback_history.append(feedback_record)
        
        return {
            'actions_taken': len(actions_taken),
            'actions': actions_taken,
            'timestamp': time.time()
        }

    def optimize_system(self) -> Dict[str, Any]:
        """
        Autonomous system optimization based on compound data.
        
        Automatically shifts resources to highest-impact areas.
        
        Returns:
            Optimization actions taken
        """
        unified = self.state.merge_all()
        consciousness = unified['consciousness']
        
        optimizations = []
        
        # Find weakest component
        components = {
            'physics': consciousness['physics_component'],
            'empathy': consciousness['empathy_component'],
            'benchmark': consciousness['benchmark_component'],
            'web3': consciousness['web3_component'],
        }
        
        weakest = min(components.items(), key=lambda x: x[1])
        strongest = max(components.items(), key=lambda x: x[1])
        
        # If imbalance is significant, boost weak component
        if strongest[1] - weakest[1] > 0.3:
            self.bus.publish(f'{weakest[0]}.boost_resources', {
                'reason': 'optimization',
                'current_score': weakest[1],
                'target_score': strongest[1]
            }, priority=EventPriority.HIGH, source='feedback_orchestrator')
            
            optimizations.append({
                'type': 'balance_components',
                'action': f'boost_{weakest[0]}',
                'current': weakest[1],
                'target': strongest[1]
            })
        
        # Enhance emergent patterns
        patterns = unified['emergent_properties']['patterns']
        for pattern in patterns:
            if pattern['strength'] > 0.8:
                # Reinforce strong patterns
                self.bus.publish('system.reinforce_pattern', {
                    'pattern_type': pattern['type'],
                    'strength': pattern['strength']
                }, priority=EventPriority.NORMAL, source='feedback_orchestrator')
                
                optimizations.append({
                    'type': 'reinforce_pattern',
                    'pattern': pattern['type'],
                    'strength': pattern['strength']
                })
        
        return {
            'optimizations': len(optimizations),
            'actions': optimizations,
            'consciousness_before': consciousness['overall'],
            'timestamp': time.time()
        }

    def get_feedback_summary(self) -> Dict:
        """Get summary of feedback loop effectiveness"""
        if not self.feedback_history:
            return {
                'total_feedbacks': 0,
                'total_actions': 0,
                'effectiveness': 0.0
            }
        
        total_actions = sum(len(f['actions']) for f in self.feedback_history)
        
        # Measure effectiveness by consciousness improvement
        if len(self.feedback_history) >= 2:
            first = self.feedback_history[0]['state_snapshot']['consciousness']['overall']
            last = self.feedback_history[-1]['state_snapshot']['consciousness']['overall']
            effectiveness = (last - first) / first if first > 0 else 0.0
        else:
            effectiveness = 0.0
        
        return {
            'total_feedbacks': len(self.feedback_history),
            'total_actions': total_actions,
            'effectiveness': effectiveness,
            'recent_actions': self.feedback_history[-5:] if len(self.feedback_history) >= 5 else self.feedback_history
        }


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("FEEDBACK ORCHESTRATOR DEMO")
    print("=" * 80)
    
    from compound_data_bus import CompoundDataBus
    
    bus = CompoundDataBus()
    bus.start()
    
    state = CompoundState()
    orchestrator = FeedbackOrchestrator(bus, state)
    
    # Simulate some state updates
    print("\nSimulating physics breakthrough...")
    bus.publish('physics.update', {
        'quantum_coherence': 0.85,
        'convergence': 0.88,
        'breakthrough_detected': True
    })
    
    time.sleep(0.2)
    
    print("\nSimulating high empathy...")
    bus.publish('empathy.update', {
        'compassion_score': 0.92,
        'emotional_coherence': 0.87
    })
    
    time.sleep(0.2)
    
    print("\nSimulating benchmark success...")
    bus.publish('benchmark.result', {
        'gaia_score': 0.89,
        'arc_score': 0.81,
        'success': True
    })
    
    time.sleep(0.2)
    
    # Apply feedback
    print("\nApplying cross-domain learning...")
    result = orchestrator.apply_cross_domain_learning()
    print(f"Actions taken: {result['actions_taken']}")
    for action in result['actions']:
        print(f"  • {action['type']}: {action['action']}")
    
    # Optimize
    print("\nOptimizing system...")
    opt_result = orchestrator.optimize_system()
    print(f"Optimizations: {opt_result['optimizations']}")
    for opt in opt_result['actions']:
        print(f"  • {opt['type']}: {opt.get('action', opt.get('pattern'))}")
    
    time.sleep(0.2)
    
    # Summary
    print("\nFeedback Summary:")
    summary = orchestrator.get_feedback_summary()
    print(f"  Total feedbacks: {summary['total_feedbacks']}")
    print(f"  Total actions: {summary['total_actions']}")
    print(f"  Effectiveness: {summary['effectiveness']:.1%}")
    
    bus.stop()
    print("\n✅ Demo complete")
