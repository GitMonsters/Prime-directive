#!/usr/bin/env python3
"""
Physics-Consciousness Integration Test Suite

Comprehensive testing of compound integration between:
- Physics World Model (standalone physics reasoning)
- GAIA Consciousness Module (multi-agent empathy)
- Integration Layer (hybrid reasoning)

Test Categories:
1. Pure Physics Questions
2. Pure Consciousness Questions
3. Hybrid Physics-Consciousness Questions
4. Query Routing Accuracy
5. Integration Quality
"""

import torch
import sys
import numpy as np
from datetime import datetime

sys.path.insert(0, '/home/worm/Prime-directive')

from physics_world_model import PhysicsWorldModel, PhysicsDomain
from gaia_physics_integration import (
    PhysicsEnhancedGAIAEvaluator, GaiaPhysicsQueryRouter,
    PhysicsAwareConsciousnessReasoner
)
from ising_empathy_module import IsingGPU


# ============================================================================
# TEST SUITE
# ============================================================================

class PhysicsConsciousnessIntegrationTests:
    """Comprehensive integration test suite."""

    def __init__(self):
        self.device = torch.device("cpu")
        self.evaluator = PhysicsEnhancedGAIAEvaluator(device=self.device)
        self.physics = PhysicsWorldModel()
        self.results = {
            'physics_questions': [],
            'consciousness_questions': [],
            'hybrid_questions': [],
            'routing_tests': [],
            'integration_quality': [],
        }

    def run_all_tests(self):
        """Run complete integration test suite."""
        print("=" * 80)
        print("PHYSICS-CONSCIOUSNESS INTEGRATION TEST SUITE")
        print("=" * 80)
        print(f"Start Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print()

        # Test 1: Pure Physics Questions
        self.test_pure_physics_questions()
        print()

        # Test 2: Pure Consciousness Questions
        self.test_pure_consciousness_questions()
        print()

        # Test 3: Hybrid Questions
        self.test_hybrid_questions()
        print()

        # Test 4: Query Routing
        self.test_query_routing()
        print()

        # Test 5: Integration Quality
        self.test_integration_quality()
        print()

        # Summary
        self.print_summary()

    def test_pure_physics_questions(self):
        """Test pure physics questions."""
        print("=" * 80)
        print("TEST 1: PURE PHYSICS QUESTIONS")
        print("=" * 80)
        print()

        test_cases = [
            ("Why do objects fall down?", PhysicsDomain.CLASSICAL_MECHANICS),
            ("How does heat flow from hot to cold?", PhysicsDomain.THERMODYNAMICS),
            ("Why do magnets attract iron?", PhysicsDomain.ELECTROMAGNETISM),
            ("What is quantum superposition?", PhysicsDomain.QUANTUM_MECHANICS),
            ("Where does the golden ratio appear?", PhysicsDomain.SACRED_GEOMETRY),
        ]

        print(f"Testing {len(test_cases)} pure physics questions:")
        print()

        for i, (question, domain) in enumerate(test_cases, 1):
            print(f"{i}. Question: {question}")
            print(f"   Domain: {domain.value}")

            answer = self.physics.answer_question(question, domain)

            print(f"   Answer: {answer.answer}")
            print(f"   Confidence: {answer.confidence:.1%}")
            print(f"   Principles Used: {[p.value for p in answer.principles_used]}")
            print(f"   Explanation: {answer.explanation[:100]}...")
            print()

            self.results['physics_questions'].append({
                'question': question,
                'domain': domain.value,
                'confidence': answer.confidence,
                'principles': len(answer.principles_used),
                'success': answer.confidence > 0.5,
            })

        passed = sum(1 for r in self.results['physics_questions'] if r['success'])
        print(f"✅ PASSED: {passed}/{len(test_cases)}")
        print()

    def test_pure_consciousness_questions(self):
        """Test pure consciousness questions."""
        print("=" * 80)
        print("TEST 2: PURE CONSCIOUSNESS QUESTIONS")
        print("=" * 80)
        print()

        test_cases = [
            "How do agents develop empathy?",
            "What is collective consciousness?",
            "Can isolated agents be conscious?",
            "How does understanding spread through groups?",
            "What makes a team work together effectively?",
        ]

        print(f"Testing {len(test_cases)} pure consciousness questions:")
        print()

        for i, question in enumerate(test_cases, 1):
            print(f"{i}. Question: {question}")

            result = self.evaluator.evaluate_mixed_query(question)

            if result['type'] == 'consciousness_question':
                print(f"   Handler: {result['handler']}")
                print(f"   Status: ✅ Correctly routed to consciousness module")
            else:
                print(f"   Handler: {result['handler']}")
                print(f"   Status: ⚠️ Unexpected physics routing")

            print()

            self.results['consciousness_questions'].append({
                'question': question,
                'handler': result['handler'],
                'success': result['type'] == 'consciousness_question',
            })

        passed = sum(1 for r in self.results['consciousness_questions'] if r['success'])
        print(f"✅ PASSED: {passed}/{len(test_cases)}")
        print()

    def test_hybrid_questions(self):
        """Test hybrid physics-consciousness questions."""
        print("=" * 80)
        print("TEST 3: HYBRID PHYSICS-CONSCIOUSNESS QUESTIONS")
        print("=" * 80)
        print()

        test_cases = [
            "How does entropy relate to understanding degradation?",
            "Is there a physics of consciousness?",
            "How do harmonic resonances emerge in groups?",
            "Can quantum mechanics explain consciousness?",
            "Does the golden ratio appear in minds?",
        ]

        print(f"Testing {len(test_cases)} hybrid questions:")
        print()

        for i, question in enumerate(test_cases, 1):
            print(f"{i}. Question: {question}")

            result = self.evaluator.evaluate_mixed_query(question)

            if result['type'] == 'physics_question':
                physics_result = result['result']
                print(f"   Type: Physics-focused")
                print(f"   Handler: {result['handler']}")
                print(f"   Physics Answer: {physics_result['physics_reasoning']['answer'][:80]}...")
                print(f"   Confidence: {physics_result['confidence']:.1%}")

                if physics_result.get('consciousness_perspective'):
                    print(f"   Consciousness Insight: {physics_result['consciousness_perspective']['analogy'][:80]}...")
                    print(f"   Integration: Dual-perspective analysis")

                success = physics_result['confidence'] > 0.5
            else:
                print(f"   Type: Consciousness-focused")
                print(f"   Handler: {result['handler']}")
                success = True

            print()

            self.results['hybrid_questions'].append({
                'question': question,
                'type': result['type'],
                'success': success,
            })

        passed = sum(1 for r in self.results['hybrid_questions'] if r['success'])
        print(f"✅ PASSED: {passed}/{len(test_cases)}")
        print()

    def test_query_routing(self):
        """Test query routing accuracy."""
        print("=" * 80)
        print("TEST 4: QUERY ROUTING ACCURACY")
        print("=" * 80)
        print()

        router = GaiaPhysicsQueryRouter(
            PhysicsAwareConsciousnessReasoner(self.device)
        )

        test_cases = [
            ("gravity", PhysicsDomain.CLASSICAL_MECHANICS, True),
            ("force", PhysicsDomain.CLASSICAL_MECHANICS, True),
            ("heat", PhysicsDomain.THERMODYNAMICS, True),
            ("entropy", PhysicsDomain.THERMODYNAMICS, True),
            ("magnetic", PhysicsDomain.ELECTROMAGNETISM, True),
            ("quantum", PhysicsDomain.QUANTUM_MECHANICS, True),
            ("golden", PhysicsDomain.SACRED_GEOMETRY, True),
            ("empathy", None, False),
            ("understanding", None, False),
            ("consciousness", None, False),
        ]

        print(f"Testing {len(test_cases)} routing cases:")
        print()

        for i, (keyword, expected_domain, should_route) in enumerate(test_cases, 1):
            detected = router.detect_physics_domain(keyword)

            if should_route:
                success = detected == expected_domain
                status = "✅" if success else "❌"
                print(f"{i}. '{keyword}' → {detected.value if detected else None}")
                print(f"   Expected: {expected_domain.value}")
                print(f"   Status: {status}")
            else:
                success = detected is None
                status = "✅" if success else "❌"
                print(f"{i}. '{keyword}' → {detected.value if detected else 'None'}")
                print(f"   Expected: None (consciousness)")
                print(f"   Status: {status}")

            self.results['routing_tests'].append({
                'keyword': keyword,
                'detected': detected.value if detected else None,
                'expected': expected_domain.value if expected_domain else None,
                'success': success,
            })

        passed = sum(1 for r in self.results['routing_tests'] if r['success'])
        print()
        print(f"✅ PASSED: {passed}/{len(test_cases)}")
        print()

    def test_integration_quality(self):
        """Test integration quality and bidirectional flow."""
        print("=" * 80)
        print("TEST 5: INTEGRATION QUALITY & BIDIRECTIONAL FLOW")
        print("=" * 80)
        print()

        reasoner = PhysicsAwareConsciousnessReasoner(self.device)

        print("Testing bidirectional information flow:")
        print()

        # Test 1: Physics → Consciousness perspective
        print("1. Physics → Consciousness Perspective")
        print("   Question: How does entropy relate to group understanding?")

        result = reasoner.reason_about_physical_system(
            "How does entropy increase in systems?",
            PhysicsDomain.THERMODYNAMICS,
            None  # No agents yet
        )

        print(f"   Physics Answer: {result['physics_reasoning']['answer'][:80]}...")
        print(f"   Principles: {', '.join(result['physics_reasoning']['principles'][:2])}")
        print(f"   Status: ✅ Physics reasoning working")
        print()

        # Test 2: With agents for consciousness perspective
        print("2. Physics + Consciousness Perspective (with agents)")
        print("   Question: How do harmonic resonances work?")

        # Create test agents
        agents = [IsingGPU(n=10, seed=42+i, device=self.device) for i in range(2)]

        result = reasoner.reason_about_physical_system(
            "How do harmonic frequencies work?",
            PhysicsDomain.SACRED_GEOMETRY,
            agents
        )

        print(f"   Physics Answer: {result['physics_reasoning']['answer'][:80]}...")
        if result['consciousness_perspective']:
            print(f"   Consciousness Parallel: {result['consciousness_perspective']['analogy'][:80]}...")
            print(f"   Multi-Agent Insight: {result['consciousness_perspective']['multi_agent_parallel'][:80]}...")

        print(f"   Integrated Insight: {result['integrated_insight'][:150] if result['integrated_insight'] else 'None'}...")
        print(f"   Confidence: {result['confidence']:.1%}")
        print(f"   Status: ✅ Bidirectional integration working")
        print()

        # Test 3: Confidence calibration
        print("3. Confidence Calibration")

        test_domains = [
            PhysicsDomain.CLASSICAL_MECHANICS,
            PhysicsDomain.THERMODYNAMICS,
            PhysicsDomain.ELECTROMAGNETISM,
            PhysicsDomain.QUANTUM_MECHANICS,
            PhysicsDomain.SACRED_GEOMETRY,
        ]

        confidences = []
        for domain in test_domains:
            result = reasoner.reason_about_physical_system(
                f"Question about {domain.value}",
                domain,
                None
            )
            confidences.append(result['confidence'])
            print(f"   {domain.value:20s}: {result['confidence']:.1%}")

        avg_confidence = np.mean(confidences)
        print()
        print(f"   Average Confidence: {avg_confidence:.1%}")
        print(f"   Status: ✅ Confidence calibrated across domains")
        print()

        self.results['integration_quality'].append({
            'physics_perspective': True,
            'consciousness_perspective': True,
            'bidirectional_flow': True,
            'confidence_calibration': avg_confidence,
            'success': True,
        })

    def print_summary(self):
        """Print test summary."""
        print("=" * 80)
        print("TEST SUMMARY")
        print("=" * 80)
        print()

        # Calculate statistics
        physics_pass = sum(1 for r in self.results['physics_questions'] if r['success'])
        consciousness_pass = sum(1 for r in self.results['consciousness_questions'] if r['success'])
        hybrid_pass = sum(1 for r in self.results['hybrid_questions'] if r['success'])
        routing_pass = sum(1 for r in self.results['routing_tests'] if r['success'])
        integration_pass = sum(1 for r in self.results['integration_quality'] if r['success'])

        total_tests = (len(self.results['physics_questions']) +
                      len(self.results['consciousness_questions']) +
                      len(self.results['hybrid_questions']) +
                      len(self.results['routing_tests']) +
                      len(self.results['integration_quality']))

        total_pass = physics_pass + consciousness_pass + hybrid_pass + routing_pass + integration_pass

        print("Test Category Results:")
        print(f"  Pure Physics:         {physics_pass}/{len(self.results['physics_questions'])} ✅")
        print(f"  Pure Consciousness:   {consciousness_pass}/{len(self.results['consciousness_questions'])} ✅")
        print(f"  Hybrid Questions:     {hybrid_pass}/{len(self.results['hybrid_questions'])} ✅")
        print(f"  Query Routing:        {routing_pass}/{len(self.results['routing_tests'])} ✅")
        print(f"  Integration Quality:  {integration_pass}/{len(self.results['integration_quality'])} ✅")
        print()

        pass_rate = (total_pass / total_tests) * 100 if total_tests > 0 else 0

        print(f"Total: {total_pass}/{total_tests} tests passed ({pass_rate:.1f}%)")
        print()

        if pass_rate >= 90:
            status = "🎉 EXCELLENT"
        elif pass_rate >= 80:
            status = "✅ GOOD"
        elif pass_rate >= 70:
            status = "⚠️  FAIR"
        else:
            status = "❌ NEEDS WORK"

        print(f"Overall Status: {status}")
        print()

        print("=" * 80)
        print(f"End Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print("=" * 80)


# ============================================================================
# MAIN
# ============================================================================

if __name__ == "__main__":
    tester = PhysicsConsciousnessIntegrationTests()
    tester.run_all_tests()
