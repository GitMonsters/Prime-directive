"""
Web3-Enhanced GAIA Benchmarking

Extends GAIA benchmarking with:
- Distributed test execution
- On-chain result verification
- zkProof generation
- Reward distribution for validators
"""

from typing import Dict, Any, List, Optional
import json

# Import Web3 modules
from web3.task_distributor import TaskDistributor, TaskStatus
from web3.zkproof_verifier import create_benchmark_proof, ZKProofVerifier
from web3.ipfs_storage import IPFSStorage, upload_benchmark_results


class GAIABenchmarkWeb3:
    """
    Web3-enhanced GAIA benchmark system
    
    Features:
    - Distributed test execution across compute network
    - Cryptographic proof generation for results
    - On-chain verification and leaderboard
    - Automatic reward distribution
    """
    
    def __init__(self, enable_web3: bool = True):
        """
        Initialize Web3-enhanced GAIA benchmark
        
        Args:
            enable_web3: Enable Web3 features (default: True)
        """
        self.enable_web3 = enable_web3
        
        if enable_web3:
            try:
                self.task_distributor = TaskDistributor(min_verifications=3)
                self.proof_verifier = ZKProofVerifier()
                self.ipfs = IPFSStorage()
                print("Web3 features enabled for GAIA benchmarks")
            except Exception as e:
                print(f"Warning: Could not initialize Web3 features: {e}")
                self.enable_web3 = False
        
        self.benchmark_results = []
        self.distributed_tasks = []
    
    def run_benchmark(self, test_cases: List[Dict[str, Any]], 
                     distributed: bool = False) -> Dict[str, Any]:
        """
        Run GAIA benchmark tests
        
        Args:
            test_cases: List of GAIA test cases
            distributed: Run on distributed compute network (default: False)
            
        Returns:
            Benchmark results
        """
        if distributed and self.enable_web3:
            return self._run_distributed_benchmark(test_cases)
        else:
            return self._run_local_benchmark(test_cases)
    
    def _run_local_benchmark(self, test_cases: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Run benchmark locally (without distribution)
        
        Args:
            test_cases: List of test cases
            
        Returns:
            Results dictionary
        """
        print(f"Running GAIA benchmark locally with {len(test_cases)} tests...")
        
        # Simulate benchmark execution
        passed = 0
        failed = 0
        
        for i, test_case in enumerate(test_cases):
            # Simulate test execution
            success = (i % 3) != 0  # 2/3 pass rate for demo
            
            if success:
                passed += 1
            else:
                failed += 1
        
        results = {
            'total_tests': len(test_cases),
            'passed': passed,
            'failed': failed,
            'score': passed / len(test_cases) if test_cases else 0,
            'distributed': False
        }
        
        # Store results
        self.benchmark_results.append(results)
        
        # Web3 integration
        if self.enable_web3:
            self._submit_results_on_chain(results)
        
        return results
    
    def _run_distributed_benchmark(self, test_cases: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Run benchmark on distributed compute network
        
        Args:
            test_cases: List of test cases
            
        Returns:
            Results dictionary
        """
        print(f"Running GAIA benchmark distributed across network...")
        print(f"  Total test cases: {len(test_cases)}")
        
        # Split test cases into chunks
        num_splits = min(4, len(test_cases))
        chunk_size = len(test_cases) // num_splits
        
        # Create distributed tasks
        tasks = []
        for i in range(num_splits):
            start_idx = i * chunk_size
            end_idx = start_idx + chunk_size if i < num_splits - 1 else len(test_cases)
            chunk = test_cases[start_idx:end_idx]
            
            # Upload chunk to IPFS (simulated)
            chunk_data = {
                'test_cases': chunk,
                'chunk_index': i,
                'total_chunks': num_splits
            }
            
            # Create task
            task = self.task_distributor.create_task(
                task_type='benchmark',
                ipfs_hash=f"QmChunk{i}",  # Mock IPFS hash
                reward=500.0,  # 500 PRIME per chunk
                requirements={'min_gpu': 1}
            )
            
            tasks.append(task)
            self.distributed_tasks.append(task.task_id)
        
        print(f"  Created {len(tasks)} distributed tasks")
        
        # Assign and execute tasks (simulated)
        results_chunks = []
        for task in tasks:
            # Assign task
            self.task_distributor.assign_task(task.task_id)
            
            # Simulate execution
            chunk_results = {
                'passed': 5,  # Simulated
                'failed': 1,
                'total': 6
            }
            
            results_chunks.append(chunk_results)
            
            # Complete task
            result_hash = f"QmResult{task.task_id}"
            self.task_distributor.complete_task(task.task_id, result_hash)
            
            # Verify
            self.task_distributor.verify_result(task.task_id, "verifier_1", True)
            self.task_distributor.verify_result(task.task_id, "verifier_2", True)
            self.task_distributor.verify_result(task.task_id, "verifier_3", True)
        
        # Aggregate results
        total_passed = sum(r['passed'] for r in results_chunks)
        total_failed = sum(r['failed'] for r in results_chunks)
        total_tests = total_passed + total_failed
        
        results = {
            'total_tests': total_tests,
            'passed': total_passed,
            'failed': total_failed,
            'score': total_passed / total_tests if total_tests > 0 else 0,
            'distributed': True,
            'num_nodes': num_splits,
            'tasks': [t.task_id for t in tasks]
        }
        
        # Store results
        self.benchmark_results.append(results)
        
        # Web3 integration
        if self.enable_web3:
            self._submit_results_on_chain(results)
        
        return results
    
    def _submit_results_on_chain(self, results: Dict[str, Any]):
        """
        Submit benchmark results to blockchain
        
        Args:
            results: Benchmark results
        """
        try:
            # 1. Upload results to IPFS
            print("Uploading results to IPFS...")
            results_hash = upload_benchmark_results(results, self.ipfs)
            
            # 2. Generate zero-knowledge proof
            print("Generating benchmark proof...")
            proof_hash, proof_data = create_benchmark_proof(
                benchmark_type='GAIA',
                score=results['score'],
                test_count=results['total_tests']
            )
            
            # 3. Submit to blockchain (simulated)
            print("Submitting to blockchain...")
            submission = {
                'results_ipfs_hash': results_hash,
                'proof_hash': proof_hash,
                'score': results['score'],
                'verified': True,
                'reward_eligible': True
            }
            
            print(f"Results submitted successfully!")
            print(f"  IPFS hash: {results_hash}")
            print(f"  Proof: {proof_hash}")
            print(f"  Score: {results['score']:.2%}")
            
            return submission
            
        except Exception as e:
            print(f"Warning: On-chain submission failed: {e}")
            return None
    
    def get_leaderboard(self) -> List[Dict[str, Any]]:
        """
        Get GAIA benchmark leaderboard
        
        Returns:
            List of top scores
        """
        # In a full implementation, this would query the blockchain
        
        leaderboard = [
            {'rank': 1, 'score': 0.92, 'address': '0xABC...123'},
            {'rank': 2, 'score': 0.89, 'address': '0xDEF...456'},
            {'rank': 3, 'score': 0.87, 'address': '0xGHI...789'}
        ]
        
        return leaderboard
    
    def claim_benchmark_reward(self, result_index: int = -1) -> float:
        """
        Claim reward for validated benchmark
        
        Args:
            result_index: Index of result to claim (default: latest)
            
        Returns:
            Reward amount in PRIME
        """
        if not self.benchmark_results:
            print("No benchmark results to claim")
            return 0
        
        result = self.benchmark_results[result_index]
        
        # Base reward: 500 PRIME per validated result
        base_reward = 500.0
        
        # Bonus for high scores
        score_bonus = result['score'] * 1000  # Up to 1000 PRIME bonus
        
        # Bonus for distributed execution
        distributed_bonus = 200.0 if result.get('distributed') else 0
        
        total_reward = base_reward + score_bonus + distributed_bonus
        
        print(f"Claiming benchmark reward: {total_reward:.0f} PRIME")
        print(f"  Base reward: {base_reward:.0f}")
        print(f"  Score bonus ({result['score']:.2%}): {score_bonus:.0f}")
        print(f"  Distributed bonus: {distributed_bonus:.0f}")
        
        return total_reward


def main():
    """Example usage of Web3-enhanced GAIA benchmarks"""
    print("GAIA Benchmark - Web3 Integration")
    print("=" * 70)
    
    # Create Web3-enhanced benchmark system
    # Set enable_web3=False if Web3 infrastructure is not available
    gaia = GAIABenchmarkWeb3(enable_web3=False)
    
    # Create sample test cases
    test_cases = [
        {'id': f'test_{i}', 'difficulty': 'medium'}
        for i in range(20)
    ]
    
    # Run local benchmark
    print("\n--- Local Benchmark ---")
    results_local = gaia.run_benchmark(test_cases, distributed=False)
    print(f"\nResults:")
    print(f"  Score: {results_local['score']:.2%}")
    print(f"  Passed: {results_local['passed']}/{results_local['total_tests']}")
    
    # Claim reward
    reward = gaia.claim_benchmark_reward()
    print(f"\nTotal reward: {reward:.0f} PRIME")
    
    # View leaderboard
    print("\n--- Leaderboard ---")
    leaderboard = gaia.get_leaderboard()
    for entry in leaderboard:
        print(f"  #{entry['rank']}: {entry['score']:.2%} ({entry['address']})")


if __name__ == "__main__":
    main()
