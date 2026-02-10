# Phase 2B: AgentRL HTTP Service Wrapper - Quick Start Guide

**Status**: Ready to implement  
**Estimated Duration**: 1-2 weeks  
**Effort**: 400 LOC (Python) + Docker setup  

---

## Overview

Phase 2B creates an HTTP wrapper around AgentCPM's AgentRL framework, allowing RustyWorm's RL optimizer to communicate with the training backend via REST API.

### Architecture

```
RustyWorm (Rust)              AgentRL Service (Python)          MongoDB
   ↓                                 ↓                              ↑
RL Optimizer                   Flask/FastAPI Server           Trajectory DB
   ↓                                 ↓
POST /predict-delta      ← HTTP API Request
   ↓                                 ↓
   ├─ profile, observation  →  AgentRL Model
   └─ (wait for response)       ↓
        ←  PersonalityDelta  (optimized)

POST /train               ← HTTP Training Request
   ↓                                 ↓
   ├─ trajectories, weights  → MINIRL Loss
   └─ (wait for completion)        ↓
        ←  training_loss, time    MongoDB Storage
```

---

## Files to Create

### 1. agentcpm-integration/agentrl_service.py (400 LOC)

```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Dict, Optional
import asyncio
import logging
from datetime import datetime

# AgentRL imports
from agentrl_wrapper import (
    AgentRL,
    TrajectoryBuffer,
    DeltaPredictor,
    MinIRLTrainer
)

# Configuration
logger = logging.getLogger(__name__)
app = FastAPI(title="AgentRL Service", version="1.0.0")

# ===== DATA MODELS =====

class AiProfile(BaseModel):
    name: str
    id: str
    # ... other fields

class PersonalityDelta(BaseModel):
    adjustments: List[tuple]
    confidence: float
    source: str

class BehaviorObservation(BaseModel):
    query: str
    response: str
    patterns: List[str]
    similarity_to_target: float
    confidence: float

class EvolutionTrajectory(BaseModel):
    id: str
    state: AiProfile
    action: PersonalityDelta
    observation: BehaviorObservation
    reward: float
    next_state: AiProfile
    timestamp: str
    used_in_training: bool
    importance_weight: float

class PredictDeltaRequest(BaseModel):
    profile: AiProfile
    observation: BehaviorObservation

class PredictDeltaResponse(BaseModel):
    delta: PersonalityDelta
    confidence: float
    reasoning: str

class TrainingRequest(BaseModel):
    trajectories: List[EvolutionTrajectory]
    importance_weights: List[float]
    loss_type: str  # "MINIRL" or "GRPO"

class TrainingResponse(BaseModel):
    loss: float
    training_time_ms: int
    num_trajectories_used: int

# ===== SERVICE INITIALIZATION =====

class AgentRLService:
    def __init__(self):
        self.delta_predictor = DeltaPredictor()
        self.trainer = MinIRLTrainer()
        self.trajectory_buffer = TrajectoryBuffer()
        
    async def predict_delta(
        self,
        profile: AiProfile,
        observation: BehaviorObservation
    ) -> PredictDeltaResponse:
        """Predict optimal PersonalityDelta using RL model"""
        try:
            delta = self.delta_predictor.predict(profile, observation)
            return PredictDeltaResponse(
                delta=delta,
                confidence=0.85,
                reasoning="Optimized via RL policy"
            )
        except Exception as e:
            logger.error(f"Prediction failed: {e}")
            raise
    
    async def train_on_trajectories(
        self,
        trajectories: List[EvolutionTrajectory],
        weights: List[float],
        loss_type: str
    ) -> TrainingResponse:
        """Train MINIRL/GRPO model on trajectories"""
        try:
            start_time = time.time()
            loss = self.trainer.train(
                trajectories=trajectories,
                importance_weights=weights,
                loss_type=loss_type
            )
            elapsed_ms = (time.time() - start_time) * 1000
            
            return TrainingResponse(
                loss=loss,
                training_time_ms=int(elapsed_ms),
                num_trajectories_used=len(trajectories)
            )
        except Exception as e:
            logger.error(f"Training failed: {e}")
            raise

# Initialize service
service = AgentRLService()

# ===== ENDPOINTS =====

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "timestamp": datetime.utcnow().isoformat()}

@app.post("/predict-delta", response_model=PredictDeltaResponse)
async def predict_delta(request: PredictDeltaRequest):
    """Predict optimal PersonalityDelta"""
    return await service.predict_delta(request.profile, request.observation)

@app.post("/train", response_model=TrainingResponse)
async def train(request: TrainingRequest):
    """Train MINIRL model on trajectory batch"""
    return await service.train_on_trajectories(
        request.trajectories,
        request.importance_weights,
        request.loss_type
    )

@app.post("/trajectory/store")
async def store_trajectory(trajectory: EvolutionTrajectory):
    """Store trajectory in MongoDB"""
    service.trajectory_buffer.store(trajectory)
    return {"stored": True, "id": trajectory.id}

@app.get("/stats")
async def get_stats():
    """Get training statistics"""
    return service.trainer.get_stats()

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8888)
```

### 2. agentcpm-integration/requirements.txt

```
# HTTP Server
fastapi>=0.104.0
uvicorn>=0.24.0
pydantic>=2.0.0

# AgentRL Integration
torch>=2.0.0
transformers>=4.30.0
accelerate>=0.20.0

# Database
motor>=3.3.0  # Async MongoDB
pymongo>=4.5.0

# Utilities
numpy>=1.24.0
tqdm>=4.65.0
pyyaml>=6.0
python-dotenv>=1.0.0

# Monitoring
prometheus-client>=0.17.0
```

### 3. agentcpm-integration/Dockerfile

```dockerfile
FROM python:3.10-slim

WORKDIR /workspace

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy AgentRL code
COPY agentrl_service.py .
COPY agentrl_wrapper.py .

# Expose port
EXPOSE 8888

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8888/health || exit 1

# Run service
CMD ["python", "agentrl_service.py"]
```

### 4. agentcpm-integration/docker-compose.yml (update)

```yaml
version: '3.8'

services:
  # ... existing services ...

  # AgentRL Training Service (NEW)
  agentrl-service:
    build:
      context: ./agentcpm-integration
      dockerfile: Dockerfile
    container_name: agentrl-service
    ports:
      - "8888:8888"
    environment:
      - MONGODB_URL=mongodb://root:password@mongodb:27017
      - LOG_LEVEL=INFO
      - BATCH_SIZE=64
      - MODEL_NAME=rl-agent
    volumes:
      - ./agentcpm-integration/models:/workspace/models
      - ./agentcpm-integration/logs:/workspace/logs
    depends_on:
      - mongodb
    networks:
      - rustyworm-net
    restart: unless-stopped

  mongodb:
    image: mongo:7.0
    container_name: mongodb
    ports:
      - "27017:27017"
    environment:
      - MONGO_INITDB_ROOT_USERNAME=root
      - MONGO_INITDB_ROOT_PASSWORD=password
    volumes:
      - mongodb_data:/data/db
    networks:
      - rustyworm-net

volumes:
  mongodb_data:

networks:
  rustyworm-net:
    driver: bridge
```

---

## Implementation Steps

### Step 1: Setup (30 min)
- [ ] Create `agentcpm-integration/` directory
- [ ] Copy AgentRL code from `/home/worm/AgentCPM/AgentCPM-Explore/AgentRL`
- [ ] Create `agentrl_wrapper.py` (wrapper for existing AgentRL)
- [ ] Create `requirements.txt` with dependencies

### Step 2: Implement Service (3-4 hours)
- [ ] Create `agentrl_service.py`
- [ ] Implement Pydantic models
- [ ] Create endpoint handlers
- [ ] Add MongoDB integration
- [ ] Error handling and logging

### Step 3: Testing (2-3 hours)
- [ ] Unit tests for each endpoint
- [ ] Integration tests with RustyWorm
- [ ] Load testing
- [ ] Error case handling

### Step 4: Deployment (2-3 hours)
- [ ] Create Dockerfile
- [ ] Update docker-compose.yml
- [ ] Build and test container
- [ ] Documentation

### Step 5: Integration (2-3 hours)
- [ ] Update `rl_optimizer.rs` HTTP calls
- [ ] End-to-end testing
- [ ] Performance profiling
- [ ] Bug fixes

---

## Testing Plan

### Unit Tests
```python
def test_predict_delta():
    """Test /predict-delta endpoint"""
    response = client.post("/predict-delta", json={...})
    assert response.status_code == 200
    assert response.json()["confidence"] > 0.7

def test_train_endpoint():
    """Test /train endpoint"""
    response = client.post("/train", json={...})
    assert response.status_code == 200
    assert response.json()["loss"] > 0.0

def test_health_check():
    """Test /health endpoint"""
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json()["status"] == "healthy"
```

### Integration Tests
```rust
#[tokio::test]
async fn test_rl_service_integration() {
    let optimizer = ReinforcementLearningOptimizer::new(config).unwrap();
    
    // Test health check
    assert!(optimizer.health_check().await.unwrap());
    
    // Test delta prediction
    let delta = optimizer.predict_delta(&profile, &observation).await.unwrap();
    assert!(delta.confidence > 0.5);
    
    // Test trajectory collection and training
    for _ in 0..50 {
        optimizer.collect_trajectory(trajectory.clone());
    }
    
    let _ = optimizer.train_on_trajectories("MINIRL").await.unwrap();
}
```

---

## Success Criteria

- [ ] HTTP service starts without errors
- [ ] All 5 endpoints working correctly
- [ ] `/predict-delta` returns valid PersonalityDelta
- [ ] `/train` completes in < 5 minutes for 100 trajectories
- [ ] MongoDB stores trajectories successfully
- [ ] Docker container builds and runs
- [ ] RustyWorm RL optimizer can communicate
- [ ] Error handling for failures
- [ ] Logging enabled for debugging

---

## Deployment Commands

```bash
# Build Docker image
cd agentcpm-integration
docker build -t agentrl-service:latest .

# Start all services
docker-compose up -d agentrl-service mongodb

# Check service health
curl http://localhost:8888/health

# View logs
docker logs -f agentrl-service

# Stop services
docker-compose down
```

---

## Environment Variables

```
MONGODB_URL=mongodb://root:password@localhost:27017
LOG_LEVEL=INFO
BATCH_SIZE=64
MAX_TRAJECTORY_BUFFER=1000
TRAINING_TIMEOUT_SECONDS=300
MODEL_CHECKPOINT_DIR=/workspace/models
```

---

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Startup time | < 5 sec | Including model loading |
| /predict-delta latency | < 500 ms | Single trajectory |
| /train latency | < 5 min | 100 trajectories, GPU |
| Throughput | 20+ requests/sec | For HTTP server |
| Memory usage | < 4 GB | Model + buffer |
| MongoDB write latency | < 100 ms | Per trajectory |

---

## Next: Phase 2C - MongoDB Integration

After Phase 2B, implement async MongoDB for:
- Persistent trajectory storage
- Training history tracking
- Model checkpoints
- Statistics and metrics

---

**Prepared by**: Integration Design Team  
**Date**: 2026-02-10  
**Status**: Ready for Implementation
