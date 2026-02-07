#!/usr/bin/env python3
"""
Test Suite for Extended Physics Domains

Tests all new extended physics capabilities:
- Domain detection
- Cross-domain reasoning
- Causal analysis
- Predictive modeling
- Uncertainty quantification
"""

import sys
sys.path.insert(0, '/home/worm/Prime-directive')

from physics_extended_domains import (
    ExtendedPhysicsKnowledgeBase,
    AdvancedPhysicsReasoner,
    ExtendedPhysicsDomain
)
from gaia_extended_physics_integration import (
    ExtendedPhysicsQueryRouter,
    GAIAExtendedPhysicsInterface
)


class TestExtendedPhysics:
    """Comprehensive test suite for extended physics."""

    def __init__(self):
        self.kb = ExtendedPhysicsKnowledgeBase()
        self.reasoner = AdvancedPhysicsReasoner(self.kb)
        self.router = ExtendedPhysicsQueryRouter()
        self.interface = GAIAExtendedPhysicsInterface()
        self.tests_passed = 0
        self.tests_failed = 0

    def assert_true(self, condition, test_name):
        """Assert a condition is true."""
        if condition:
            self.tests_passed += 1
            print(f"  ✅ {test_name}")
        else:
            self.tests_failed += 1
            print(f"  ❌ {test_name}")

    def assert_equal(self, actual, expected, test_name):
        """Assert two values are equal."""
        if actual == expected:
            self.tests_passed += 1
            print(f"  ✅ {test_name}")
        else:
            self.tests_failed += 1
            print(f"  ❌ {test_name} (expected {expected}, got {actual})")

    def assert_greater(self, value, minimum, test_name):
        """Assert value is greater than minimum."""
        if value > minimum:
            self.tests_passed += 1
            print(f"  ✅ {test_name}")
        else:
            self.tests_failed += 1
            print(f"  ❌ {test_name} (expected > {minimum}, got {value})")

    # ─────────────────────────────────────────────────────────────
    # TEST SUITE
    # ─────────────────────────────────────────────────────────────

    def test_knowledge_base_initialization(self):
        """Test KB initialization and law loading."""
        print("\n" + "=" * 80)
        print("TEST 1: Knowledge Base Initialization")
        print("=" * 80)

        self.assert_greater(len(self.kb.laws), 5, "KB has multiple laws")
        self.assert_greater(len(self.kb.principles), 10, "KB has multiple principles")

        # Check specific laws exist
        e_mc2 = self.kb.get_law('E=mc2')
        self.assert_true(e_mc2 is not None, "E=mc² law found")

        if e_mc2:
            self.assert_equal(e_mc2.domain, ExtendedPhysicsDomain.RELATIVITY,
                            "E=mc² is in Relativity domain")

    def test_domain_keywords(self):
        """Test domain keyword detection."""
        print("\n" + "=" * 80)
        print("TEST 2: Domain Detection via Keywords")
        print("=" * 80)

        test_queries = [
            ("black hole and spacetime", ExtendedPhysicsDomain.RELATIVITY),
            ("fluid flow and turbulence", ExtendedPhysicsDomain.FLUID_DYNAMICS),
            ("quantum field interactions", ExtendedPhysicsDomain.QUANTUM_FIELD_THEORY),
            ("expanding universe", ExtendedPhysicsDomain.COSMOLOGY),
            ("particle decay processes", ExtendedPhysicsDomain.PARTICLE_PHYSICS),
            ("light interference patterns", ExtendedPhysicsDomain.OPTICS),
        ]

        for query, expected_domain in test_queries:
            detected, confidence = self.router.detect_domain(query)
            self.assert_equal(detected, expected_domain,
                            f"Query '{query}' → {expected_domain.value}")

    def test_reasoning_type_detection(self):
        """Test reasoning type detection."""
        print("\n" + "=" * 80)
        print("TEST 3: Reasoning Type Detection")
        print("=" * 80)

        test_cases = [
            ("Why does gravity exist?", "explanation"),
            ("What will happen to the universe?", "prediction"),
            ("How does mass cause spacetime curvature?", "causal"),
            ("How are black holes and white holes related?", "cross_domain"),
            ("What is the measurement uncertainty?", "uncertainty"),
        ]

        for query, expected_type in test_cases:
            detected_type = self.router.detect_reasoning_type(query)
            self.assert_equal(detected_type, expected_type,
                            f"'{query}' → {expected_type}")

    def test_causal_reasoning(self):
        """Test causal reasoning chain."""
        print("\n" + "=" * 80)
        print("TEST 4: Causal Reasoning")
        print("=" * 80)

        result = self.reasoner.causal_reasoning(
            "mass",
            ExtendedPhysicsDomain.RELATIVITY
        )

        self.assert_equal(result['domain'], ExtendedPhysicsDomain.RELATIVITY.value,
                        "Causal reasoning returns correct domain")
        self.assert_greater(len(result['effects']), 0, "Causal chain has effects")
        self.assert_greater(result['confidence'], 0.5, "Causal confidence > 0.5")

    def test_predictive_reasoning(self):
        """Test predictive outcomes."""
        print("\n" + "=" * 80)
        print("TEST 5: Predictive Reasoning")
        print("=" * 80)

        # Cosmology prediction
        result = self.reasoner.predict_outcome(
            {},
            ExtendedPhysicsDomain.COSMOLOGY,
            "long"
        )

        self.assert_equal(result['domain'], ExtendedPhysicsDomain.COSMOLOGY.value,
                        "Prediction for correct domain")
        self.assert_greater(len(result['predictions']), 0, "Predictions generated")
        self.assert_equal(result['time_scale'], "long", "Correct time scale")

    def test_uncertainty_quantification(self):
        """Test uncertainty quantification."""
        print("\n" + "=" * 80)
        print("TEST 6: Uncertainty Quantification")
        print("=" * 80)

        result = self.reasoner.uncertainty_quantification(
            "hubble_constant",
            ExtendedPhysicsDomain.COSMOLOGY
        )

        self.assert_equal(result['measurement'], 'hubble_constant',
                        "Correct measurement identified")
        self.assert_greater(result['confidence'], 0.9, "High confidence for standard measurements")
        self.assert_true('uncertainty_percent' in result, "Uncertainty quantified")

    def test_query_routing(self):
        """Test complete query routing."""
        print("\n" + "=" * 80)
        print("TEST 7: Query Routing")
        print("=" * 80)

        query = "What is the mass-energy relationship in special relativity?"
        routing = self.router.route_query(query)

        self.assert_true('domain' in routing, "Routing returns domain")
        self.assert_true('reasoning_type' in routing, "Routing returns reasoning type")
        self.assert_true('handler' in routing, "Routing returns handler function")
        self.assert_greater(routing['domain_confidence'], 0.3, "Confidence above threshold")

    def test_interface_processing(self):
        """Test GAIA interface query processing."""
        print("\n" + "=" * 80)
        print("TEST 8: GAIA Interface Processing")
        print("=" * 80)

        query = "Why does mass curve spacetime?"
        result = self.interface.process_physics_query(query)

        self.assert_true('routing' in result, "Result has routing info")
        self.assert_true('physics_answer' in result, "Result has physics answer")
        self.assert_greater(result['physics_answer']['confidence'], 0.5, "Answer confidence > 0.5")

    def test_domain_capabilities(self):
        """Test domain capability reporting."""
        print("\n" + "=" * 80)
        print("TEST 9: Domain Capabilities")
        print("=" * 80)

        caps = self.interface.get_domain_capabilities('relativity')

        self.assert_true('domain' in caps, "Capabilities include domain")
        self.assert_greater(caps['laws_count'], 0, "Domain has laws")
        self.assert_greater(len(caps['reasoning_capabilities']), 0, "Domain has reasoning capabilities")

    def test_cross_domain_analogies(self):
        """Test cross-domain analogies."""
        print("\n" + "=" * 80)
        print("TEST 10: Cross-Domain Analogies")
        print("=" * 80)

        # Test known analogy
        analogy = self.interface.explain_analogy('fluid_dynamics', 'plasma')
        self.assert_true(analogy['analogy'] is not None, "Analogy between fluid dynamics and plasma")

    def test_batch_processing(self):
        """Test batch query processing."""
        print("\n" + "=" * 80)
        print("TEST 11: Batch Query Processing")
        print("=" * 80)

        queries = [
            "What is spacetime?",
            "How do stars form?",
            "What are quarks?",
        ]

        results = self.interface.batch_process_queries(queries)
        self.assert_equal(len(results), len(queries), "All queries processed")
        self.assert_true(all('physics_answer' in r for r in results), "All results have physics answers")

    def test_extended_domain_count(self):
        """Test that all extended domains are available."""
        print("\n" + "=" * 80)
        print("TEST 12: Extended Domain Coverage")
        print("=" * 80)

        expected_domains = [
            'relativity', 'fluid_dynamics', 'qft', 'cosmology', 'particle_physics',
            'optics', 'acoustics', 'statistical_mechanics', 'plasma', 'astrophysics'
        ]

        all_domains = [d.value for d in ExtendedPhysicsDomain]
        for domain in expected_domains:
            self.assert_true(domain in all_domains, f"Domain '{domain}' available")

    def test_mathematical_complexity_levels(self):
        """Test that laws have appropriate complexity levels."""
        print("\n" + "=" * 80)
        print("TEST 13: Mathematical Complexity Levels")
        print("=" * 80)

        complexity_levels = ['simple', 'intermediate', 'advanced']
        for law in self.kb.laws.values():
            self.assert_true(law.mathematical_complexity in complexity_levels,
                           f"Law '{law.name}' has valid complexity: {law.mathematical_complexity}")

    # ─────────────────────────────────────────────────────────────
    # RUN ALL TESTS
    # ─────────────────────────────────────────────────────────────

    def run_all(self):
        """Run all tests."""
        print("\n" + "╔" + "=" * 78 + "╗")
        print("║  EXTENDED PHYSICS TEST SUITE".ljust(79) + "║")
        print("║  Testing domain detection, reasoning, and integration".ljust(79) + "║")
        print("╚" + "=" * 78 + "╝")

        self.test_knowledge_base_initialization()
        self.test_domain_keywords()
        self.test_reasoning_type_detection()
        self.test_causal_reasoning()
        self.test_predictive_reasoning()
        self.test_uncertainty_quantification()
        self.test_query_routing()
        self.test_interface_processing()
        self.test_domain_capabilities()
        self.test_cross_domain_analogies()
        self.test_batch_processing()
        self.test_extended_domain_count()
        self.test_mathematical_complexity_levels()

        self.print_summary()

    def print_summary(self):
        """Print test summary."""
        total = self.tests_passed + self.tests_failed
        percentage = (self.tests_passed / total * 100) if total > 0 else 0

        print("\n" + "=" * 80)
        print("TEST SUMMARY")
        print("=" * 80)
        print(f"\nTotal Tests: {total}")
        print(f"Passed: {self.tests_passed} ✅")
        print(f"Failed: {self.tests_failed} ❌")
        print(f"Success Rate: {percentage:.1f}%")

        if self.tests_failed == 0:
            print("\n🎉 ALL TESTS PASSED!")
        else:
            print(f"\n⚠️  {self.tests_failed} test(s) failed")

        print("=" * 80)


if __name__ == '__main__':
    tester = TestExtendedPhysics()
    tester.run_all()
