"""
Connectors Package

Domain connectors that bridge existing modules with the compound data bus.
Each connector listens to its domain and publishes events to the central bus.
"""

from .physics_connector import PhysicsConnector
from .empathy_connector import EmpathyConnector
from .benchmark_connector import BenchmarkConnector
from .web3_connector import Web3Connector

__all__ = [
    'PhysicsConnector',
    'EmpathyConnector',
    'BenchmarkConnector',
    'Web3Connector',
]
