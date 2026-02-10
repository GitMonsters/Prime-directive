"""
AgentRL HTTP Service
====================

FastAPI service for the AgentRL integration with RustyWorm.
Provides REST API endpoints for:
- Delta prediction
- RL training
- Trajectory storage (MongoDB persistent + in-memory buffer)
- Trajectory retrieval and listing
- Health checks
- Statistics

Port: 8888

Phase 2C: Added async MongoDB integration for trajectory persistence
"""

import asyncio
import logging
import os
import json
from typing import List, Dict, Optional, Any
from datetime import datetime
from contextlib import asynccontextmanager

from fastapi import FastAPI, HTTPException, Query, Path
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

# Import MongoDB client
from mongodb_client import (
    MongoDBClient,
    TrajectoryRepository,
    MongoDBConnectionError,
    TrajectoryNotFoundError,
)

# Logging setup
logging.basicConfig(
    level=os.getenv("LOG_LEVEL", "INFO"),
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

# Configuration
MONGODB_ENABLED = os.getenv("MONGODB_ENABLED", "true").lower() in ("true", "1", "yes")
MODEL_NAME = os.getenv("MODEL_NAME", "rl-agent")

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
    model_name: Optional[str] = None  # Phase 2C: Added model_name


class EvolutionTrajectoryResponse(BaseModel):
    """Response model for Evolution Trajectory with MongoDB metadata"""
    id: str
    state: AiProfileRequest
    action: PersonalityDeltaRequest
    observation: BehaviorObservationRequest
    reward: float
    next_state: AiProfileRequest
    timestamp: str
    used_in_training: bool = False
    importance_weight: float = 1.0
    model_name: Optional[str] = None


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
    mongodb_status: Optional[str] = None  # Phase 2C: Added MongoDB status


class StatsResponse(BaseModel):
    """Response from /stats endpoint"""
    training_runs: int
    average_loss: float
    min_loss: float
    max_loss: float
    last_loss: float
    buffer_size: int
    # Phase 2C: MongoDB statistics
    mongodb_connected: bool = False
    mongodb_total_trajectories: int = 0
    mongodb_used_in_training: int = 0
    mongodb_unused: int = 0
    mongodb_avg_reward: float = 0.0
    mongodb_models: Dict[str, int] = Field(default_factory=dict)


class StoreTrajectoryResponse(BaseModel):
    """Response from /trajectory/store endpoint"""
    stored: bool
    id: str
    timestamp: str
    mongodb_id: Optional[str] = None  # Phase 2C: MongoDB document ID


class ListTrajectoriesResponse(BaseModel):
    """Response from /trajectories endpoint"""
    trajectories: List[EvolutionTrajectoryResponse]
    total: int
    limit: int
    offset: int


class TrainingBatchRequest(BaseModel):
    """Request for getting training batch from MongoDB"""
    batch_size: int = 32
    min_reward: float = 0.0
    model_name: Optional[str] = None
    exclude_used: bool = True


class TrainingBatchResponse(BaseModel):
    """Response with training batch"""
    trajectories: List[EvolutionTrajectoryResponse]
    count: int
    avg_reward: float


# ===== SERVICE STATE =====

class AgentRLService:
    """Service wrapper combining all AgentRL components with MongoDB"""
    
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
        
        # Phase 2C: MongoDB client
        self.mongodb_client: Optional[MongoDBClient] = None
        self.mongodb_connected = False
        
        logger.info("AgentRL Service initialized successfully")
    
    async def connect_mongodb(self):
        """Connect to MongoDB if enabled"""
        if not MONGODB_ENABLED:
            logger.info("MongoDB integration disabled")
            return
        
        try:
            self.mongodb_client = MongoDBClient()
            await self.mongodb_client.connect()
            self.mongodb_connected = True
            logger.info("MongoDB connected successfully")
        except MongoDBConnectionError as e:
            logger.warning(f"MongoDB connection failed (service will continue without persistence): {e}")
            self.mongodb_connected = False
        except Exception as e:
            logger.error(f"Unexpected MongoDB error: {e}")
            self.mongodb_connected = False
    
    async def close_mongodb(self):
        """Close MongoDB connection"""
        if self.mongodb_client:
            await self.mongodb_client.close()
            self.mongodb_connected = False
            logger.info("MongoDB connection closed")
    
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
            
            # Phase 2C: Mark trajectories as used in MongoDB
            if self.mongodb_connected and self.mongodb_client:
                trajectory_ids = [t.id for t in trajectories]
                await self.mongodb_client.repository.mark_batch_as_used(trajectory_ids)
                logger.info(f"Marked {len(trajectory_ids)} trajectories as used in MongoDB")
            
            logger.info(f"Training completed: {result}")
            return result
        except Exception as e:
            logger.error(f"Training failed: {e}", exc_info=True)
            raise
    
    async def store_trajectory(
        self, 
        trajectory: EvolutionTrajectory,
        model_name: str = MODEL_NAME
    ) -> tuple[bool, Optional[str]]:
        """
        Store trajectory in buffer and optionally MongoDB.
        
        Returns:
            Tuple of (stored_in_buffer, mongodb_id)
        """
        mongodb_id = None
        
        try:
            # Always store in memory buffer
            self.buffer.add(trajectory)
            logger.info(f"Trajectory stored in buffer: {trajectory.id}")
            
            # Phase 2C: Store in MongoDB if connected
            if self.mongodb_connected and self.mongodb_client:
                try:
                    mongodb_id = await self.mongodb_client.store_trajectory(
                        trajectory, 
                        model_name=model_name
                    )
                    logger.info(f"Trajectory stored in MongoDB: {mongodb_id}")
                except Exception as e:
                    logger.warning(f"Failed to store trajectory in MongoDB: {e}")
            
            return True, mongodb_id
        except Exception as e:
            logger.error(f"Failed to store trajectory: {e}", exc_info=True)
            raise
    
    async def get_trajectory(self, trajectory_id: str) -> Optional[EvolutionTrajectory]:
        """
        Get trajectory by ID from MongoDB.
        
        Returns:
            EvolutionTrajectory if found, None otherwise
        """
        if not self.mongodb_connected or not self.mongodb_client:
            raise HTTPException(
                status_code=503,
                detail="MongoDB not connected"
            )
        
        try:
            return await self.mongodb_client.get_trajectory(trajectory_id)
        except TrajectoryNotFoundError:
            return None
    
    async def list_trajectories(
        self,
        filter_params: Optional[Dict[str, Any]] = None,
        limit: int = 100,
        offset: int = 0
    ) -> tuple[List[EvolutionTrajectory], int]:
        """
        List trajectories from MongoDB.
        
        Returns:
            Tuple of (trajectories, total_count)
        """
        if not self.mongodb_connected or not self.mongodb_client:
            # Fall back to buffer
            trajectories = self.buffer.get_all()[offset:offset + limit]
            return trajectories, self.buffer.size()
        
        trajectories = await self.mongodb_client.list_trajectories(
            filter_params=filter_params,
            limit=limit,
            offset=offset
        )
        
        # Get total count
        stats = await self.mongodb_client.get_statistics()
        total = stats.get("total_count", len(trajectories))
        
        return trajectories, total
    
    async def get_training_batch(
        self,
        batch_size: int = 32,
        min_reward: float = 0.0,
        model_name: Optional[str] = None,
        exclude_used: bool = True
    ) -> List[EvolutionTrajectory]:
        """
        Get a batch of trajectories for training from MongoDB.
        """
        if not self.mongodb_connected or not self.mongodb_client:
            # Fall back to buffer
            return self.buffer.get_batch(batch_size)
        
        return await self.mongodb_client.repository.get_training_batch(
            batch_size=batch_size,
            min_reward=min_reward,
            model_name=model_name,
            exclude_used=exclude_used
        )
    
    async def delete_trajectory(self, trajectory_id: str) -> bool:
        """Delete trajectory from MongoDB."""
        if not self.mongodb_connected or not self.mongodb_client:
            raise HTTPException(
                status_code=503,
                detail="MongoDB not connected"
            )
        
        return await self.mongodb_client.delete_trajectory(trajectory_id)
    
    async def get_stats(self) -> Dict[str, Any]:
        """Get service statistics including MongoDB stats"""
        trainer_stats = self.agentrl.get_trainer_stats()
        
        stats = {
            **trainer_stats,
            "buffer_size": self.buffer.size(),
            "mongodb_connected": self.mongodb_connected,
            "mongodb_total_trajectories": 0,
            "mongodb_used_in_training": 0,
            "mongodb_unused": 0,
            "mongodb_avg_reward": 0.0,
            "mongodb_models": {}
        }
        
        # Phase 2C: Add MongoDB statistics
        if self.mongodb_connected and self.mongodb_client:
            try:
                mongo_stats = await self.mongodb_client.get_statistics()
                stats.update({
                    "mongodb_total_trajectories": mongo_stats.get("total_count", 0),
                    "mongodb_used_in_training": mongo_stats.get("used_in_training", 0),
                    "mongodb_unused": mongo_stats.get("unused", 0),
                    "mongodb_avg_reward": mongo_stats.get("avg_reward", 0.0),
                    "mongodb_models": mongo_stats.get("models", {})
                })
            except Exception as e:
                logger.warning(f"Failed to get MongoDB stats: {e}")
        
        return stats


# ===== FASTAPI APP =====

# Global service instance
service: Optional[AgentRLService] = None


# Lifespan context manager
@asynccontextmanager
async def lifespan(app: FastAPI):
    global service
    
    # Startup
    logger.info("AgentRL HTTP Service starting up...")
    service = AgentRLService()
    
    # Phase 2C: Connect to MongoDB
    await service.connect_mongodb()
    
    yield
    
    # Shutdown
    logger.info("AgentRL HTTP Service shutting down...")
    
    # Phase 2C: Close MongoDB connection
    if service:
        await service.close_mongodb()


app = FastAPI(
    title="AgentRL HTTP Service",
    description="REST API for AgentRL integration with RustyWorm. Includes MongoDB persistence.",
    version="1.1.0",  # Updated for Phase 2C
    lifespan=lifespan
)


# ===== HELPER FUNCTIONS =====

def get_service() -> AgentRLService:
    """Get the service instance"""
    if service is None:
        raise HTTPException(status_code=503, detail="Service not initialized")
    return service


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


def convert_trajectory_to_response(traj: EvolutionTrajectory) -> EvolutionTrajectoryResponse:
    """Convert internal trajectory to response model"""
    return EvolutionTrajectoryResponse(
        id=traj.id,
        state=AiProfileRequest(
            id=traj.state.id,
            name=traj.state.name,
            speech_pattern=traj.state.speech_pattern,
            knowledge_style=traj.state.knowledge_style,
            reasoning_style=traj.state.reasoning_style,
            creativity=traj.state.creativity,
            carefulness=traj.state.carefulness,
            empathy=traj.state.empathy
        ),
        action=PersonalityDeltaRequest(
            adjustments=traj.action.adjustments,
            confidence=traj.action.confidence,
            source=traj.action.source
        ),
        observation=BehaviorObservationRequest(
            query=traj.observation.query,
            response=traj.observation.response,
            patterns=traj.observation.patterns or [],
            similarity_to_target=traj.observation.similarity_to_target,
            confidence=traj.observation.confidence
        ),
        reward=traj.reward,
        next_state=AiProfileRequest(
            id=traj.next_state.id,
            name=traj.next_state.name,
            speech_pattern=traj.next_state.speech_pattern,
            knowledge_style=traj.next_state.knowledge_style,
            reasoning_style=traj.next_state.reasoning_style,
            creativity=traj.next_state.creativity,
            carefulness=traj.next_state.carefulness,
            empathy=traj.next_state.empathy
        ),
        timestamp=traj.timestamp,
        used_in_training=traj.used_in_training,
        importance_weight=traj.importance_weight
    )


def convert_request_to_trajectory(traj_req: EvolutionTrajectoryRequest) -> EvolutionTrajectory:
    """Convert request model to internal trajectory"""
    return EvolutionTrajectory(
        id=traj_req.id,
        state=convert_to_internal(traj_req.state),
        action=PersonalityDelta(
            adjustments=traj_req.action.adjustments,
            confidence=traj_req.action.confidence,
            source=traj_req.action.source
        ),
        observation=convert_observation(traj_req.observation),
        reward=traj_req.reward,
        next_state=convert_to_internal(traj_req.next_state),
        timestamp=traj_req.timestamp,
        used_in_training=traj_req.used_in_training,
        importance_weight=traj_req.importance_weight
    )


# ===== ENDPOINTS =====

@app.get("/health", response_model=HealthCheckResponse)
async def health_check():
    """
    Health check endpoint
    
    Returns:
        HealthCheckResponse with status and timestamp
    """
    svc = get_service()
    
    mongodb_status = "disabled"
    if MONGODB_ENABLED:
        if svc.mongodb_connected:
            mongodb_status = "connected"
        else:
            mongodb_status = "disconnected"
    
    return HealthCheckResponse(
        status="healthy",
        timestamp=datetime.utcnow().isoformat(),
        service_version="1.1.0",
        mongodb_status=mongodb_status
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
        svc = get_service()
        profile = convert_to_internal(request.profile)
        observation = convert_observation(request.observation)
        
        delta, confidence = await svc.predict_delta(profile, observation)
        
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
        svc = get_service()
        
        # Convert trajectories
        trajectories = [convert_request_to_trajectory(t) for t in request.trajectories]
        
        # Train
        result = await svc.train_on_trajectories(
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
    Store trajectory in memory buffer and MongoDB (if enabled)
    
    Args:
        trajectory: Evolution trajectory to store
    
    Returns:
        StoreTrajectoryResponse with ID, timestamp, and MongoDB ID
    
    Raises:
        HTTPException: If storage fails
    """
    try:
        svc = get_service()
        traj = convert_request_to_trajectory(trajectory)
        model_name = trajectory.model_name or MODEL_NAME
        
        stored, mongodb_id = await svc.store_trajectory(traj, model_name=model_name)
        
        return StoreTrajectoryResponse(
            stored=stored,
            id=trajectory.id,
            timestamp=datetime.utcnow().isoformat(),
            mongodb_id=mongodb_id
        )
    except Exception as e:
        logger.error(f"Store trajectory endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


# Phase 2C: New endpoint - Get trajectory by ID
@app.get("/trajectory/{trajectory_id}", response_model=EvolutionTrajectoryResponse)
async def get_trajectory(trajectory_id: str = Path(..., description="Trajectory ID")):
    """
    Get a trajectory by ID from MongoDB
    
    Args:
        trajectory_id: The trajectory ID
    
    Returns:
        EvolutionTrajectoryResponse with trajectory data
    
    Raises:
        HTTPException: If trajectory not found or MongoDB not connected
    """
    try:
        svc = get_service()
        traj = await svc.get_trajectory(trajectory_id)
        
        if traj is None:
            raise HTTPException(status_code=404, detail=f"Trajectory not found: {trajectory_id}")
        
        return convert_trajectory_to_response(traj)
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Get trajectory endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


# Phase 2C: New endpoint - Delete trajectory
@app.delete("/trajectory/{trajectory_id}")
async def delete_trajectory(trajectory_id: str = Path(..., description="Trajectory ID")):
    """
    Delete a trajectory from MongoDB
    
    Args:
        trajectory_id: The trajectory ID
    
    Returns:
        Success message
    
    Raises:
        HTTPException: If trajectory not found or MongoDB not connected
    """
    try:
        svc = get_service()
        deleted = await svc.delete_trajectory(trajectory_id)
        
        if not deleted:
            raise HTTPException(status_code=404, detail=f"Trajectory not found: {trajectory_id}")
        
        return {"deleted": True, "id": trajectory_id}
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Delete trajectory endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


# Phase 2C: New endpoint - List trajectories
@app.get("/trajectories", response_model=ListTrajectoriesResponse)
async def list_trajectories(
    limit: int = Query(100, ge=1, le=1000, description="Maximum number of trajectories"),
    offset: int = Query(0, ge=0, description="Number of trajectories to skip"),
    model_name: Optional[str] = Query(None, description="Filter by model name"),
    used_in_training: Optional[bool] = Query(None, description="Filter by training status"),
    min_reward: Optional[float] = Query(None, description="Minimum reward filter")
):
    """
    List trajectories from MongoDB with optional filtering
    
    Args:
        limit: Maximum number of results
        offset: Number of results to skip
        model_name: Optional filter by model name
        used_in_training: Optional filter by training status
        min_reward: Optional minimum reward filter
    
    Returns:
        ListTrajectoriesResponse with trajectories and pagination info
    """
    try:
        svc = get_service()
        
        # Build filter
        filter_params = {}
        if model_name:
            filter_params["model_name"] = model_name
        if used_in_training is not None:
            filter_params["used_in_training"] = used_in_training
        if min_reward is not None:
            filter_params["reward"] = {"$gte": min_reward}
        
        trajectories, total = await svc.list_trajectories(
            filter_params=filter_params if filter_params else None,
            limit=limit,
            offset=offset
        )
        
        return ListTrajectoriesResponse(
            trajectories=[convert_trajectory_to_response(t) for t in trajectories],
            total=total,
            limit=limit,
            offset=offset
        )
    except Exception as e:
        logger.error(f"List trajectories endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


# Phase 2C: New endpoint - Get training batch
@app.post("/trajectories/training-batch", response_model=TrainingBatchResponse)
async def get_training_batch(request: TrainingBatchRequest):
    """
    Get a batch of trajectories optimized for training
    
    Retrieves high-reward, unused trajectories from MongoDB.
    
    Args:
        request: Batch parameters (size, min_reward, model_name, exclude_used)
    
    Returns:
        TrainingBatchResponse with trajectories and statistics
    """
    try:
        svc = get_service()
        
        trajectories = await svc.get_training_batch(
            batch_size=request.batch_size,
            min_reward=request.min_reward,
            model_name=request.model_name,
            exclude_used=request.exclude_used
        )
        
        avg_reward = 0.0
        if trajectories:
            avg_reward = sum(t.reward for t in trajectories) / len(trajectories)
        
        return TrainingBatchResponse(
            trajectories=[convert_trajectory_to_response(t) for t in trajectories],
            count=len(trajectories),
            avg_reward=avg_reward
        )
    except Exception as e:
        logger.error(f"Training batch endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


# Phase 2C: Updated stats endpoint
@app.get("/stats", response_model=StatsResponse)
async def get_stats():
    """
    Get training and buffer statistics, including MongoDB stats
    
    Returns:
        StatsResponse with training metrics, buffer info, and MongoDB stats
    """
    try:
        svc = get_service()
        stats = await svc.get_stats()
        
        return StatsResponse(
            training_runs=stats.get("training_runs", 0),
            average_loss=stats.get("average_loss", 0.0),
            min_loss=stats.get("min_loss", 0.0),
            max_loss=stats.get("max_loss", 0.0),
            last_loss=stats.get("last_loss", 0.0),
            buffer_size=stats.get("buffer_size", 0),
            mongodb_connected=stats.get("mongodb_connected", False),
            mongodb_total_trajectories=stats.get("mongodb_total_trajectories", 0),
            mongodb_used_in_training=stats.get("mongodb_used_in_training", 0),
            mongodb_unused=stats.get("mongodb_unused", 0),
            mongodb_avg_reward=stats.get("mongodb_avg_reward", 0.0),
            mongodb_models=stats.get("mongodb_models", {})
        )
    except Exception as e:
        logger.error(f"Stats endpoint error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/version")
async def get_version():
    """Get service version"""
    svc = get_service()
    return {
        "service": "agentrl-http",
        "version": "1.1.0",
        "phase": "2C",
        "mongodb_enabled": MONGODB_ENABLED,
        "mongodb_connected": svc.mongodb_connected if svc else False,
        "timestamp": datetime.utcnow().isoformat()
    }


@app.post("/test-connection")
async def test_connection():
    """Test connection endpoint for debugging"""
    svc = get_service()
    return {
        "status": "connected",
        "service": "agentrl-http",
        "mongodb_connected": svc.mongodb_connected,
        "timestamp": datetime.utcnow().isoformat()
    }


# Phase 2C: MongoDB health check endpoint
@app.get("/mongodb/health")
async def mongodb_health():
    """
    Check MongoDB connection health
    
    Returns:
        MongoDB health status
    """
    svc = get_service()
    
    if not MONGODB_ENABLED:
        return {"status": "disabled", "message": "MongoDB integration is disabled"}
    
    if not svc.mongodb_client:
        return {"status": "not_initialized", "message": "MongoDB client not initialized"}
    
    health = await svc.mongodb_client.health_check()
    return health


# ===== ERROR HANDLERS =====

@app.exception_handler(ValueError)
async def value_error_handler(request, exc):
    """Handle ValueError exceptions"""
    return JSONResponse(
        status_code=400,
        content={"detail": str(exc)}
    )


@app.exception_handler(TrajectoryNotFoundError)
async def trajectory_not_found_handler(request, exc):
    """Handle TrajectoryNotFoundError exceptions"""
    return JSONResponse(
        status_code=404,
        content={"detail": str(exc)}
    )


@app.exception_handler(MongoDBConnectionError)
async def mongodb_connection_error_handler(request, exc):
    """Handle MongoDB connection errors"""
    return JSONResponse(
        status_code=503,
        content={"detail": f"MongoDB connection error: {str(exc)}"}
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
    logger.info(f"MongoDB integration: {'enabled' if MONGODB_ENABLED else 'disabled'}")
    
    uvicorn.run(
        app,
        host=host,
        port=port,
        log_level=os.getenv("LOG_LEVEL", "info").lower(),
        workers=1,  # Single worker for async
    )


if __name__ == "__main__":
    main()
