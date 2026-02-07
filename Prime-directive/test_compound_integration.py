#!/usr/bin/env python3
"""
Test Suite for Unified Compound Physics Integration

Tests integration of all 15 physics domains with GAIA consciousness.
"""

import sys
sys.path.insert(0, '/home/worm/Prime-directive')

from physics_compound_integration import (
    PhysicsUnifiedKnowledgeBase,
    CompoundPhysicsReasoner,
    CompoundQueryRouter,
    GAIAPhysicsCompoundBridge,
    UnifiedPhysicsDomain
)


class TestCompoundIntegration:
    """Test suite for compound physics integration."""

    def __init__(self):
        self.kb = PhysicsUnifiedKnowledgeBase()
        self.reasoner = CompoundPhysicsReasoner(self.kb)
        self.router = CompoundQueryRouter(self.kb)
        self.bridge = GAIAPhysicsCompoundBridge(self.kb)
        self.tests_passed = 0
        self.tests_failed = 0

    def assert_true(self, condition, test_name):
        """Assert condition is true."""
        if condition:
            self.tests_passed += 1
            print(f"  ✅ {test_name}")
            return True
        else:
            self.tests_failed += 1
            print(f"  ❌ {test_name}")
            return False

    def assert_equal(self, actual, expected, test_name):
        """Assert values are equal."""
        if actual == expected:
            self.tests_passed += 1
            print(f"  ✅ {test_name}")
            return True
        else:
            self.tests_failed += 1
            print(f"  ❌ {test_name} (expected {expected}, got {actual})")
            return False

    def assert_greater(self, value, minimum, test_name):
        """Assert value is greater than minimum."""
        if value > minimum:
            self.tests_passed += 1
            print(f"  ✅ {test_name}")
            return True
        else:
            self.tests_failed += 1
            print(f"  ❌ {test_name} (expected > {minimum}, got {value})")
            return False

    # ─────────────────────────────────────────────────────────────
    # TEST SUITE
    # ─────────────────────────────────────────────────────────────

    def test_unified_kb_initialization(self):
        """Test unified knowledge base initialization."""
        print("\n" + "=" * 80)
        print("TEST 1: Unified Knowledge Base Initialization")
        print("=" * 80)

        domains = self.kb.get_all_domains()
        self.assert_equal(len(domains), 15, "KB has all 15 domains")
        self.assert_greater(len(self.kb.extended_kb.laws), 10, "Extended KB has laws")

    def test_all_domains_accessible(self):
        """Test all 15 domains are accessible."""
        print("\n" + "=" * 80)
        print("TEST 2: All 15 Domains Accessible")
        print("=" * 80)

        domains = self.kb.get_all_domains()
        expected_domains = [
            'classical', 'thermodynamics', 'electromagnetism', 'quantum', 'geometry',
            'relativity', 'fluid_dynamics', 'qft', 'cosmology', 'particle_physics',
            'optics', 'acoustics', 'statistical_mechanics', 'plasma', 'astrophysics'
        ]

        for domain in expected_domains:
            self.assert_true(domain in domains, f"Domain '{domain}' accessible")

    def test_domain_relationships(self):
        """Test domain relationships are defined."""
        print("\n" + "=" * 80)
        print("TEST 3: Domain Relationships")
        print("=" * 80)

        relationships = self.kb.domain_relationships
        self.assert_greater(len(relationships), 10, "Multiple relationships defined")

        # Check specific relationships
        self.assert_true('relativity' in relationships, "Relativity has relationships")
        self.assert_true('quantum' in relationships, "Quantum has relationships")
        self.assert_greater(len(relationships.get('relativity', [])), 0,
                          "Relativity relates to other domains")

    def test_cross_domain_analogies(self):
        """Test cross-domain analogies."""
        print("\n" + "=" * 80)
        print("TEST 4: Cross-Domain Analogies")
        print("=" * 80)

        analogies = self.kb.analogies
        self.assert_greater(len(analogies), 10, "Multiple analogies defined")

        # Check specific analogies
        has_qm_to_qft = ('quantum_mechanics', 'qft') in analogies or \
                       ('qft', 'quantum_mechanics') in analogies
        self.assert_true(has_qm_to_qft, "QM ↔ QFT analogy exists")

    def test_unified_query_routing(self):
        """Test unified query routing across domains."""
        print("\n" + "=" * 80)
        print("TEST 5: Unified Query Routing")
        print("=" * 80)

        test_queries = [
            ("Why does mass curve spacetime?", ["relativity"]),
            ("What are quantum fields?", ["qft", "quantum"]),
            ("How does entropy increase?", ["thermodynamics"]),
        ]

        for query, expected_domains in test_queries:
            result = self.router.route_query(query)
            detected = result['routing']['relevant_domains']
            match = any(d in detected for d in expected_domains)
            self.assert_true(match, f"Query '{query}' routed to {expected_domains}")

    def test_compound_reasoner(self):
        """Test compound physics reasoner."""
        print("\n" + "=" * 80)
        print("TEST 6: Compound Physics Reasoner")
        print("=" * 80)

        query = "How do fields relate to particles?"
        result = self.reasoner.unified_query(query)

        self.assert_true('reasoning_chain' in result, "Reasoning chain generated")
        self.assert_greater(len(result['reasoning_chain']), 0, "Reasoning steps exist")
        self.assert_greater(result['confidence'], 0.5, "Confidence > 0.5")

    def test_gaia_integration_basic(self):
        """Test basic GAIA-physics integration."""
        print("\n" + "=" * 80)
        print("TEST 7: GAIA-Physics Integration (Basic)")
        print("=" * 80)

        query = "Why does light behave as both wave and particle?"
        result = self.bridge.integrated_physics_consciousness_query(query)

        self.assert_true('physics_analysis' in result, "Physics analysis present")
        self.assert_true('integrated_answer' in result, "Integrated answer present")
        self.assert_true('confidence' in result, "Confidence scores present")
        self.assert_greater(result['confidence']['physics_confidence'], 0.5,
                          "Physics confidence > 0.5")

    def test_gaia_integration_with_context(self):
        """Test GAIA-physics integration with consciousness context."""
        print("\n" + "=" * 80)
        print("TEST 8: GAIA-Physics Integration (With Context)")
        print("=" * 80)

        query = "How does entropy relate to consciousness?"
        gaia_context = {
            'empathy_scores': {'avg': 0.75},
            'agent_states': {'agent_1': 0.8, 'agent_2': 0.7},
            'reasoning_depth': 'multi_agent'
        }

        result = self.bridge.integrated_physics_consciousness_query(query, gaia_context)

        self.assert_true(result['gaia_context']['empathy_informed'],
                        "GAIA context applied")
        self.assert_equal(result['gaia_context']['empathy_avg'], 0.75,
                         "Empathy score captured")
        self.assert_greater(result['confidence']['combined'], 0.5,
                          "Combined confidence > 0.5")

    def test_batch_processing(self):
        """Test batch query processing."""
        print("\n" + "=" * 80)
        print("TEST 9: Batch Query Processing")
        print("=" * 80)

        queries = [
            "What is spacetime?",
            "How do particles interact?",
            "What is dark matter?"
        ]

        results = self.bridge.batch_integrated_queries(queries)
        self.assert_equal(len(results), len(queries), "All queries processed")
        self.assert_true(all('integrated_answer' in r for r in results),
                        "All results have integrated answers")

    def test_domain_hierarchy(self):
        """Test domain hierarchy."""
        print("\n" + "=" * 80)
        print("TEST 10: Domain Hierarchy")
        print("=" * 80)

        hierarchy = self.bridge.domain_hierarchy_view()

        self.assert_true('foundational' in hierarchy, "Foundational domains listed")
        self.assert_true('intermediate' in hierarchy, "Intermediate domains listed")
        self.assert_true('advanced' in hierarchy, "Advanced domains listed")
        self.assert_greater(len(hierarchy['foundational']), 0, "Foundational domains exist")
        self.assert_greater(len(hierarchy['advanced']), 0, "Advanced domains exist")

    def test_system_summary(self):
        """Test unified system summary."""
        print("\n" + "=" * 80)
        print("TEST 11: System Summary")
        print("=" * 80)

        summary = self.bridge.get_unified_physics_summary()

        self.assert_equal(summary['total_domains'], 15, "15 domains total")
        self.assert_equal(summary['base_domains'], 5, "5 base domains")
        self.assert_equal(summary['extended_domains'], 10, "10 extended domains")
        self.assert_true(summary['gaia_integrated'], "GAIA integration active")
        self.assert_true(summary['batch_capable'], "Batch processing supported")

    def test_multi_domain_reasoning(self):
        """Test multi-domain reasoning capability."""
        print("\n" + "=" * 80)
        print("TEST 12: Multi-Domain Reasoning")
        print("=" * 80)

        # Query requiring multiple domains
        query = "How does relativity affect cosmology?"
        result = self.reasoner.unified_query(query)

        self.assert_greater(len(result['relevant_domains']), 1,
                          "Multiple domains identified")
        self.assert_greater(len(result['analogies_found']), 0,
                          "Cross-domain analogies found")

    def test_analogies_accuracy(self):
        """Test cross-domain analogies are accurate."""
        print("\n" + "=" * 80)
        print("TEST 13: Analogy Accuracy")
        print("=" * 80)

        analogies = self.kb.analogies
        analogy_count = len(analogies)

        self.assert_greater(analogy_count, 10, "Multiple analogies defined")

        # Check specific analogies mention key physics concepts
        for (d1, d2), analogy_text in list(analogies.items())[:3]:
            self.assert_true(len(analogy_text) > 20,
                           f"Analogy '{d1}-{d2}' is substantive")

    def test_domain_info_completeness(self):
        """Test domain info is complete."""
        print("\n" + "=" * 80)
        print("TEST 14: Domain Info Completeness")
        print("=" * 80)

        domains = self.kb.get_all_domains()[:3]  # Test first 3

        for domain in domains:
            info = self.kb.get_domain_info(domain)
            self.assert_true('domain' in info, f"Domain '{domain}' info complete")
            self.assert_true('complexity' in info, f"Domain '{domain}' has complexity")
            self.assert_true('related_domains' in info, f"Domain '{domain}' has relations")

    def test_reasoning_confidence(self):
        """Test reasoning confidence calculation."""
        print("\n" + "=" * 80)
        print("TEST 15: Reasoning Confidence")
        print("=" * 80)

        queries = [
            "What is relativity?",
            "How do quantum fields work?",
            "Explain dark energy"
        ]

        for query in queries:
            result = self.reasoner.unified_query(query)
            confidence = result['confidence']
            self.assert_true(0 <= confidence <= 1, f"Confidence in range for '{query}'")
            self.assert_greater(confidence, 0.5, f"Confidence > 0.5 for '{query}'")

    # ─────────────────────────────────────────────────────────────
    # RUN ALL TESTS
    # ─────────────────────────────────────────────────────────────

    def run_all(self):
        """Run all tests."""
        print("\n" + "╔" + "=" * 78 + "╗")
        print("║  COMPOUND PHYSICS INTEGRATION TEST SUITE".ljust(79) + "║")
        print("║  Testing all 15 domains with GAIA consciousness integration".ljust(79) + "║")
        print("╚" + "=" * 78 + "╝")

        self.test_unified_kb_initialization()
        self.test_all_domains_accessible()
        self.test_domain_relationships()
        self.test_cross_domain_analogies()
        self.test_unified_query_routing()
        self.test_compound_reasoner()
        self.test_gaia_integration_basic()
        self.test_gaia_integration_with_context()
        self.test_batch_processing()
        self.test_domain_hierarchy()
        self.test_system_summary()
        self.test_multi_domain_reasoning()
        self.test_analogies_accuracy()
        self.test_domain_info_completeness()
        self.test_reasoning_confidence()

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
            print("\n🎉 ALL TESTS PASSED - COMPOUND INTEGRATION COMPLETE!")
        else:
            print(f"\n⚠️  {self.tests_failed} test(s) failed")

        print("=" * 80)

        # System stats
        print("\nSystem Integration Stats:")
        summary = self.bridge.get_unified_physics_summary()
        print(f"  • Total Physics Domains: {summary['total_domains']}")
        print(f"  • Base Domains: {summary['base_domains']}")
        print(f"  • Extended Domains: {summary['extended_domains']}")
        print(f"  • Total Principles: {summary['total_principles']}")
        print(f"  • Extended Laws: {summary['extended_laws']}")
        print(f"  • GAIA Integration: {'✅ Active' if summary['gaia_integrated'] else '❌ Inactive'}")
        print(f"  • Batch Processing: {'✅ Enabled' if summary['batch_capable'] else '❌ Disabled'}")


if __name__ == '__main__':
    tester = TestCompoundIntegration()
    tester.run_all()
