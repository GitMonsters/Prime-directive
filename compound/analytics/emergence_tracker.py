#!/usr/bin/env python3
"""
Emergence Tracker

Tracks emergent behavior - unexpected patterns that arise
from interactions between system components.
"""

import time
from typing import Dict, List, Optional, Any
from collections import deque


class EmergenceTracker:
    """
    Detect emergent behaviors and unexpected system properties.
    
    Tracks phenomena like:
    - Feedback loop oscillations
    - Synchronization between domains
    - Phase transitions in consciousness
    - Unexpected performance improvements
    """

    def __init__(self, history_size: int = 100):
        """
        Initialize emergence tracker.
        
        Args:
            history_size: Number of states to keep in history
        """
        self.history_size = history_size
        self.state_history = deque(maxlen=history_size)
        
        # Detected emergent patterns
        self.emergent_patterns = []
        
        # Thresholds for emergence detection
        self.sync_threshold = 0.15  # Max deviation for synchronization
        self.oscillation_period_min = 3  # Minimum oscillation period

    def add_state(self, unified_state: Dict) -> None:
        """
        Add system state for emergence tracking.
        
        Args:
            unified_state: Complete system state snapshot
        """
        self.state_history.append({
            'timestamp': time.time(),
            'state': unified_state
        })

    def detect_emergence(self) -> List[Dict]:
        """
        Detect emergent patterns in system behavior.
        
        Returns:
            List of detected emergent patterns
        """
        if len(self.state_history) < 10:
            return []
        
        patterns = []
        
        # Detect synchronization
        sync = self._detect_synchronization()
        if sync:
            patterns.append(sync)
        
        # Detect oscillations
        osc = self._detect_oscillations()
        if osc:
            patterns.append(osc)
        
        # Detect phase transitions
        phase = self._detect_phase_transition()
        if phase:
            patterns.append(phase)
        
        # Detect coherence emergence
        coherence = self._detect_coherence_emergence()
        if coherence:
            patterns.append(coherence)
        
        self.emergent_patterns = patterns
        return patterns

    def _detect_synchronization(self) -> Optional[Dict]:
        """Detect if multiple domains are synchronizing"""
        recent = list(self.state_history)[-10:]
        
        # Extract component scores
        physics_scores = [s['state']['consciousness']['physics_component'] 
                         for s in recent]
        empathy_scores = [s['state']['consciousness']['empathy_component'] 
                         for s in recent]
        benchmark_scores = [s['state']['consciousness']['benchmark_component'] 
                           for s in recent]
        
        # Check if they're converging (low variance)
        deviations = []
        for p, e, b in zip(physics_scores, empathy_scores, benchmark_scores):
            avg = (p + e + b) / 3
            dev = max(abs(p - avg), abs(e - avg), abs(b - avg))
            deviations.append(dev)
        
        avg_dev = sum(deviations) / len(deviations)
        
        if avg_dev < self.sync_threshold:
            return {
                'type': 'synchronization',
                'description': 'Domains are synchronizing - all components converging',
                'strength': 1.0 - avg_dev,
                'domains': ['physics', 'empathy', 'benchmark'],
                'timestamp': time.time()
            }
        
        return None

    def _detect_oscillations(self) -> Optional[Dict]:
        """Detect oscillatory behavior in consciousness score"""
        if len(self.state_history) < 20:
            return None
        
        recent = list(self.state_history)[-20:]
        scores = [s['state']['consciousness']['overall'] for s in recent]
        
        # Simple oscillation detection: count zero crossings of derivative
        diffs = [scores[i+1] - scores[i] for i in range(len(scores)-1)]
        sign_changes = sum(1 for i in range(len(diffs)-1) 
                          if (diffs[i] > 0) != (diffs[i+1] > 0))
        
        # If enough sign changes, we have oscillation
        if sign_changes >= 4:  # At least 2 cycles
            period = len(scores) / (sign_changes / 2)
            return {
                'type': 'oscillation',
                'description': f'Consciousness oscillating with period ~{period:.1f} steps',
                'strength': sign_changes / len(diffs),
                'period': period,
                'timestamp': time.time()
            }
        
        return None

    def _detect_phase_transition(self) -> Optional[Dict]:
        """Detect sudden phase transitions in system behavior"""
        if len(self.state_history) < 15:
            return None
        
        recent = list(self.state_history)[-15:]
        scores = [s['state']['consciousness']['overall'] for s in recent]
        
        # Check for sudden jump (phase transition)
        for i in range(len(scores) - 1):
            jump = abs(scores[i+1] - scores[i])
            if jump > 0.3:  # Significant jump
                return {
                    'type': 'phase_transition',
                    'description': f'Consciousness jumped by {jump:.2f}',
                    'strength': jump,
                    'direction': 'increase' if scores[i+1] > scores[i] else 'decrease',
                    'timestamp': recent[i+1]['timestamp']
                }
        
        return None

    def _detect_coherence_emergence(self) -> Optional[Dict]:
        """Detect emergence of system-wide coherence"""
        if len(self.state_history) < 10:
            return None
        
        recent = list(self.state_history)[-10:]
        coherence_scores = [s['state']['consciousness'].get('coherence', 0) 
                           for s in recent]
        
        # Check if coherence is increasing
        if len(coherence_scores) >= 5:
            early_avg = sum(coherence_scores[:5]) / 5
            late_avg = sum(coherence_scores[5:]) / 5
            
            if late_avg > early_avg + 0.2:  # Significant increase
                return {
                    'type': 'coherence_emergence',
                    'description': 'System coherence emerging',
                    'strength': late_avg,
                    'improvement': late_avg - early_avg,
                    'timestamp': time.time()
                }
        
        return None

    def get_emergence_summary(self) -> Dict:
        """Get summary of emergent behavior"""
        return {
            'total_patterns': len(self.emergent_patterns),
            'patterns': self.emergent_patterns,
            'state_history_size': len(self.state_history),
            'timestamp': time.time()
        }


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("EMERGENCE TRACKER DEMO")
    print("=" * 80)
    
    import random
    
    tracker = EmergenceTracker()
    
    # Simulate system states with emergent behavior
    print("\n📊 Simulating system evolution...")
    
    # Phase 1: Random
    for i in range(10):
        tracker.add_state({
            'consciousness': {
                'overall': random.uniform(0.4, 0.6),
                'physics_component': random.uniform(0.4, 0.6),
                'empathy_component': random.uniform(0.4, 0.6),
                'benchmark_component': random.uniform(0.4, 0.6),
                'coherence': random.uniform(0.3, 0.5)
            }
        })
    
    # Phase 2: Synchronization
    for i in range(10):
        base = 0.7 + random.uniform(-0.05, 0.05)
        tracker.add_state({
            'consciousness': {
                'overall': base,
                'physics_component': base + random.uniform(-0.03, 0.03),
                'empathy_component': base + random.uniform(-0.03, 0.03),
                'benchmark_component': base + random.uniform(-0.03, 0.03),
                'coherence': 0.6 + i * 0.03
            }
        })
    
    # Detect emergence
    print("\n🔍 Detecting emergent patterns...")
    patterns = tracker.detect_emergence()
    
    print(f"\nDetected {len(patterns)} emergent patterns:")
    for pattern in patterns:
        print(f"\n  • Type: {pattern['type']}")
        print(f"    Description: {pattern['description']}")
        print(f"    Strength: {pattern['strength']:.2f}")
        if 'domains' in pattern:
            print(f"    Domains: {', '.join(pattern['domains'])}")
    
    # Summary
    print("\n📈 Emergence Summary:")
    summary = tracker.get_emergence_summary()
    print(f"  Total patterns detected: {summary['total_patterns']}")
    print(f"  State history size: {summary['state_history_size']}")
    
    print("\n✅ Demo complete")
