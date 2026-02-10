#!/usr/bin/env python3
"""
Quick Example: AgentRL HTTP Service Integration
================================================

Demonstrates:
1. Starting the service
2. Making predictions
3. Training on trajectories
4. Monitoring statistics
"""

import asyncio
import requests
import json
from datetime import datetime

# Service URL
SERVICE_URL = "http://localhost:8888"

# Sample data
SAMPLE_PROFILE = {
    "id": "ai-gpt4",
    "name": "GPT-4-Like",
    "speech_pattern": 0.7,
    "knowledge_style": 0.8,
    "reasoning_style": 0.75,
    "creativity": 0.6,
    "carefulness": 0.8,
    "empathy": 0.7
}

SAMPLE_OBSERVATION = {
    "query": "Explain machine learning in simple terms",
    "response": "ML is a way for computers to learn from data...",
    "patterns": ["educational", "technical", "clear"],
    "similarity_to_target": 0.88,
    "confidence": 0.92
}


def print_header(title):
    """Print formatted header"""
    print(f"\n{'='*60}")
    print(f"  {title}")
    print(f"{'='*60}\n")


def check_health():
    """Check service health"""
    print_header("1. Health Check")
    
    response = requests.get(f"{SERVICE_URL}/health")
    
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Service Status: {data['status']}")
        print(f"   Version: {data['service_version']}")
        print(f"   Timestamp: {data['timestamp']}")
        return True
    else:
        print(f"❌ Service unavailable (Status: {response.status_code})")
        return False


def predict_delta():
    """Predict personality delta"""
    print_header("2. Predict Delta")
    
    request_data = {
        "profile": SAMPLE_PROFILE,
        "observation": SAMPLE_OBSERVATION
    }
    
    print("Request:")
    print(f"  Profile: {SAMPLE_PROFILE['name']}")
    print(f"  Query: {SAMPLE_OBSERVATION['query']}")
    print(f"  Similarity to target: {SAMPLE_OBSERVATION['similarity_to_target']}")
    
    response = requests.post(
        f"{SERVICE_URL}/predict-delta",
        json=request_data
    )
    
    if response.status_code == 200:
        data = response.json()
        print("\n✅ Prediction successful:")
        print(f"   Confidence: {data['confidence']:.4f}")
        print(f"   Suggested adjustments:")
        for axis, amount in data['delta']['adjustments']:
            direction = "↑" if amount > 0 else "↓"
            print(f"     {direction} {axis}: {amount:+.4f}")
        print(f"   Reasoning: {data['reasoning']}")
        return data['delta']
    else:
        print(f"❌ Prediction failed (Status: {response.status_code})")
        print(f"   Error: {response.text}")
        return None


def store_trajectory(delta):
    """Store trajectory in buffer"""
    print_header("3. Store Trajectory")
    
    trajectory = {
        "id": f"example-traj-{int(datetime.now().timestamp())}",
        "state": SAMPLE_PROFILE,
        "action": delta or {"adjustments": [], "confidence": 0.5, "source": "example"},
        "observation": SAMPLE_OBSERVATION,
        "reward": 0.85,
        "next_state": SAMPLE_PROFILE,
        "timestamp": datetime.utcnow().isoformat(),
        "used_in_training": False,
        "importance_weight": 1.0
    }
    
    response = requests.post(
        f"{SERVICE_URL}/trajectory/store",
        json=trajectory
    )
    
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Trajectory stored:")
        print(f"   ID: {data['id']}")
        print(f"   Timestamp: {data['timestamp']}")
        return trajectory
    else:
        print(f"❌ Storage failed (Status: {response.status_code})")
        return None


def train_model(trajectory):
    """Train model on trajectories"""
    print_header("4. Train Model")
    
    if trajectory is None:
        print("⚠️  No trajectory to train on, creating test trajectory...")
        trajectory = {
            "id": f"test-traj-{int(datetime.now().timestamp())}",
            "state": SAMPLE_PROFILE,
            "action": {"adjustments": [["speech_pattern", 0.1]], "confidence": 0.7, "source": "rl"},
            "observation": SAMPLE_OBSERVATION,
            "reward": 0.8,
            "next_state": SAMPLE_PROFILE,
            "timestamp": datetime.utcnow().isoformat(),
            "used_in_training": False,
            "importance_weight": 1.0
        }
    
    request_data = {
        "trajectories": [trajectory],
        "importance_weights": [1.0],
        "loss_type": "MINIRL"
    }
    
    print(f"Training on 1 trajectory with MINIRL loss...")
    
    response = requests.post(
        f"{SERVICE_URL}/train",
        json=request_data
    )
    
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Training completed:")
        print(f"   Loss: {data['loss']:.6f}")
        print(f"   Time: {data['training_time_ms']}ms")
        print(f"   Trajectories used: {data['num_trajectories_used']}")
        print(f"   Loss type: {data['loss_type']}")
    else:
        print(f"❌ Training failed (Status: {response.status_code})")
        print(f"   Error: {response.text}")


def get_statistics():
    """Get service statistics"""
    print_header("5. Service Statistics")
    
    response = requests.get(f"{SERVICE_URL}/stats")
    
    if response.status_code == 200:
        data = response.json()
        print("✅ Statistics:")
        print(f"   Training runs: {data['training_runs']}")
        print(f"   Average loss: {data['average_loss']:.6f}")
        print(f"   Min loss: {data['min_loss']:.6f}")
        print(f"   Max loss: {data['max_loss']:.6f}")
        print(f"   Last loss: {data['last_loss']:.6f}")
        print(f"   Buffer size: {data['buffer_size']} trajectories")
    else:
        print(f"❌ Failed to get stats (Status: {response.status_code})")


def main():
    """Run example workflow"""
    print("\n" + "="*60)
    print("  AgentRL HTTP Service - Example Workflow")
    print("="*60)
    print("\nThis example demonstrates:")
    print("1. Health check")
    print("2. Delta prediction")
    print("3. Trajectory storage")
    print("4. Model training")
    print("5. Statistics monitoring")
    print("\nNote: Service must be running on localhost:8888")
    
    # 1. Check health
    if not check_health():
        print("\n❌ Service is not running. Start it with:")
        print("   python agentrl_service.py")
        print("   or: docker-compose up agentrl-service")
        return
    
    # 2. Predict delta
    delta = predict_delta()
    
    # 3. Store trajectory
    trajectory = store_trajectory(delta)
    
    # 4. Train model
    train_model(trajectory)
    
    # 5. Get statistics
    get_statistics()
    
    print_header("Workflow Complete")
    print("✅ All operations completed successfully!")
    print("\nFor more information, see README.md")


if __name__ == "__main__":
    try:
        main()
    except requests.exceptions.ConnectionError:
        print("\n❌ Cannot connect to service at http://localhost:8888")
        print("Please start the service first:")
        print("  python agentrl_service.py")
        print("  or: docker-compose up agentrl-service")
    except KeyboardInterrupt:
        print("\n\nExample interrupted by user")
    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()
