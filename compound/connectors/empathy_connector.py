#!/usr/bin/env python3
"""
Empathy Connector

Connects empathy module to compound data bus.
Streams emotional state changes and triggers compound interactions.
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


class EmpathyConnector:
    """
    Connect empathy module to compound data flow.
    
    Publishes emotional state changes and compassion events
    to the central data bus.
    """

    def __init__(self, data_bus: Optional[CompoundDataBus] = None,
                 empathy_model: Optional[Any] = None):
        """
        Initialize empathy connector.
        
        Args:
            data_bus: Data bus instance (uses global if None)
            empathy_model: Empathy model instance (optional)
        """
        self.bus = data_bus or get_global_bus()
        self.empathy_model = empathy_model
        
        # Streaming control
        self._streaming = False
        self._stream_thread = None
        
        # Metrics
        self.metrics = {
            'updates_published': 0,
            'compassion_spikes': 0,
            'emotions_processed': 0,
        }

    def stream_emotions(self, interval: float = 1.0,
                       callback: Optional[Callable] = None) -> None:
        """
        Continuously publish emotional state.
        
        Args:
            interval: Seconds between updates
            callback: Optional callback to generate emotion state
        """
        if self._streaming:
            print("⚠️  Empathy connector already streaming")
            return
        
        self._streaming = True
        self._stream_thread = threading.Thread(
            target=self._stream_loop,
            args=(interval, callback),
            daemon=True
        )
        self._stream_thread.start()
        print(f"💝 Empathy connector streaming (interval: {interval}s)")

    def stop_streaming(self) -> None:
        """Stop emotion streaming"""
        self._streaming = False
        if self._stream_thread:
            self._stream_thread.join(timeout=2.0)
        print("🛑 Empathy connector stopped")

    def _stream_loop(self, interval: float, callback: Optional[Callable]) -> None:
        """Main streaming loop"""
        while self._streaming:
            try:
                # Get emotion state
                if callback:
                    event = callback()
                elif self.empathy_model and hasattr(self.empathy_model, 'get_state'):
                    event = self.empathy_model.get_state()
                else:
                    # Simulated state for demo
                    event = self._generate_simulated_event()
                
                # Publish update
                self.publish_emotion(event)
                
                self.metrics['emotions_processed'] += 1
                time.sleep(interval)
                
            except Exception as e:
                print(f"❌ Error in empathy stream: {e}")
                time.sleep(interval)

    def _generate_simulated_event(self) -> Dict[str, Any]:
        """Generate simulated emotion event for testing"""
        import random
        
        compassion = random.uniform(0.5, 0.98)
        
        event = {
            'type': 'compassion_spike' if compassion > 0.9 else 'emotion_update',
            'compassion_score': compassion,
            'emotional_coherence': random.uniform(0.6, 0.95),
            'spin_state': [[random.choice([-1, 1]) for _ in range(10)] for _ in range(10)],
            'empathy_depth': random.uniform(0.5, 0.9),
            'timestamp': time.time(),
        }
        
        return event

    def publish_emotion(self, event: Dict[str, Any]) -> None:
        """
        Publish emotion event to bus.
        
        Args:
            event: Emotion event dictionary
        """
        # Determine priority
        is_spike = event.get('type') == 'compassion_spike'
        priority = EventPriority.HIGH if is_spike else EventPriority.NORMAL
        
        # Publish to bus
        topic = 'empathy.compassion_spike' if is_spike else 'empathy.update'
        self.bus.publish(topic, event, priority=priority, source='empathy_connector')
        
        self.metrics['updates_published'] += 1
        
        # Trigger compound interactions for compassion spikes
        if is_spike:
            self.metrics['compassion_spikes'] += 1
            
            # Adjust physics to model cooperative systems
            self.bus.publish('physics.set_cooperative_mode', {
                'empathy_level': event.get('compassion_score', 0)
            }, priority=EventPriority.NORMAL, source='empathy_connector')
            
            # Trigger empathy-focused benchmarks
            self.bus.publish('benchmark.run_empathy_tests', {
                'empathy_data': event
            }, priority=EventPriority.NORMAL, source='empathy_connector')
            
            # Reward on blockchain
            self.bus.publish('web3.reward_empathy', {
                'compassion_score': event.get('compassion_score', 0)
            }, priority=EventPriority.HIGH, source='empathy_connector')

    def on_boost(self, multiplier: float, reason: str) -> None:
        """Handle empathy boost request"""
        print(f"💝 Empathy: Boosted by {multiplier}x (reason: {reason})")
        # Could adjust empathy model parameters here

    def on_learn_from(self, source: str, data: Dict) -> None:
        """Handle learning transfer from other domains"""
        print(f"💝 Empathy: Learning from {source}")
        # Could update training data or model here

    def get_metrics(self) -> Dict[str, int]:
        """Get connector metrics"""
        return self.metrics.copy()


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("EMPATHY CONNECTOR DEMO")
    print("=" * 80)
    
    # Create connector
    connector = EmpathyConnector()
    
    # Start streaming
    connector.stream_emotions(interval=1.0)
    
    # Let it run for a bit
    print("\n⏳ Streaming empathy updates...")
    time.sleep(5.0)
    
    # Stop streaming
    connector.stop_streaming()
    
    # Show metrics
    print("\n📈 Metrics:")
    metrics = connector.get_metrics()
    for key, value in metrics.items():
        print(f"  {key}: {value}")
    
    print("\n✅ Demo complete")
