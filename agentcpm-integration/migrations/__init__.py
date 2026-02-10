"""
MongoDB Migrations
==================

Migration scripts for MongoDB database setup and maintenance.

Scripts:
- init_indexes.py: Create indexes on the trajectories collection
- mongo-init.js: MongoDB initialization script (runs on container startup)
"""

from .init_indexes import run_migration, INDEXES, DATABASE_NAME, COLLECTION_NAME

__all__ = [
    "run_migration",
    "INDEXES",
    "DATABASE_NAME",
    "COLLECTION_NAME",
]
