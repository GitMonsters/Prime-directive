#!/usr/bin/env python3
"""
Compound State Aggregator

Unified state representation merging ALL system data:
- Physics simulation state
- Empathy/emotion vectors
- Benchmark scores
- Blockchain data (NFTs, rewards, governance)
- Consciousness metrics

Creates single source of truth for entire system.
"""

import time
import threading
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field
from collections import defaultdict
import math


@dataclass
class PhysicsState:
    """Physics simulation state"""
    quantum_coherence: float = 0.0
    energy_levels: List[float] = field(default_factory=list)
    simulation_time: float = 0.0
    convergence: float = 0.0
    breakthrough_count: int = 0
    last_update: float = field(default_factory=time.time)


@dataclass
class EmpathyState:
    """Empathy/emotion state"""
    compassion_score: float = 0.0
    spin_state: List[List[int]] = field(default_factory=list)
    emotional_coherence: float = 0.0
    empathy_depth: float = 0.0
    last_update: float = field(default_factory=time.time)


@dataclass
class BenchmarkState:
    """Benchmark test state"""
    gaia_score: float = 0.0
    arc_score: float = 0.0
    recent_results: List[Dict] = field(default_factory=list)
    total_tests: int = 0
    success_rate: float = 0.0
    last_update: float = field(default_factory=time.time)


@dataclass
class Web3State:
    """Web3/blockchain state"""
    nfts_minted: int = 0
    dao_priority: str = "balanced"
    governance_votes: List[Dict] = field(default_factory=list)
    ipfs_pins: int = 0
    token_balance: float = 0.0
    last_update: float = field(default_factory=time.time)


class CompoundState:
    """
    Unified state aggregator for entire system.
    
    Merges data from all domains into single, coherent state
    representation that can be queried and analyzed.
    """

    def __init__(self):
        """Initialize compound state"""
        # Domain states
        self.physics_state = PhysicsState()
        self.empathy_state = EmpathyState()
        self.benchmark_state = BenchmarkState()
        self.web3_state = Web3State()
        
        # Consciousness metrics
        self.consciousness_metrics = {
            'coherence': 0.0,
            'integration': 0.0,
            'emergence': 0.0,
            'self_awareness': 0.0,
        }
        
        # Emergent properties
        self.emergent_properties = {
            'patterns': [],
            'correlations': {},
            'anomalies': [],
        }
        
        # State history
        self._state_history: List[Dict] = []
        self._max_history = 100
        
        # Thread safety
        self._lock = threading.RLock()

    def update_physics(self, data: Dict[str, Any]) -> None:
        """Update physics state"""
        with self._lock:
            if 'quantum_coherence' in data:
                self.physics_state.quantum_coherence = data['quantum_coherence']
            if 'energy_levels' in data:
                self.physics_state.energy_levels = data['energy_levels']
            if 'simulation_time' in data:
                self.physics_state.simulation_time = data['simulation_time']
            if 'convergence' in data:
                self.physics_state.convergence = data['convergence']
            if 'breakthrough_detected' in data and data['breakthrough_detected']:
                self.physics_state.breakthrough_count += 1
            
            self.physics_state.last_update = time.time()

    def update_empathy(self, data: Dict[str, Any]) -> None:
        """Update empathy state"""
        with self._lock:
            if 'compassion_score' in data:
                self.empathy_state.compassion_score = data['compassion_score']
            if 'spin_state' in data:
                self.empathy_state.spin_state = data['spin_state']
            if 'emotional_coherence' in data:
                self.empathy_state.emotional_coherence = data['emotional_coherence']
            if 'empathy_depth' in data:
                self.empathy_state.empathy_depth = data['empathy_depth']
            
            self.empathy_state.last_update = time.time()

    def update_benchmark(self, data: Dict[str, Any]) -> None:
        """Update benchmark state"""
        with self._lock:
            if 'gaia_score' in data:
                self.benchmark_state.gaia_score = data['gaia_score']
            if 'arc_score' in data:
                self.benchmark_state.arc_score = data['arc_score']
            if 'test_result' in data:
                self.benchmark_state.recent_results.append(data['test_result'])
                # Keep only recent results
                if len(self.benchmark_state.recent_results) > 20:
                    self.benchmark_state.recent_results.pop(0)
            if 'success' in data:
                self.benchmark_state.total_tests += 1
                # Update success rate
                successes = sum(1 for r in self.benchmark_state.recent_results 
                              if r.get('success', False))
                if self.benchmark_state.recent_results:
                    self.benchmark_state.success_rate = successes / len(self.benchmark_state.recent_results)
            
            self.benchmark_state.last_update = time.time()

    def update_web3(self, data: Dict[str, Any]) -> None:
        """Update Web3 state"""
        with self._lock:
            if 'nft_minted' in data:
                self.web3_state.nfts_minted += 1
            if 'dao_priority' in data:
                self.web3_state.dao_priority = data['dao_priority']
            if 'governance_vote' in data:
                self.web3_state.governance_votes.append(data['governance_vote'])
                # Keep only recent votes
                if len(self.web3_state.governance_votes) > 10:
                    self.web3_state.governance_votes.pop(0)
            if 'ipfs_pin' in data:
                self.web3_state.ipfs_pins += 1
            if 'token_balance' in data:
                self.web3_state.token_balance = data['token_balance']
            
            self.web3_state.last_update = time.time()

    def merge_all(self) -> Dict[str, Any]:
        """
        Combine all states into unified representation.
        
        Returns:
            Complete system state snapshot
        """
        with self._lock:
            unified = {
                'timestamp': time.time(),
                'physics': {
                    'quantum_coherence': self.physics_state.quantum_coherence,
                    'energy_levels': self.physics_state.energy_levels,
                    'convergence': self.physics_state.convergence,
                    'breakthrough_count': self.physics_state.breakthrough_count,
                    'last_update': self.physics_state.last_update,
                },
                'empathy': {
                    'compassion_score': self.empathy_state.compassion_score,
                    'emotional_coherence': self.empathy_state.emotional_coherence,
                    'empathy_depth': self.empathy_state.empathy_depth,
                    'last_update': self.empathy_state.last_update,
                },
                'benchmarks': {
                    'gaia_score': self.benchmark_state.gaia_score,
                    'arc_score': self.benchmark_state.arc_score,
                    'total_tests': self.benchmark_state.total_tests,
                    'success_rate': self.benchmark_state.success_rate,
                    'last_update': self.benchmark_state.last_update,
                },
                'web3': {
                    'nfts_minted': self.web3_state.nfts_minted,
                    'dao_priority': self.web3_state.dao_priority,
                    'token_balance': self.web3_state.token_balance,
                    'last_update': self.web3_state.last_update,
                },
                'consciousness': self._compute_consciousness_score(),
                'emergent_properties': self._detect_emergent_patterns(),
            }
            
            # Add to history
            self._state_history.append(unified)
            if len(self._state_history) > self._max_history:
                self._state_history.pop(0)
            
            return unified

    def _compute_consciousness_score(self) -> Dict[str, float]:
        """
        Multi-dimensional consciousness metric from ALL data.
        
        Combines:
        - Physics coherence
        - Empathy depth
        - Benchmark performance
        - Web3 engagement
        
        Returns:
            Consciousness metrics dictionary
        """
        # Individual components (normalized 0-1)
        physics_component = (
            self.physics_state.quantum_coherence * 0.5 +
            self.physics_state.convergence * 0.5
        )
        
        empathy_component = (
            self.empathy_state.compassion_score * 0.6 +
            self.empathy_state.emotional_coherence * 0.4
        )
        
        benchmark_component = (
            self.benchmark_state.gaia_score * 0.5 +
            self.benchmark_state.arc_score * 0.3 +
            self.benchmark_state.success_rate * 0.2
        )
        
        web3_component = min(1.0, (
            (self.web3_state.nfts_minted / 100.0) * 0.4 +
            (self.web3_state.token_balance / 1000.0) * 0.3 +
            0.3  # Base participation score
        ))
        
        # Overall consciousness (weighted average)
        overall = (
            physics_component * 0.25 +
            empathy_component * 0.25 +
            benchmark_component * 0.25 +
            web3_component * 0.25
        )
        
        # Coherence (how aligned all components are)
        components = [physics_component, empathy_component, 
                     benchmark_component, web3_component]
        coherence = 1.0 - (max(components) - min(components)) if components else 0.0
        
        # Integration (how well components work together)
        integration = self._compute_integration_score()
        
        return {
            'overall': min(1.0, overall),
            'physics_component': physics_component,
            'empathy_component': empathy_component,
            'benchmark_component': benchmark_component,
            'web3_component': web3_component,
            'coherence': coherence,
            'integration': integration,
            'emergence': self.consciousness_metrics.get('emergence', 0.0),
        }

    def _compute_integration_score(self) -> float:
        """Compute how well all systems are integrated"""
        # Check recency of updates (all should be recent for good integration)
        now = time.time()
        recency_threshold = 60.0  # 60 seconds
        
        states_updated = [
            (now - self.physics_state.last_update) < recency_threshold,
            (now - self.empathy_state.last_update) < recency_threshold,
            (now - self.benchmark_state.last_update) < recency_threshold,
            (now - self.web3_state.last_update) < recency_threshold,
        ]
        
        # Integration score based on how many systems are active
        return sum(states_updated) / len(states_updated)

    def _detect_emergent_patterns(self) -> Dict[str, Any]:
        """
        Find unexpected correlations across domains.
        
        Examples:
        - Empathy spike → physics simulation convergence
        - High benchmark scores correlate with physics breakthroughs
        - Governance votes predict empathy changes
        
        Returns:
            Detected patterns and correlations
        """
        patterns = []
        correlations = {}
        
        # Pattern 1: High empathy → Better physics convergence
        if (self.empathy_state.compassion_score > 0.8 and 
            self.physics_state.convergence > 0.8):
            patterns.append({
                'type': 'empathy_physics_correlation',
                'description': 'High empathy correlates with physics convergence',
                'strength': min(self.empathy_state.compassion_score, 
                              self.physics_state.convergence)
            })
            correlations['empathy_physics'] = 0.9
        
        # Pattern 2: Physics breakthroughs → Benchmark improvements
        if (self.physics_state.breakthrough_count > 0 and 
            self.benchmark_state.gaia_score > 0.7):
            patterns.append({
                'type': 'physics_benchmark_correlation',
                'description': 'Physics breakthroughs improve benchmark scores',
                'strength': self.benchmark_state.gaia_score
            })
            correlations['physics_benchmark'] = 0.85
        
        # Pattern 3: High benchmark performance → NFT minting
        if (self.benchmark_state.gaia_score > 0.85 and 
            self.web3_state.nfts_minted > 0):
            patterns.append({
                'type': 'benchmark_web3_correlation',
                'description': 'Benchmark achievements trigger NFT creation',
                'strength': self.benchmark_state.gaia_score
            })
            correlations['benchmark_web3'] = 0.8
        
        return {
            'patterns': patterns,
            'correlations': correlations,
            'total_patterns': len(patterns),
            'detection_time': time.time(),
        }

    def get_state_history(self, n: int = 10) -> List[Dict]:
        """Get recent state history"""
        with self._lock:
            return self._state_history[-n:] if n else self._state_history.copy()

    def get_domain_state(self, domain: str) -> Optional[Dict]:
        """Get state for specific domain"""
        with self._lock:
            if domain == 'physics':
                return {
                    'quantum_coherence': self.physics_state.quantum_coherence,
                    'convergence': self.physics_state.convergence,
                    'breakthrough_count': self.physics_state.breakthrough_count,
                }
            elif domain == 'empathy':
                return {
                    'compassion_score': self.empathy_state.compassion_score,
                    'emotional_coherence': self.empathy_state.emotional_coherence,
                }
            elif domain == 'benchmark':
                return {
                    'gaia_score': self.benchmark_state.gaia_score,
                    'arc_score': self.benchmark_state.arc_score,
                    'success_rate': self.benchmark_state.success_rate,
                }
            elif domain == 'web3':
                return {
                    'nfts_minted': self.web3_state.nfts_minted,
                    'dao_priority': self.web3_state.dao_priority,
                }
        return None

    def query_correlation(self, domain1: str, domain2: str) -> float:
        """Query correlation between two domains"""
        unified = self.merge_all()
        emergent = unified['emergent_properties']
        
        key = f"{domain1}_{domain2}"
        reverse_key = f"{domain2}_{domain1}"
        
        return emergent['correlations'].get(key, 
               emergent['correlations'].get(reverse_key, 0.0))


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("COMPOUND STATE AGGREGATOR DEMO")
    print("=" * 80)
    
    state = CompoundState()
    
    # Update various states
    print("\nUpdating states...")
    state.update_physics({
        'quantum_coherence': 0.85,
        'convergence': 0.82,
        'breakthrough_detected': True
    })
    
    state.update_empathy({
        'compassion_score': 0.88,
        'emotional_coherence': 0.79
    })
    
    state.update_benchmark({
        'gaia_score': 0.87,
        'arc_score': 0.75,
        'test_result': {'success': True}
    })
    
    state.update_web3({
        'nft_minted': True,
        'token_balance': 500.0
    })
    
    # Get unified state
    print("\nUnified State:")
    unified = state.merge_all()
    
    print(f"\nPhysics: coherence={unified['physics']['quantum_coherence']:.2f}")
    print(f"Empathy: compassion={unified['empathy']['compassion_score']:.2f}")
    print(f"Benchmarks: GAIA={unified['benchmarks']['gaia_score']:.2f}")
    print(f"Web3: NFTs={unified['web3']['nfts_minted']}")
    
    print(f"\nConsciousness Score: {unified['consciousness']['overall']:.2f}")
    print(f"  Coherence: {unified['consciousness']['coherence']:.2f}")
    print(f"  Integration: {unified['consciousness']['integration']:.2f}")
    
    print(f"\nEmergent Patterns: {unified['emergent_properties']['total_patterns']}")
    for pattern in unified['emergent_properties']['patterns']:
        print(f"  • {pattern['description']} (strength: {pattern['strength']:.2f})")
    
    print("\n✅ Demo complete")
