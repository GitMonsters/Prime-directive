#!/usr/bin/env python3
"""
Phase 2 Diagnostic: Debug Empirical Simulation Accuracy

Tests C1_001 and C1_002 against analytical solutions to identify bottlenecks.
"""

import torch
import math
import sys
from typing import Dict, Tuple

sys.path.insert(0, '/home/worm/Prime-directive')

try:
    from ising_empathy_module import IsingGPU, IsingEmpathyModule
except ImportError:
    print("❌ Cannot import Ising module")
    sys.exit(1)


class Phase2Diagnostic:
    """Diagnostic tools for Phase 2 debugging."""

    def __init__(self):
        self.device = torch.device('cpu')  # Use CPU for deterministic testing
        self.empathy = IsingEmpathyModule(device=self.device)
        self.results = {}

    # =========================================================================
    # C1_001: OPPOSITE AGENT EMPATHY
    # =========================================================================

    def test_c1_001_opposite_empathy(self) -> Dict:
        """
        Test C1_001: Empathy between agents with opposite states.

        Theory: If Agent A has state |↓↑↓↑...⟩ and Agent B has |↑↓↑↓...⟩,
        they should have low empathy (0.3-0.5) because:
        - State overlap: nearly 0 (all opposite)
        - Energy: interactions will be frustrated
        - Coupling similarity: depends on structure
        """

        print("\n" + "="*80)
        print("TEST C1_001: Opposite Agent Empathy")
        print("="*80)

        # Create two agents with OPPOSITE initial states
        print("\nSetup: Two agents with opposite states")
        agent_a = IsingGPU(n=20, seed=42, device=self.device)
        agent_b = IsingGPU(n=43, seed=43, device=self.device)

        # Force opposite states for deterministic testing
        print("Forcing opposite states for testing...")
        agent_a.spins = torch.ones(20, device=self.device)  # All up
        agent_b.spins = -torch.ones(20, device=self.device)  # All down (opposite)

        print(f"Agent A state: {agent_a.spins[:5].tolist()} ... (all +1)")
        print(f"Agent B state: {agent_b.spins[:5].tolist()} ... (all -1)")
        print(f"Agent A magnetization: {agent_a.magnetization():.3f}")
        print(f"Agent B magnetization: {agent_b.magnetization():.3f}")

        # Test with various annealing steps
        print("\nTesting empathy with different annealing steps:")
        print("Steps | Empathy | Overlap | Energy Error | Coupling Sim | Issue?")
        print("------|---------|---------|--------------|--------------|-------")

        results = []
        for anneal_steps in [10, 30, 50, 100, 200]:
            empathy_result = self.empathy.compute_empathy(
                agent_a, agent_b,
                anneal_steps=anneal_steps,
                seed=12345
            )

            overlap = empathy_result['state_overlap']
            energy_err = empathy_result['energy_error']
            coupling_sim = empathy_result['coupling_similarity']
            empathy = empathy_result['empathy_score']

            # Diagnose issues
            issues = []
            if overlap > 0.3:
                issues.append("overlap HIGH (should be ~0)")
            if energy_err > 2.0:
                issues.append("energy_err HIGH")
            if empathy > 0.6:
                issues.append("empathy HIGH (expect 0.3-0.5)")

            issue_str = " | ".join(issues) if issues else "OK"

            results.append({
                'steps': anneal_steps,
                'empathy': empathy,
                'overlap': overlap,
                'energy_error': energy_err,
                'coupling_sim': coupling_sim,
                'issues': issues
            })

            print(f"{anneal_steps:5d} | {empathy:7.1%} | {overlap:7.1%} | "
                  f"{energy_err:12.3f} | {coupling_sim:12.3f} | {issue_str}")

        # Analysis
        print("\n" + "-"*80)
        print("ANALYSIS:")
        print("-"*80)

        avg_empathy = sum(r['empathy'] for r in results) / len(results)
        print(f"Average empathy: {avg_empathy:.1%}")
        print(f"Expected: 0.3-0.5 (30-50%)")
        print(f"Actual: {avg_empathy:.1%}")

        if avg_empathy > 0.5:
            print("❌ PROBLEM: Empathy too high for opposite states")
            print("   Root causes:")
            print("   1. Theory of Mind simulation may not properly separate agents")
            print("   2. Coupling similarity might override state overlap")
            print("   3. Energy error calculation might be incorrect")
        else:
            print("✅ OK: Empathy in expected range")

        return {
            'test_name': 'C1_001_opposite_empathy',
            'results': results,
            'avg_empathy': avg_empathy,
            'expected': (0.3, 0.5),
            'pass': 0.3 <= avg_empathy <= 0.5
        }

    # =========================================================================
    # C1_002: IDENTICAL COUPLING EMPATHY
    # =========================================================================

    def test_c1_002_identical_coupling(self) -> Dict:
        """
        Test C1_002: Empathy when two agents have identical Hamiltonian couplings.

        Theory: If J_ij is identical for both agents, they should reach perfect
        understanding (empathy = 1.0) because:
        - Same coupling -> same ground state preferences
        - Both want to align same way
        - Empathy measures understanding -> perfect for same physics
        """

        print("\n" + "="*80)
        print("TEST C1_002: Identical Coupling Empathy")
        print("="*80)

        print("\nSetup: Two agents with IDENTICAL couplings")

        # Create two agents
        agent_a = IsingGPU(n=20, seed=42, device=self.device)
        agent_b = IsingGPU(n=20, seed=43, device=self.device)

        # Force identical couplings (copy A's to B)
        print("Making couplings identical (agent_b.J = agent_a.J)...")
        agent_b.coupling = agent_a.coupling.clone()
        agent_b.field = agent_a.field.clone()

        print(f"Agent A coupling[0,1:5]: {agent_a.coupling[0, 1:5].tolist()}")
        print(f"Agent B coupling[0,1:5]: {agent_b.coupling[0, 1:5].tolist()}")
        print(f"Couplings identical: {torch.allclose(agent_a.coupling, agent_b.coupling)}")

        # Now let both anneal to ground state with identical physics
        print("\nAnnealing both agents with identical couplings...")
        agent_a.anneal(100, seed=999)
        agent_b.anneal(100, seed=999)

        print(f"Agent A final state: {agent_a.spins[:5].tolist()}")
        print(f"Agent B final state: {agent_b.spins[:5].tolist()}")
        print(f"Agent A magnetization: {agent_a.magnetization():.3f}")
        print(f"Agent B magnetization: {agent_b.magnetization():.3f}")

        # Test empathy
        print("\nTesting empathy with identical couplings:")
        print("Steps | Empathy | Overlap | Energy Error | Coupling Sim | Issue?")
        print("------|---------|---------|--------------|--------------|-------")

        results = []
        for anneal_steps in [10, 30, 50, 100, 200]:
            empathy_result = self.empathy.compute_empathy(
                agent_a, agent_b,
                anneal_steps=anneal_steps,
                seed=12345
            )

            overlap = empathy_result['state_overlap']
            energy_err = empathy_result['energy_error']
            coupling_sim = empathy_result['coupling_similarity']
            empathy = empathy_result['empathy_score']

            # Diagnose issues
            issues = []
            if coupling_sim < 0.95:
                issues.append("coupling_sim LOW (should be ~1.0)")
            if empathy < 0.8:
                issues.append("empathy LOW (expect 1.0)")

            issue_str = " | ".join(issues) if issues else "OK"

            results.append({
                'steps': anneal_steps,
                'empathy': empathy,
                'overlap': overlap,
                'energy_error': energy_err,
                'coupling_sim': coupling_sim,
                'issues': issues
            })

            print(f"{anneal_steps:5d} | {empathy:7.1%} | {overlap:7.1%} | "
                  f"{energy_err:12.3f} | {coupling_sim:12.3f} | {issue_str}")

        # Analysis
        print("\n" + "-"*80)
        print("ANALYSIS:")
        print("-"*80)

        avg_empathy = sum(r['empathy'] for r in results) / len(results)
        avg_coupling_sim = sum(r['coupling_sim'] for r in results) / len(results)

        print(f"Average empathy: {avg_empathy:.1%}")
        print(f"Expected: 1.0 (perfect)")
        print(f"Actual: {avg_empathy:.1%}")
        print(f"\nAverage coupling similarity: {avg_coupling_sim:.1%}")
        print(f"Expected: 1.0 (identical)")

        if avg_empathy < 0.8:
            print("❌ PROBLEM: Empathy too low for identical couplings")
            print("   Root causes:")
            print("   1. Coupling similarity calculation might be wrong")
            print("   2. State overlap might not account for Z2 symmetry properly")
            print("   3. Energy error might not handle identical physics")
        else:
            print("✅ OK: Empathy high for identical couplings")

        return {
            'test_name': 'C1_002_identical_coupling',
            'results': results,
            'avg_empathy': avg_empathy,
            'expected': 1.0,
            'pass': avg_empathy >= 0.8
        }

    # =========================================================================
    # ANALYTICAL BASELINE TEST
    # =========================================================================

    def test_analytical_two_agent_baseline(self) -> Dict:
        """
        Test against analytical solution: two agents with known coupling.

        Analytical case:
        - Agent A: spin s_A
        - Agent B: spin s_B
        - Coupling: J
        - Energy: H = -J * s_A * s_B

        Ground states:
        - J > 0 (ferromagnetic): s_A = s_B (aligned) minimizes H
        - J < 0 (antiferromagnetic): s_A ≠ s_B (opposite) minimizes H

        This should give empathy close to analytical predictions.
        """

        print("\n" + "="*80)
        print("TEST ANALYTICAL BASELINE: Two-Agent System")
        print("="*80)

        print("\nAnalytical case: Two spins with coupling J")
        print("Expected ground states:")
        print("  J > 0 (ferromagnetic): s_A = s_B (both up or both down)")
        print("  J < 0 (antiferromagnetic): s_A ≠ s_B (opposite)")

        results = []

        # Test ferromagnetic (should lead to same state)
        print("\n" + "-"*80)
        print("Case 1: Ferromagnetic coupling (J > 0)")
        print("-"*80)

        agent_a = IsingGPU(n=2, seed=42, device=self.device)
        agent_b = IsingGPU(n=2, seed=43, device=self.device)

        # Set strong ferromagnetic coupling
        agent_a.coupling[0, 1] = 1.0
        agent_a.coupling[1, 0] = 1.0
        agent_b.coupling[0, 1] = 1.0
        agent_b.coupling[1, 0] = 1.0

        # Anneal both
        agent_a.anneal(100, seed=999)
        agent_b.anneal(100, seed=999)

        print(f"Agent A state: {agent_a.spins.tolist()}")
        print(f"Agent B state: {agent_b.spins.tolist()}")
        print(f"Magnetizations: A={agent_a.magnetization():.3f}, B={agent_b.magnetization():.3f}")

        # Calculate empathy
        empathy_result = self.empathy.compute_empathy(agent_a, agent_b, anneal_steps=100, seed=12345)
        overlap = empathy_result['state_overlap']
        coupling_sim = empathy_result['coupling_similarity']
        empathy = empathy_result['empathy_score']

        print(f"\nEmpathy score: {empathy:.1%}")
        print(f"  State overlap: {overlap:.1%}")
        print(f"  Coupling similarity: {coupling_sim:.1%}")
        print(f"Analysis: {'✅ HIGH' if empathy > 0.7 else '❌ LOW'} (expect high for same coupling)")

        results.append({
            'case': 'Ferromagnetic',
            'empathy': empathy,
            'overlap': overlap,
            'expected': 'high',
            'pass': empathy > 0.7
        })

        return {
            'test_name': 'analytical_baseline',
            'results': results
        }

    # =========================================================================
    # RUN ALL TESTS
    # =========================================================================

    def run_all_tests(self) -> Dict:
        """Run all Phase 2 diagnostic tests."""

        print("\n\n" + "="*80)
        print("PHASE 2 DIAGNOSTIC: Empirical Simulation Accuracy")
        print("="*80)

        test_c1_001 = self.test_c1_001_opposite_empathy()
        test_c1_002 = self.test_c1_002_identical_coupling()
        test_analytical = self.test_analytical_two_agent_baseline()

        print("\n\n" + "="*80)
        print("SUMMARY")
        print("="*80)

        print(f"\nC1_001 (Opposite agents):")
        print(f"  Result: {test_c1_001['avg_empathy']:.1%}")
        print(f"  Expected: 0.3-0.5")
        print(f"  Status: {'✅ PASS' if test_c1_001['pass'] else '❌ FAIL'}")

        print(f"\nC1_002 (Identical coupling):")
        print(f"  Result: {test_c1_002['avg_empathy']:.1%}")
        print(f"  Expected: 1.0")
        print(f"  Status: {'✅ PASS' if test_c1_002['pass'] else '❌ FAIL'}")

        print(f"\nAnalytical baseline:")
        for result in test_analytical['results']:
            print(f"  {result['case']}: {result['empathy']:.1%} (expect {result['expected']})")

        return {
            'tests': {
                'C1_001': test_c1_001,
                'C1_002': test_c1_002,
                'analytical': test_analytical
            },
            'overall_pass': test_c1_001['pass'] and test_c1_002['pass']
        }


if __name__ == "__main__":
    diagnostic = Phase2Diagnostic()
    results = diagnostic.run_all_tests()

    if not results['overall_pass']:
        print("\n\n" + "="*80)
        print("NEXT STEPS FOR PHASE 2")
        print("="*80)
        print("\nIdentified issues. Recommended fixes:")
        print("1. Check empathy score weighting (0.4/0.3/0.3)")
        print("2. Verify state overlap calculation (handle Z2 symmetry)")
        print("3. Test with clean analytical cases")
        print("4. Increase MCMC iterations")
        print("5. Validate coupling similarity calculation")
