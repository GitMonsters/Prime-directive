// Compute Node for Prime-directive Web3 Integration
// P2P compute node implementation using libp2p
//
// NOTE: This is a minimal implementation stub. 
// Full production implementation would require:
// - libp2p for P2P networking
// - tokio for async runtime
// - GPU resource allocation
// - Result verification
// - Reward claiming
//
// To enable full implementation, add these dependencies to Cargo.toml:
// libp2p = "0.53"
// tokio = { version = "1", features = ["full"] }
// anchor-lang = "0.29"
// solana-sdk = "1.17"

use std::collections::HashMap;
use std::time::SystemTime;

/// Represents a compute node in the network
#[derive(Debug, Clone)]
pub struct ComputeNode {
    pub node_id: String,
    pub endpoint: String,
    pub gpu_count: u32,
    pub available_memory_gb: u64,
    pub tasks_completed: u64,
    pub reputation: f64,
    pub is_active: bool,
}

impl ComputeNode {
    /// Create a new compute node
    pub fn new(node_id: String, endpoint: String, gpu_count: u32, memory_gb: u64) -> Self {
        ComputeNode {
            node_id,
            endpoint,
            gpu_count,
            available_memory_gb: memory_gb,
            tasks_completed: 0,
            reputation: 1.0,
            is_active: true,
        }
    }

    /// Start the compute node
    pub fn start(&mut self) -> Result<(), String> {
        println!("Starting compute node: {}", self.node_id);
        println!("  Endpoint: {}", self.endpoint);
        println!("  GPUs: {}", self.gpu_count);
        println!("  Memory: {} GB", self.available_memory_gb);
        
        self.is_active = true;
        
        // In a full implementation, this would:
        // 1. Connect to libp2p network
        // 2. Register with the blockchain
        // 3. Start listening for tasks
        // 4. Manage GPU resources
        
        Ok(())
    }

    /// Stop the compute node
    pub fn stop(&mut self) -> Result<(), String> {
        println!("Stopping compute node: {}", self.node_id);
        
        self.is_active = false;
        
        // In a full implementation, this would:
        // 1. Disconnect from network
        // 2. Complete pending tasks
        // 3. Withdraw stake
        
        Ok(())
    }

    /// Process a compute task
    pub fn process_task(&mut self, task: &ComputeTask) -> Result<TaskResult, String> {
        if !self.is_active {
            return Err("Node is not active".to_string());
        }

        println!("Processing task: {}", task.task_id);
        
        // In a full implementation, this would:
        // 1. Download task data from IPFS
        // 2. Allocate GPU resources
        // 3. Execute the computation
        // 4. Upload results to IPFS
        // 5. Submit proof to blockchain
        
        // Simulate processing
        let result = TaskResult {
            task_id: task.task_id.clone(),
            result_hash: format!("Qm{}", task.task_id), // Mock IPFS hash
            completed_at: SystemTime::now(),
            success: true,
        };
        
        self.tasks_completed += 1;
        
        Ok(result)
    }

    /// Update reputation based on performance
    pub fn update_reputation(&mut self, delta: f64) {
        self.reputation = (self.reputation + delta).max(0.0).min(5.0);
        println!("Reputation updated for {}: {:.2}", self.node_id, self.reputation);
    }
}

/// Represents a compute task
#[derive(Debug, Clone)]
pub struct ComputeTask {
    pub task_id: String,
    pub task_type: String,
    pub ipfs_hash: String,
    pub reward: f64,
    pub gpu_required: u32,
    pub memory_required_gb: u64,
}

/// Represents the result of a compute task
#[derive(Debug)]
pub struct TaskResult {
    pub task_id: String,
    pub result_hash: String,
    pub completed_at: SystemTime,
    pub success: bool,
}

/// Manages multiple compute nodes
pub struct NodeManager {
    nodes: HashMap<String, ComputeNode>,
}

impl NodeManager {
    pub fn new() -> Self {
        NodeManager {
            nodes: HashMap::new(),
        }
    }

    /// Register a new node
    pub fn register_node(&mut self, node: ComputeNode) {
        println!("Registering node: {}", node.node_id);
        self.nodes.insert(node.node_id.clone(), node);
    }

    /// Get a node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&ComputeNode> {
        self.nodes.get(node_id)
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut ComputeNode> {
        self.nodes.get_mut(node_id)
    }

    /// List all active nodes
    pub fn list_active_nodes(&self) -> Vec<&ComputeNode> {
        self.nodes.values().filter(|n| n.is_active).collect()
    }

    /// Find the best node for a task
    pub fn find_best_node(&self, task: &ComputeTask) -> Option<&ComputeNode> {
        self.nodes
            .values()
            .filter(|n| {
                n.is_active
                    && n.gpu_count >= task.gpu_required
                    && n.available_memory_gb >= task.memory_required_gb
            })
            .max_by(|a, b| a.reputation.partial_cmp(&b.reputation).unwrap())
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_node_creation() {
        let mut node = ComputeNode::new(
            "node_1".to_string(),
            "127.0.0.1:8080".to_string(),
            4,
            64,
        );

        assert!(node.start().is_ok());
        assert!(node.is_active);
        assert_eq!(node.tasks_completed, 0);
    }

    #[test]
    fn test_task_processing() {
        let mut node = ComputeNode::new(
            "node_1".to_string(),
            "127.0.0.1:8080".to_string(),
            4,
            64,
        );

        node.start().unwrap();

        let task = ComputeTask {
            task_id: "task_123".to_string(),
            task_type: "training".to_string(),
            ipfs_hash: "QmXXX".to_string(),
            reward: 1000.0,
            gpu_required: 2,
            memory_required_gb: 32,
        };

        let result = node.process_task(&task);
        assert!(result.is_ok());
        assert_eq!(node.tasks_completed, 1);
    }

    #[test]
    fn test_node_manager() {
        let mut manager = NodeManager::new();

        let node1 = ComputeNode::new(
            "node_1".to_string(),
            "127.0.0.1:8080".to_string(),
            4,
            64,
        );

        let node2 = ComputeNode::new(
            "node_2".to_string(),
            "127.0.0.1:8081".to_string(),
            8,
            128,
        );

        manager.register_node(node1);
        manager.register_node(node2);

        let active = manager.list_active_nodes();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_reputation_update() {
        let mut node = ComputeNode::new(
            "node_1".to_string(),
            "127.0.0.1:8080".to_string(),
            4,
            64,
        );

        assert_eq!(node.reputation, 1.0);

        node.update_reputation(0.5);
        assert_eq!(node.reputation, 1.5);

        node.update_reputation(-0.3);
        assert_eq!(node.reputation, 1.2);
    }
}

fn main() {
    println!("Prime-directive Web3 Compute Node");
    println!("==================================");

    let mut manager = NodeManager::new();

    // Create a compute node
    let mut node = ComputeNode::new(
        "node_local_1".to_string(),
        "127.0.0.1:8080".to_string(),
        4,
        64,
    );

    // Start the node
    if let Err(e) = node.start() {
        eprintln!("Failed to start node: {}", e);
        return;
    }

    // Register with manager
    manager.register_node(node);

    // Create a sample task
    let task = ComputeTask {
        task_id: "task_001".to_string(),
        task_type: "training".to_string(),
        ipfs_hash: "QmSampleHash123".to_string(),
        reward: 1000.0,
        gpu_required: 2,
        memory_required_gb: 32,
    };

    // Find best node
    if let Some(best_node) = manager.find_best_node(&task) {
        println!("\nBest node for task: {}", best_node.node_id);
        println!("  Reputation: {:.2}", best_node.reputation);
        println!("  GPUs: {}", best_node.gpu_count);
    }

    // Process task
    if let Some(node) = manager.get_node_mut("node_local_1") {
        match node.process_task(&task) {
            Ok(result) => {
                println!("\nTask completed successfully!");
                println!("  Task ID: {}", result.task_id);
                println!("  Result hash: {}", result.result_hash);
                println!("  Tasks completed: {}", node.tasks_completed);
            }
            Err(e) => {
                eprintln!("Task failed: {}", e);
            }
        }
    }

    println!("\nCompute node demo completed.");
}
