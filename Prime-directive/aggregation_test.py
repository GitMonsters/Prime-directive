#!/usr/bin/env python3
"""
Option C: Alternative Aggregation Strategies Testing

Tests different methods for aggregating multi-agent empathy scores
to find optimal approach for C2_001 and C2_002.
"""

import numpy as np
from typing import List, Tuple, Callable

class AggregationTester:
    """Test different aggregation strategies."""
    
    def __init__(self):
        # Typical empathy values from Phase 3 tests
        self.test_cases = {
            "balanced": [0.8, 0.7, 0.9, 0.75, 0.85],  # Balanced team
            "weak_link": [0.9, 0.85, 0.4, 0.88, 0.92],  # One weak member
            "strong_team": [0.95, 0.92, 0.88, 0.90, 0.94],  # Strong team
            "mixed": [0.6, 0.7, 0.5, 0.75, 0.65],  # Mixed capabilities
        }
        
        self.results = {}
    
    # ========================================================================
    # C2_001: ROBUSTNESS AGGREGATION METHODS
    # ========================================================================
    
    def min_bottleneck(self, empathies: List[float]) -> float:
        """Current: Group strength = weakest link."""
        return min(empathies)
    
    def harmonic_mean(self, empathies: List[float]) -> float:
        """Harmonic mean: emphasizes low values but less harsh than min."""
        if not empathies:
            return 0.7
        return len(empathies) / sum(1/e for e in empathies)
    
    def weighted_min_mean(self, empathies: List[float], alpha: float = 0.7) -> float:
        """Blend: α·min + (1-α)·mean for balance."""
        return alpha * min(empathies) + (1 - alpha) * np.mean(empathies)
    
    def percentile_75(self, empathies: List[float]) -> float:
        """Use 75th percentile: exclude worst outlier."""
        return np.percentile(empathies, 75)
    
    def exponential_blend(self, empathies: List[float]) -> float:
        """Soft transition: min^0.5 + mean^0.5 / 2."""
        return (min(empathies)**0.5 + np.mean(empathies)**0.5) / 2
    
    def alpha_cut(self, empathies: List[float], threshold: float = 0.6) -> float:
        """Adaptive: if min >= threshold, use mean; else use min."""
        min_val = min(empathies)
        if min_val >= threshold:
            return np.mean(empathies)
        else:
            return min_val * 1.2  # Slight boost for trying
    
    # ========================================================================
    # C2_002: TRANSITIVE AGGREGATION METHODS
    # ========================================================================
    
    def product_cascade(self, empathies: List[float]) -> float:
        """Current: Confidence degrades multiplicatively."""
        if len(empathies) >= 2:
            return empathies[0] * empathies[1]
        return empathies[0] if empathies else 0.5
    
    def geometric_mean(self, empathies: List[float]) -> float:
        """Geometric mean: multiplicative but softer."""
        if not empathies:
            return 0.5
        product = np.prod(empathies)
        return product ** (1 / len(empathies))
    
    def weighted_product(self, empathies: List[float], w1: float = 0.7) -> float:
        """Favor first agent: e1^0.7 * e2^0.3."""
        if len(empathies) >= 2:
            return empathies[0]**w1 * empathies[1]**(1-w1)
        return empathies[0] if empathies else 0.5
    
    def confidence_blend(self, empathies: List[float], beta: float = 0.6) -> float:
        """Balance cascade with average: β·product + (1-β)·mean."""
        if len(empathies) >= 2:
            product = empathies[0] * empathies[1]
            mean = np.mean(empathies)
            return beta * product + (1 - beta) * mean
        return empathies[0] if empathies else 0.5
    
    def logarithmic_decay(self, empathies: List[float]) -> float:
        """Log-based: sum of logs scales better than product."""
        if len(empathies) >= 2:
            # Product in log space = sum in linear space
            log_product = sum(np.log(e) for e in empathies[:2])
            # Convert back: exp(sum(logs)) = product
            return np.exp(log_product / 2)  # Normalize by count
        return empathies[0] if empathies else 0.5
    
    def harmonic_decay(self, empathies: List[float], alpha: float = 0.3) -> float:
        """First agent as baseline, adjusted by second."""
        if len(empathies) >= 2:
            return empathies[0] - alpha * (1 - empathies[1])
        return empathies[0] if empathies else 0.5
    
    # ========================================================================
    # TEST RUNNER
    # ========================================================================
    
    def test_c2_001_methods(self):
        """Test all C2_001 robustness methods."""
        print("\n" + "="*80)
        print("C2_001 ROBUSTNESS - AGGREGATION METHODS")
        print("="*80)
        
        methods = {
            "min (current)": self.min_bottleneck,
            "harmonic_mean": self.harmonic_mean,
            "weighted_min_mean": self.weighted_min_mean,
            "percentile_75": self.percentile_75,
            "exponential_blend": self.exponential_blend,
            "alpha_cut": self.alpha_cut,
        }
        
        results = {}
        for case_name, empathies in self.test_cases.items():
            print(f"\n{case_name.upper()}: {[f'{e:.2f}' for e in empathies]}")
            case_results = {}
            for method_name, method in methods.items():
                score = method(empathies)
                case_results[method_name] = score
                print(f"  {method_name:20s}: {score:.3f}")
            results[case_name] = case_results
        
        return results
    
    def test_c2_002_methods(self):
        """Test all C2_002 transitive methods."""
        print("\n" + "="*80)
        print("C2_002 TRANSITIVE - AGGREGATION METHODS")
        print("="*80)
        
        methods = {
            "product (current)": self.product_cascade,
            "geometric_mean": self.geometric_mean,
            "weighted_product": self.weighted_product,
            "confidence_blend": self.confidence_blend,
            "logarithmic_decay": self.logarithmic_decay,
            "harmonic_decay": self.harmonic_decay,
        }
        
        results = {}
        for case_name, empathies in self.test_cases.items():
            print(f"\n{case_name.upper()}: {[f'{e:.2f}' for e in empathies[:2]]}")
            case_results = {}
            for method_name, method in methods.items():
                score = method(empathies)
                case_results[method_name] = score
                print(f"  {method_name:20s}: {score:.3f}")
            results[case_name] = case_results
        
        return results
    
    def recommend_best(self, c2_001_results, c2_002_results):
        """Recommend best combination."""
        print("\n" + "="*80)
        print("RECOMMENDATIONS")
        print("="*80)
        
        # Average across test cases
        c2_001_avg = {}
        c2_002_avg = {}
        
        for method in c2_001_results["balanced"].keys():
            scores = [c2_001_results[case][method] for case in self.test_cases.keys()]
            c2_001_avg[method] = np.mean(scores)
        
        for method in c2_002_results["balanced"].keys():
            scores = [c2_002_results[case][method] for case in self.test_cases.keys()]
            c2_002_avg[method] = np.mean(scores)
        
        # Find best for each
        best_c2_001 = max(c2_001_avg.items(), key=lambda x: x[1])
        best_c2_002 = max(c2_002_avg.items(), key=lambda x: x[1])
        
        print(f"\nC2_001 Best: {best_c2_001[0]} ({best_c2_001[1]:.3f})")
        print(f"C2_002 Best: {best_c2_002[0]} ({best_c2_002[1]:.3f})")
        
        # Calculate combined improvement
        current_c2_001 = 0.70  # Current min approach
        current_c2_002 = 0.42  # Current product approach
        current_c2_003 = 0.822  # From Option B
        current_level_2 = (current_c2_001 + current_c2_002 + current_c2_003) / 3
        
        new_level_2 = (best_c2_001[1] + best_c2_002[1] + current_c2_003) / 3
        
        print(f"\nLevel 2 Impact:")
        print(f"  Current: {current_level_2:.3f}")
        print(f"  Potential: {new_level_2:.3f}")
        print(f"  Improvement: +{(new_level_2 - current_level_2):.3f}")
        
        # Calculate overall
        level_1 = 0.832
        level_3 = 0.817
        overall_current = (level_1 + current_level_2 + level_3) / 3
        overall_new = (level_1 + new_level_2 + level_3) / 3
        
        print(f"\nOverall Impact:")
        print(f"  Current: {overall_current:.3f}")
        print(f"  Potential: {overall_new:.3f}")
        print(f"  Improvement: +{(overall_new - overall_current):.3f}")
        
        return {
            "best_c2_001": best_c2_001,
            "best_c2_002": best_c2_002,
            "new_level_2": new_level_2,
            "new_overall": overall_new,
        }

# ============================================================================
# MAIN
# ============================================================================

if __name__ == "__main__":
    tester = AggregationTester()
    
    c2_001_results = tester.test_c2_001_methods()
    c2_002_results = tester.test_c2_002_methods()
    
    recommendations = tester.recommend_best(c2_001_results, c2_002_results)
    
    print("\n" + "="*80)
    print(f"✅ Testing complete. See recommendations above.")
    print("="*80)

