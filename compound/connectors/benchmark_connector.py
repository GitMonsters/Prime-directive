#!/usr/bin/env python3
"""
Benchmark Connector

Connects benchmarks to compound data bus.
Streams test results and triggers compound interactions.
"""

import time
import threading
from typing import Dict, Optional, Any, Callable

# Handle imports
try:
    from ..compound_data_bus import CompoundDataBus, EventPriority, get_global_bus
except ImportError:
    import sys
    sys.path.insert(0, '..')
    from compound_data_bus import CompoundDataBus, EventPriority, get_global_bus


class BenchmarkConnector:
    """
    Connect benchmarks to compound data flow.
    
    Publishes test results and triggers learning updates.
    """

    def __init__(self, data_bus: Optional[CompoundDataBus] = None):
        """
        Initialize benchmark connector.
        
        Args:
            data_bus: Data bus instance (uses global if None)
        """
        self.bus = data_bus or get_global_bus()
        
        # Streaming control
        self._streaming = False
        self._stream_thread = None
        
        # Metrics
        self.metrics = {
            'results_published': 0,
            'high_scores': 0,
            'tests_run': 0,
        }

    def stream_results(self, interval: float = 2.0,
                      callback: Optional[Callable] = None) -> None:
        """
        Continuously publish benchmark results.
        
        Args:
            interval: Seconds between test runs
            callback: Optional callback to generate results
        """
        if self._streaming:
            print("⚠️  Benchmark connector already streaming")
            return
        
        self._streaming = True
        self._stream_thread = threading.Thread(
            target=self._stream_loop,
            args=(interval, callback),
            daemon=True
        )
        self._stream_thread.start()
        print(f"🎯 Benchmark connector streaming (interval: {interval}s)")

    def stop_streaming(self) -> None:
        """Stop benchmark streaming"""
        self._streaming = False
        if self._stream_thread:
            self._stream_thread.join(timeout=2.0)
        print("🛑 Benchmark connector stopped")

    def _stream_loop(self, interval: float, callback: Optional[Callable]) -> None:
        """Main streaming loop"""
        while self._streaming:
            try:
                # Get test result
                if callback:
                    result = callback()
                else:
                    # Simulated result for demo
                    result = self._generate_simulated_result()
                
                # Publish result
                self.publish_result(result)
                
                self.metrics['tests_run'] += 1
                time.sleep(interval)
                
            except Exception as e:
                print(f"❌ Error in benchmark stream: {e}")
                time.sleep(interval)

    def _generate_simulated_result(self) -> Dict[str, Any]:
        """Generate simulated benchmark result for testing"""
        import random
        
        score = random.uniform(0.5, 0.98)
        
        result = {
            'test_name': random.choice(['gaia_level_1', 'gaia_level_2', 'arc_task_1', 'arc_task_2']),
            'score': score,
            'success': score > 0.7,
            'reasoning_trace': ['step1', 'step2', 'step3'],
            'timestamp': time.time(),
        }
        
        return result

    def publish_result(self, result: Dict[str, Any]) -> None:
        """
        Publish benchmark result to bus.
        
        Args:
            result: Benchmark result dictionary
        """
        # Determine priority
        score = result.get('score', 0)
        priority = EventPriority.HIGH if score > 0.9 else EventPriority.NORMAL
        
        # Publish to bus
        self.bus.publish('benchmark.result', result, 
                        priority=priority, source='benchmark_connector')
        
        self.metrics['results_published'] += 1
        
        # Trigger compound effects for high scores
        if score > 0.9:
            self.metrics['high_scores'] += 1
            
            # Update empathy training with successful patterns
            self.bus.publish('empathy.learn_from', {
                'source': 'successful_benchmark',
                'result': result
            }, priority=EventPriority.NORMAL, source='benchmark_connector')
            
            # Adjust physics based on reasoning strategies
            self.bus.publish('physics.adopt_strategy', {
                'strategy': result.get('reasoning_trace', []),
                'score': score
            }, priority=EventPriority.NORMAL, source='benchmark_connector')
            
            # Mint achievement NFT
            self.bus.publish('web3.mint_achievement', {
                'benchmark_result': result
            }, priority=EventPriority.HIGH, source='benchmark_connector')

    def on_run_empathy_tests(self, empathy_data: Dict) -> None:
        """Handle request to run empathy-focused tests"""
        print(f"🎯 Benchmark: Running empathy tests (compassion: {empathy_data.get('compassion_score', 0):.2f})")
        # Could trigger specific empathy benchmarks here

    def on_validate(self, physics_result: Dict) -> None:
        """Handle validation request from physics"""
        print(f"🎯 Benchmark: Validating physics result")
        # Could run validation benchmarks here

    def get_metrics(self) -> Dict[str, int]:
        """Get connector metrics"""
        return self.metrics.copy()


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("BENCHMARK CONNECTOR DEMO")
    print("=" * 80)
    
    # Create connector
    connector = BenchmarkConnector()
    
    # Start streaming
    connector.stream_results(interval=1.5)
    
    # Let it run for a bit
    print("\n⏳ Streaming benchmark results...")
    time.sleep(6.0)
    
    # Stop streaming
    connector.stop_streaming()
    
    # Show metrics
    print("\n📈 Metrics:")
    metrics = connector.get_metrics()
    for key, value in metrics.items():
        print(f"  {key}: {value}")
    
    print("\n✅ Demo complete")
