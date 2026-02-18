"""
Distributed Task Distribution for Prime-directive Web3

Distributes AI training/inference tasks across the compute network.
Handles task splitting, assignment, result aggregation, and reward distribution.
"""

from typing import Dict, Any, List, Optional, Callable
import json
import hashlib
from datetime import datetime
from enum import Enum
import time


class TaskStatus(Enum):
    """Task status enumeration"""
    PENDING = "pending"
    ASSIGNED = "assigned"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    VERIFIED = "verified"
    FAILED = "failed"


class ComputeTask:
    """Represents a single compute task"""
    
    def __init__(self, task_id: str, task_type: str, ipfs_hash: str,
                 reward: float, requirements: Optional[Dict[str, Any]] = None):
        self.task_id = task_id
        self.task_type = task_type  # "training", "inference", "benchmark"
        self.ipfs_hash = ipfs_hash
        self.reward = reward
        self.requirements = requirements or {}
        self.status = TaskStatus.PENDING
        self.assigned_node: Optional[str] = None
        self.created_at = datetime.now().isoformat()
        self.completed_at: Optional[str] = None
        self.result_hash: Optional[str] = None
        self.verification_count = 0
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return {
            'task_id': self.task_id,
            'task_type': self.task_type,
            'ipfs_hash': self.ipfs_hash,
            'reward': self.reward,
            'requirements': self.requirements,
            'status': self.status.value,
            'assigned_node': self.assigned_node,
            'created_at': self.created_at,
            'completed_at': self.completed_at,
            'result_hash': self.result_hash,
            'verification_count': self.verification_count
        }


class TaskDistributor:
    """Manages distribution of compute tasks across the network"""
    
    def __init__(self, min_verifications: int = 3):
        """
        Initialize task distributor
        
        Args:
            min_verifications: Minimum number of nodes to verify results
        """
        self.tasks: Dict[str, ComputeTask] = {}
        self.available_nodes: List[Dict[str, Any]] = []
        self.min_verifications = min_verifications
        self.task_counter = 0
    
    def register_node(self, node_id: str, capabilities: Dict[str, Any]):
        """
        Register a compute node
        
        Args:
            node_id: Unique node identifier
            capabilities: Node capabilities (GPU count, RAM, etc.)
        """
        node_info = {
            'node_id': node_id,
            'capabilities': capabilities,
            'registered_at': datetime.now().isoformat(),
            'tasks_completed': 0,
            'reputation': 1.0
        }
        
        self.available_nodes.append(node_info)
        print(f"Node registered: {node_id}")
    
    def create_task(self, task_type: str, ipfs_hash: str, reward: float,
                   requirements: Optional[Dict[str, Any]] = None) -> ComputeTask:
        """
        Create a new compute task
        
        Args:
            task_type: Type of task (training, inference, benchmark)
            ipfs_hash: IPFS hash of task data
            reward: Reward in PRIME tokens
            requirements: Hardware/software requirements
            
        Returns:
            Created ComputeTask
        """
        task_id = self._generate_task_id()
        
        task = ComputeTask(
            task_id=task_id,
            task_type=task_type,
            ipfs_hash=ipfs_hash,
            reward=reward,
            requirements=requirements
        )
        
        self.tasks[task_id] = task
        print(f"Task created: {task_id} ({task_type})")
        
        return task
    
    def split_large_job(self, job_data: Dict[str, Any],
                       num_splits: int = 4) -> List[ComputeTask]:
        """
        Split a large job into smaller tasks
        
        Args:
            job_data: Job data to split
            num_splits: Number of sub-tasks to create
            
        Returns:
            List of created tasks
        """
        tasks = []
        base_reward = job_data.get('reward', 1000) / num_splits
        
        for i in range(num_splits):
            sub_task_data = {
                'parent_job': job_data.get('job_id'),
                'split_index': i,
                'total_splits': num_splits,
                'data': job_data.get('data', {})
            }
            
            # In a real implementation, this would upload to IPFS
            ipfs_hash = self._mock_ipfs_upload(sub_task_data)
            
            task = self.create_task(
                task_type=job_data.get('task_type', 'training'),
                ipfs_hash=ipfs_hash,
                reward=base_reward,
                requirements=job_data.get('requirements')
            )
            
            tasks.append(task)
        
        return tasks
    
    def assign_task(self, task_id: str, node_id: Optional[str] = None) -> bool:
        """
        Assign a task to a compute node
        
        Args:
            task_id: Task identifier
            node_id: Node identifier (auto-select if None)
            
        Returns:
            True if assignment successful
        """
        task = self.tasks.get(task_id)
        if not task or task.status != TaskStatus.PENDING:
            return False
        
        # Auto-select node if not specified
        if node_id is None:
            node_id = self._select_best_node(task)
        
        if not node_id:
            print(f"No suitable node found for task {task_id}")
            return False
        
        task.assigned_node = node_id
        task.status = TaskStatus.ASSIGNED
        
        print(f"Task {task_id} assigned to node {node_id}")
        return True
    
    def complete_task(self, task_id: str, result_hash: str) -> bool:
        """
        Mark a task as completed
        
        Args:
            task_id: Task identifier
            result_hash: IPFS hash of the result
            
        Returns:
            True if successful
        """
        task = self.tasks.get(task_id)
        if not task or task.status != TaskStatus.ASSIGNED:
            return False
        
        task.status = TaskStatus.COMPLETED
        task.result_hash = result_hash
        task.completed_at = datetime.now().isoformat()
        
        print(f"Task {task_id} completed with result {result_hash}")
        return True
    
    def verify_result(self, task_id: str, verifier_node_id: str,
                     is_valid: bool) -> bool:
        """
        Verify a task result
        
        Args:
            task_id: Task identifier
            verifier_node_id: Node performing verification
            is_valid: Whether result is valid
            
        Returns:
            True if verification recorded
        """
        task = self.tasks.get(task_id)
        if not task or task.status != TaskStatus.COMPLETED:
            return False
        
        task.verification_count += 1 if is_valid else 0
        
        # If minimum verifications reached, mark as verified
        if task.verification_count >= self.min_verifications:
            task.status = TaskStatus.VERIFIED
            print(f"Task {task_id} verified by network")
        
        return True
    
    def aggregate_results(self, parent_job_id: str) -> Optional[Dict[str, Any]]:
        """
        Aggregate results from split tasks
        
        Args:
            parent_job_id: Parent job identifier
            
        Returns:
            Aggregated results or None
        """
        # Find all tasks for this job
        job_tasks = [
            task for task in self.tasks.values()
            if task.task_type == parent_job_id or
            (hasattr(task, 'parent_job') and task.parent_job == parent_job_id)
        ]
        
        if not job_tasks:
            return None
        
        # Check all tasks are verified
        all_verified = all(task.status == TaskStatus.VERIFIED for task in job_tasks)
        
        if not all_verified:
            print(f"Not all tasks verified for job {parent_job_id}")
            return None
        
        # Aggregate results
        aggregated = {
            'parent_job': parent_job_id,
            'num_tasks': len(job_tasks),
            'results': [task.result_hash for task in job_tasks],
            'total_reward': sum(task.reward for task in job_tasks),
            'aggregated_at': datetime.now().isoformat()
        }
        
        return aggregated
    
    def get_task_status(self, task_id: str) -> Optional[Dict[str, Any]]:
        """
        Get current status of a task
        
        Args:
            task_id: Task identifier
            
        Returns:
            Task status dictionary or None
        """
        task = self.tasks.get(task_id)
        return task.to_dict() if task else None
    
    def list_pending_tasks(self) -> List[ComputeTask]:
        """
        List all pending tasks
        
        Returns:
            List of pending tasks
        """
        return [
            task for task in self.tasks.values()
            if task.status == TaskStatus.PENDING
        ]
    
    def calculate_rewards(self, task_id: str) -> Dict[str, float]:
        """
        Calculate reward distribution for a verified task
        
        Args:
            task_id: Task identifier
            
        Returns:
            Dictionary mapping node_id to reward amount
        """
        task = self.tasks.get(task_id)
        if not task or task.status != TaskStatus.VERIFIED:
            return {}
        
        # 90% to compute node, 10% split among verifiers
        compute_reward = task.reward * 0.9
        verifier_reward = task.reward * 0.1 / self.min_verifications
        
        rewards = {}
        if task.assigned_node:
            rewards[task.assigned_node] = compute_reward
        
        # In a real system, we'd track verifiers
        # For now, just show the structure
        rewards['verifiers'] = verifier_reward
        
        return rewards
    
    def _generate_task_id(self) -> str:
        """Generate unique task ID"""
        self.task_counter += 1
        data = f"task_{self.task_counter}_{datetime.now().isoformat()}"
        return hashlib.sha256(data.encode()).hexdigest()[:16]
    
    def _select_best_node(self, task: ComputeTask) -> Optional[str]:
        """Select best node for a task based on capabilities and reputation"""
        if not self.available_nodes:
            return None
        
        # Simple selection: highest reputation node
        best_node = max(self.available_nodes, key=lambda n: n['reputation'])
        return best_node['node_id']
    
    def _mock_ipfs_upload(self, data: Dict[str, Any]) -> str:
        """Mock IPFS upload for demonstration"""
        data_str = json.dumps(data, sort_keys=True)
        return hashlib.sha256(data_str.encode()).hexdigest()


if __name__ == "__main__":
    # Example usage
    print("Task Distributor Example")
    print("-" * 50)
    
    distributor = TaskDistributor(min_verifications=3)
    
    # Register some nodes
    distributor.register_node("node_1", {"gpu_count": 4, "ram_gb": 64})
    distributor.register_node("node_2", {"gpu_count": 8, "ram_gb": 128})
    
    # Create a task
    task = distributor.create_task(
        task_type="training",
        ipfs_hash="QmXxxx123",
        reward=1000.0,
        requirements={"min_gpu": 4}
    )
    
    # Assign and complete
    distributor.assign_task(task.task_id)
    distributor.complete_task(task.task_id, "QmResultXxxx")
    
    # Verify
    distributor.verify_result(task.task_id, "node_1", True)
    distributor.verify_result(task.task_id, "node_2", True)
    distributor.verify_result(task.task_id, "node_3", True)
    
    # Calculate rewards
    rewards = distributor.calculate_rewards(task.task_id)
    print(f"\nRewards: {rewards}")
    
    print(f"\nTask status: {distributor.get_task_status(task.task_id)}")
