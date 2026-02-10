"""
AgentRL Wrapper Module
======================

Provides a clean interface to the AgentRL framework for integrating with RustyWorm.
Handles trajectory prediction, training, and reward modeling.

This wrapper abstracts away the complexity of AgentRL's internal architecture
and provides simple, asyncio-friendly interfaces for:
- Delta prediction using learned RL policy
- Training on trajectories using MINIRL/GRPO losses
- Trajectory buffering and importance weighting
"""

import asyncio
import logging
from typing import List, Dict, Tuple, Optional, Any
from dataclasses import dataclass, asdict
from datetime import datetime
import json
import numpy as np
from abc import ABC, abstractmethod

logger = logging.getLogger(__name__)


# ===== DATA CLASSES =====

@dataclass
class AiProfile:
    """Represents an AI personality profile (RustyWorm compatible)"""
    id: str
    name: str
    speech_pattern: float = 0.0
    knowledge_style: float = 0.0
    reasoning_style: float = 0.0
    creativity: float = 0.0
    carefulness: float = 0.0
    empathy: float = 0.0
    
    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)
    
    @staticmethod
    def from_dict(data: Dict[str, Any]) -> 'AiProfile':
        return AiProfile(**data)


@dataclass
class PersonalityDelta:
    """Represents adjustments to a personality profile"""
    adjustments: List[Tuple[str, float]]  # [(axis_name, adjustment_amount)]
    confidence: float = 0.5
    source: str = "rl_optimizer"
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "adjustments": self.adjustments,
            "confidence": self.confidence,
            "source": self.source
        }
    
    @staticmethod
    def from_dict(data: Dict[str, Any]) -> 'PersonalityDelta':
        return PersonalityDelta(**data)


@dataclass
class BehaviorObservation:
    """Represents observed behavior from a target model"""
    query: str
    response: str
    patterns: List[str] = None
    similarity_to_target: float = 0.0
    confidence: float = 0.0
    
    def __post_init__(self):
        if self.patterns is None:
            self.patterns = []
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "query": self.query,
            "response": self.response,
            "patterns": self.patterns,
            "similarity_to_target": self.similarity_to_target,
            "confidence": self.confidence
        }
    
    @staticmethod
    def from_dict(data: Dict[str, Any]) -> 'BehaviorObservation':
        return BehaviorObservation(**data)


@dataclass
class EvolutionTrajectory:
    """Represents a single RL trajectory for learning"""
    id: str
    state: AiProfile
    action: PersonalityDelta
    observation: BehaviorObservation
    reward: float
    next_state: AiProfile
    timestamp: str
    used_in_training: bool = False
    importance_weight: float = 1.0
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "state": self.state.to_dict() if isinstance(self.state, AiProfile) else self.state,
            "action": self.action.to_dict() if isinstance(self.action, PersonalityDelta) else self.action,
            "observation": self.observation.to_dict() if isinstance(self.observation, BehaviorObservation) else self.observation,
            "reward": self.reward,
            "next_state": self.next_state.to_dict() if isinstance(self.next_state, AiProfile) else self.next_state,
            "timestamp": self.timestamp,
            "used_in_training": self.used_in_training,
            "importance_weight": self.importance_weight
        }
    
    @staticmethod
    def from_dict(data: Dict[str, Any]) -> 'EvolutionTrajectory':
        return EvolutionTrajectory(
            id=data["id"],
            state=AiProfile.from_dict(data["state"]) if isinstance(data["state"], dict) else data["state"],
            action=PersonalityDelta.from_dict(data["action"]) if isinstance(data["action"], dict) else data["action"],
            observation=BehaviorObservation.from_dict(data["observation"]) if isinstance(data["observation"], dict) else data["observation"],
            reward=data["reward"],
            next_state=AiProfile.from_dict(data["next_state"]) if isinstance(data["next_state"], dict) else data["next_state"],
            timestamp=data["timestamp"],
            used_in_training=data.get("used_in_training", False),
            importance_weight=data.get("importance_weight", 1.0)
        )


# ===== PREDICTOR INTERFACE =====

class DeltaPredictor(ABC):
    """Abstract base for delta prediction using RL policy"""
    
    @abstractmethod
    async def predict(
        self,
        profile: AiProfile,
        observation: BehaviorObservation
    ) -> Tuple[PersonalityDelta, float]:
        """
        Predict optimal PersonalityDelta given profile and observation.
        
        Returns:
            (PersonalityDelta, confidence_score)
        """
        pass


class SimpleDeltaPredictor(DeltaPredictor):
    """Simple rule-based delta predictor for testing"""
    
    async def predict(
        self,
        profile: AiProfile,
        observation: BehaviorObservation
    ) -> Tuple[PersonalityDelta, float]:
        """Predict delta based on observation similarity gap"""
        
        # Simple heuristic: adjust based on how different we are from target
        gap = 1.0 - observation.similarity_to_target
        
        # Create adjustment for similarity
        adjustments = []
        if observation.similarity_to_target < 0.5:
            # Large gap - suggest bigger adjustments
            adjustments.append(("speech_pattern", gap * 0.2))
            adjustments.append(("reasoning_style", gap * 0.15))
        elif observation.similarity_to_target < 0.8:
            # Medium gap - smaller adjustments
            adjustments.append(("knowledge_style", gap * 0.1))
        
        confidence = observation.confidence * observation.similarity_to_target
        
        delta = PersonalityDelta(
            adjustments=adjustments,
            confidence=confidence,
            source="simple_predictor"
        )
        
        return delta, confidence


class MLDeltaPredictor(DeltaPredictor):
    """ML-based delta predictor using learned model"""
    
    def __init__(self, model_path: Optional[str] = None):
        self.model_path = model_path
        self.model = None
        logger.info(f"Initialized MLDeltaPredictor with model_path={model_path}")
    
    async def predict(
        self,
        profile: AiProfile,
        observation: BehaviorObservation
    ) -> Tuple[PersonalityDelta, float]:
        """
        Predict delta using ML model.
        Falls back to simple predictor if model not available.
        """
        
        # TODO: Integrate actual ML model inference here
        # For now, use simple predictor as fallback
        simple = SimpleDeltaPredictor()
        return await simple.predict(profile, observation)


# ===== TRAINER INTERFACE =====

class MinIRLTrainer:
    """Trainer using MINIRL loss for stable learning"""
    
    def __init__(self, learning_rate: float = 1e-4, batch_size: int = 32):
        self.learning_rate = learning_rate
        self.batch_size = batch_size
        self.training_history: List[Dict[str, float]] = []
        logger.info(f"Initialized MinIRLTrainer with lr={learning_rate}, batch_size={batch_size}")
    
    async def train(
        self,
        trajectories: List[EvolutionTrajectory],
        importance_weights: List[float],
        loss_type: str = "MINIRL"
    ) -> Dict[str, Any]:
        """
        Train on trajectories using MINIRL loss.
        
        Args:
            trajectories: List of evolution trajectories
            importance_weights: Weights for each trajectory
            loss_type: "MINIRL" or "GRPO"
        
        Returns:
            Dictionary with training results (loss, time, etc.)
        """
        
        if len(trajectories) == 0:
            raise ValueError("No trajectories to train on")
        
        start_time = datetime.now()
        
        # Calculate loss from trajectories
        loss = self._calculate_loss(trajectories, importance_weights, loss_type)
        
        elapsed_ms = int((datetime.now() - start_time).total_seconds() * 1000)
        
        result = {
            "loss": loss,
            "training_time_ms": elapsed_ms,
            "num_trajectories_used": len(trajectories),
            "loss_type": loss_type,
            "batch_size": self.batch_size,
            "learning_rate": self.learning_rate
        }
        
        self.training_history.append(result)
        
        logger.info(
            f"[MINIRL] Training completed: loss={loss:.4f}, "
            f"time={elapsed_ms}ms, trajectories={len(trajectories)}"
        )
        
        return result
    
    def _calculate_loss(
        self,
        trajectories: List[EvolutionTrajectory],
        weights: List[float],
        loss_type: str
    ) -> float:
        """Calculate loss from trajectories"""
        
        if not trajectories:
            return 0.0
        
        # Normalize weights
        weights_array = np.array(weights if weights else [1.0] * len(trajectories))
        weights_array = weights_array / (weights_array.sum() + 1e-8)
        
        # Calculate weighted loss based on trajectory rewards
        rewards = np.array([t.reward for t in trajectories])
        
        if loss_type == "MINIRL":
            # MINIRL loss: maximize expected reward with stability
            # Loss = -E[reward] + entropy_regularization
            loss = -(weights_array * rewards).sum() + 0.01 * entropy(weights_array)
        elif loss_type == "GRPO":
            # GRPO loss: gradient-based policy optimization
            # Loss = variance_of_weighted_rewards
            loss = np.var(weights_array * rewards)
        else:
            loss = -rewards.mean()
        
        return float(loss)
    
    def get_stats(self) -> Dict[str, Any]:
        """Get training statistics"""
        if not self.training_history:
            return {"training_runs": 0, "average_loss": 0.0}
        
        losses = [h["loss"] for h in self.training_history]
        return {
            "training_runs": len(self.training_history),
            "average_loss": float(np.mean(losses)),
            "min_loss": float(np.min(losses)),
            "max_loss": float(np.max(losses)),
            "last_loss": float(losses[-1])
        }


# ===== TRAJECTORY BUFFER =====

class TrajectoryBuffer:
    """In-memory buffer for trajectories"""
    
    def __init__(self, max_size: int = 1000, importance_threshold: float = 0.3):
        self.max_size = max_size
        self.importance_threshold = importance_threshold
        self.buffer: List[EvolutionTrajectory] = []
        logger.info(f"Initialized TrajectoryBuffer with max_size={max_size}")
    
    def add(self, trajectory: EvolutionTrajectory):
        """Add trajectory to buffer"""
        self.buffer.append(trajectory)
        
        # Prune if over capacity
        if len(self.buffer) > self.max_size:
            self._prune_low_importance()
    
    def _prune_low_importance(self):
        """Remove low importance trajectories"""
        if not self.buffer:
            return
        
        # Sort by importance weight
        self.buffer.sort(key=lambda t: t.importance_weight, reverse=True)
        
        # Keep only top trajectories
        self.buffer = self.buffer[:self.max_size]
    
    def get_batch(self, batch_size: int) -> List[EvolutionTrajectory]:
        """Get a batch of trajectories"""
        return self.buffer[-batch_size:] if len(self.buffer) > 0 else []
    
    def get_all(self) -> List[EvolutionTrajectory]:
        """Get all trajectories"""
        return self.buffer.copy()
    
    def clear(self):
        """Clear buffer"""
        self.buffer.clear()
    
    def size(self) -> int:
        """Get buffer size"""
        return len(self.buffer)


# ===== UTILITY FUNCTIONS =====

def entropy(p: np.ndarray) -> float:
    """Calculate entropy of probability distribution"""
    p = np.clip(p, 1e-10, 1.0)
    return -np.sum(p * np.log(p))


class AgentRL:
    """
    Main AgentRL class that combines all components.
    Simplified interface for RustyWorm integration.
    """
    
    def __init__(
        self,
        predictor: Optional[DeltaPredictor] = None,
        trainer: Optional[MinIRLTrainer] = None,
        buffer_size: int = 1000
    ):
        self.predictor = predictor or SimpleDeltaPredictor()
        self.trainer = trainer or MinIRLTrainer()
        self.buffer = TrajectoryBuffer(max_size=buffer_size)
        logger.info("Initialized AgentRL service")
    
    async def predict_delta(
        self,
        profile: AiProfile,
        observation: BehaviorObservation
    ) -> Tuple[PersonalityDelta, float]:
        """Predict optimal delta"""
        return await self.predictor.predict(profile, observation)
    
    async def train(
        self,
        trajectories: List[EvolutionTrajectory],
        importance_weights: List[float],
        loss_type: str = "MINIRL"
    ) -> Dict[str, Any]:
        """Train on trajectories"""
        return await self.trainer.train(trajectories, importance_weights, loss_type)
    
    def buffer_trajectory(self, trajectory: EvolutionTrajectory):
        """Add trajectory to buffer"""
        self.buffer.add(trajectory)
    
    def get_trainer_stats(self) -> Dict[str, Any]:
        """Get trainer statistics"""
        return self.trainer.get_stats()


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
