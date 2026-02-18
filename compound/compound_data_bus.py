#!/usr/bin/env python3
"""
Central Data Bus - Event-Driven Message Broker

Pub/sub architecture connecting ALL system components:
- Physics engine
- Empathy AI
- Benchmarks
- Web3 layer

Features:
- Topic-based publish/subscribe
- Priority queues for critical events
- Dead letter queue for failed messages
- Event history and metrics tracking
- Async event processing
"""

import time
import threading
import queue
from typing import Dict, List, Callable, Optional, Any
from dataclasses import dataclass, field
from enum import IntEnum
from collections import defaultdict, deque


class EventPriority(IntEnum):
    """Event priority levels (higher = more urgent)"""
    LOW = 1
    NORMAL = 5
    HIGH = 7
    CRITICAL = 10


@dataclass
class Event:
    """Event published to the data bus"""
    topic: str
    data: Dict[str, Any]
    priority: EventPriority = EventPriority.NORMAL
    timestamp: float = field(default_factory=time.time)
    source: Optional[str] = None
    event_id: Optional[str] = None

    def __post_init__(self):
        if self.event_id is None:
            self.event_id = f"{self.topic}-{int(self.timestamp * 1000000)}"

    def __lt__(self, other):
        """For priority queue sorting (higher priority first)"""
        return self.priority > other.priority


class CompoundDataBus:
    """
    Central message broker for compound data flow.
    
    All system components publish events here, creating a unified
    event stream that enables cross-domain interactions.
    """

    def __init__(self, max_history: int = 1000):
        """
        Initialize data bus.
        
        Args:
            max_history: Maximum events to keep in history
        """
        # Topic subscriptions: topic -> list of callbacks
        self.subscribers: Dict[str, List[Callable]] = defaultdict(list)
        
        # Priority queue for event processing
        self.message_queue = queue.PriorityQueue()
        
        # Event history (for analytics and debugging)
        self.event_history: deque = deque(maxlen=max_history)
        
        # Dead letter queue (failed events)
        self.dead_letter_queue: List[Dict] = []
        
        # Metrics
        self.metrics = {
            'total_published': 0,
            'total_delivered': 0,
            'total_failed': 0,
            'events_by_topic': defaultdict(int),
            'events_by_priority': defaultdict(int),
        }
        
        # Processing control
        self._running = False
        self._processor_thread = None
        self._lock = threading.RLock()

    def publish(self, topic: str, data: Dict[str, Any], 
                priority: EventPriority = EventPriority.NORMAL,
                source: Optional[str] = None) -> str:
        """
        Publish event to topic.
        
        Args:
            topic: Event topic (e.g., 'physics.update', 'empathy.change')
            data: Event data dictionary
            priority: Event priority level
            source: Optional source identifier
            
        Returns:
            Event ID
        """
        event = Event(
            topic=topic,
            data=data,
            priority=priority,
            source=source
        )
        
        with self._lock:
            # Add to queue
            self.message_queue.put(event)
            
            # Update metrics
            self.metrics['total_published'] += 1
            self.metrics['events_by_topic'][topic] += 1
            self.metrics['events_by_priority'][priority.name] += 1
            
            # Add to history
            self.event_history.append({
                'event_id': event.event_id,
                'topic': topic,
                'timestamp': event.timestamp,
                'priority': priority.name,
                'source': source
            })
        
        return event.event_id

    def subscribe(self, topic: str, callback: Callable[[Event], None]) -> None:
        """
        Subscribe to topic with callback.
        
        Args:
            topic: Topic to subscribe to (supports wildcards with '*')
            callback: Function to call when event occurs
                     Signature: callback(event: Event) -> None
        """
        with self._lock:
            self.subscribers[topic].append(callback)

    def unsubscribe(self, topic: str, callback: Callable) -> bool:
        """
        Unsubscribe callback from topic.
        
        Args:
            topic: Topic to unsubscribe from
            callback: Callback function to remove
            
        Returns:
            True if callback was found and removed
        """
        with self._lock:
            if topic in self.subscribers and callback in self.subscribers[topic]:
                self.subscribers[topic].remove(callback)
                return True
        return False

    def _match_topic(self, pattern: str, topic: str) -> bool:
        """Check if topic matches subscription pattern (with wildcard support)"""
        if pattern == topic:
            return True
        
        # Support wildcard matching (e.g., 'physics.*' matches 'physics.update')
        if '*' in pattern:
            pattern_parts = pattern.split('.')
            topic_parts = topic.split('.')
            
            if len(pattern_parts) != len(topic_parts):
                return False
            
            for pp, tp in zip(pattern_parts, topic_parts):
                if pp != '*' and pp != tp:
                    return False
            return True
        
        return False

    def _process_event(self, event: Event) -> None:
        """Process single event by calling all matching subscribers"""
        delivered = 0
        failed = 0
        
        with self._lock:
            # Find all matching subscribers
            callbacks = []
            for topic_pattern, topic_callbacks in self.subscribers.items():
                if self._match_topic(topic_pattern, event.topic):
                    callbacks.extend(topic_callbacks)
        
        # Call each callback
        for callback in callbacks:
            try:
                callback(event)
                delivered += 1
            except Exception as e:
                failed += 1
                # Add to dead letter queue
                self.dead_letter_queue.append({
                    'event': event,
                    'callback': callback.__name__ if hasattr(callback, '__name__') else str(callback),
                    'error': str(e),
                    'timestamp': time.time()
                })
        
        # Update metrics
        with self._lock:
            self.metrics['total_delivered'] += delivered
            self.metrics['total_failed'] += failed

    def start(self) -> None:
        """Start event processing thread"""
        if self._running:
            return
        
        self._running = True
        self._processor_thread = threading.Thread(target=self._process_loop, daemon=True)
        self._processor_thread.start()

    def stop(self) -> None:
        """Stop event processing thread"""
        self._running = False
        if self._processor_thread:
            self._processor_thread.join(timeout=1.0)

    def _process_loop(self) -> None:
        """Main event processing loop (runs in separate thread)"""
        while self._running:
            try:
                # Get next event (with timeout to allow checking _running)
                event = self.message_queue.get(timeout=0.1)
                self._process_event(event)
                self.message_queue.task_done()
            except queue.Empty:
                continue
            except Exception as e:
                print(f"Error in event processing loop: {e}")

    def get_metrics(self) -> Dict:
        """Get current metrics"""
        with self._lock:
            return {
                **self.metrics,
                'queue_size': self.message_queue.qsize(),
                'dead_letter_size': len(self.dead_letter_queue),
                'active_topics': len(self.subscribers),
                'total_subscribers': sum(len(cbs) for cbs in self.subscribers.values()),
            }

    def get_recent_events(self, n: int = 10, topic: Optional[str] = None) -> List[Dict]:
        """
        Get recent events from history.
        
        Args:
            n: Number of recent events to return
            topic: Optional topic filter
            
        Returns:
            List of event dictionaries
        """
        with self._lock:
            events = list(self.event_history)
            
            if topic:
                events = [e for e in events if e['topic'] == topic]
            
            return events[-n:] if n else events

    def clear_dead_letter_queue(self) -> List[Dict]:
        """Clear and return dead letter queue"""
        with self._lock:
            dlq = self.dead_letter_queue.copy()
            self.dead_letter_queue.clear()
            return dlq

    def route_compound(self, event: Event) -> None:
        """
        Intelligent routing based on event type and system state.
        
        Creates compound interactions:
        - Physics events trigger empathy recalculation
        - Benchmark results mint NFTs
        - Empathy changes update world model
        """
        topic = event.topic
        data = event.data
        
        # Physics events → trigger empathy & benchmarks
        if topic.startswith('physics.'):
            if data.get('breakthrough_detected'):
                self.publish('empathy.celebrate', {
                    'reason': 'physics_breakthrough',
                    'physics_data': data
                }, priority=EventPriority.HIGH, source='compound_router')
                
                self.publish('benchmark.validate', {
                    'physics_result': data
                }, priority=EventPriority.NORMAL, source='compound_router')
        
        # Empathy events → adjust physics and benchmarks
        elif topic.startswith('empathy.'):
            if data.get('type') == 'compassion_spike':
                self.publish('physics.set_cooperative_mode', {
                    'empathy_level': data.get('compassion_score', 0)
                }, priority=EventPriority.NORMAL, source='compound_router')
                
                self.publish('benchmark.run_empathy_tests', {
                    'empathy_data': data
                }, priority=EventPriority.NORMAL, source='compound_router')
        
        # Benchmark events → update empathy and web3
        elif topic.startswith('benchmark.'):
            if data.get('score', 0) > 0.9:
                self.publish('empathy.learn_from', {
                    'successful_pattern': data
                }, priority=EventPriority.NORMAL, source='compound_router')
                
                self.publish('web3.mint_achievement', {
                    'benchmark_result': data
                }, priority=EventPriority.HIGH, source='compound_router')
        
        # Web3 events → redirect research priorities
        elif topic.startswith('web3.'):
            if data.get('type') == 'governance_vote':
                priority = data.get('decision')
                for domain in ['physics', 'empathy', 'benchmark']:
                    self.publish(f'{domain}.set_priority', {
                        'priority': priority
                    }, priority=EventPriority.HIGH, source='compound_router')


# Global singleton instance
_global_bus: Optional[CompoundDataBus] = None


def get_global_bus() -> CompoundDataBus:
    """Get global singleton data bus instance"""
    global _global_bus
    if _global_bus is None:
        _global_bus = CompoundDataBus()
        _global_bus.start()
    return _global_bus


def reset_global_bus() -> None:
    """Reset global bus (mainly for testing)"""
    global _global_bus
    if _global_bus:
        _global_bus.stop()
    _global_bus = None


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("COMPOUND DATA BUS DEMO")
    print("=" * 80)
    
    bus = CompoundDataBus()
    bus.start()
    
    # Subscribe to events
    def physics_handler(event: Event):
        print(f"📊 Physics: {event.topic} - {event.data}")
    
    def empathy_handler(event: Event):
        print(f"💝 Empathy: {event.topic} - {event.data}")
    
    bus.subscribe('physics.*', physics_handler)
    bus.subscribe('empathy.*', empathy_handler)
    
    # Publish some events
    print("\nPublishing events...")
    bus.publish('physics.update', {'coherence': 0.85}, priority=EventPriority.HIGH)
    bus.publish('empathy.change', {'compassion': 0.92}, priority=EventPriority.NORMAL)
    bus.publish('physics.breakthrough', {'breakthrough_detected': True}, priority=EventPriority.CRITICAL)
    
    # Wait for processing
    time.sleep(0.5)
    
    # Show metrics
    print("\nMetrics:")
    metrics = bus.get_metrics()
    for key, value in metrics.items():
        print(f"  {key}: {value}")
    
    bus.stop()
    print("\n✅ Demo complete")
