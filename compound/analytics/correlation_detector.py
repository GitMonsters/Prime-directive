#!/usr/bin/env python3
"""
Correlation Detector

Finds cross-domain patterns and correlations.
Discovers unexpected relationships between different system components.
"""

import time
from typing import Dict, List, Tuple, Optional
from collections import deque
import statistics


class CorrelationDetector:
    """
    Detect correlations across physics/empathy/benchmarks/web3.
    
    Automatically discovers patterns like:
    - High empathy → Better physics convergence
    - Governance votes correlate with benchmark scores
    - Physics breakthroughs trigger empathy spikes
    """

    def __init__(self, history_size: int = 100):
        """
        Initialize correlation detector.
        
        Args:
            history_size: Number of data points to keep for correlation
        """
        self.history_size = history_size
        
        # Data history for each domain
        self.physics_history = deque(maxlen=history_size)
        self.empathy_history = deque(maxlen=history_size)
        self.benchmark_history = deque(maxlen=history_size)
        self.web3_history = deque(maxlen=history_size)
        
        # Detected correlations
        self.correlations = {}
        
        # Correlation thresholds
        self.correlation_threshold = 0.6  # Pearson correlation threshold

    def add_data_point(self, domain: str, data: Dict) -> None:
        """
        Add data point for correlation analysis.
        
        Args:
            domain: Domain name (physics/empathy/benchmark/web3)
            data: Data point dictionary
        """
        timestamp = time.time()
        point = {'timestamp': timestamp, **data}
        
        if domain == 'physics':
            self.physics_history.append(point)
        elif domain == 'empathy':
            self.empathy_history.append(point)
        elif domain == 'benchmark':
            self.benchmark_history.append(point)
        elif domain == 'web3':
            self.web3_history.append(point)

    def detect_correlations(self) -> Dict[str, List[Dict]]:
        """
        Detect all cross-domain correlations.
        
        Returns:
            Dictionary of detected correlations
        """
        correlations = []
        
        # Physics ↔ Empathy
        if len(self.physics_history) >= 10 and len(self.empathy_history) >= 10:
            corr = self._correlate_domains(
                self.physics_history, 'quantum_coherence',
                self.empathy_history, 'compassion_score'
            )
            if abs(corr) > self.correlation_threshold:
                correlations.append({
                    'domains': ('physics', 'empathy'),
                    'variables': ('quantum_coherence', 'compassion_score'),
                    'correlation': corr,
                    'strength': 'strong' if abs(corr) > 0.8 else 'moderate',
                    'interpretation': 'High empathy correlates with physics coherence' if corr > 0
                                    else 'Negative correlation detected'
                })
        
        # Physics ↔ Benchmarks
        if len(self.physics_history) >= 10 and len(self.benchmark_history) >= 10:
            corr = self._correlate_domains(
                self.physics_history, 'convergence',
                self.benchmark_history, 'gaia_score'
            )
            if abs(corr) > self.correlation_threshold:
                correlations.append({
                    'domains': ('physics', 'benchmark'),
                    'variables': ('convergence', 'gaia_score'),
                    'correlation': corr,
                    'strength': 'strong' if abs(corr) > 0.8 else 'moderate',
                    'interpretation': 'Physics convergence correlates with benchmark performance'
                })
        
        # Empathy ↔ Benchmarks
        if len(self.empathy_history) >= 10 and len(self.benchmark_history) >= 10:
            corr = self._correlate_domains(
                self.empathy_history, 'compassion_score',
                self.benchmark_history, 'gaia_score'
            )
            if abs(corr) > self.correlation_threshold:
                correlations.append({
                    'domains': ('empathy', 'benchmark'),
                    'variables': ('compassion_score', 'gaia_score'),
                    'correlation': corr,
                    'strength': 'strong' if abs(corr) > 0.8 else 'moderate',
                    'interpretation': 'Empathy levels correlate with benchmark success'
                })
        
        self.correlations = {'detected': correlations, 'timestamp': time.time()}
        return self.correlations

    def _correlate_domains(self, history1: deque, key1: str,
                          history2: deque, key2: str) -> float:
        """
        Calculate Pearson correlation between two variables.
        
        Args:
            history1: First domain history
            key1: Variable key in first domain
            history2: Second domain history
            key2: Variable key in second domain
            
        Returns:
            Pearson correlation coefficient (-1 to 1)
        """
        # Extract values
        values1 = [p.get(key1, 0) for p in history1]
        values2 = [p.get(key2, 0) for p in history2]
        
        # Align by timestamp (use minimum length)
        min_len = min(len(values1), len(values2))
        values1 = values1[-min_len:]
        values2 = values2[-min_len:]
        
        if len(values1) < 2:
            return 0.0
        
        # Calculate Pearson correlation
        try:
            mean1 = statistics.mean(values1)
            mean2 = statistics.mean(values2)
            
            numerator = sum((v1 - mean1) * (v2 - mean2) 
                          for v1, v2 in zip(values1, values2))
            
            denom1 = sum((v1 - mean1) ** 2 for v1 in values1)
            denom2 = sum((v2 - mean2) ** 2 for v2 in values2)
            
            denominator = (denom1 * denom2) ** 0.5
            
            if denominator == 0:
                return 0.0
            
            correlation = numerator / denominator
            return correlation
            
        except Exception:
            return 0.0

    def get_top_correlations(self, n: int = 5) -> List[Dict]:
        """Get top N strongest correlations"""
        if not self.correlations or 'detected' not in self.correlations:
            return []
        
        correlations = self.correlations['detected']
        sorted_corrs = sorted(correlations, 
                             key=lambda x: abs(x['correlation']), 
                             reverse=True)
        return sorted_corrs[:n]

    def predict_impact(self, domain: str, variable: str, 
                      change: float) -> Dict[str, float]:
        """
        Predict impact of changing a variable on other domains.
        
        Args:
            domain: Source domain
            variable: Variable to change
            change: Magnitude of change
            
        Returns:
            Predicted impacts on other domains
        """
        if not self.correlations or 'detected' not in self.correlations:
            return {}
        
        predictions = {}
        
        for corr in self.correlations['detected']:
            if domain in corr['domains'] and variable in corr['variables']:
                # Find other domain
                other_domain = corr['domains'][1] if corr['domains'][0] == domain \
                             else corr['domains'][0]
                other_var = corr['variables'][1] if corr['variables'][0] == variable \
                           else corr['variables'][0]
                
                # Predict impact based on correlation
                predicted_change = change * corr['correlation']
                predictions[f"{other_domain}.{other_var}"] = predicted_change
        
        return predictions


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("CORRELATION DETECTOR DEMO")
    print("=" * 80)
    
    import random
    
    detector = CorrelationDetector()
    
    # Simulate correlated data
    print("\n📊 Generating correlated data...")
    for i in range(30):
        base_value = 0.5 + (i / 60.0)  # Increasing trend
        
        detector.add_data_point('physics', {
            'quantum_coherence': base_value + random.uniform(-0.1, 0.1),
            'convergence': base_value + random.uniform(-0.1, 0.1)
        })
        
        detector.add_data_point('empathy', {
            'compassion_score': base_value + random.uniform(-0.1, 0.1)
        })
        
        detector.add_data_point('benchmark', {
            'gaia_score': base_value + random.uniform(-0.1, 0.1)
        })
    
    # Detect correlations
    print("\n🔍 Detecting correlations...")
    results = detector.detect_correlations()
    
    print(f"\nDetected {len(results['detected'])} correlations:")
    for corr in results['detected']:
        print(f"\n  • {corr['domains'][0]} ↔ {corr['domains'][1]}")
        print(f"    Variables: {corr['variables'][0]} ↔ {corr['variables'][1]}")
        print(f"    Correlation: {corr['correlation']:.3f} ({corr['strength']})")
        print(f"    {corr['interpretation']}")
    
    # Predict impact
    print("\n🔮 Predicting impact of empathy boost...")
    predictions = detector.predict_impact('empathy', 'compassion_score', 0.1)
    for key, value in predictions.items():
        print(f"  {key}: +{value:.3f}")
    
    print("\n✅ Demo complete")
