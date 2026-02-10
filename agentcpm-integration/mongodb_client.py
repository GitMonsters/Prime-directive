"""
MongoDB Async Client for Trajectory Persistence
================================================

Provides async MongoDB integration for storing and retrieving 
RL trajectories using the motor driver.

Features:
- Async CRUD operations with motor driver
- Connection pooling and retry logic  
- Index creation for query optimization
- Batch retrieval for training
- Statistics and monitoring

Database: rustyworm_rl
Collection: trajectories

Usage:
    client = MongoDBClient()
    await client.connect()
    trajectory_id = await client.store_trajectory(trajectory)
    await client.close()
"""

import asyncio
import logging
import os
from typing import List, Dict, Optional, Any
from datetime import datetime
from dataclasses import asdict

from motor.motor_asyncio import AsyncIOMotorClient, AsyncIOMotorDatabase
from pymongo import DESCENDING, ASCENDING, IndexModel
from pymongo.errors import (
    ConnectionFailure,
    ServerSelectionTimeoutError,
    DuplicateKeyError,
    OperationFailure,
)
from bson import ObjectId

from agentrl_wrapper import (
    AiProfile,
    PersonalityDelta,
    BehaviorObservation,
    EvolutionTrajectory,
)

logger = logging.getLogger(__name__)

# Configuration
DEFAULT_MONGODB_URL = "mongodb://root:password@localhost:27017"
DATABASE_NAME = "rustyworm_rl"
COLLECTION_NAME = "trajectories"

# Retry configuration
MAX_RETRIES = 3
RETRY_DELAY_SECONDS = 1.0
CONNECTION_TIMEOUT_MS = 5000
SERVER_SELECTION_TIMEOUT_MS = 5000


class MongoDBConnectionError(Exception):
    """Raised when MongoDB connection fails"""
    pass


class TrajectoryNotFoundError(Exception):
    """Raised when trajectory is not found"""
    pass


class TrajectoryRepository:
    """
    Repository for RL trajectory persistence in MongoDB.
    
    Provides CRUD operations, batch retrieval, and statistics
    for evolution trajectories used in RL training.
    """
    
    def __init__(
        self,
        mongodb_url: Optional[str] = None,
        database_name: str = DATABASE_NAME,
        collection_name: str = COLLECTION_NAME,
        max_pool_size: int = 100,
        min_pool_size: int = 10,
    ):
        """
        Initialize TrajectoryRepository.
        
        Args:
            mongodb_url: MongoDB connection string
            database_name: Database name
            collection_name: Collection name
            max_pool_size: Maximum connection pool size
            min_pool_size: Minimum connection pool size
        """
        self.mongodb_url = mongodb_url or os.getenv("MONGODB_URL", DEFAULT_MONGODB_URL)
        self.database_name = database_name
        self.collection_name = collection_name
        self.max_pool_size = max_pool_size
        self.min_pool_size = min_pool_size
        
        self._client: Optional[AsyncIOMotorClient] = None
        self._db: Optional[AsyncIOMotorDatabase] = None
        self._connected = False
        
        logger.info(
            f"Initialized TrajectoryRepository: db={database_name}, "
            f"collection={collection_name}"
        )
    
    async def connect(self) -> bool:
        """
        Establish connection to MongoDB with retry logic.
        
        Returns:
            True if connection successful
            
        Raises:
            MongoDBConnectionError: If connection fails after retries
        """
        for attempt in range(MAX_RETRIES):
            try:
                logger.info(f"Connecting to MongoDB (attempt {attempt + 1}/{MAX_RETRIES})...")
                
                self._client = AsyncIOMotorClient(
                    self.mongodb_url,
                    maxPoolSize=self.max_pool_size,
                    minPoolSize=self.min_pool_size,
                    connectTimeoutMS=CONNECTION_TIMEOUT_MS,
                    serverSelectionTimeoutMS=SERVER_SELECTION_TIMEOUT_MS,
                )
                
                # Verify connection with ping
                await self._client.admin.command('ping')
                
                self._db = self._client[self.database_name]
                self._connected = True
                
                logger.info("MongoDB connection established successfully")
                
                # Create indexes on first connect
                await self._ensure_indexes()
                
                return True
                
            except (ConnectionFailure, ServerSelectionTimeoutError) as e:
                logger.warning(f"MongoDB connection attempt {attempt + 1} failed: {e}")
                if attempt < MAX_RETRIES - 1:
                    await asyncio.sleep(RETRY_DELAY_SECONDS * (attempt + 1))
                else:
                    raise MongoDBConnectionError(
                        f"Failed to connect to MongoDB after {MAX_RETRIES} attempts: {e}"
                    )
        
        return False
    
    async def close(self):
        """Close MongoDB connection"""
        if self._client:
            self._client.close()
            self._connected = False
            logger.info("MongoDB connection closed")
    
    async def _ensure_indexes(self):
        """Create indexes for optimal query performance"""
        collection = self._db[self.collection_name]
        
        indexes = [
            # Timestamp descending - for time-based queries
            IndexModel([("timestamp", DESCENDING)], name="idx_timestamp_desc"),
            
            # Reward descending - for batch selection by reward
            IndexModel([("reward", DESCENDING)], name="idx_reward_desc"),
            
            # Used in training - for filtering unused trajectories
            IndexModel([("used_in_training", ASCENDING)], name="idx_used_in_training"),
            
            # Model name - for per-model queries
            IndexModel([("model_name", ASCENDING)], name="idx_model_name"),
            
            # Compound index: used_in_training + reward for training batch selection
            IndexModel(
                [("used_in_training", ASCENDING), ("reward", DESCENDING)],
                name="idx_training_batch"
            ),
            
            # Trajectory ID - unique for deduplication
            IndexModel([("trajectory_id", ASCENDING)], unique=True, name="idx_trajectory_id"),
            
            # Created at - for cleanup operations
            IndexModel([("created_at", DESCENDING)], name="idx_created_at"),
        ]
        
        try:
            await collection.create_indexes(indexes)
            logger.info(f"Created {len(indexes)} indexes on {self.collection_name}")
        except OperationFailure as e:
            logger.warning(f"Index creation warning: {e}")
    
    @property
    def collection(self):
        """Get the trajectories collection"""
        if not self._connected:
            raise MongoDBConnectionError("Not connected to MongoDB")
        return self._db[self.collection_name]
    
    def _trajectory_to_document(
        self, 
        trajectory: EvolutionTrajectory,
        model_name: str = "default"
    ) -> Dict[str, Any]:
        """
        Convert EvolutionTrajectory to MongoDB document.
        
        Args:
            trajectory: Trajectory to convert
            model_name: Name of the model being observed
            
        Returns:
            MongoDB document dictionary
        """
        now = datetime.utcnow()
        
        # Convert state and next_state to dicts
        state_dict = trajectory.state.to_dict() if hasattr(trajectory.state, 'to_dict') else asdict(trajectory.state)
        next_state_dict = trajectory.next_state.to_dict() if hasattr(trajectory.next_state, 'to_dict') else asdict(trajectory.next_state)
        action_dict = trajectory.action.to_dict() if hasattr(trajectory.action, 'to_dict') else {
            "adjustments": trajectory.action.adjustments,
            "confidence": trajectory.action.confidence,
            "source": trajectory.action.source
        }
        observation_dict = trajectory.observation.to_dict() if hasattr(trajectory.observation, 'to_dict') else {
            "query": trajectory.observation.query,
            "response": trajectory.observation.response,
            "patterns": trajectory.observation.patterns or [],
            "similarity_to_target": trajectory.observation.similarity_to_target,
            "confidence": trajectory.observation.confidence
        }
        
        return {
            "trajectory_id": trajectory.id,
            "state": state_dict,
            "action": action_dict,
            "observation": observation_dict,
            "reward": trajectory.reward,
            "next_state": next_state_dict,
            "timestamp": datetime.fromisoformat(trajectory.timestamp) if isinstance(trajectory.timestamp, str) else trajectory.timestamp,
            "used_in_training": trajectory.used_in_training,
            "importance_weight": trajectory.importance_weight,
            "model_name": model_name,
            "created_at": now,
            "updated_at": now,
        }
    
    def _document_to_trajectory(self, doc: Dict[str, Any]) -> EvolutionTrajectory:
        """
        Convert MongoDB document to EvolutionTrajectory.
        
        Args:
            doc: MongoDB document
            
        Returns:
            EvolutionTrajectory instance
        """
        # Reconstruct state
        state = AiProfile.from_dict(doc["state"])
        next_state = AiProfile.from_dict(doc["next_state"])
        
        # Reconstruct action
        action_data = doc["action"]
        action = PersonalityDelta(
            adjustments=action_data.get("adjustments", []),
            confidence=action_data.get("confidence", 0.5),
            source=action_data.get("source", "unknown")
        )
        
        # Reconstruct observation
        obs_data = doc["observation"]
        observation = BehaviorObservation(
            query=obs_data.get("query", ""),
            response=obs_data.get("response", ""),
            patterns=obs_data.get("patterns", []),
            similarity_to_target=obs_data.get("similarity_to_target", 0.0),
            confidence=obs_data.get("confidence", 0.0)
        )
        
        # Convert timestamp
        timestamp = doc["timestamp"]
        if isinstance(timestamp, datetime):
            timestamp = timestamp.isoformat()
        
        return EvolutionTrajectory(
            id=doc["trajectory_id"],
            state=state,
            action=action,
            observation=observation,
            reward=doc["reward"],
            next_state=next_state,
            timestamp=timestamp,
            used_in_training=doc.get("used_in_training", False),
            importance_weight=doc.get("importance_weight", 1.0)
        )
    
    async def store_trajectory(
        self,
        trajectory: EvolutionTrajectory,
        model_name: str = "default"
    ) -> str:
        """
        Store a trajectory in MongoDB.
        
        Args:
            trajectory: Evolution trajectory to store
            model_name: Name of the observed model
            
        Returns:
            String ID of the stored document
            
        Raises:
            MongoDBConnectionError: If not connected
        """
        document = self._trajectory_to_document(trajectory, model_name)
        
        for attempt in range(MAX_RETRIES):
            try:
                result = await self.collection.insert_one(document)
                logger.debug(f"Stored trajectory: {trajectory.id}")
                return str(result.inserted_id)
            except DuplicateKeyError:
                # Trajectory already exists - update instead
                await self.collection.update_one(
                    {"trajectory_id": trajectory.id},
                    {"$set": {**document, "updated_at": datetime.utcnow()}}
                )
                existing = await self.collection.find_one({"trajectory_id": trajectory.id})
                return str(existing["_id"])
            except Exception as e:
                logger.warning(f"Store attempt {attempt + 1} failed: {e}")
                if attempt < MAX_RETRIES - 1:
                    await asyncio.sleep(RETRY_DELAY_SECONDS)
                else:
                    raise
        
        return ""
    
    async def get_trajectory(self, trajectory_id: str) -> EvolutionTrajectory:
        """
        Retrieve a trajectory by ID.
        
        Args:
            trajectory_id: Trajectory ID (not MongoDB _id)
            
        Returns:
            EvolutionTrajectory instance
            
        Raises:
            TrajectoryNotFoundError: If trajectory not found
        """
        doc = await self.collection.find_one({"trajectory_id": trajectory_id})
        
        if doc is None:
            # Try by MongoDB _id
            try:
                doc = await self.collection.find_one({"_id": ObjectId(trajectory_id)})
            except Exception:
                pass
        
        if doc is None:
            raise TrajectoryNotFoundError(f"Trajectory not found: {trajectory_id}")
        
        return self._document_to_trajectory(doc)
    
    async def list_trajectories(
        self,
        filter_params: Optional[Dict[str, Any]] = None,
        limit: int = 100,
        offset: int = 0,
        sort_by: str = "timestamp",
        sort_order: int = DESCENDING
    ) -> List[EvolutionTrajectory]:
        """
        List trajectories with optional filtering and pagination.
        
        Args:
            filter_params: MongoDB filter query
            limit: Maximum number of results
            offset: Number of results to skip
            sort_by: Field to sort by
            sort_order: ASCENDING or DESCENDING
            
        Returns:
            List of EvolutionTrajectory instances
        """
        query = filter_params or {}
        
        cursor = self.collection.find(query).sort(
            sort_by, sort_order
        ).skip(offset).limit(limit)
        
        trajectories = []
        async for doc in cursor:
            try:
                traj = self._document_to_trajectory(doc)
                trajectories.append(traj)
            except Exception as e:
                logger.warning(f"Failed to deserialize trajectory: {e}")
        
        return trajectories
    
    async def delete_trajectory(self, trajectory_id: str) -> bool:
        """
        Delete a trajectory by ID.
        
        Args:
            trajectory_id: Trajectory ID
            
        Returns:
            True if deleted, False if not found
        """
        result = await self.collection.delete_one({"trajectory_id": trajectory_id})
        
        if result.deleted_count == 0:
            # Try by MongoDB _id
            try:
                result = await self.collection.delete_one({"_id": ObjectId(trajectory_id)})
            except Exception:
                pass
        
        return result.deleted_count > 0
    
    async def get_training_batch(
        self,
        batch_size: int = 32,
        min_reward: float = 0.0,
        model_name: Optional[str] = None,
        exclude_used: bool = True
    ) -> List[EvolutionTrajectory]:
        """
        Get a batch of trajectories for training.
        
        Retrieves high-reward, unused trajectories optimal for training.
        
        Args:
            batch_size: Number of trajectories to retrieve
            min_reward: Minimum reward threshold
            model_name: Filter by specific model (optional)
            exclude_used: Exclude trajectories already used in training
            
        Returns:
            List of EvolutionTrajectory instances
        """
        query: Dict[str, Any] = {"reward": {"$gte": min_reward}}
        
        if exclude_used:
            query["used_in_training"] = False
        
        if model_name:
            query["model_name"] = model_name
        
        # Sort by reward descending to get best trajectories first
        # Then by importance_weight descending
        cursor = self.collection.find(query).sort([
            ("reward", DESCENDING),
            ("importance_weight", DESCENDING)
        ]).limit(batch_size)
        
        trajectories = []
        async for doc in cursor:
            try:
                traj = self._document_to_trajectory(doc)
                trajectories.append(traj)
            except Exception as e:
                logger.warning(f"Failed to deserialize trajectory: {e}")
        
        logger.info(f"Retrieved training batch: {len(trajectories)} trajectories")
        return trajectories
    
    async def update_trajectory_status(
        self,
        trajectory_id: str,
        used_in_training: bool = True
    ) -> bool:
        """
        Update the training status of a trajectory.
        
        Args:
            trajectory_id: Trajectory ID
            used_in_training: New training status
            
        Returns:
            True if updated, False if not found
        """
        result = await self.collection.update_one(
            {"trajectory_id": trajectory_id},
            {
                "$set": {
                    "used_in_training": used_in_training,
                    "updated_at": datetime.utcnow()
                }
            }
        )
        
        return result.modified_count > 0
    
    async def mark_batch_as_used(self, trajectory_ids: List[str]) -> int:
        """
        Mark multiple trajectories as used in training.
        
        Args:
            trajectory_ids: List of trajectory IDs
            
        Returns:
            Number of trajectories updated
        """
        result = await self.collection.update_many(
            {"trajectory_id": {"$in": trajectory_ids}},
            {
                "$set": {
                    "used_in_training": True,
                    "updated_at": datetime.utcnow()
                }
            }
        )
        
        return result.modified_count
    
    async def get_statistics(self) -> Dict[str, Any]:
        """
        Get statistics about stored trajectories.
        
        Returns:
            Dictionary with statistics:
            - total_count: Total number of trajectories
            - used_in_training: Count used in training
            - unused: Count not yet used
            - avg_reward: Average reward
            - min_reward: Minimum reward
            - max_reward: Maximum reward
            - models: Dict of counts per model
        """
        pipeline = [
            {
                "$facet": {
                    "total": [{"$count": "count"}],
                    "used": [
                        {"$match": {"used_in_training": True}},
                        {"$count": "count"}
                    ],
                    "unused": [
                        {"$match": {"used_in_training": False}},
                        {"$count": "count"}
                    ],
                    "reward_stats": [
                        {
                            "$group": {
                                "_id": None,
                                "avg": {"$avg": "$reward"},
                                "min": {"$min": "$reward"},
                                "max": {"$max": "$reward"},
                                "sum": {"$sum": "$reward"}
                            }
                        }
                    ],
                    "by_model": [
                        {
                            "$group": {
                                "_id": "$model_name",
                                "count": {"$sum": 1}
                            }
                        }
                    ]
                }
            }
        ]
        
        result = await self.collection.aggregate(pipeline).to_list(1)
        
        if not result:
            return {
                "total_count": 0,
                "used_in_training": 0,
                "unused": 0,
                "avg_reward": 0.0,
                "min_reward": 0.0,
                "max_reward": 0.0,
                "total_reward": 0.0,
                "models": {},
                "connected": self._connected
            }
        
        data = result[0]
        
        total = data["total"][0]["count"] if data["total"] else 0
        used = data["used"][0]["count"] if data["used"] else 0
        unused = data["unused"][0]["count"] if data["unused"] else 0
        
        reward_stats = data["reward_stats"][0] if data["reward_stats"] else {}
        
        models = {
            item["_id"]: item["count"] 
            for item in data["by_model"] 
            if item["_id"]
        }
        
        return {
            "total_count": total,
            "used_in_training": used,
            "unused": unused,
            "avg_reward": reward_stats.get("avg", 0.0) or 0.0,
            "min_reward": reward_stats.get("min", 0.0) or 0.0,
            "max_reward": reward_stats.get("max", 0.0) or 0.0,
            "total_reward": reward_stats.get("sum", 0.0) or 0.0,
            "models": models,
            "connected": self._connected
        }
    
    async def cleanup_old_trajectories(
        self,
        days_old: int = 30,
        keep_min: int = 1000
    ) -> int:
        """
        Remove old trajectories to manage storage.
        
        Args:
            days_old: Delete trajectories older than this
            keep_min: Keep at least this many trajectories
            
        Returns:
            Number of deleted trajectories
        """
        from datetime import timedelta
        
        # Get total count
        total = await self.collection.count_documents({})
        
        if total <= keep_min:
            logger.info(f"Cleanup skipped: only {total} trajectories (min: {keep_min})")
            return 0
        
        cutoff_date = datetime.utcnow() - timedelta(days=days_old)
        
        # Delete old trajectories, but respect minimum
        to_delete = total - keep_min
        
        result = await self.collection.delete_many({
            "created_at": {"$lt": cutoff_date},
            "used_in_training": True  # Only delete used ones
        })
        
        deleted = min(result.deleted_count, to_delete)
        logger.info(f"Cleaned up {deleted} old trajectories")
        
        return deleted
    
    async def health_check(self) -> Dict[str, Any]:
        """
        Perform health check on MongoDB connection.
        
        Returns:
            Health status dictionary
        """
        try:
            if not self._client:
                return {"status": "disconnected", "error": "No client"}
            
            await self._client.admin.command('ping')
            
            return {
                "status": "healthy",
                "connected": True,
                "database": self.database_name,
                "collection": self.collection_name
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "connected": False,
                "error": str(e)
            }


class MongoDBClient:
    """
    Convenience wrapper for TrajectoryRepository.
    
    Provides simplified interface for common operations.
    """
    
    def __init__(
        self,
        mongodb_url: Optional[str] = None,
        database_name: str = DATABASE_NAME,
    ):
        self.repository = TrajectoryRepository(
            mongodb_url=mongodb_url,
            database_name=database_name
        )
    
    async def connect(self) -> bool:
        """Connect to MongoDB"""
        return await self.repository.connect()
    
    async def close(self):
        """Close connection"""
        await self.repository.close()
    
    async def store_trajectory(
        self, 
        trajectory: EvolutionTrajectory,
        model_name: str = "default"
    ) -> str:
        """Store trajectory"""
        return await self.repository.store_trajectory(trajectory, model_name)
    
    async def get_trajectory(self, trajectory_id: str) -> EvolutionTrajectory:
        """Get trajectory by ID"""
        return await self.repository.get_trajectory(trajectory_id)
    
    async def list_trajectories(
        self,
        filter_params: Optional[Dict[str, Any]] = None,
        limit: int = 100,
        offset: int = 0
    ) -> List[EvolutionTrajectory]:
        """List trajectories"""
        return await self.repository.list_trajectories(filter_params, limit, offset)
    
    async def delete_trajectory(self, trajectory_id: str) -> bool:
        """Delete trajectory"""
        return await self.repository.delete_trajectory(trajectory_id)
    
    async def get_training_batch(
        self,
        batch_size: int = 32,
        min_reward: float = 0.0
    ) -> List[EvolutionTrajectory]:
        """Get training batch"""
        return await self.repository.get_training_batch(batch_size, min_reward)
    
    async def update_trajectory_status(
        self,
        trajectory_id: str,
        used_in_training: bool = True
    ) -> bool:
        """Update trajectory status"""
        return await self.repository.update_trajectory_status(trajectory_id, used_in_training)
    
    async def get_statistics(self) -> Dict[str, Any]:
        """Get statistics"""
        return await self.repository.get_statistics()
    
    async def health_check(self) -> Dict[str, Any]:
        """Health check"""
        return await self.repository.health_check()


# Export public API
__all__ = [
    "MongoDBClient",
    "TrajectoryRepository",
    "MongoDBConnectionError",
    "TrajectoryNotFoundError",
    "DATABASE_NAME",
    "COLLECTION_NAME",
]
