#!/usr/bin/env python3
"""
MongoDB Index Initialization Script
====================================

Creates indexes on the trajectories collection for optimal query performance.

Usage:
    python migrations/init_indexes.py
    
Environment Variables:
    MONGODB_URL: MongoDB connection string (default: mongodb://root:password@localhost:27017)

Indexes Created:
    - idx_timestamp_desc: timestamp (descending) - for time-based queries
    - idx_reward_desc: reward (descending) - for batch selection
    - idx_used_in_training: used_in_training - for filtering
    - idx_model_name: model_name - for per-model queries
    - idx_training_batch: compound (used_in_training, reward) - for training batch
    - idx_trajectory_id: trajectory_id (unique) - for deduplication
    - idx_created_at: created_at (descending) - for cleanup operations
"""

import asyncio
import logging
import os
import sys
from datetime import datetime

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from motor.motor_asyncio import AsyncIOMotorClient
from pymongo import DESCENDING, ASCENDING, IndexModel
from pymongo.errors import OperationFailure, CollectionInvalid

# Configuration
DEFAULT_MONGODB_URL = "mongodb://root:password@localhost:27017"
DATABASE_NAME = "rustyworm_rl"
COLLECTION_NAME = "trajectories"

# Logging setup
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


# Index definitions
INDEXES = [
    # Timestamp descending - for time-based queries
    IndexModel(
        [("timestamp", DESCENDING)],
        name="idx_timestamp_desc",
        background=True
    ),
    
    # Reward descending - for batch selection by reward
    IndexModel(
        [("reward", DESCENDING)],
        name="idx_reward_desc",
        background=True
    ),
    
    # Used in training - for filtering unused trajectories
    IndexModel(
        [("used_in_training", ASCENDING)],
        name="idx_used_in_training",
        background=True
    ),
    
    # Model name - for per-model queries
    IndexModel(
        [("model_name", ASCENDING)],
        name="idx_model_name",
        background=True
    ),
    
    # Compound index: used_in_training + reward for training batch selection
    # This is the most critical index for training performance
    IndexModel(
        [("used_in_training", ASCENDING), ("reward", DESCENDING)],
        name="idx_training_batch",
        background=True
    ),
    
    # Trajectory ID - unique for deduplication
    IndexModel(
        [("trajectory_id", ASCENDING)],
        unique=True,
        name="idx_trajectory_id",
        background=True
    ),
    
    # Created at - for cleanup operations
    IndexModel(
        [("created_at", DESCENDING)],
        name="idx_created_at",
        background=True
    ),
    
    # Importance weight - for prioritized sampling
    IndexModel(
        [("importance_weight", DESCENDING)],
        name="idx_importance_weight",
        background=True
    ),
    
    # Compound: model_name + reward - for per-model batch selection
    IndexModel(
        [("model_name", ASCENDING), ("reward", DESCENDING)],
        name="idx_model_reward",
        background=True
    ),
    
    # Compound: model_name + used_in_training + reward
    # Optimal for training batch queries filtered by model
    IndexModel(
        [
            ("model_name", ASCENDING),
            ("used_in_training", ASCENDING),
            ("reward", DESCENDING)
        ],
        name="idx_model_training_batch",
        background=True
    ),
]


async def create_collection_if_not_exists(db, collection_name: str) -> bool:
    """
    Create collection if it doesn't exist.
    
    Args:
        db: MongoDB database object
        collection_name: Name of collection to create
        
    Returns:
        True if created or already exists
    """
    try:
        collections = await db.list_collection_names()
        
        if collection_name not in collections:
            await db.create_collection(collection_name)
            logger.info(f"Created collection: {collection_name}")
        else:
            logger.info(f"Collection already exists: {collection_name}")
        
        return True
    except CollectionInvalid as e:
        logger.warning(f"Collection creation warning: {e}")
        return True
    except Exception as e:
        logger.error(f"Failed to create collection: {e}")
        return False


async def drop_existing_indexes(collection) -> int:
    """
    Drop all non-default indexes.
    
    Args:
        collection: MongoDB collection
        
    Returns:
        Number of indexes dropped
    """
    try:
        # Get current indexes
        indexes = await collection.index_information()
        
        dropped = 0
        for name in indexes:
            if name != "_id_":  # Don't drop the default _id index
                try:
                    await collection.drop_index(name)
                    logger.info(f"Dropped index: {name}")
                    dropped += 1
                except Exception as e:
                    logger.warning(f"Failed to drop index {name}: {e}")
        
        return dropped
    except Exception as e:
        logger.error(f"Error dropping indexes: {e}")
        return 0


async def create_indexes(collection, indexes: list) -> tuple:
    """
    Create indexes on collection.
    
    Args:
        collection: MongoDB collection
        indexes: List of IndexModel objects
        
    Returns:
        Tuple of (created_count, failed_count)
    """
    created = 0
    failed = 0
    
    for index in indexes:
        try:
            await collection.create_indexes([index])
            logger.info(f"Created index: {index.document.get('name', 'unnamed')}")
            created += 1
        except OperationFailure as e:
            if "already exists" in str(e):
                logger.info(f"Index already exists: {index.document.get('name', 'unnamed')}")
                created += 1
            else:
                logger.error(f"Failed to create index: {e}")
                failed += 1
        except Exception as e:
            logger.error(f"Error creating index: {e}")
            failed += 1
    
    return created, failed


async def verify_indexes(collection) -> dict:
    """
    Verify all expected indexes exist.
    
    Args:
        collection: MongoDB collection
        
    Returns:
        Dictionary with index verification results
    """
    try:
        indexes = await collection.index_information()
        
        expected_names = {idx.document.get('name') for idx in INDEXES}
        actual_names = set(indexes.keys()) - {"_id_"}
        
        missing = expected_names - actual_names
        extra = actual_names - expected_names
        
        return {
            "expected": len(expected_names),
            "actual": len(actual_names),
            "missing": list(missing),
            "extra": list(extra),
            "verified": len(missing) == 0
        }
    except Exception as e:
        logger.error(f"Error verifying indexes: {e}")
        return {"error": str(e)}


async def run_migration(
    mongodb_url: str = None,
    database_name: str = DATABASE_NAME,
    collection_name: str = COLLECTION_NAME,
    drop_existing: bool = False
) -> dict:
    """
    Run the index migration.
    
    Args:
        mongodb_url: MongoDB connection string
        database_name: Target database name
        collection_name: Target collection name
        drop_existing: Whether to drop existing indexes first
        
    Returns:
        Migration result dictionary
    """
    url = mongodb_url or os.getenv("MONGODB_URL", DEFAULT_MONGODB_URL)
    
    logger.info("=" * 60)
    logger.info("MongoDB Index Migration")
    logger.info("=" * 60)
    logger.info(f"Database: {database_name}")
    logger.info(f"Collection: {collection_name}")
    logger.info(f"Indexes to create: {len(INDEXES)}")
    logger.info("=" * 60)
    
    result = {
        "started_at": datetime.utcnow().isoformat(),
        "database": database_name,
        "collection": collection_name,
        "success": False,
        "indexes_created": 0,
        "indexes_failed": 0,
        "dropped": 0
    }
    
    try:
        # Connect to MongoDB
        logger.info("Connecting to MongoDB...")
        client = AsyncIOMotorClient(url, serverSelectionTimeoutMS=5000)
        
        # Verify connection
        await client.admin.command('ping')
        logger.info("Connected successfully")
        
        db = client[database_name]
        
        # Create collection if needed
        await create_collection_if_not_exists(db, collection_name)
        
        collection = db[collection_name]
        
        # Optionally drop existing indexes
        if drop_existing:
            logger.info("Dropping existing indexes...")
            result["dropped"] = await drop_existing_indexes(collection)
        
        # Create indexes
        logger.info("Creating indexes...")
        created, failed = await create_indexes(collection, INDEXES)
        result["indexes_created"] = created
        result["indexes_failed"] = failed
        
        # Verify
        logger.info("Verifying indexes...")
        verification = await verify_indexes(collection)
        result["verification"] = verification
        
        # Close connection
        client.close()
        
        result["success"] = verification.get("verified", False)
        result["completed_at"] = datetime.utcnow().isoformat()
        
        # Summary
        logger.info("=" * 60)
        logger.info("Migration Summary")
        logger.info("=" * 60)
        logger.info(f"Indexes created: {created}")
        logger.info(f"Indexes failed: {failed}")
        if drop_existing:
            logger.info(f"Indexes dropped: {result['dropped']}")
        logger.info(f"Verification: {'PASSED' if result['success'] else 'FAILED'}")
        
        if verification.get("missing"):
            logger.warning(f"Missing indexes: {verification['missing']}")
        
        logger.info("=" * 60)
        
        return result
        
    except Exception as e:
        logger.error(f"Migration failed: {e}")
        result["error"] = str(e)
        return result


def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Initialize MongoDB indexes for trajectory collection"
    )
    parser.add_argument(
        "--drop-existing",
        action="store_true",
        help="Drop existing indexes before creating new ones"
    )
    parser.add_argument(
        "--mongodb-url",
        type=str,
        default=None,
        help="MongoDB connection URL"
    )
    parser.add_argument(
        "--database",
        type=str,
        default=DATABASE_NAME,
        help=f"Database name (default: {DATABASE_NAME})"
    )
    parser.add_argument(
        "--collection",
        type=str,
        default=COLLECTION_NAME,
        help=f"Collection name (default: {COLLECTION_NAME})"
    )
    
    args = parser.parse_args()
    
    result = asyncio.run(run_migration(
        mongodb_url=args.mongodb_url,
        database_name=args.database,
        collection_name=args.collection,
        drop_existing=args.drop_existing
    ))
    
    if result.get("success"):
        logger.info("Migration completed successfully!")
        sys.exit(0)
    else:
        logger.error("Migration failed!")
        sys.exit(1)


if __name__ == "__main__":
    main()
