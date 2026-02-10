"""
Unit Tests for AgentRL HTTP Service
====================================

Tests for all endpoints, data models, and core functionality.
Run with: pytest test_agentrl_service.py -v
"""

import pytest
import asyncio
from datetime import datetime
from httpx import AsyncClient

from fastapi.testclient import TestClient
from agentrl_service import app, service
from agentrl_wrapper import (
    AiProfile,
    PersonalityDelta,
    BehaviorObservation,
    EvolutionTrajectory,
    SimpleDeltaPredictor,
    MinIRLTrainer,
    TrajectoryBuffer,
)


# ===== FIXTURES =====

@pytest.fixture
def client():
    """FastAPI test client"""
    return TestClient(app)


@pytest.fixture
def sample_profile():
    """Sample AI profile for testing"""
    return AiProfile(
        id="test-ai-001",
        name="TestAI",
        speech_pattern=0.5,
        knowledge_style=0.6,
        reasoning_style=0.4,
        creativity=0.7,
        carefulness=0.5,
        empathy=0.6
    )


@pytest.fixture
def sample_observation():
    """Sample behavior observation for testing"""
    return BehaviorObservation(
        query="What is 2+2?",
        response="The answer is 4.",
        patterns=["mathematical_reasoning"],
        similarity_to_target=0.92,
        confidence=0.95
    )


@pytest.fixture
def sample_delta():
    """Sample personality delta for testing"""
    delta = PersonalityDelta(
        adjustments=[("speech_pattern", 0.1), ("reasoning_style", -0.05)],
        confidence=0.85,
        source="rl_optimizer"
    )
    return delta


@pytest.fixture
def sample_trajectory(sample_profile, sample_delta, sample_observation):
    """Sample evolution trajectory for testing"""
    return EvolutionTrajectory(
        id="traj-001",
        state=sample_profile,
        action=sample_delta,
        observation=sample_observation,
        reward=0.8,
        next_state=sample_profile,
        timestamp=datetime.utcnow().isoformat(),
        used_in_training=False,
        importance_weight=1.0
    )


# ===== TESTS: HEALTH CHECK =====

def test_health_check(client):
    """Test /health endpoint"""
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "healthy"
    assert "timestamp" in data
    assert data["service_version"] == "1.0.0"


def test_health_check_response_time(client):
    """Test health check responds quickly"""
    import time
    start = time.time()
    response = client.get("/health")
    elapsed = time.time() - start
    assert elapsed < 1.0  # Should be very fast
    assert response.status_code == 200


# ===== TESTS: PREDICT-DELTA ENDPOINT =====

def test_predict_delta_basic(client, sample_profile, sample_observation):
    """Test /predict-delta endpoint with valid data"""
    request_data = {
        "profile": {
            "id": sample_profile.id,
            "name": sample_profile.name,
            "speech_pattern": sample_profile.speech_pattern,
            "knowledge_style": sample_profile.knowledge_style,
            "reasoning_style": sample_profile.reasoning_style,
            "creativity": sample_profile.creativity,
            "carefulness": sample_profile.carefulness,
            "empathy": sample_profile.empathy,
        },
        "observation": {
            "query": sample_observation.query,
            "response": sample_observation.response,
            "patterns": sample_observation.patterns,
            "similarity_to_target": sample_observation.similarity_to_target,
            "confidence": sample_observation.confidence,
        }
    }
    
    response = client.post("/predict-delta", json=request_data)
    assert response.status_code == 200
    
    data = response.json()
    assert "delta" in data
    assert "confidence" in data
    assert "reasoning" in data
    assert data["confidence"] >= 0.0
    assert data["confidence"] <= 1.0


def test_predict_delta_adjustments(client, sample_profile, sample_observation):
    """Test that predict-delta returns valid adjustments"""
    request_data = {
        "profile": {
            "id": sample_profile.id,
            "name": sample_profile.name,
            "speech_pattern": sample_profile.speech_pattern,
            "knowledge_style": sample_profile.knowledge_style,
            "reasoning_style": sample_profile.reasoning_style,
            "creativity": sample_profile.creativity,
            "carefulness": sample_profile.carefulness,
            "empathy": sample_profile.empathy,
        },
        "observation": {
            "query": sample_observation.query,
            "response": sample_observation.response,
            "patterns": sample_observation.patterns,
            "similarity_to_target": sample_observation.similarity_to_target,
            "confidence": sample_observation.confidence,
        }
    }
    
    response = client.post("/predict-delta", json=request_data)
    assert response.status_code == 200
    
    data = response.json()
    delta = data["delta"]
    
    # Should have adjustments list
    assert isinstance(delta["adjustments"], list)
    assert delta["confidence"] > 0.0


def test_predict_delta_missing_profile(client, sample_observation):
    """Test /predict-delta with missing profile"""
    request_data = {
        "observation": {
            "query": sample_observation.query,
            "response": sample_observation.response,
            "patterns": sample_observation.patterns,
            "similarity_to_target": sample_observation.similarity_to_target,
            "confidence": sample_observation.confidence,
        }
    }
    
    response = client.post("/predict-delta", json=request_data)
    assert response.status_code in [400, 422]  # Validation error


# ===== TESTS: TRAIN ENDPOINT =====

def test_train_basic(client, sample_trajectory):
    """Test /train endpoint with valid data"""
    traj_dict = sample_trajectory.to_dict()
    
    request_data = {
        "trajectories": [traj_dict],
        "importance_weights": [1.0],
        "loss_type": "MINIRL"
    }
    
    response = client.post("/train", json=request_data)
    assert response.status_code == 200
    
    data = response.json()
    assert "loss" in data
    assert "training_time_ms" in data
    assert "num_trajectories_used" in data
    assert data["num_trajectories_used"] == 1
    assert data["loss_type"] == "MINIRL"


def test_train_multiple_trajectories(client, sample_trajectory):
    """Test /train with multiple trajectories"""
    traj_dict = sample_trajectory.to_dict()
    trajectories = [traj_dict, traj_dict, traj_dict]
    
    request_data = {
        "trajectories": trajectories,
        "importance_weights": [0.9, 0.8, 0.7],
        "loss_type": "MINIRL"
    }
    
    response = client.post("/train", json=request_data)
    assert response.status_code == 200
    
    data = response.json()
    assert data["num_trajectories_used"] == 3
    assert data["training_time_ms"] > 0


def test_train_grpo_loss(client, sample_trajectory):
    """Test /train with GRPO loss type"""
    traj_dict = sample_trajectory.to_dict()
    
    request_data = {
        "trajectories": [traj_dict],
        "importance_weights": [1.0],
        "loss_type": "GRPO"
    }
    
    response = client.post("/train", json=request_data)
    assert response.status_code == 200
    
    data = response.json()
    assert data["loss_type"] == "GRPO"


def test_train_no_trajectories(client):
    """Test /train with empty trajectories"""
    request_data = {
        "trajectories": [],
        "importance_weights": [],
        "loss_type": "MINIRL"
    }
    
    response = client.post("/train", json=request_data)
    assert response.status_code == 500  # Training error


# ===== TESTS: TRAJECTORY STORE ENDPOINT =====

def test_store_trajectory(client, sample_trajectory):
    """Test /trajectory/store endpoint"""
    traj_dict = sample_trajectory.to_dict()
    
    response = client.post("/trajectory/store", json=traj_dict)
    assert response.status_code == 200
    
    data = response.json()
    assert data["stored"] is True
    assert data["id"] == sample_trajectory.id
    assert "timestamp" in data


def test_store_multiple_trajectories(client, sample_trajectory):
    """Test storing multiple trajectories"""
    traj_dict = sample_trajectory.to_dict()
    
    for i in range(5):
        traj_dict["id"] = f"traj-{i}"
        response = client.post("/trajectory/store", json=traj_dict)
        assert response.status_code == 200
    
    # Check buffer size
    assert service.buffer.size() >= 5


# ===== TESTS: STATS ENDPOINT =====

def test_stats_endpoint(client):
    """Test /stats endpoint"""
    response = client.get("/stats")
    assert response.status_code == 200
    
    data = response.json()
    assert "training_runs" in data
    assert "average_loss" in data
    assert "min_loss" in data
    assert "max_loss" in data
    assert "last_loss" in data
    assert "buffer_size" in data


def test_stats_after_training(client, sample_trajectory):
    """Test stats reflect training"""
    traj_dict = sample_trajectory.to_dict()
    
    # Train
    request_data = {
        "trajectories": [traj_dict],
        "importance_weights": [1.0],
        "loss_type": "MINIRL"
    }
    
    client.post("/train", json=request_data)
    
    # Check stats
    response = client.get("/stats")
    data = response.json()
    assert data["training_runs"] > 0


# ===== TESTS: VERSION AND CONNECTION =====

def test_version_endpoint(client):
    """Test /version endpoint"""
    response = client.get("/version")
    assert response.status_code == 200
    
    data = response.json()
    assert data["service"] == "agentrl-http"
    assert data["version"] == "1.0.0"
    assert "timestamp" in data


def test_test_connection(client):
    """Test /test-connection endpoint"""
    response = client.post("/test-connection")
    assert response.status_code == 200
    
    data = response.json()
    assert data["status"] == "connected"


# ===== TESTS: DATA MODELS =====

class TestAiProfile:
    """Tests for AiProfile model"""
    
    def test_create_profile(self):
        """Test creating AI profile"""
        profile = AiProfile(
            id="test-001",
            name="TestAI",
            speech_pattern=0.5
        )
        assert profile.id == "test-001"
        assert profile.name == "TestAI"
        assert profile.speech_pattern == 0.5
    
    def test_profile_to_dict(self):
        """Test AiProfile.to_dict()"""
        profile = AiProfile(
            id="test-001",
            name="TestAI",
            speech_pattern=0.5
        )
        data = profile.to_dict()
        assert isinstance(data, dict)
        assert data["id"] == "test-001"
        assert data["speech_pattern"] == 0.5
    
    def test_profile_from_dict(self):
        """Test AiProfile.from_dict()"""
        data = {
            "id": "test-001",
            "name": "TestAI",
            "speech_pattern": 0.5,
            "knowledge_style": 0.6,
            "reasoning_style": 0.4,
            "creativity": 0.7,
            "carefulness": 0.5,
            "empathy": 0.6,
        }
        profile = AiProfile.from_dict(data)
        assert profile.id == "test-001"
        assert profile.name == "TestAI"


class TestPersonalityDelta:
    """Tests for PersonalityDelta model"""
    
    def test_create_delta(self):
        """Test creating personality delta"""
        delta = PersonalityDelta(
            adjustments=[("speech_pattern", 0.1)],
            confidence=0.85
        )
        assert len(delta.adjustments) == 1
        assert delta.confidence == 0.85
    
    def test_delta_serialization(self):
        """Test delta serialization"""
        delta = PersonalityDelta(
            adjustments=[("speech_pattern", 0.1), ("reasoning_style", -0.05)],
            confidence=0.85
        )
        
        dict_form = delta.to_dict()
        restored = PersonalityDelta.from_dict(dict_form)
        
        assert restored.confidence == delta.confidence
        assert len(restored.adjustments) == 2


class TestBehaviorObservation:
    """Tests for BehaviorObservation model"""
    
    def test_create_observation(self):
        """Test creating observation"""
        obs = BehaviorObservation(
            query="test query",
            response="test response",
            similarity_to_target=0.9,
            confidence=0.95
        )
        assert obs.query == "test query"
        assert obs.response == "test response"
    
    def test_observation_with_patterns(self):
        """Test observation with patterns"""
        obs = BehaviorObservation(
            query="test",
            response="response",
            patterns=["pattern1", "pattern2"],
            similarity_to_target=0.8
        )
        assert len(obs.patterns) == 2


# ===== TESTS: WRAPPER CLASSES =====

class TestTrajectoryBuffer:
    """Tests for TrajectoryBuffer"""
    
    def test_create_buffer(self):
        """Test creating trajectory buffer"""
        buffer = TrajectoryBuffer(max_size=100)
        assert buffer.size() == 0
    
    def test_add_trajectory(self):
        """Test adding trajectory to buffer"""
        buffer = TrajectoryBuffer()
        traj = EvolutionTrajectory(
            id="test-1",
            state=AiProfile("id1", "profile"),
            action=PersonalityDelta(),
            observation=BehaviorObservation("q", "r"),
            reward=0.5,
            next_state=AiProfile("id2", "profile"),
            timestamp=datetime.utcnow().isoformat()
        )
        buffer.add(traj)
        assert buffer.size() == 1
    
    def test_get_batch(self):
        """Test getting batch from buffer"""
        buffer = TrajectoryBuffer()
        for i in range(5):
            traj = EvolutionTrajectory(
                id=f"test-{i}",
                state=AiProfile(f"id{i}", "profile"),
                action=PersonalityDelta(),
                observation=BehaviorObservation("q", "r"),
                reward=0.5,
                next_state=AiProfile(f"id{i}", "profile"),
                timestamp=datetime.utcnow().isoformat()
            )
            buffer.add(traj)
        
        batch = buffer.get_batch(3)
        assert len(batch) == 3


class TestMinIRLTrainer:
    """Tests for MinIRLTrainer"""
    
    @pytest.mark.asyncio
    async def test_train_basic(self):
        """Test basic training"""
        trainer = MinIRLTrainer()
        
        # Create test trajectories
        trajectories = [
            EvolutionTrajectory(
                id="t1",
                state=AiProfile("id1", "name"),
                action=PersonalityDelta(),
                observation=BehaviorObservation("q", "r"),
                reward=0.8,
                next_state=AiProfile("id1", "name"),
                timestamp=datetime.utcnow().isoformat()
            ),
            EvolutionTrajectory(
                id="t2",
                state=AiProfile("id2", "name"),
                action=PersonalityDelta(),
                observation=BehaviorObservation("q", "r"),
                reward=0.6,
                next_state=AiProfile("id2", "name"),
                timestamp=datetime.utcnow().isoformat()
            ),
        ]
        
        result = await trainer.train(
            trajectories=trajectories,
            importance_weights=[0.5, 0.5],
            loss_type="MINIRL"
        )
        
        assert "loss" in result
        assert "training_time_ms" in result
        assert result["num_trajectories_used"] == 2
    
    def test_trainer_stats(self):
        """Test getting trainer statistics"""
        trainer = MinIRLTrainer()
        stats = trainer.get_stats()
        
        assert "training_runs" in stats
        assert "average_loss" in stats


class TestSimpleDeltaPredictor:
    """Tests for SimpleDeltaPredictor"""
    
    @pytest.mark.asyncio
    async def test_predict(self):
        """Test delta prediction"""
        predictor = SimpleDeltaPredictor()
        
        profile = AiProfile("id1", "name")
        observation = BehaviorObservation(
            query="test",
            response="response",
            similarity_to_target=0.7,
            confidence=0.9
        )
        
        delta, confidence = await predictor.predict(profile, observation)
        
        assert isinstance(delta, PersonalityDelta)
        assert 0.0 <= confidence <= 1.0


# ===== INTEGRATION TESTS =====

def test_full_workflow(client, sample_profile, sample_observation, sample_trajectory):
    """Test full workflow: predict -> train -> stats"""
    
    # 1. Predict delta
    predict_request = {
        "profile": {
            "id": sample_profile.id,
            "name": sample_profile.name,
            "speech_pattern": sample_profile.speech_pattern,
            "knowledge_style": sample_profile.knowledge_style,
            "reasoning_style": sample_profile.reasoning_style,
            "creativity": sample_profile.creativity,
            "carefulness": sample_profile.carefulness,
            "empathy": sample_profile.empathy,
        },
        "observation": {
            "query": sample_observation.query,
            "response": sample_observation.response,
            "patterns": sample_observation.patterns,
            "similarity_to_target": sample_observation.similarity_to_target,
            "confidence": sample_observation.confidence,
        }
    }
    
    predict_response = client.post("/predict-delta", json=predict_request)
    assert predict_response.status_code == 200
    
    # 2. Store trajectory
    store_response = client.post(
        "/trajectory/store",
        json=sample_trajectory.to_dict()
    )
    assert store_response.status_code == 200
    
    # 3. Train
    train_request = {
        "trajectories": [sample_trajectory.to_dict()],
        "importance_weights": [1.0],
        "loss_type": "MINIRL"
    }
    
    train_response = client.post("/train", json=train_request)
    assert train_response.status_code == 200
    
    # 4. Check stats
    stats_response = client.get("/stats")
    assert stats_response.status_code == 200
    stats = stats_response.json()
    assert stats["buffer_size"] >= 1


# ===== RUN TESTS =====

if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
