#!/usr/bin/env python3
"""
Web3 Connector

Connects Web3/blockchain layer to compound data bus.
Streams blockchain events and processes governance decisions.
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


class Web3Connector:
    """
    Connect blockchain/IPFS to compound data flow.
    
    Publishes Web3 events and processes governance decisions.
    """

    def __init__(self, data_bus: Optional[CompoundDataBus] = None):
        """
        Initialize Web3 connector.
        
        Args:
            data_bus: Data bus instance (uses global if None)
        """
        self.bus = data_bus or get_global_bus()
        
        # Streaming control
        self._streaming = False
        self._stream_thread = None
        
        # Metrics
        self.metrics = {
            'events_published': 0,
            'nfts_minted': 0,
            'governance_votes': 0,
        }

    def stream_blockchain_events(self, interval: float = 3.0,
                                 callback: Optional[Callable] = None) -> None:
        """
        Continuously publish Web3 events.
        
        Args:
            interval: Seconds between events
            callback: Optional callback to generate events
        """
        if self._streaming:
            print("⚠️  Web3 connector already streaming")
            return
        
        self._streaming = True
        self._stream_thread = threading.Thread(
            target=self._stream_loop,
            args=(interval, callback),
            daemon=True
        )
        self._stream_thread.start()
        print(f"🌐 Web3 connector streaming (interval: {interval}s)")

    def stop_streaming(self) -> None:
        """Stop blockchain event streaming"""
        self._streaming = False
        if self._stream_thread:
            self._stream_thread.join(timeout=2.0)
        print("🛑 Web3 connector stopped")

    def _stream_loop(self, interval: float, callback: Optional[Callable]) -> None:
        """Main streaming loop"""
        while self._streaming:
            try:
                # Get blockchain event
                if callback:
                    event = callback()
                else:
                    # Simulated event for demo
                    event = self._generate_simulated_event()
                
                # Publish event
                self.publish_event(event)
                
                time.sleep(interval)
                
            except Exception as e:
                print(f"❌ Error in Web3 stream: {e}")
                time.sleep(interval)

    def _generate_simulated_event(self) -> Dict[str, Any]:
        """Generate simulated Web3 event for testing"""
        import random
        
        event_types = ['nft_minted', 'governance_vote', 'token_transfer', 'ipfs_pin']
        event_type = random.choice(event_types)
        
        event = {
            'type': event_type,
            'timestamp': time.time(),
        }
        
        if event_type == 'nft_minted':
            event.update({
                'token_id': random.randint(1000, 9999),
                'owner': f'0x{random.randint(1000, 9999):04x}',
                'metadata': {'type': random.choice(['achievement', 'breakthrough', 'milestone'])}
            })
        elif event_type == 'governance_vote':
            event.update({
                'proposal_id': random.randint(1, 100),
                'decision': random.choice(['quantum_research', 'empathy_focus', 'benchmark_expansion', 'balanced']),
                'votes': random.randint(100, 1000)
            })
        elif event_type == 'ipfs_pin':
            event.update({
                'cid': f'Qm{random.randint(10000, 99999)}',
                'size': random.randint(1000, 100000)
            })
        
        return event

    def publish_event(self, event: Dict[str, Any]) -> None:
        """
        Publish Web3 event to bus.
        
        Args:
            event: Web3 event dictionary
        """
        event_type = event.get('type', 'unknown')
        
        # Determine priority
        priority = EventPriority.HIGH if event_type == 'governance_vote' \
                  else EventPriority.NORMAL
        
        # Publish to bus
        self.bus.publish(f'web3.{event_type}', event, 
                        priority=priority, source='web3_connector')
        
        self.metrics['events_published'] += 1
        
        # Track specific metrics
        if event_type == 'nft_minted':
            self.metrics['nfts_minted'] += 1
        elif event_type == 'governance_vote':
            self.metrics['governance_votes'] += 1
        
        # Trigger compound interactions
        if event_type == 'governance_vote':
            # Redirect research based on DAO decision
            priority = event.get('decision', 'balanced')
            
            for domain in ['physics', 'empathy', 'benchmark']:
                self.bus.publish(f'{domain}.set_priority', {
                    'priority': priority,
                    'dao_priority': priority
                }, priority=EventPriority.HIGH, source='web3_connector')
        
        elif event_type == 'nft_minted':
            # Celebrate across all systems
            self.bus.publish('empathy.positive_reinforcement', {
                'reason': 'nft_minted',
                'token_id': event.get('token_id')
            }, priority=EventPriority.NORMAL, source='web3_connector')
            
            self.bus.publish('physics.log_milestone', {
                'event': event
            }, priority=EventPriority.LOW, source='web3_connector')

    def mint_nft(self, nft_type: str, data: Dict) -> Dict:
        """
        Mint NFT for achievement/milestone.
        
        Args:
            nft_type: Type of NFT (achievement, breakthrough, etc.)
            data: NFT data
            
        Returns:
            Minting result
        """
        import random
        
        token_id = random.randint(10000, 99999)
        
        result = {
            'success': True,
            'token_id': token_id,
            'type': nft_type,
            'data': data,
            'timestamp': time.time()
        }
        
        # Publish minting event
        self.bus.publish('web3.nft_minted', result, 
                        priority=EventPriority.HIGH, source='web3_connector')
        
        print(f"🌐 Web3: NFT minted (type: {nft_type}, token_id: {token_id})")
        
        return result

    def get_metrics(self) -> Dict[str, int]:
        """Get connector metrics"""
        return self.metrics.copy()


if __name__ == '__main__':
    # Demo
    print("=" * 80)
    print("WEB3 CONNECTOR DEMO")
    print("=" * 80)
    
    # Create connector
    connector = Web3Connector()
    
    # Start streaming
    connector.stream_blockchain_events(interval=2.0)
    
    # Let it run for a bit
    print("\n⏳ Streaming Web3 events...")
    time.sleep(7.0)
    
    # Stop streaming
    connector.stop_streaming()
    
    # Show metrics
    print("\n📈 Metrics:")
    metrics = connector.get_metrics()
    for key, value in metrics.items():
        print(f"  {key}: {value}")
    
    # Test NFT minting
    print("\n🌐 Minting achievement NFT...")
    result = connector.mint_nft('achievement', {'score': 0.95, 'test': 'GAIA'})
    print(f"  Token ID: {result['token_id']}")
    
    print("\n✅ Demo complete")
