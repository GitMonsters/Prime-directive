// MongoDB Initialization Script for RustyWorm RL
// ================================================
// This script runs when MongoDB container starts for the first time.
// Creates the database, collection, and initial indexes.

// Switch to the rustyworm_rl database
db = db.getSiblingDB('rustyworm_rl');

// Create the trajectories collection with validation
db.createCollection('trajectories', {
    validator: {
        $jsonSchema: {
            bsonType: 'object',
            required: ['trajectory_id', 'state', 'action', 'observation', 'reward', 'next_state', 'timestamp'],
            properties: {
                trajectory_id: {
                    bsonType: 'string',
                    description: 'Unique trajectory identifier from Rust'
                },
                state: {
                    bsonType: 'object',
                    description: 'AI Profile state'
                },
                action: {
                    bsonType: 'object',
                    description: 'Personality delta action'
                },
                observation: {
                    bsonType: 'object',
                    description: 'Behavior observation'
                },
                reward: {
                    bsonType: 'double',
                    description: 'Reward value'
                },
                next_state: {
                    bsonType: 'object',
                    description: 'Next AI Profile state'
                },
                timestamp: {
                    bsonType: 'date',
                    description: 'Trajectory timestamp'
                },
                used_in_training: {
                    bsonType: 'bool',
                    description: 'Whether used in training'
                },
                importance_weight: {
                    bsonType: 'double',
                    description: 'Importance weight for prioritized replay'
                },
                model_name: {
                    bsonType: 'string',
                    description: 'Name of observed model'
                },
                created_at: {
                    bsonType: 'date',
                    description: 'Document creation time'
                },
                updated_at: {
                    bsonType: 'date',
                    description: 'Last update time'
                }
            }
        }
    },
    validationLevel: 'moderate',
    validationAction: 'warn'
});

print('Created trajectories collection with validation');

// Create indexes
db.trajectories.createIndex(
    { "trajectory_id": 1 },
    { unique: true, name: "idx_trajectory_id" }
);

db.trajectories.createIndex(
    { "timestamp": -1 },
    { name: "idx_timestamp_desc" }
);

db.trajectories.createIndex(
    { "reward": -1 },
    { name: "idx_reward_desc" }
);

db.trajectories.createIndex(
    { "used_in_training": 1 },
    { name: "idx_used_in_training" }
);

db.trajectories.createIndex(
    { "model_name": 1 },
    { name: "idx_model_name" }
);

db.trajectories.createIndex(
    { "used_in_training": 1, "reward": -1 },
    { name: "idx_training_batch" }
);

db.trajectories.createIndex(
    { "created_at": -1 },
    { name: "idx_created_at" }
);

db.trajectories.createIndex(
    { "importance_weight": -1 },
    { name: "idx_importance_weight" }
);

db.trajectories.createIndex(
    { "model_name": 1, "reward": -1 },
    { name: "idx_model_reward" }
);

db.trajectories.createIndex(
    { "model_name": 1, "used_in_training": 1, "reward": -1 },
    { name: "idx_model_training_batch" }
);

print('Created all indexes');

// Create a read-write user for the application
db.createUser({
    user: 'rustyworm',
    pwd: 'rl_trajectories_2024',
    roles: [
        { role: 'readWrite', db: 'rustyworm_rl' }
    ]
});

print('Created application user');

// Insert a test document to verify setup
db.trajectories.insertOne({
    trajectory_id: 'init-test-001',
    state: {
        id: 'init-profile',
        name: 'InitTest',
        speech_pattern: 0.5,
        knowledge_style: 0.5,
        reasoning_style: 0.5,
        creativity: 0.5,
        carefulness: 0.5,
        empathy: 0.5
    },
    action: {
        adjustments: [],
        confidence: 0.5,
        source: 'init'
    },
    observation: {
        query: 'init test',
        response: 'init response',
        patterns: [],
        similarity_to_target: 0.5,
        confidence: 0.5
    },
    reward: 0.5,
    next_state: {
        id: 'init-profile',
        name: 'InitTest',
        speech_pattern: 0.5,
        knowledge_style: 0.5,
        reasoning_style: 0.5,
        creativity: 0.5,
        carefulness: 0.5,
        empathy: 0.5
    },
    timestamp: new Date(),
    used_in_training: true,
    importance_weight: 1.0,
    model_name: 'init',
    created_at: new Date(),
    updated_at: new Date()
});

print('Inserted test document');

// Remove test document
db.trajectories.deleteOne({ trajectory_id: 'init-test-001' });

print('Cleaned up test document');
print('MongoDB initialization complete!');
