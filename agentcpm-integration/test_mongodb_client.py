"""
Unit and Integration Tests for MongoDB Client
==============================================

Tests for TrajectoryRepository and MongoDBClient classes.
Includes mocked unit tests and integration tests (requires running MongoDB).

Run with: pytest test_mongodb_client.py -v
For integration tests: pytest test_mongodb_client.py -v -m integration
"""

import pytest
import asyncio
from datetime import datetime
from unittest.mock import AsyncMock, MagicMock, patch
from typing import List, Dict, Any

from agentrl_wrapper import (
    AiProfile,
    PersonalityDelta,
    BehaviorObservation,
    EvolutionTrajectory,
)

from mongodb_client import (
    MongoDBClient,
    TrajectoryRepository,
    MongoDBConnectionError,
    TrajectoryNotFoundError,
    DATABASE_NAME,
    COLLECTION_NAME,
)


# ===== FIXTURES =====

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
    return PersonalityDelta(
        adjustments=[("speech_pattern", 0.1), ("reasoning_style", -0.05)],
        confidence=0.85,
        source="rl_optimizer"
    )


@pytest.fixture
def sample_trajectory(sample_profile, sample_delta, sample_observation):
    """Sample evolution trajectory for testing"""
    return EvolutionTrajectory(
        id="traj-test-001",
        state=sample_profile,
        action=sample_delta,
        observation=sample_observation,
        reward=0.8,
        next_state=sample_profile,
        timestamp=datetime.utcnow().isoformat(),
        used_in_training=False,
        importance_weight=1.0
    )


@pytest.fixture
def mock_collection():
    """Mock MongoDB collection"""
    mock = AsyncMock()
    mock.insert_one = AsyncMock(return_value=MagicMock(inserted_id="mock_id"))
    mock.find_one = AsyncMock()
    mock.find = MagicMock()
    mock.update_one = AsyncMock(return_value=MagicMock(modified_count=1))
    mock.delete_one = AsyncMock(return_value=MagicMock(deleted_count=1))
    mock.create_indexes = AsyncMock()
    mock.count_documents = AsyncMock(return_value=0)
    mock.aggregate = MagicMock()
    return mock


@pytest.fixture
def mock_db(mock_collection):
    """Mock MongoDB database"""
    mock = MagicMock()
    mock.__getitem__ = MagicMock(return_value=mock_collection)
    return mock


@pytest.fixture
def mock_client(mock_db):
    """Mock MongoDB client"""
    mock = MagicMock()
    mock.admin = MagicMock()
    mock.admin.command = AsyncMock(return_value={"ok": 1})
    mock.__getitem__ = MagicMock(return_value=mock_db)
    mock.close = MagicMock()
    return mock


# ===== UNIT TESTS: TrajectoryRepository =====

class TestTrajectoryRepositoryInit:
    """Tests for TrajectoryRepository initialization"""
    
    def test_init_default_values(self):
        """Test initialization with default values"""
        repo = TrajectoryRepository()
        assert repo.database_name == DATABASE_NAME
        assert repo.collection_name == COLLECTION_NAME
        assert not repo._connected
    
    def test_init_custom_values(self):
        """Test initialization with custom values"""
        repo = TrajectoryRepository(
            mongodb_url="mongodb://custom:27017",
            database_name="custom_db",
            collection_name="custom_collection",
            max_pool_size=50
        )
        assert repo.mongodb_url == "mongodb://custom:27017"
        assert repo.database_name == "custom_db"
        assert repo.collection_name == "custom_collection"
        assert repo.max_pool_size == 50


class TestTrajectoryConversion:
    """Tests for trajectory to/from document conversion"""
    
    def test_trajectory_to_document(self, sample_trajectory):
        """Test converting trajectory to MongoDB document"""
        repo = TrajectoryRepository()
        doc = repo._trajectory_to_document(sample_trajectory, model_name="test-model")
        
        assert doc["trajectory_id"] == sample_trajectory.id
        assert doc["reward"] == sample_trajectory.reward
        assert doc["model_name"] == "test-model"
        assert doc["used_in_training"] == False
        assert "created_at" in doc
        assert "updated_at" in doc
        assert "state" in doc
        assert "action" in doc
        assert "observation" in doc
    
    def test_document_to_trajectory(self, sample_trajectory):
        """Test converting MongoDB document to trajectory"""
        repo = TrajectoryRepository()
        
        # First convert to document
        doc = repo._trajectory_to_document(sample_trajectory)
        
        # Then convert back
        result = repo._document_to_trajectory(doc)
        
        assert result.id == sample_trajectory.id
        assert result.reward == sample_trajectory.reward
        assert result.state.name == sample_trajectory.state.name
        assert result.observation.query == sample_trajectory.observation.query


class TestTrajectoryRepositoryOperations:
    """Tests for TrajectoryRepository CRUD operations with mocks"""
    
    @pytest.mark.asyncio
    async def test_connect_success(self, mock_client):
        """Test successful connection"""
        with patch('mongodb_client.AsyncIOMotorClient', return_value=mock_client):
            repo = TrajectoryRepository()
            result = await repo.connect()
            
            assert result is True
            assert repo._connected is True
    
    @pytest.mark.asyncio
    async def test_store_trajectory(self, mock_client, mock_collection, sample_trajectory):
        """Test storing a trajectory"""
        with patch('mongodb_client.AsyncIOMotorClient', return_value=mock_client):
            repo = TrajectoryRepository()
            await repo.connect()
            
            # Mock the collection
            repo._db = MagicMock()
            repo._db.__getitem__ = MagicMock(return_value=mock_collection)
            
            result = await repo.store_trajectory(sample_trajectory)
            
            assert result == "mock_id"
            mock_collection.insert_one.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_get_trajectory_found(self, mock_client, mock_collection, sample_trajectory):
        """Test getting an existing trajectory"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        # Mock document return
        doc = repo._trajectory_to_document(sample_trajectory)
        mock_collection.find_one = AsyncMock(return_value=doc)
        
        result = await repo.get_trajectory(sample_trajectory.id)
        
        assert result.id == sample_trajectory.id
        assert result.reward == sample_trajectory.reward
    
    @pytest.mark.asyncio
    async def test_get_trajectory_not_found(self, mock_client, mock_collection):
        """Test getting a non-existent trajectory"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        mock_collection.find_one = AsyncMock(return_value=None)
        
        with pytest.raises(TrajectoryNotFoundError):
            await repo.get_trajectory("non-existent-id")
    
    @pytest.mark.asyncio
    async def test_delete_trajectory(self, mock_client, mock_collection):
        """Test deleting a trajectory"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        result = await repo.delete_trajectory("test-id")
        
        assert result is True
        mock_collection.delete_one.assert_called()
    
    @pytest.mark.asyncio
    async def test_update_trajectory_status(self, mock_client, mock_collection):
        """Test updating trajectory training status"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        result = await repo.update_trajectory_status("test-id", used_in_training=True)
        
        assert result is True
        mock_collection.update_one.assert_called_once()


class TestListTrajectories:
    """Tests for listing trajectories"""
    
    @pytest.mark.asyncio
    async def test_list_trajectories_empty(self, mock_collection):
        """Test listing when no trajectories exist"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        # Mock cursor with async iteration
        mock_cursor = MagicMock()
        mock_cursor.sort = MagicMock(return_value=mock_cursor)
        mock_cursor.skip = MagicMock(return_value=mock_cursor)
        mock_cursor.limit = MagicMock(return_value=mock_cursor)
        mock_cursor.__aiter__ = lambda self: self
        mock_cursor.__anext__ = AsyncMock(side_effect=StopAsyncIteration)
        
        mock_collection.find = MagicMock(return_value=mock_cursor)
        
        result = await repo.list_trajectories()
        
        assert result == []


class TestTrainingBatch:
    """Tests for training batch retrieval"""
    
    @pytest.mark.asyncio
    async def test_get_training_batch(self, mock_collection, sample_trajectory):
        """Test getting training batch"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        # Create mock documents
        doc = repo._trajectory_to_document(sample_trajectory)
        
        # Mock cursor with async iteration
        mock_cursor = MagicMock()
        mock_cursor.sort = MagicMock(return_value=mock_cursor)
        mock_cursor.limit = MagicMock(return_value=mock_cursor)
        
        # Async iterator
        docs = [doc]
        mock_cursor.__aiter__ = MagicMock(return_value=iter(docs))
        
        async def async_iter():
            for d in docs:
                yield d
        
        mock_cursor.__aiter__ = lambda self: async_iter()
        
        mock_collection.find = MagicMock(return_value=mock_cursor)
        
        result = await repo.get_training_batch(batch_size=10, min_reward=0.5)
        
        # Verify query was made
        mock_collection.find.assert_called_once()


class TestStatistics:
    """Tests for statistics retrieval"""
    
    @pytest.mark.asyncio
    async def test_get_statistics_empty(self, mock_collection):
        """Test statistics on empty collection"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        # Mock aggregation pipeline result
        mock_cursor = MagicMock()
        mock_cursor.to_list = AsyncMock(return_value=[])
        mock_collection.aggregate = MagicMock(return_value=mock_cursor)
        
        result = await repo.get_statistics()
        
        assert result["total_count"] == 0
        assert result["connected"] is True
    
    @pytest.mark.asyncio
    async def test_get_statistics_with_data(self, mock_collection):
        """Test statistics with data"""
        repo = TrajectoryRepository()
        repo._connected = True
        repo._db = MagicMock()
        repo._db.__getitem__ = MagicMock(return_value=mock_collection)
        
        # Mock aggregation result
        mock_result = [{
            "total": [{"count": 100}],
            "used": [{"count": 30}],
            "unused": [{"count": 70}],
            "reward_stats": [{"avg": 0.75, "min": 0.1, "max": 0.99, "sum": 75.0}],
            "by_model": [{"_id": "gpt4", "count": 50}, {"_id": "claude", "count": 50}]
        }]
        
        mock_cursor = MagicMock()
        mock_cursor.to_list = AsyncMock(return_value=mock_result)
        mock_collection.aggregate = MagicMock(return_value=mock_cursor)
        
        result = await repo.get_statistics()
        
        assert result["total_count"] == 100
        assert result["used_in_training"] == 30
        assert result["unused"] == 70
        assert result["avg_reward"] == 0.75
        assert result["models"]["gpt4"] == 50


class TestHealthCheck:
    """Tests for health check"""
    
    @pytest.mark.asyncio
    async def test_health_check_connected(self, mock_client):
        """Test health check when connected"""
        with patch('mongodb_client.AsyncIOMotorClient', return_value=mock_client):
            repo = TrajectoryRepository()
            await repo.connect()
            
            result = await repo.health_check()
            
            assert result["status"] == "healthy"
            assert result["connected"] is True
    
    @pytest.mark.asyncio
    async def test_health_check_disconnected(self):
        """Test health check when disconnected"""
        repo = TrajectoryRepository()
        
        result = await repo.health_check()
        
        assert result["status"] == "disconnected"


# ===== UNIT TESTS: MongoDBClient Wrapper =====

class TestMongoDBClientWrapper:
    """Tests for MongoDBClient convenience wrapper"""
    
    def test_client_init(self):
        """Test client initialization"""
        client = MongoDBClient()
        assert client.repository is not None
        assert client.repository.database_name == DATABASE_NAME
    
    def test_client_custom_url(self):
        """Test client with custom URL"""
        client = MongoDBClient(
            mongodb_url="mongodb://test:27017",
            database_name="custom_db"
        )
        assert client.repository.mongodb_url == "mongodb://test:27017"
        assert client.repository.database_name == "custom_db"


# ===== INTEGRATION TESTS =====
# These require a running MongoDB instance

@pytest.mark.integration
class TestMongoDBIntegration:
    """Integration tests requiring running MongoDB"""
    
    @pytest.fixture
    async def connected_client(self):
        """Fixture that provides connected client"""
        client = MongoDBClient()
        try:
            await client.connect()
            yield client
        finally:
            await client.close()
    
    @pytest.mark.asyncio
    async def test_connect_real_mongodb(self):
        """Test connecting to real MongoDB"""
        client = MongoDBClient()
        try:
            result = await client.connect()
            assert result is True
            
            health = await client.health_check()
            assert health["status"] == "healthy"
        except MongoDBConnectionError:
            pytest.skip("MongoDB not available")
        finally:
            await client.close()
    
    @pytest.mark.asyncio
    async def test_store_and_retrieve_trajectory(
        self, 
        connected_client, 
        sample_trajectory
    ):
        """Test storing and retrieving a trajectory"""
        # Store
        stored_id = await connected_client.store_trajectory(sample_trajectory)
        assert stored_id is not None
        
        # Retrieve
        retrieved = await connected_client.get_trajectory(sample_trajectory.id)
        assert retrieved.id == sample_trajectory.id
        assert retrieved.reward == sample_trajectory.reward
        
        # Cleanup
        await connected_client.delete_trajectory(sample_trajectory.id)
    
    @pytest.mark.asyncio
    async def test_list_trajectories(self, connected_client, sample_trajectory):
        """Test listing trajectories"""
        # Store some trajectories
        for i in range(5):
            traj = EvolutionTrajectory(
                id=f"list-test-{i}",
                state=sample_trajectory.state,
                action=sample_trajectory.action,
                observation=sample_trajectory.observation,
                reward=0.5 + i * 0.1,
                next_state=sample_trajectory.next_state,
                timestamp=datetime.utcnow().isoformat(),
                used_in_training=False,
                importance_weight=1.0
            )
            await connected_client.store_trajectory(traj)
        
        # List
        result = await connected_client.list_trajectories(limit=10)
        assert len(result) >= 5
        
        # Cleanup
        for i in range(5):
            await connected_client.delete_trajectory(f"list-test-{i}")
    
    @pytest.mark.asyncio
    async def test_training_batch_selection(self, connected_client, sample_trajectory):
        """Test training batch selection"""
        # Store trajectories with varying rewards
        for i in range(10):
            traj = EvolutionTrajectory(
                id=f"batch-test-{i}",
                state=sample_trajectory.state,
                action=sample_trajectory.action,
                observation=sample_trajectory.observation,
                reward=0.1 * (i + 1),  # 0.1 to 1.0
                next_state=sample_trajectory.next_state,
                timestamp=datetime.utcnow().isoformat(),
                used_in_training=False,
                importance_weight=1.0
            )
            await connected_client.store_trajectory(traj)
        
        # Get batch with min_reward filter
        batch = await connected_client.get_training_batch(
            batch_size=5, 
            min_reward=0.5
        )
        
        # Should get 5 trajectories with reward >= 0.5
        assert len(batch) <= 5
        for traj in batch:
            assert traj.reward >= 0.5
        
        # Cleanup
        for i in range(10):
            await connected_client.delete_trajectory(f"batch-test-{i}")
    
    @pytest.mark.asyncio
    async def test_update_training_status(self, connected_client, sample_trajectory):
        """Test updating training status"""
        # Store
        await connected_client.store_trajectory(sample_trajectory)
        
        # Update status
        result = await connected_client.update_trajectory_status(
            sample_trajectory.id, 
            used_in_training=True
        )
        assert result is True
        
        # Verify
        retrieved = await connected_client.get_trajectory(sample_trajectory.id)
        assert retrieved.used_in_training is True
        
        # Cleanup
        await connected_client.delete_trajectory(sample_trajectory.id)
    
    @pytest.mark.asyncio
    async def test_statistics(self, connected_client, sample_trajectory):
        """Test getting statistics"""
        # Store some trajectories
        for i in range(3):
            traj = EvolutionTrajectory(
                id=f"stats-test-{i}",
                state=sample_trajectory.state,
                action=sample_trajectory.action,
                observation=sample_trajectory.observation,
                reward=0.5 + i * 0.1,
                next_state=sample_trajectory.next_state,
                timestamp=datetime.utcnow().isoformat(),
                used_in_training=i % 2 == 0,  # Alternate
                importance_weight=1.0
            )
            await connected_client.store_trajectory(traj, model_name="test-model")
        
        # Get stats
        stats = await connected_client.get_statistics()
        
        assert stats["total_count"] >= 3
        assert stats["connected"] is True
        assert "avg_reward" in stats
        
        # Cleanup
        for i in range(3):
            await connected_client.delete_trajectory(f"stats-test-{i}")
    
    @pytest.mark.asyncio
    async def test_delete_trajectory(self, connected_client, sample_trajectory):
        """Test deleting a trajectory"""
        # Store
        await connected_client.store_trajectory(sample_trajectory)
        
        # Delete
        result = await connected_client.delete_trajectory(sample_trajectory.id)
        assert result is True
        
        # Verify deleted
        with pytest.raises(TrajectoryNotFoundError):
            await connected_client.get_trajectory(sample_trajectory.id)
    
    @pytest.mark.asyncio
    async def test_batch_performance(self, connected_client, sample_trajectory):
        """Test batch retrieval performance (<100ms for 100 trajectories)"""
        import time
        
        # Store 150 trajectories
        for i in range(150):
            traj = EvolutionTrajectory(
                id=f"perf-test-{i}",
                state=sample_trajectory.state,
                action=sample_trajectory.action,
                observation=sample_trajectory.observation,
                reward=0.5 + (i % 50) * 0.01,
                next_state=sample_trajectory.next_state,
                timestamp=datetime.utcnow().isoformat(),
                used_in_training=False,
                importance_weight=1.0
            )
            await connected_client.store_trajectory(traj)
        
        # Time batch retrieval
        start = time.time()
        batch = await connected_client.get_training_batch(batch_size=100, min_reward=0.0)
        elapsed_ms = (time.time() - start) * 1000
        
        assert len(batch) == 100
        assert elapsed_ms < 100, f"Batch retrieval took {elapsed_ms:.2f}ms (should be <100ms)"
        
        # Cleanup
        for i in range(150):
            await connected_client.delete_trajectory(f"perf-test-{i}")


# ===== RUN TESTS =====

if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short", "-m", "not integration"])
