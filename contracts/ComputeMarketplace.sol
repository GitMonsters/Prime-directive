// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./PrimeToken.sol";

/**
 * @title ComputeMarketplace
 * @dev Decentralized compute task distribution and verification
 */
contract ComputeMarketplace is Ownable, ReentrancyGuard {
    PrimeToken public primeToken;
    
    struct ComputeNode {
        address operator;
        string endpoint;
        uint256 gpuCount;
        uint256 stakedAmount;
        uint256 tasksCompleted;
        bool active;
    }
    
    struct ComputeTask {
        uint256 id;
        address requester;
        string ipfsHash;
        uint256 reward;
        address assignedNode;
        TaskStatus status;
        uint256 createdAt;
        uint256 completedAt;
    }
    
    enum TaskStatus {
        Pending,
        Assigned,
        Completed,
        Verified,
        Disputed
    }
    
    mapping(address => ComputeNode) public nodes;
    mapping(uint256 => ComputeTask) public tasks;
    uint256 public taskCount;
    
    uint256 public constant MIN_STAKE = 10000 * 10**18; // 10k PRIME minimum stake
    uint256 public constant SLASH_AMOUNT = 1000 * 10**18; // 1k PRIME slash for bad actors
    
    event NodeRegistered(address indexed operator, uint256 gpuCount);
    event NodeDeactivated(address indexed operator);
    event TaskSubmitted(uint256 indexed taskId, address indexed requester, string ipfsHash);
    event TaskAssigned(uint256 indexed taskId, address indexed node);
    event TaskCompleted(uint256 indexed taskId, address indexed node);
    event NodeSlashed(address indexed operator, uint256 amount, string reason);
    
    constructor(address _primeToken) Ownable(msg.sender) {
        primeToken = PrimeToken(_primeToken);
    }
    
    /**
     * @dev Register as a compute node
     */
    function registerNode(string memory endpoint, uint256 gpuCount) external {
        require(
            primeToken.balanceOf(msg.sender) >= MIN_STAKE,
            "Insufficient PRIME for staking"
        );
        require(!nodes[msg.sender].active, "Node already registered");
        
        // Transfer stake to contract
        primeToken.transferFrom(msg.sender, address(this), MIN_STAKE);
        
        nodes[msg.sender] = ComputeNode({
            operator: msg.sender,
            endpoint: endpoint,
            gpuCount: gpuCount,
            stakedAmount: MIN_STAKE,
            tasksCompleted: 0,
            active: true
        });
        
        emit NodeRegistered(msg.sender, gpuCount);
    }
    
    /**
     * @dev Deactivate compute node and withdraw stake
     */
    function deactivateNode() external nonReentrant {
        ComputeNode storage node = nodes[msg.sender];
        require(node.active, "Node not active");
        
        node.active = false;
        uint256 stakeToReturn = node.stakedAmount;
        node.stakedAmount = 0;
        
        primeToken.transfer(msg.sender, stakeToReturn);
        
        emit NodeDeactivated(msg.sender);
    }
    
    /**
     * @dev Submit a compute task
     */
    function submitTask(string memory ipfsHash, uint256 reward) external returns (uint256) {
        require(reward > 0, "Reward must be positive");
        
        // Transfer reward to contract
        primeToken.transferFrom(msg.sender, address(this), reward);
        
        uint256 taskId = taskCount++;
        
        tasks[taskId] = ComputeTask({
            id: taskId,
            requester: msg.sender,
            ipfsHash: ipfsHash,
            reward: reward,
            assignedNode: address(0),
            status: TaskStatus.Pending,
            createdAt: block.timestamp,
            completedAt: 0
        });
        
        emit TaskSubmitted(taskId, msg.sender, ipfsHash);
        return taskId;
    }
    
    /**
     * @dev Assign task to compute node (called by node)
     */
    function claimTask(uint256 taskId) external {
        ComputeNode storage node = nodes[msg.sender];
        require(node.active, "Node not active");
        
        ComputeTask storage task = tasks[taskId];
        require(task.status == TaskStatus.Pending, "Task not available");
        
        task.assignedNode = msg.sender;
        task.status = TaskStatus.Assigned;
        
        emit TaskAssigned(taskId, msg.sender);
    }
    
    /**
     * @dev Complete a task and claim reward
     */
    function completeTask(uint256 taskId, string memory resultIpfsHash) external nonReentrant {
        ComputeTask storage task = tasks[taskId];
        require(task.assignedNode == msg.sender, "Not assigned to you");
        require(task.status == TaskStatus.Assigned, "Task not in progress");
        
        task.status = TaskStatus.Completed;
        task.completedAt = block.timestamp;
        
        // Update node stats
        ComputeNode storage node = nodes[msg.sender];
        node.tasksCompleted++;
        
        // Pay reward
        primeToken.transfer(msg.sender, task.reward);
        
        emit TaskCompleted(taskId, msg.sender);
    }
    
    /**
     * @dev Slash node for bad behavior
     */
    function slashNode(address nodeOperator, string memory reason) external onlyOwner {
        ComputeNode storage node = nodes[nodeOperator];
        require(node.active, "Node not active");
        require(node.stakedAmount >= SLASH_AMOUNT, "Insufficient stake");
        
        node.stakedAmount -= SLASH_AMOUNT;
        
        emit NodeSlashed(nodeOperator, SLASH_AMOUNT, reason);
    }
    
    /**
     * @dev Get task details
     */
    function getTask(uint256 taskId) external view returns (ComputeTask memory) {
        return tasks[taskId];
    }
    
    /**
     * @dev Get node details
     */
    function getNode(address operator) external view returns (ComputeNode memory) {
        return nodes[operator];
    }
}
