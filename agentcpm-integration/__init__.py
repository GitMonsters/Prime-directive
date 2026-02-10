"""
AgentCPM Integration Package
============================

Provides HTTP service wrapper for AgentRL framework integration with RustyWorm.

Phase 2C: Added async MongoDB integration for trajectory persistence.
"""

__version__ = "1.1.0"
__author__ = "Integration Team"

from agentrl_wrapper import (
    AiProfile,
    PersonalityDelta,
    BehaviorObservation,
    EvolutionTrajectory,
    DeltaPredictor,
    SimpleDeltaPredictor,
    MLDeltaPredictor,
    MinIRLTrainer,
    TrajectoryBuffer,
    AgentRL,
)

# Phase 2C: MongoDB integration
from mongodb_client import (
    MongoDBClient,
    TrajectoryRepository,
    MongoDBConnectionError,
    TrajectoryNotFoundError,
    DATABASE_NAME,
    COLLECTION_NAME,
)

__all__ = [
    # AgentRL wrapper classes
    "AiProfile",
    "PersonalityDelta",
    "BehaviorObservation",
    "EvolutionTrajectory",
    "DeltaPredictor",
    "SimpleDeltaPredictor",
    "MLDeltaPredictor",
    "MinIRLTrainer",
    "TrajectoryBuffer",
    "AgentRL",
    # MongoDB client classes
    "MongoDBClient",
    "TrajectoryRepository",
    "MongoDBConnectionError",
    "TrajectoryNotFoundError",
    "DATABASE_NAME",
    "COLLECTION_NAME",
]
