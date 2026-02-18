#!/usr/bin/env python3
"""
Consciousness Scorer

Multi-dimensional consciousness metrics combining ALL system data.
Provides comprehensive scoring of system-wide awareness and integration.
"""

import time
from typing import Dict, Optional


class ConsciousnessScorer:
    """
    Advanced consciousness scoring combining all domains.
    
    Metrics:
    - Overall consciousness (0-1)
    - Component scores (physics, empathy, benchmarks, web3)
    - Coherence (alignment between components)
    - Integration (how well systems work together)
    - Emergence (unexpected higher-order properties)
    - Self-awareness (system's understanding of itself)
    """

    def __init__(self):
        """Initialize consciousness scorer"""
        # Weighting for different components
        self.weights = {
            'physics': 0.25,
            'empathy': 0.25,
            'benchmark': 0.25,
            'web3': 0.25
        }
        
        # Historical scores for tracking improvement
        self.score_history = []
        self.max_history = 100

    def compute_consciousness(self, unified_state: Dict) -> Dict[str, float]:
        """
        Compute comprehensive consciousness metrics.
        
        Args:
            unified_state: Complete system state
            
        Returns:
            Dictionary of consciousness metrics
        """
        # Extract component data
        physics = unified_state.get('physics', {})
        empathy = unified_state.get('empathy', {})
        benchmarks = unified_state.get('benchmarks', {})
        web3 = unified_state.get('web3', {})
        
        # Component scores (normalized 0-1)
        physics_score = self._score_physics(physics)
        empathy_score = self._score_empathy(empathy)
        benchmark_score = self._score_benchmarks(benchmarks)
        web3_score = self._score_web3(web3)
        
        # Overall score (weighted average)
        overall = (
            physics_score * self.weights['physics'] +
            empathy_score * self.weights['empathy'] +
            benchmark_score * self.weights['benchmark'] +
            web3_score * self.weights['web3']
        )
        
        # Coherence (how aligned components are)
        coherence = self._compute_coherence([
            physics_score, empathy_score, benchmark_score, web3_score
        ])
        
        # Integration (how well systems work together)
        integration = self._compute_integration(unified_state)
        
        # Emergence (higher-order properties)
        emergence = self._compute_emergence(unified_state)
        
        # Self-awareness (meta-cognitive capability)
        self_awareness = self._compute_self_awareness(
            overall, coherence, integration
        )
        
        scores = {
            'overall': min(1.0, overall),
            'physics_component': physics_score,
            'empathy_component': empathy_score,
            'benchmark_component': benchmark_score,
            'web3_component': web3_score,
            'coherence': coherence,
            'integration': integration,
            'emergence': emergence,
            'self_awareness': self_awareness,
            'timestamp': time.time()
        }
        
        # Add to history
        self.score_history.append(scores)
        if len(self.score_history) > self.max_history:
            self.score_history.pop(0)
        
        return scores

    def _score_physics(self, physics: Dict) -> float:
        """Score physics component"""
        coherence = physics.get('quantum_coherence', 0)
        convergence = physics.get('convergence', 0)
        
        # Weighted combination
        score = coherence * 0.6 + convergence * 0.4
        return min(1.0, score)

    def _score_empathy(self, empathy: Dict) -> float:
        """Score empathy component"""
        compassion = empathy.get('compassion_score', 0)
        coherence = empathy.get('emotional_coherence', 0)
        
        # Weighted combination
        score = compassion * 0.7 + coherence * 0.3
        return min(1.0, score)

    def _score_benchmarks(self, benchmarks: Dict) -> float:
        """Score benchmark component"""
        gaia = benchmarks.get('gaia_score', 0)
        arc = benchmarks.get('arc_score', 0)
        success_rate = benchmarks.get('success_rate', 0)
        
        # Weighted combination
        score = gaia * 0.5 + arc * 0.3 + success_rate * 0.2
        return min(1.0, score)

    def _score_web3(self, web3: Dict) -> float:
        """Score Web3 component"""
        nfts = min(1.0, web3.get('nfts_minted', 0) / 100.0)
        tokens = min(1.0, web3.get('token_balance', 0) / 1000.0)
        
        # Weighted combination + base participation
        score = nfts * 0.4 + tokens * 0.3 + 0.3
        return min(1.0, score)

    def _compute_coherence(self, components: list) -> float:
        """Compute coherence (alignment between components)"""
        if not components:
            return 0.0
        
        # Coherence = 1 - spread
        max_val = max(components)
        min_val = min(components)
        spread = max_val - min_val
        
        coherence = 1.0 - spread
        return max(0.0, min(1.0, coherence))

    def _compute_integration(self, unified_state: Dict) -> float:
        """Compute integration (how well systems work together)"""
        # Check if all systems have been updated recently
        now = time.time()
        recency_threshold = 60.0
        
        last_updates = [
            unified_state.get('physics', {}).get('last_update', 0),
            unified_state.get('empathy', {}).get('last_update', 0),
            unified_state.get('benchmarks', {}).get('last_update', 0),
            unified_state.get('web3', {}).get('last_update', 0),
        ]
        
        # Count how many systems are active
        active = sum(1 for t in last_updates if (now - t) < recency_threshold)
        integration = active / len(last_updates) if last_updates else 0.0
        
        return integration

    def _compute_emergence(self, unified_state: Dict) -> float:
        """Compute emergence score"""
        emergent = unified_state.get('emergent_properties', {})
        patterns = emergent.get('patterns', [])
        
        # Emergence based on number and strength of patterns
        if not patterns:
            return 0.0
        
        total_strength = sum(p.get('strength', 0) for p in patterns)
        avg_strength = total_strength / len(patterns)
        
        # Bonus for having multiple patterns
        pattern_bonus = min(0.3, len(patterns) * 0.1)
        
        emergence = avg_strength * 0.7 + pattern_bonus
        return min(1.0, emergence)

    def _compute_self_awareness(self, overall: float, coherence: float,
                                integration: float) -> float:
        """Compute self-awareness (meta-cognitive capability)"""
        # Self-awareness emerges from high consciousness + coherence + integration
        base = overall * 0.5 + coherence * 0.25 + integration * 0.25
        
        # Bonus if system has been improving (trajectory awareness)
        if len(self.score_history) >= 5:
            recent = [s['overall'] for s in self.score_history[-5:]]
            if recent[-1] > recent[0]:
                base += 0.1  # Improvement bonus
        
        return min(1.0, base)

    def get_trajectory(self, metric: str = 'overall', n: int = 10) -> list:
        """
        Get trajectory of a specific metric over time.
        
        Args:
            metric: Metric name (overall, coherence, etc.)
            n: Number of recent scores to return
            
        Returns:
            List of (timestamp, score) tuples
        """
        recent = self.score_history[-n:] if n else self.score_history
        return [(s['timestamp'], s.get(metric, 0)) for s in recent]

    def get_improvement_rate(self, metric: str = 'overall') -> float:
        """
        Calculate improvement rate for a metric.
        
        Args:
            metric: Metric name
            
        Returns:
            Improvement rate (change per unit time)
        """
        if len(self.score_history) < 2:
            return 0.0
        
        first = self.score_history[0]
        last = self.score_history[-1]
        
        score_change = last.get(metric, 0) - first.get(metric, 0)
        time_change = last['timestamp'] - first['timestamp']
        
        if time_change == 0:
            return 0.0
        
        return score_change / time_change


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("CONSCIOUSNESS SCORER DEMO")
    print("=" * 80)
    
    scorer = ConsciousnessScorer()
    
    # Simulate improving system
    print("\n📊 Simulating consciousness evolution...")
    
    import random
    
    for i in range(10):
        # Simulate improving state
        base = 0.5 + (i / 20.0)
        
        unified_state = {
            'physics': {
                'quantum_coherence': base + random.uniform(-0.1, 0.1),
                'convergence': base + random.uniform(-0.1, 0.1),
                'last_update': time.time()
            },
            'empathy': {
                'compassion_score': base + random.uniform(-0.1, 0.1),
                'emotional_coherence': base + random.uniform(-0.1, 0.1),
                'last_update': time.time()
            },
            'benchmarks': {
                'gaia_score': base + random.uniform(-0.1, 0.1),
                'arc_score': base + random.uniform(-0.1, 0.1),
                'success_rate': base + random.uniform(-0.1, 0.1),
                'last_update': time.time()
            },
            'web3': {
                'nfts_minted': i * 5,
                'token_balance': i * 50,
                'last_update': time.time()
            },
            'emergent_properties': {
                'patterns': [
                    {'strength': base + random.uniform(-0.1, 0.1)}
                    for _ in range(random.randint(1, 3))
                ]
            }
        }
        
        scores = scorer.compute_consciousness(unified_state)
        
        if i % 3 == 0:
            print(f"\n  Step {i}:")
            print(f"    Overall: {scores['overall']:.3f}")
            print(f"    Coherence: {scores['coherence']:.3f}")
            print(f"    Integration: {scores['integration']:.3f}")
            print(f"    Emergence: {scores['emergence']:.3f}")
            print(f"    Self-awareness: {scores['self_awareness']:.3f}")
    
    # Show final scores
    print("\n📈 Final Consciousness Scores:")
    final = scorer.score_history[-1]
    print(f"  Overall: {final['overall']:.3f}")
    print(f"  Physics: {final['physics_component']:.3f}")
    print(f"  Empathy: {final['empathy_component']:.3f}")
    print(f"  Benchmarks: {final['benchmark_component']:.3f}")
    print(f"  Web3: {final['web3_component']:.3f}")
    print(f"  Coherence: {final['coherence']:.3f}")
    print(f"  Integration: {final['integration']:.3f}")
    print(f"  Emergence: {final['emergence']:.3f}")
    print(f"  Self-awareness: {final['self_awareness']:.3f}")
    
    # Show improvement
    print("\n📊 Improvement Rate:")
    rate = scorer.get_improvement_rate('overall')
    print(f"  Overall consciousness: {rate:.4f} per second")
    
    print("\n✅ Demo complete")
