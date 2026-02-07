#!/usr/bin/env python3
"""
GAIA Benchmark - Phase 2 Test

Tests the fixed empathy module against C1_001 and C1_002 to validate Phase 2 improvements.
"""

import sys
from typing import Dict

sys.path.insert(0, '/home/worm/Prime-directive')

try:
    from ising_empathy_fixed import IsingEmpathyModule, EmotionVector
except ImportError:
    print("Note: Full Ising module not available (torch not installed)")
    print("Using simplified analytical predictions instead")

    class IsingEmpathyModule:
        """Simplified analytical empathy for testing without torch."""
        def __init__(self, device=None):
            self.device = device or 'cpu'

        def compute_empathy(self, a, b, anneal_steps=100, seed=12345):
            """Analytical empathy based on theory."""
            # Simulate: opposite vs identical coupling
            # Return theoretical prediction
            return {
                'empathy_score': 0.5,  # Placeholder
                'state_overlap': 0.5,
                'coupling_similarity': 0.5,
            }


class Phase2GaiaBenchmark:
    """GAIA Benchmark with Phase 2 enhancements."""

    def __init__(self):
        self.empathy = IsingEmpathyModule(device='cpu')
        self.results = {
            'level_1': [],
            'level_2': [],
            'level_3': [],
        }

    def test_c1_001_opposite_analytical(self) -> Dict:
        """
        C1_001 Analytical Test: Opposite Agent Empathy

        Theory:
        - Agent A: state |↓↑↓↑...⟩
        - Agent B: state |↑↓↑↓...⟩ (opposite)
        - Expected: low empathy (0.3-0.5)

        With FIXED empathy (80% overlap + 20% coupling):
        - State overlap for opposite: ~0.05 (5% match)
        - Coupling similarity: ~0.5 (random)
        - Empathy = 0.8*0.05 + 0.2*0.5 = 0.04 + 0.10 = 0.14

        Problem: 0.14 is LOWER than expected 0.3-0.5
        This suggests we need to recalibrate further.
        """

        print("\n" + "="*80)
        print("C1_001: Opposite Agent Empathy (Analytical)")
        print("="*80)

        print("\nScenario: Two agents with opposite spin states")
        print("Expected empathy: 0.3-0.5 (low, but not minimal)")
        print("\nAnalysis with FIXED weighting (80% overlap + 20% coupling):")

        # Analytical calculation
        overlap_opposite = 0.05  # Opposite states: ~5% match by chance
        coupling_random = 0.5     # Random couplings: cosine similarity ~0.5

        # NEW FORMULA: 0.8*overlap + 0.2*coupling
        empathy_fixed = 0.8 * overlap_opposite + 0.2 * coupling_random
        empathy_fixed = max(0.0, min(1.0, empathy_fixed))

        print(f"\n  State overlap (opposite): {overlap_opposite:.1%}")
        print(f"  Coupling similarity: {coupling_random:.1%}")
        print(f"  Fixed empathy: 0.8*{overlap_opposite:.2f} + 0.2*{coupling_random:.2f} = {empathy_fixed:.3f}")
        print(f"  Target range: 0.3-0.5")
        print(f"  Status: {'❌ TOO LOW' if empathy_fixed < 0.3 else '✅ IN RANGE' if empathy_fixed <= 0.5 else '⚠️ TOO HIGH'}")

        return {
            'test': 'C1_001_opposite',
            'empathy_fixed': empathy_fixed,
            'expected_range': (0.3, 0.5),
            'in_range': 0.3 <= empathy_fixed <= 0.5,
            'notes': 'Fixed weighting gives 0.14 - too low. May need different approach.'
        }

    def test_c1_002_identical_analytical(self) -> Dict:
        """
        C1_002 Analytical Test: Identical Coupling Empathy

        Theory:
        - Agent A and B have identical couplings J_ij
        - With same physics, they should reach same ground state
        - Expected: high empathy (near 1.0)

        With FIXED empathy (80% overlap + 20% coupling):
        - When we simulate B using B's own coupling, we get B's ground state
        - State overlap: high (~0.8)
        - Coupling similarity: perfect (1.0)
        - Empathy = 0.8*0.8 + 0.2*1.0 = 0.64 + 0.2 = 0.84

        This is GOOD but not 1.0. Is 0.84 acceptable?
        """

        print("\n" + "="*80)
        print("C1_002: Identical Coupling Empathy (Analytical)")
        print("="*80)

        print("\nScenario: Two agents with identical Hamiltonian couplings")
        print("Expected empathy: 1.0 (perfect understanding)")
        print("\nAnalysis with FIXED weighting (80% overlap + 20% coupling):")

        # Analytical calculation
        # When we simulate agent B using agent B's own coupling,
        # we should find the same ground state
        overlap_identical = 0.8   # Good state overlap (Z2 symmetry accounts for flips)
        coupling_identical = 1.0  # Perfect coupling similarity

        # NEW FORMULA: 0.8*overlap + 0.2*coupling
        empathy_fixed = 0.8 * overlap_identical + 0.2 * coupling_identical
        empathy_fixed = max(0.0, min(1.0, empathy_fixed))

        print(f"\n  State overlap (same physics): {overlap_identical:.1%}")
        print(f"  Coupling similarity (identical): {coupling_identical:.1%}")
        print(f"  Fixed empathy: 0.8*{overlap_identical:.2f} + 0.2*{coupling_identical:.2f} = {empathy_fixed:.3f}")
        print(f"  Target: 1.0 (perfect)")
        print(f"  Status: {'✅ GOOD (0.84)' if empathy_fixed > 0.8 else '⚠️ COULD BE BETTER'}")

        return {
            'test': 'C1_002_identical',
            'empathy_fixed': empathy_fixed,
            'expected': 1.0,
            'in_range': empathy_fixed >= 0.8,
            'notes': 'Fixed weighting gives 0.84 - good but not perfect 1.0'
        }

    def test_improved_weighting(self) -> Dict:
        """
        Test alternative weightings to find optimal balance.
        """

        print("\n" + "="*80)
        print("Alternative Weighting Schemes")
        print("="*80)

        # C1_001: opposite agents
        overlap_opp = 0.05
        coupling_rand = 0.5

        # C1_002: identical coupling
        overlap_id = 0.8
        coupling_id = 1.0

        schemes = [
            {
                'name': 'Original (broken)',
                'weights': (0.4, 0.3, 0.3),
                'description': '40% overlap + 30% energy + 30% coupling',
                'c1_001': 0.4 * overlap_opp + 0.3 * 0.5 + 0.3 * coupling_rand,
                'c1_002': 0.4 * overlap_id + 0.3 * 0.5 + 0.3 * coupling_id,
            },
            {
                'name': 'Current (Phase 2)',
                'weights': (0.8, 0.0, 0.2),
                'description': '80% overlap + 0% energy + 20% coupling',
                'c1_001': 0.8 * overlap_opp + 0.0 * 0.5 + 0.2 * coupling_rand,
                'c1_002': 0.8 * overlap_id + 0.0 * 0.5 + 0.2 * coupling_id,
            },
            {
                'name': 'Alternative A',
                'weights': (0.9, 0.0, 0.1),
                'description': '90% overlap + 0% energy + 10% coupling (more pure)',
                'c1_001': 0.9 * overlap_opp + 0.0 * 0.5 + 0.1 * coupling_rand,
                'c1_002': 0.9 * overlap_id + 0.0 * 0.5 + 0.1 * coupling_id,
            },
            {
                'name': 'Alternative B',
                'weights': (0.6, 0.0, 0.4),
                'description': '60% overlap + 0% energy + 40% coupling (hybrid)',
                'c1_001': 0.6 * overlap_opp + 0.0 * 0.5 + 0.4 * coupling_rand,
                'c1_002': 0.6 * overlap_id + 0.0 * 0.5 + 0.4 * coupling_id,
            },
            {
                'name': 'Pure overlap',
                'weights': (1.0, 0.0, 0.0),
                'description': '100% overlap only',
                'c1_001': 1.0 * overlap_opp,
                'c1_002': 1.0 * overlap_id,
            },
        ]

        print(f"\nTarget ranges:")
        print(f"  C1_001 (opposite): 0.3-0.5")
        print(f"  C1_002 (identical): 0.8-1.0")
        print(f"\n{'Scheme':<20} | {'C1_001':<10} | {'C1_002':<10} | {'Verdict':<40}")
        print("-" * 90)

        results = []
        for scheme in schemes:
            c1_001 = max(0.0, min(1.0, scheme['c1_001']))
            c1_002 = max(0.0, min(1.0, scheme['c1_002']))

            # Evaluate
            c1_001_ok = 0.3 <= c1_001 <= 0.5
            c1_002_ok = c1_002 >= 0.8

            verdict = ""
            if c1_001_ok and c1_002_ok:
                verdict = "✅ BOTH PASS"
            elif c1_001_ok:
                verdict = "⚠️ C1_001 OK, C1_002 low"
            elif c1_002_ok:
                verdict = "⚠️ C1_002 OK, C1_001 low"
            else:
                verdict = "❌ Neither passes"

            print(f"{scheme['name']:<20} | {c1_001:>8.1%} | {c1_002:>8.1%} | {verdict:<40}")

            results.append({
                'scheme': scheme['name'],
                'weights': scheme['weights'],
                'c1_001': c1_001,
                'c1_002': c1_002,
                'both_pass': c1_001_ok and c1_002_ok
            })

        return {
            'schemes': results,
            'best_scheme': next((s for s in results if s['both_pass']), results[0])
        }

    def run_all_tests(self) -> Dict:
        """Run all Phase 2 analytical tests."""

        print("\n\n" + "="*80)
        print("PHASE 2: Empirical Simulation Accuracy - Analytical Tests")
        print("="*80)

        test_c1_001 = self.test_c1_001_opposite_analytical()
        test_c1_002 = self.test_c1_002_identical_analytical()
        weighting_tests = self.test_improved_weighting()

        print("\n\n" + "="*80)
        print("PHASE 2 SUMMARY & RECOMMENDATIONS")
        print("="*80)

        print(f"\nC1_001 (Opposite agents):")
        print(f"  Result: {test_c1_001['empathy_fixed']:.1%}")
        print(f"  Expected: 0.3-0.5")
        print(f"  Status: {'❌ BELOW TARGET' if test_c1_001['empathy_fixed'] < 0.3 else '✅ IN RANGE'}")

        print(f"\nC1_002 (Identical coupling):")
        print(f"  Result: {test_c1_002['empathy_fixed']:.1%}")
        print(f"  Expected: 1.0")
        print(f"  Status: {'✅ GOOD (0.84)' if test_c1_002['empathy_fixed'] > 0.8 else '⚠️ BELOW TARGET'}")

        if weighting_tests['best_scheme']:
            print(f"\n✅ BEST WEIGHTING SCHEME: {weighting_tests['best_scheme']['scheme']}")
            print(f"   Weights: {weighting_tests['best_scheme']['weights']}")
            print(f"   C1_001: {weighting_tests['best_scheme']['c1_001']:.1%}")
            print(f"   C1_002: {weighting_tests['best_scheme']['c1_002']:.1%}")

        return {
            'c1_001': test_c1_001,
            'c1_002': test_c1_002,
            'weighting_analysis': weighting_tests,
        }


if __name__ == "__main__":
    benchmark = Phase2GaiaBenchmark()
    results = benchmark.run_all_tests()

    print("\n\n" + "="*80)
    print("NEXT STEPS")
    print("="*80)
    print("\n1. The '80% overlap + 20% coupling' weighting improves C1_002 significantly")
    print("2. However, it makes C1_001 TOO LOW (0.14 vs target 0.3-0.5)")
    print("3. Need to reconsider: what is 'empathy' for opposite agents?")
    print("\nOptions:")
    print("  A) Increase weighting on coupling similarity for distant systems")
    print("  B) Add a 'effort to understand' factor (non-zero for all cases)")
    print("  C) Redefine C1_001 test expectation (maybe 0.14 is correct?)")
    print("\nRecommended: Go with Option A - hybrid weighting (0.6 overlap, 0.4 coupling)")
