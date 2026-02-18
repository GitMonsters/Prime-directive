#!/usr/bin/env python3
"""
Physics Connector

Connects physics engine to compound data bus.
Streams physics simulation state and triggers downstream effects.
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


class PhysicsConnector:
    """
    Connect physics engine to compound data flow.
    
    Publishes physics state updates and breakthrough events
    to the central data bus.
    """

    def __init__(self, data_bus: Optional[CompoundDataBus] = None,
                 physics_model: Optional[Any] = None):
        """
        Initialize physics connector.
        
        Args:
            data_bus: Data bus instance (uses global if None)
            physics_model: Physics model instance (optional)
        """
        self.bus = data_bus or get_global_bus()
        self.physics_model = physics_model
        
        # Streaming control
        self._streaming = False
        self._stream_thread = None
        
        # Metrics
        self.metrics = {
            'updates_published': 0,
            'breakthroughs_detected': 0,
            'simulations_run': 0,
        }

    def stream_simulations(self, interval: float = 1.0,
                          callback: Optional[Callable] = None) -> None:
        """
        Continuously publish physics state.
        
        Args:
            interval: Seconds between updates
            callback: Optional callback to generate simulation state
        """
        if self._streaming:
            print("⚠️  Physics connector already streaming")
            return
        
        self._streaming = True
        self._stream_thread = threading.Thread(
            target=self._stream_loop,
            args=(interval, callback),
            daemon=True
        )
        self._stream_thread.start()
        print(f"📊 Physics connector streaming (interval: {interval}s)")

    def stop_streaming(self) -> None:
        """Stop simulation streaming"""
        self._streaming = False
        if self._stream_thread:
            self._stream_thread.join(timeout=2.0)
        print("🛑 Physics connector stopped")

    def _stream_loop(self, interval: float, callback: Optional[Callable]) -> None:
        """Main streaming loop"""
        while self._streaming:
            try:
                # Get simulation state
                if callback:
                    state = callback()
                elif self.physics_model and hasattr(self.physics_model, 'get_state'):
                    state = self.physics_model.get_state()
                else:
                    # Simulated state for demo
                    state = self._generate_simulated_state()
                
                # Publish update
                self.publish_state(state)
                
                self.metrics['simulations_run'] += 1
                time.sleep(interval)
                
            except Exception as e:
                print(f"❌ Error in physics stream: {e}")
                time.sleep(interval)

    def _generate_simulated_state(self) -> Dict[str, Any]:
        """Generate simulated physics state for testing"""
        import random
        
        state = {
            'quantum_coherence': random.uniform(0.6, 0.95),
            'energy_levels': [random.uniform(0, 10) for _ in range(5)],
            'convergence': random.uniform(0.5, 0.9),
            'simulation_time': time.time(),
            'breakthrough_detected': random.random() > 0.95,  # 5% chance
        }
        
        return state

    def publish_state(self, state: Dict[str, Any]) -> None:
        """
        Publish physics state to bus.
        
        Args:
            state: Physics state dictionary
        """
        # Determine priority
        priority = EventPriority.CRITICAL if state.get('breakthrough_detected') \
                  else EventPriority.HIGH if state.get('quantum_coherence', 0) > 0.9 \
                  else EventPriority.NORMAL
        
        # Publish to bus
        self.bus.publish('physics.state_update', state, 
                        priority=priority, source='physics_connector')
        
        self.metrics['updates_published'] += 1
        
        # Trigger downstream effects if breakthrough
        if state.get('breakthrough_detected'):
            self.metrics['breakthroughs_detected'] += 1
            
            # Celebrate with empathy
            self.bus.publish('empathy.celebrate', {
                'reason': 'physics_breakthrough',
                'physics_data': state
            }, priority=EventPriority.HIGH, source='physics_connector')
            
            # Mint NFT
            self.bus.publish('web3.mint_nft', {
                'type': 'physics_breakthrough',
                'data': state
            }, priority=EventPriority.HIGH, source='physics_connector')
            
            # Validate with benchmarks
            self.bus.publish('benchmark.validate', {
                'physics_result': state
            }, priority=EventPriority.NORMAL, source='physics_connector')

    def on_priority_change(self, priority: str) -> None:
        """Handle priority change from governance"""
        print(f"📊 Physics: Priority changed to {priority}")
        # Could adjust simulation parameters here

    def on_cooperative_mode(self, empathy_level: float) -> None:
        """Handle cooperative mode request from empathy"""
        print(f"📊 Physics: Switching to cooperative mode (empathy: {empathy_level:.2f})")
        # Could adjust physics to model cooperative systems

    def get_metrics(self) -> Dict[str, int]:
        """Get connector metrics"""
        return self.metrics.copy()


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("PHYSICS CONNECTOR DEMO")
    print("=" * 80)
    
    # Create connector
    connector = PhysicsConnector()
    
    # Start streaming
    connector.stream_simulations(interval=1.0)
    
    # Let it run for a bit
    print("\n⏳ Streaming physics simulations...")
    time.sleep(5.0)
    
    # Stop streaming
    connector.stop_streaming()
    
    # Show metrics
    print("\n📈 Metrics:")
    metrics = connector.get_metrics()
    for key, value in metrics.items():
        print(f"  {key}: {value}")
    
    print("\n✅ Demo complete")
