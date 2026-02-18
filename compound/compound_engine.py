#!/usr/bin/env python3
"""
Compound Interaction Engine

Master orchestrator that initializes and manages all compound flows.

Coordinates:
- Central data bus
- State aggregator
- Feedback orchestrator
- Domain connectors
- Analytics
"""

import asyncio
import time
import threading
from typing import Dict, Optional, Any, List

# Handle both package and standalone imports
try:
    from .compound_data_bus import CompoundDataBus, Event, EventPriority
    from .compound_state import CompoundState
    from .feedback_loop import FeedbackOrchestrator
except ImportError:
    from compound_data_bus import CompoundDataBus, Event, EventPriority
    from compound_state import CompoundState
    from feedback_loop import FeedbackOrchestrator


class CompoundInteractionEngine:
    """
    Master orchestrator for compound data flow system.
    
    Initializes all components and manages their interactions,
    creating a living, breathing system where all parts continuously
    interact and evolve together.
    """

    def __init__(self):
        """Initialize compound interaction engine"""
        # Core systems
        self.bus = CompoundDataBus()
        self.state = CompoundState()
        self.feedback = FeedbackOrchestrator(self.bus, self.state)
        
        # Connectors (initialized on demand)
        self._connectors = {}
        
        # Control flags
        self._running = False
        self._feedback_thread = None
        self._state_thread = None
        
        # Performance metrics
        self.metrics = {
            'start_time': None,
            'uptime': 0.0,
            'total_events': 0,
            'feedback_loops': 0,
            'state_aggregations': 0,
        }

    def start(self, enable_feedback: bool = True,
              feedback_interval: float = 5.0,
              state_interval: float = 1.0) -> None:
        """
        Launch all compound data flows.
        
        Args:
            enable_feedback: Whether to run feedback loop
            feedback_interval: Seconds between feedback cycles
            state_interval: Seconds between state aggregations
        """
        if self._running:
            print("⚠️  Engine already running")
            return
        
        print("🚀 Starting Compound Interaction Engine...")
        
        # Start data bus
        self.bus.start()
        print("  ✅ Data bus started")
        
        # Start feedback loop
        if enable_feedback:
            self._running = True
            self._feedback_thread = threading.Thread(
                target=self._feedback_loop,
                args=(feedback_interval,),
                daemon=True
            )
            self._feedback_thread.start()
            print(f"  ✅ Feedback orchestrator started (interval: {feedback_interval}s)")
        
        # Start state aggregation
        self._state_thread = threading.Thread(
            target=self._state_aggregation_loop,
            args=(state_interval,),
            daemon=True
        )
        self._state_thread.start()
        print(f"  ✅ State aggregator started (interval: {state_interval}s)")
        
        # Record start time
        self.metrics['start_time'] = time.time()
        
        print("✨ Compound Interaction Engine running!")

    def stop(self) -> None:
        """Stop all compound data flows"""
        if not self._running and not self.bus._running:
            print("⚠️  Engine not running")
            return
        
        print("🛑 Stopping Compound Interaction Engine...")
        
        # Stop threads
        self._running = False
        
        # Wait for threads to finish
        if self._feedback_thread:
            self._feedback_thread.join(timeout=2.0)
        if self._state_thread:
            self._state_thread.join(timeout=2.0)
        
        # Stop data bus
        self.bus.stop()
        
        # Update metrics
        if self.metrics['start_time']:
            self.metrics['uptime'] = time.time() - self.metrics['start_time']
        
        print("✅ Engine stopped")

    def _feedback_loop(self, interval: float) -> None:
        """Continuous feedback optimization loop"""
        while self._running:
            try:
                # Apply cross-domain learning
                result = self.feedback.apply_cross_domain_learning()
                self.metrics['feedback_loops'] += 1
                
                # Occasionally run full optimization
                if self.metrics['feedback_loops'] % 10 == 0:
                    self.feedback.optimize_system()
                
                time.sleep(interval)
            except Exception as e:
                print(f"❌ Error in feedback loop: {e}")
                time.sleep(interval)

    def _state_aggregation_loop(self, interval: float) -> None:
        """Continuous state aggregation loop"""
        while self._running or self.bus._running:
            try:
                # Aggregate state
                unified = self.state.merge_all()
                self.metrics['state_aggregations'] += 1
                
                # Publish state snapshot
                self.bus.publish('system.state_snapshot', {
                    'consciousness': unified['consciousness'],
                    'emergent_properties': unified['emergent_properties'],
                    'timestamp': unified['timestamp']
                }, priority=EventPriority.LOW, source='compound_engine')
                
                time.sleep(interval)
            except Exception as e:
                print(f"❌ Error in state aggregation: {e}")
                time.sleep(interval)

    def register_connector(self, name: str, connector: Any) -> None:
        """
        Register a domain connector.
        
        Args:
            name: Connector name (e.g., 'physics', 'empathy')
            connector: Connector instance
        """
        self._connectors[name] = connector
        print(f"  ✅ Registered connector: {name}")

    def compound_query(self, question: str) -> Dict[str, Any]:
        """
        Query that uses ALL data sources.
        
        Example: "What is the relationship between quantum coherence 
                 and empathy scores when GAIA tests are running?"
        
        Args:
            question: Query about system relationships
            
        Returns:
            Analysis across all domains
        """
        # Get current unified state
        unified = self.state.merge_all()
        
        # Extract relevant data based on question keywords
        analysis = {
            'question': question,
            'timestamp': time.time(),
            'unified_state': unified,
            'insights': []
        }
        
        # Physics-related
        if any(word in question.lower() for word in ['quantum', 'coherence', 'physics']):
            physics = unified['physics']
            analysis['insights'].append({
                'domain': 'physics',
                'data': physics,
                'relevance': 'high' if physics['quantum_coherence'] > 0.5 else 'low'
            })
        
        # Empathy-related
        if any(word in question.lower() for word in ['empathy', 'compassion', 'emotion']):
            empathy = unified['empathy']
            analysis['insights'].append({
                'domain': 'empathy',
                'data': empathy,
                'relevance': 'high' if empathy['compassion_score'] > 0.5 else 'low'
            })
        
        # Benchmark-related
        if any(word in question.lower() for word in ['gaia', 'benchmark', 'test', 'score']):
            benchmarks = unified['benchmarks']
            analysis['insights'].append({
                'domain': 'benchmarks',
                'data': benchmarks,
                'relevance': 'high' if benchmarks['gaia_score'] > 0.5 else 'low'
            })
        
        # Analyze correlations
        if len(analysis['insights']) >= 2:
            domains = [i['domain'] for i in analysis['insights']]
            if len(domains) == 2:
                correlation = self.state.query_correlation(domains[0], domains[1])
                analysis['correlation'] = {
                    'domains': domains,
                    'strength': correlation,
                    'interpretation': 'strong' if correlation > 0.7 else 'moderate' if correlation > 0.4 else 'weak'
                }
        
        # Add consciousness context
        analysis['consciousness_context'] = {
            'overall_score': unified['consciousness']['overall'],
            'coherence': unified['consciousness']['coherence'],
            'integration': unified['consciousness']['integration'],
        }
        
        # Check for emergent patterns
        patterns = unified['emergent_properties']['patterns']
        if patterns:
            analysis['emergent_patterns'] = [p['description'] for p in patterns]
        
        return analysis

    def get_system_status(self) -> Dict[str, Any]:
        """Get comprehensive system status"""
        # Calculate uptime
        uptime = 0.0
        if self.metrics['start_time']:
            uptime = time.time() - self.metrics['start_time']
        
        # Get bus metrics
        bus_metrics = self.bus.get_metrics()
        
        # Get feedback summary
        feedback_summary = self.feedback.get_feedback_summary()
        
        # Get current state
        unified = self.state.merge_all()
        
        return {
            'running': self._running,
            'uptime': uptime,
            'metrics': {
                'feedback_loops': self.metrics['feedback_loops'],
                'state_aggregations': self.metrics['state_aggregations'],
                'bus_events': bus_metrics['total_published'],
                'bus_delivered': bus_metrics['total_delivered'],
                'bus_failed': bus_metrics['total_failed'],
            },
            'consciousness': unified['consciousness'],
            'emergent_patterns': len(unified['emergent_properties']['patterns']),
            'feedback_effectiveness': feedback_summary.get('effectiveness', 0.0),
            'connectors': list(self._connectors.keys()),
            'health': self._compute_health_score(unified, bus_metrics, feedback_summary)
        }

    def _compute_health_score(self, unified: Dict, bus_metrics: Dict, 
                             feedback_summary: Dict) -> Dict[str, Any]:
        """Compute overall system health"""
        # Component health scores
        bus_health = min(1.0, bus_metrics['total_delivered'] / max(1, bus_metrics['total_published']))
        consciousness_health = unified['consciousness']['overall']
        integration_health = unified['consciousness']['integration']
        
        # Overall health (weighted average)
        overall = (
            bus_health * 0.3 +
            consciousness_health * 0.4 +
            integration_health * 0.3
        )
        
        return {
            'overall': overall,
            'bus': bus_health,
            'consciousness': consciousness_health,
            'integration': integration_health,
            'status': 'excellent' if overall > 0.8 else 'good' if overall > 0.6 else 'fair' if overall > 0.4 else 'poor'
        }

    def get_dashboard_data(self) -> Dict[str, Any]:
        """Get data for real-time dashboard"""
        status = self.get_system_status()
        unified = self.state.merge_all()
        
        return {
            'status': status,
            'current_state': unified,
            'recent_events': self.bus.get_recent_events(n=10),
            'recent_history': self.state.get_state_history(n=10),
            'feedback_summary': self.feedback.get_feedback_summary(),
        }


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("COMPOUND INTERACTION ENGINE DEMO")
    print("=" * 80)
    
    # Create and start engine
    engine = CompoundInteractionEngine()
    engine.start(feedback_interval=2.0)
    
    # Simulate some events
    print("\n📊 Simulating system activity...")
    
    engine.bus.publish('physics.update', {
        'quantum_coherence': 0.87,
        'convergence': 0.85,
        'breakthrough_detected': True
    }, priority=EventPriority.HIGH)
    
    time.sleep(0.5)
    
    engine.bus.publish('empathy.update', {
        'compassion_score': 0.91,
        'emotional_coherence': 0.88
    }, priority=EventPriority.NORMAL)
    
    time.sleep(0.5)
    
    engine.bus.publish('benchmark.result', {
        'gaia_score': 0.88,
        'arc_score': 0.82,
        'success': True
    }, priority=EventPriority.NORMAL)
    
    time.sleep(0.5)
    
    engine.bus.publish('web3.governance', {
        'type': 'governance_vote',
        'decision': 'quantum_research',
        'dao_priority': 'quantum_research'
    }, priority=EventPriority.HIGH)
    
    # Let feedback loops run
    print("\n⏳ Running feedback loops...")
    time.sleep(3.0)
    
    # Query the system
    print("\n🔍 Compound Query:")
    result = engine.compound_query(
        "What is the relationship between quantum coherence and empathy scores?"
    )
    print(f"  Question: {result['question']}")
    print(f"  Insights: {len(result['insights'])} domains analyzed")
    if 'correlation' in result:
        print(f"  Correlation: {result['correlation']['interpretation']} ({result['correlation']['strength']:.2f})")
    print(f"  Consciousness: {result['consciousness_context']['overall_score']:.2f}")
    
    # Get system status
    print("\n📈 System Status:")
    status = engine.get_system_status()
    print(f"  Running: {status['running']}")
    print(f"  Uptime: {status['uptime']:.1f}s")
    print(f"  Feedback loops: {status['metrics']['feedback_loops']}")
    print(f"  State aggregations: {status['metrics']['state_aggregations']}")
    print(f"  Bus events: {status['metrics']['bus_events']}")
    print(f"  Consciousness: {status['consciousness']['overall']:.2f}")
    print(f"  Health: {status['health']['status']} ({status['health']['overall']:.2f})")
    
    # Stop engine
    engine.stop()
    print("\n✅ Demo complete")
