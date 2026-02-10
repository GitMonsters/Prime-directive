"""
AgentCPM Integration Package
============================

Provides HTTP service wrapper for AgentRL framework integration with RustyWorm.
"""

__version__ = "1.0.0"
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

__all__ = [
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
]
