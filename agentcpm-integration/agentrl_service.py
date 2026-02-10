"""
AgentRL HTTP Service
====================

FastAPI service for the AgentRL integration with RustyWorm.
Provides REST API endpoints for:
- Delta prediction
- RL training
- Trajectory storage
- Health checks
- Statistics

Port: 8888
"""

import asyncio
import logging
import os
import json
from typing import List, Dict, Optional, Any
from datetime import datetime
from contextlib import asynccontextmanager

from fastapi import FastAPI, HTTPException, Query
from fastapi.responses import JSONResponse
from pydantic import BaseModel, Field
import uvicorn

# Import AgentRL wrapper
from agentrl_wrapper import (
    AiProfile,
    PersonalityDelta,
    BehaviorObservation,
    EvolutionTrajectory,
    SimpleDeltaPredictor,
    MinIRLTrainer,
    TrajectoryBuffer,
    AgentRL,
)

# Logging setup
logging.basicConfig(
    level=os.getenv("LOG_LEVEL", "INFO"),
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

# ===== PYDANTIC MODELS FOR HTTP =====

class AiProfileRequest(BaseModel):
    """Request/Response model for AI Profile"""
    id: str
    name: str
    speech_pattern: float = 0.0
    knowledge_style: float = 0.0
    reasoning_style: float = 0.0
    creativity: float = 0.0
    carefulness: float = 0.0
    empathy: float = 0.0

    class Config:
        json_schema_extra = {
            "example": {
                "id": "test-ai-001",
                "name": "TestAI",
                "speech_pattern": 0.5,
                "knowledge_style": 0.6,
                "reasoning_style": 0.4,
                "creativity": 0.7,
                "carefulness": 0.5,
                "empathy": 0.6
            }
        }


class PersonalityDeltaRequest(BaseModel):
    """Request/Response model for Personality Delta"""
    adjustments: List[tuple] = Field(default_factory=list)
    confidence: float = 0.5
    source: str = "rl_optimizer"

    class Config:
        json_schema_extra = {
            "example": {
                "adjustments": [("speech_pattern", 0.1), ("reasoning_style", -0.05)],
                "confidence": 0.85,
                "source": "rl_optimizer"
            }
        }


class BehaviorObservationRequest(BaseModel):
    """Request/Response model for Behavior Observation"""
    query: str
    response: str
    patterns: List[str] = Field(default_factory=list)
    similarity_to_target: float = 0.0
    confidence: float = 0.0

    class Config:
        json_schema_extra = {
            "example": {
                "query": "What is 2+2?",
                "response": "The answer is 4.",
                "patterns": ["mathematical_reasoning"],
                "similarity_to_target": 0.92,
                "confidence": 0.95
            }
        }


class EvolutionTrajectoryRequest(BaseModel):
    """Request/Response model for Evolution Trajectory"""
    id: str
    state: AiProfileRequest
    action: PersonalityDeltaRequest
    observation: BehaviorObservationRequest
    reward: float
    next_state: AiProfileRequest
    timestamp: str
    used_in_training: bool = False
    importance_weight: float = 1.0


class PredictDeltaRequest(BaseModel):
    """Request to /predict-delta endpoint"""
    profile: AiProfileRequest
    observation: BehaviorObservationRequest


class PredictDeltaResponse(BaseModel):
    """Response from /predict-delta endpoint"""
    delta: PersonalityDeltaRequest
    confidence: float
    reasoning: str


class TrainingRequest(BaseModel):
    """Request to /train endpoint"""
    trajectories: List[EvolutionTrajectoryRequest]
    importance_weights: Optional[List[float]] = None
    loss_type: str = "MINIRL"  # "MINIRL" or "GRPO"

    class Config:
        json_schema_extra = {
            "example": {
                "trajectories": [],
                "importance_weights": [0.9, 0.8, 0.7],
                "loss_type": "MINIRL"
            }
        }


class TrainingResponse(BaseModel):
    """Response from /train endpoint"""
    loss: float
    training_time_ms: int
    num_trajectories_used: int
    loss_type: str


class HealthCheckResponse(BaseModel):
    """Response from /health endpoint"""
    status: str
    timestamp: str
    service_version: str = "1.0.0"


class StatsResponse(BaseModel):
    """Response from /stats endpoint"""
    training_runs: int
    average_loss: float
    min_loss: float
    max_loss: float
    last_loss: float
    buffer_size: int


class StoreTrajectoryResponse(BaseModel):
    """Response from /trajectory/store endpoint"""
    stored: bool
    id: str
    timestamp: str


# ===== SERVICE STATE =====

class AgentRLService:
    """Service wrapper combining all AgentRL components"""
    
    def __init__(self):
        logger.info("Initializing AgentRL Service...")
        self.predictor = SimpleDeltaPredictor()
        self.trainer = MinIRLTrainer(
            learning_rate=float(os.getenv("LEARNING_RATE", "1e-4")),
            batch_size=int(os.getenv("BATCH_SIZE", "32"))
        )
        self.buffer = TrajectoryBuffer(
            max_size=int(os.getenv("MAX_TRAJECTORY_BUFFER", "1000"))
        )
        self.agentrl = AgentRL(
            predictor=self.predictor,
            trainer=self.trainer,
            buffer_size=int(os.getenv("MAX_TRAJECTORY_BUFFER", "1000"))
        )
        logger.info("AgentRL Service initialized successfully")
    
    async def predict_delta(
        self,
        profile: AiProfile,
        observation: BehaviorObservation
    ) -> tuple[PersonalityDelta, float]:
        """Predict optimal PersonalityDelta"""
        try:
            delta, confidence = await self.agentrl.predict_delta(profile, observation)
            logger.info(f"Delta prediction successful: confidence={confidence:.4f}")
            return delta, confidence
        except Exception as e:
            logger.error(f"Delta prediction failed: {e}", exc_info=True)
            raise
    
    async def train_on_trajectories(
        self,
        trajectories: List[EvolutionTrajectory],
        weights: Optional[List[float]] = None,
        loss_type: str = "MINIRL"
    ) -> Dict[str, Any]:
        """Train on trajectories"""
        try:
            if not trajectories:
                raise ValueError("No trajectories provided for training")
            
            # Use default weights if not provided
            if weights is None:
                weights = [1.0] * len(trajectories)
            
            logger.info(
                f"Starting training: {len(trajectories)} trajectories, "
                f"loss_type={loss_type}"
            )
            
            result = await self.agentrl.train(
                trajectories=trajectories,
                importance_weights=weights,
                loss_type=loss_type
            )
            
            logger.info(f"Training completed: {result}")
            return result
        except Exception as e:
            logger.error(f"Training failed: {e}", exc_info=True)
            raise
    
    def store_trajectory(self, trajectory: EvolutionTrajectory) -> bool:
        """Store trajectory in buffer"""
        try:
            self.buffer.add(trajectory)
            logger.info(f"Trajectory stored: {trajectory.id}")
            return True
        except Exception as e:
            logger.error(f"Failed to store trajectory: {e}", exc_info=True)
            raise
    
    def get_stats(self) -> Dict[str, Any]:
        """Get service statistics"""
        trainer_stats = self.agentrl.get_trainer_stats()
        return {
            **trainer_stats,
            "buffer_size": self.buffer.size()
        }


# ===== FASTAPI APP =====

# Lifespan context manager
@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    logger.info("AgentRL HTTP Service starting up...")
    yield
    # Shutdown
    logger.info("AgentRL HTTP Service shutting down...")


app = FastAPI(
    title="AgentRL HTTP Service",
    description="REST API for AgentRL integration with RustyWorm",
    version="1.0.0",
    lifespan=lifespan
)

# Initialize service
service = AgentRLService()


# ===== HELPER FUNCTIONS =====

def convert_to_internal(profile_req: AiProfileRequest) -> AiProfile:
    """Convert Pydantic model to internal AiProfile"""
    return AiProfile(
        id=profile_req.id,
        name=profile_req.name,
        speech_pattern=profile_req.speech_pattern,
        knowledge_style=profile_req.knowledge_style,
        reasoning_style=profile_req.reasoning_style,
        creativity=profile_req.creativity,
        carefulness=profile_req.carefulness,
        empathy=profile_req.empathy
    )


def convert_observation(obs_req: BehaviorObservationRequest) -> BehaviorObservation:
    """Convert Pydantic model to internal BehaviorObservation"""
    return BehaviorObservation(
        query=obs_req.query,
        response=obs_req.response,
        patterns=obs_req.patterns or [],
        similarity_to_target=obs_req.similarity_to_target,
        confidence=obs_req.confidence
    )


def convert_delta_to_request(delta: PersonalityDelta) -> PersonalityDeltaRequest:
    """Convert internal PersonalityDelta to Pydantic model"""
    return PersonalityDeltaRequest(
        adjustments=delta.adjustments,
        confidence=delta.confidence,
        source=delta.source
    )


# ===== ENDPOINTS =====

@app.get("/health", response_model=HealthCheckResponse)
async def health_check():
    """
    Health check endpoint
    
    Returns:
        HealthCheckResponse with status and timestamp
    """
    return HealthCheckResponse(
        status="healthy",
        timestamp=datetime.utcnow().isoformat(),
        service_version="1.0.0"
    )


@app.post("/predict-delta", response_model=PredictDeltaResponse)
async def predict_delta(request: PredictDeltaRequest):
    """
    Predict optimal PersonalityDelta using RL model
    
    Args:
        request: Contains AI profile and behavior observation
    
    Returns:
        PredictDeltaResponse with optimized delta and confidence
    
    Raises:
        HTTPException: If prediction fails
    """
    try:
        profile = convert_to_internal(request.profile)
        observation = convert_observation(request.observation)
        
        delta, confidence = await service.predict_delta(profile, observation)
        
        return PredictDeltaResponse(
            delta=convert_delta_to_request(delta),
            confidence=confidence,
            reasoning="Optimized via RL policy"
        )
    except Exception as e:
        logger.error(f"Prediction endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/train", response_model=TrainingResponse)
async def train(request: TrainingRequest):
    """
    Train MINIRL/GRPO model on trajectory batch
    
    Args:
        request: Contains trajectories, weights, and loss type
    
    Returns:
        TrainingResponse with loss, time, and statistics
    
    Raises:
        HTTPException: If training fails
    """
    try:
        # Convert trajectories
        trajectories = []
        for traj_req in request.trajectories:
            state = convert_to_internal(traj_req.state)
            next_state = convert_to_internal(traj_req.next_state)
            
            action = PersonalityDelta(
                adjustments=traj_req.action.adjustments,
                confidence=traj_req.action.confidence,
                source=traj_req.action.source
            )
            
            observation = convert_observation(traj_req.observation)
            
            trajectory = EvolutionTrajectory(
                id=traj_req.id,
                state=state,
                action=action,
                observation=observation,
                reward=traj_req.reward,
                next_state=next_state,
                timestamp=traj_req.timestamp,
                used_in_training=traj_req.used_in_training,
                importance_weight=traj_req.importance_weight
            )
            trajectories.append(trajectory)
        
        # Train
        result = await service.train_on_trajectories(
            trajectories=trajectories,
            weights=request.importance_weights,
            loss_type=request.loss_type
        )
        
        return TrainingResponse(
            loss=result["loss"],
            training_time_ms=result["training_time_ms"],
            num_trajectories_used=result["num_trajectories_used"],
            loss_type=result["loss_type"]
        )
    except Exception as e:
        logger.error(f"Training endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/trajectory/store", response_model=StoreTrajectoryResponse)
async def store_trajectory(trajectory: EvolutionTrajectoryRequest):
    """
    Store trajectory in memory buffer (MongoDB integration optional)
    
    Args:
        trajectory: Evolution trajectory to store
    
    Returns:
        StoreTrajectoryResponse with ID and timestamp
    
    Raises:
        HTTPException: If storage fails
    """
    try:
        state = convert_to_internal(trajectory.state)
        next_state = convert_to_internal(trajectory.next_state)
        
        action = PersonalityDelta(
            adjustments=trajectory.action.adjustments,
            confidence=trajectory.action.confidence,
            source=trajectory.action.source
        )
        
        observation = convert_observation(trajectory.observation)
        
        traj = EvolutionTrajectory(
            id=trajectory.id,
            state=state,
            action=action,
            observation=observation,
            reward=trajectory.reward,
            next_state=next_state,
            timestamp=trajectory.timestamp,
            used_in_training=trajectory.used_in_training,
            importance_weight=trajectory.importance_weight
        )
        
        service.store_trajectory(traj)
        
        return StoreTrajectoryResponse(
            stored=True,
            id=trajectory.id,
            timestamp=datetime.utcnow().isoformat()
        )
    except Exception as e:
        logger.error(f"Store trajectory endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/stats", response_model=StatsResponse)
async def get_stats():
    """
    Get training and buffer statistics
    
    Returns:
        StatsResponse with training metrics and buffer info
    """
    try:
        stats = service.get_stats()
        return StatsResponse(
            training_runs=stats.get("training_runs", 0),
            average_loss=stats.get("average_loss", 0.0),
            min_loss=stats.get("min_loss", 0.0),
            max_loss=stats.get("max_loss", 0.0),
            last_loss=stats.get("last_loss", 0.0),
            buffer_size=stats.get("buffer_size", 0)
        )
    except Exception as e:
        logger.error(f"Stats endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/version")
async def get_version():
    """Get service version"""
    return {
        "service": "agentrl-http",
        "version": "1.0.0",
        "timestamp": datetime.utcnow().isoformat()
    }


@app.post("/test-connection")
async def test_connection():
    """Test connection endpoint for debugging"""
    return {
        "status": "connected",
        "service": "agentrl-http",
        "timestamp": datetime.utcnow().isoformat()
    }


# ===== ERROR HANDLERS =====

@app.exception_handler(ValueError)
async def value_error_handler(request, exc):
    """Handle ValueError exceptions"""
    return JSONResponse(
        status_code=400,
        content={"detail": str(exc)}
    )


@app.exception_handler(Exception)
async def general_exception_handler(request, exc):
    """Handle general exceptions"""
    logger.error(f"Unhandled exception: {exc}", exc_info=True)
    return JSONResponse(
        status_code=500,
        content={"detail": "Internal server error"}
    )


# ===== MAIN =====

def main():
    """Main entry point"""
    host = os.getenv("SERVICE_HOST", "0.0.0.0")
    port = int(os.getenv("SERVICE_PORT", "8888"))
    
    logger.info(f"Starting AgentRL HTTP Service on {host}:{port}")
    
    uvicorn.run(
        app,
        host=host,
        port=port,
        log_level=os.getenv("LOG_LEVEL", "info").lower(),
        workers=1,  # Single worker for async
    )


if __name__ == "__main__":
    main()
