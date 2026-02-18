"""
Integration tests for Web3 components
"""

import pytest
import sys
import os

# Add parent directory to path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from web3.tokenomics import PrimeTokenomics
from web3.task_distributor import TaskDistributor, TaskStatus
from web3.zkproof_verifier import ZKProofVerifier


class TestTokenomics:
    """Test PRIME token economics"""
    
    def test_token_distribution(self):
        """Test initial token distribution"""
        tokenomics = PrimeTokenomics()
        allocation = tokenomics.get_token_allocation()
        
        # Verify total adds up to 1 billion
        total = sum(allocation.values())
        assert total == 1_000_000_000
        
        # Verify percentages
        assert allocation['community_rewards'] == 400_000_000  # 40%
        assert allocation['research_grants'] == 250_000_000    # 25%
        assert allocation['core_team'] == 200_000_000          # 20%
    
    def test_compute_rewards(self):
        """Test compute reward calculation"""
        tokenomics = PrimeTokenomics()
        
        reward = tokenomics.calculate_compute_reward(10)  # 10 GPU hours
        assert reward == 10_000  # 10 * 1000
    
    def test_staking(self):
        """Test token staking"""
        tokenomics = PrimeTokenomics()
        
        # Stake tokens
        assert tokenomics.stake_tokens("0x123", 10000)
        assert tokenomics.staked_tokens["0x123"] == 10000
        
        # Unstake
        assert tokenomics.unstake_tokens("0x123", 5000)
        assert tokenomics.staked_tokens["0x123"] == 5000


class TestTaskDistributor:
    """Test distributed task system"""
    
    def test_node_registration(self):
        """Test compute node registration"""
        distributor = TaskDistributor()
        
        distributor.register_node("node_1", {"gpu_count": 4, "ram_gb": 64})
        assert len(distributor.available_nodes) == 1
    
    def test_task_creation(self):
        """Test task creation"""
        distributor = TaskDistributor()
        
        task = distributor.create_task(
            task_type="training",
            ipfs_hash="QmXXX",
            reward=1000.0
        )
        
        assert task.task_id is not None
        assert task.status == TaskStatus.PENDING
        assert task.reward == 1000.0
    
    def test_task_assignment(self):
        """Test task assignment to nodes"""
        distributor = TaskDistributor()
        
        # Register node
        distributor.register_node("node_1", {"gpu_count": 4})
        
        # Create task
        task = distributor.create_task(
            task_type="training",
            ipfs_hash="QmXXX",
            reward=1000.0
        )
        
        # Assign task
        assert distributor.assign_task(task.task_id, "node_1")
        assert task.status == TaskStatus.ASSIGNED
        assert task.assigned_node == "node_1"
    
    def test_task_completion(self):
        """Test task completion workflow"""
        distributor = TaskDistributor()
        
        distributor.register_node("node_1", {"gpu_count": 4})
        
        task = distributor.create_task(
            task_type="training",
            ipfs_hash="QmXXX",
            reward=1000.0
        )
        
        distributor.assign_task(task.task_id)
        assert distributor.complete_task(task.task_id, "QmResult")
        assert task.status == TaskStatus.COMPLETED


class TestZKProofVerifier:
    """Test zero-knowledge proof system"""
    
    def test_training_proof(self):
        """Test training proof generation"""
        verifier = ZKProofVerifier()
        
        training_data = {
            'epochs': 100,
            'accuracy': 0.95,
            'model_hash': 'abc123'
        }
        
        proof_hash, proof_data = verifier.generate_training_proof(training_data)
        
        assert proof_hash is not None
        assert proof_data['type'] == 'training_completion'
        assert proof_data['public_inputs']['final_accuracy'] == 0.95
    
    def test_benchmark_proof(self):
        """Test benchmark proof generation"""
        verifier = ZKProofVerifier()
        
        benchmark_results = {
            'benchmark_type': 'GAIA',
            'score': 0.87,
            'test_count': 100
        }
        
        proof_hash, proof_data = verifier.generate_benchmark_proof(benchmark_results)
        
        assert proof_hash is not None
        assert proof_data['type'] == 'benchmark_authenticity'
        assert proof_data['public_inputs']['score'] == 0.87
    
    def test_proof_verification(self):
        """Test proof verification"""
        verifier = ZKProofVerifier()
        
        training_data = {
            'epochs': 100,
            'accuracy': 0.95,
            'model_hash': 'abc123'
        }
        
        proof_hash, _ = verifier.generate_training_proof(training_data)
        
        # Verify the proof
        assert verifier.verify_proof(proof_hash)
        
        # Verify with public inputs
        assert verifier.verify_proof(
            proof_hash,
            public_inputs={'final_accuracy': 0.95, 'epochs_completed': 100}
        )


def test_integration_workflow():
    """Test complete Web3 integration workflow"""
    # Initialize components
    tokenomics = PrimeTokenomics()
    distributor = TaskDistributor()
    verifier = ZKProofVerifier()
    
    # Register compute node
    distributor.register_node("node_1", {"gpu_count": 4, "ram_gb": 64})
    
    # Create and assign task
    task = distributor.create_task(
        task_type="training",
        ipfs_hash="QmModel123",
        reward=1000.0
    )
    
    assert distributor.assign_task(task.task_id)
    
    # Complete task
    assert distributor.complete_task(task.task_id, "QmResult123")
    
    # Generate proof
    training_data = {
        'epochs': 100,
        'accuracy': 0.95,
        'model_hash': 'QmResult123'
    }
    
    proof_hash, _ = verifier.generate_training_proof(training_data)
    assert verifier.verify_proof(proof_hash)
    
    # Calculate rewards
    rewards = distributor.calculate_rewards(task.task_id)
    assert 'node_1' in rewards or 'verifiers' in rewards


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
