"""
Compound Data Flow System

Event-driven orchestration system connecting all Prime-directive components:
- Physics simulations
- Empathy/emotion AI
- Benchmarks (GAIA, ARC)
- Web3 (blockchain, IPFS)

All components interact through a central data bus, creating compound
feedback loops where insights from one domain improve others.
"""

__version__ = "1.0.0"

from .compound_data_bus import CompoundDataBus, Event, EventPriority
from .compound_state import CompoundState
from .feedback_loop import FeedbackOrchestrator
from .compound_engine import CompoundInteractionEngine

__all__ = [
    'CompoundDataBus',
    'Event',
    'EventPriority',
    'CompoundState',
    'FeedbackOrchestrator',
    'CompoundInteractionEngine',
]
