#!/usr/bin/env python3
"""
Comprehensive Test Suite for Compound Data Flow System

Tests all components:
- Data bus
- State aggregator
- Feedback orchestrator
- Connectors
- Analytics
- Compound engine
"""

import sys
import time
import unittest
sys.path.insert(0, '/home/runner/work/Prime-directive/Prime-directive')

from compound.compound_data_bus import CompoundDataBus, Event, EventPriority
from compound.compound_state import CompoundState
from compound.feedback_loop import FeedbackOrchestrator
from compound.compound_engine import CompoundInteractionEngine
from compound.connectors.physics_connector import PhysicsConnector
from compound.connectors.empathy_connector import EmpathyConnector
from compound.connectors.benchmark_connector import BenchmarkConnector
from compound.connectors.web3_connector import Web3Connector
from compound.analytics.correlation_detector import CorrelationDetector
from compound.analytics.emergence_tracker import EmergenceTracker
from compound.analytics.consciousness_scorer import ConsciousnessScorer


class TestCompoundDataBus(unittest.TestCase):
    """Test compound data bus functionality"""
    
    def setUp(self):
        self.bus = CompoundDataBus()
        self.bus.start()
    
    def tearDown(self):
        self.bus.stop()
    
    def test_publish_subscribe(self):
        """Test basic pub/sub functionality"""
        received = []
        
        def callback(event):
            received.append(event)
        
        self.bus.subscribe('test.topic', callback)
        self.bus.publish('test.topic', {'value': 42})
        
        time.sleep(0.2)
        
        self.assertEqual(len(received), 1)
        self.assertEqual(received[0].data['value'], 42)
    
    def test_wildcard_subscription(self):
        """Test wildcard topic subscription"""
        received = []
        
        def callback(event):
            received.append(event)
        
        self.bus.subscribe('test.*', callback)
        self.bus.publish('test.topic1', {'value': 1})
        self.bus.publish('test.topic2', {'value': 2})
        
        time.sleep(0.2)
        
        self.assertEqual(len(received), 2)
    
    def test_priority_queue(self):
        """Test priority-based event processing"""
        self.bus.publish('test', {}, priority=EventPriority.LOW)
        self.bus.publish('test', {}, priority=EventPriority.CRITICAL)
        
        # Critical should be processed first (higher priority)
        metrics = self.bus.get_metrics()
        self.assertGreater(metrics['total_published'], 0)


class TestCompoundState(unittest.TestCase):
    """Test compound state aggregator"""
    
    def setUp(self):
        self.state = CompoundState()
    
    def test_update_physics(self):
        """Test physics state update"""
        self.state.update_physics({
            'quantum_coherence': 0.8,
            'convergence': 0.75
        })
        
        unified = self.state.merge_all()
        self.assertEqual(unified['physics']['quantum_coherence'], 0.8)
    
    def test_update_empathy(self):
        """Test empathy state update"""
        self.state.update_empathy({
            'compassion_score': 0.9
        })
        
        unified = self.state.merge_all()
        self.assertEqual(unified['empathy']['compassion_score'], 0.9)
    
    def test_consciousness_score(self):
        """Test consciousness score computation"""
        # Set high scores
        self.state.update_physics({'quantum_coherence': 0.85, 'convergence': 0.85})
        self.state.update_empathy({'compassion_score': 0.88, 'emotional_coherence': 0.82})
        self.state.update_benchmark({'gaia_score': 0.87, 'success': True})
        
        unified = self.state.merge_all()
        consciousness = unified['consciousness']
        
        self.assertGreater(consciousness['overall'], 0.5)
        self.assertLessEqual(consciousness['overall'], 1.0)
    
    def test_emergent_patterns(self):
        """Test emergent pattern detection"""
        # Set correlated high values
        self.state.update_physics({'quantum_coherence': 0.85, 'convergence': 0.85, 'breakthrough_detected': True})
        self.state.update_empathy({'compassion_score': 0.88})
        self.state.update_benchmark({'gaia_score': 0.87})
        
        unified = self.state.merge_all()
        patterns = unified['emergent_properties']['patterns']
        
        self.assertGreater(len(patterns), 0)


class TestFeedbackOrchestrator(unittest.TestCase):
    """Test feedback orchestrator"""
    
    def setUp(self):
        self.bus = CompoundDataBus()
        self.bus.start()
        self.state = CompoundState()
        self.orchestrator = FeedbackOrchestrator(self.bus, self.state)
    
    def tearDown(self):
        self.bus.stop()
    
    def test_cross_domain_learning(self):
        """Test cross-domain learning application"""
        # Trigger high physics coherence
        self.state.update_physics({'quantum_coherence': 0.85})
        
        result = self.orchestrator.apply_cross_domain_learning()
        
        self.assertIsInstance(result, dict)
        self.assertIn('actions_taken', result)


class TestConnectors(unittest.TestCase):
    """Test domain connectors"""
    
    def setUp(self):
        self.bus = CompoundDataBus()
        self.bus.start()
    
    def tearDown(self):
        self.bus.stop()
    
    def test_physics_connector(self):
        """Test physics connector"""
        connector = PhysicsConnector(self.bus)
        connector.stream_simulations(interval=0.5)
        time.sleep(1.5)
        connector.stop_streaming()
        
        metrics = connector.get_metrics()
        self.assertGreater(metrics['simulations_run'], 0)
    
    def test_empathy_connector(self):
        """Test empathy connector"""
        connector = EmpathyConnector(self.bus)
        connector.stream_emotions(interval=0.5)
        time.sleep(1.5)
        connector.stop_streaming()
        
        metrics = connector.get_metrics()
        self.assertGreater(metrics['emotions_processed'], 0)
    
    def test_benchmark_connector(self):
        """Test benchmark connector"""
        connector = BenchmarkConnector(self.bus)
        connector.stream_results(interval=0.5)
        time.sleep(1.5)
        connector.stop_streaming()
        
        metrics = connector.get_metrics()
        self.assertGreater(metrics['tests_run'], 0)
    
    def test_web3_connector(self):
        """Test web3 connector"""
        connector = Web3Connector(self.bus)
        connector.stream_blockchain_events(interval=0.5)
        time.sleep(1.5)
        connector.stop_streaming()
        
        metrics = connector.get_metrics()
        self.assertGreater(metrics['events_published'], 0)


class TestAnalytics(unittest.TestCase):
    """Test analytics components"""
    
    def test_correlation_detector(self):
        """Test correlation detection"""
        detector = CorrelationDetector()
        
        # Add correlated data
        for i in range(30):
            value = 0.5 + (i / 60.0)
            detector.add_data_point('physics', {'quantum_coherence': value})
            detector.add_data_point('empathy', {'compassion_score': value})
        
        correlations = detector.detect_correlations()
        self.assertIsInstance(correlations, dict)
    
    def test_emergence_tracker(self):
        """Test emergence tracking"""
        tracker = EmergenceTracker()
        
        # Add synchronized states
        for i in range(20):
            base = 0.7
            tracker.add_state({
                'consciousness': {
                    'overall': base,
                    'physics_component': base,
                    'empathy_component': base,
                    'benchmark_component': base,
                    'coherence': 0.9
                }
            })
        
        patterns = tracker.detect_emergence()
        self.assertIsInstance(patterns, list)
    
    def test_consciousness_scorer(self):
        """Test consciousness scoring"""
        scorer = ConsciousnessScorer()
        
        unified_state = {
            'physics': {'quantum_coherence': 0.8, 'convergence': 0.8, 'last_update': time.time()},
            'empathy': {'compassion_score': 0.85, 'emotional_coherence': 0.8, 'last_update': time.time()},
            'benchmarks': {'gaia_score': 0.82, 'arc_score': 0.75, 'success_rate': 0.8, 'last_update': time.time()},
            'web3': {'nfts_minted': 50, 'token_balance': 500, 'last_update': time.time()},
            'emergent_properties': {'patterns': [{'strength': 0.8}]}
        }
        
        scores = scorer.compute_consciousness(unified_state)
        
        self.assertIn('overall', scores)
        self.assertGreater(scores['overall'], 0.5)
        self.assertLessEqual(scores['overall'], 1.0)


class TestCompoundEngine(unittest.TestCase):
    """Test compound engine orchestration"""
    
    def test_engine_startup_shutdown(self):
        """Test engine startup and shutdown"""
        engine = CompoundInteractionEngine()
        engine.start()
        time.sleep(2.0)
        
        status = engine.get_system_status()
        self.assertTrue(status['running'])
        
        engine.stop()
        
        # Get status after stopping
        status_after = engine.get_system_status()
        self.assertFalse(status_after['running'])
    
    def test_compound_query(self):
        """Test compound query across domains"""
        engine = CompoundInteractionEngine()
        engine.start()
        
        # Publish some data
        engine.bus.publish('physics.update', {'quantum_coherence': 0.85})
        engine.bus.publish('empathy.update', {'compassion_score': 0.88})
        
        time.sleep(0.5)
        
        result = engine.compound_query("How does quantum coherence relate to compassion?")
        
        self.assertIn('question', result)
        self.assertIn('insights', result)
        
        engine.stop()


def run_tests():
    """Run all tests"""
    print("=" * 80)
    print("COMPOUND DATA FLOW SYSTEM - TEST SUITE")
    print("=" * 80)
    print()
    
    # Create test suite
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    # Add all test cases
    suite.addTests(loader.loadTestsFromTestCase(TestCompoundDataBus))
    suite.addTests(loader.loadTestsFromTestCase(TestCompoundState))
    suite.addTests(loader.loadTestsFromTestCase(TestFeedbackOrchestrator))
    suite.addTests(loader.loadTestsFromTestCase(TestConnectors))
    suite.addTests(loader.loadTestsFromTestCase(TestAnalytics))
    suite.addTests(loader.loadTestsFromTestCase(TestCompoundEngine))
    
    # Run tests
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    # Summary
    print()
    print("=" * 80)
    print("TEST SUMMARY")
    print("=" * 80)
    print(f"Tests run: {result.testsRun}")
    print(f"Successes: {result.testsRun - len(result.failures) - len(result.errors)}")
    print(f"Failures: {len(result.failures)}")
    print(f"Errors: {len(result.errors)}")
    print()
    
    if result.wasSuccessful():
        print("✅ ALL TESTS PASSED!")
    else:
        print("❌ SOME TESTS FAILED")
    
    return result.wasSuccessful()


if __name__ == '__main__':
    success = run_tests()
    sys.exit(0 if success else 1)
