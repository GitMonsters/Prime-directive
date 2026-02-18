// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title BenchmarkVerifier
 * @dev On-chain verification of AI benchmarks with cryptographic proofs
 */
contract BenchmarkVerifier is Ownable {
    struct BenchmarkResult {
        uint256 id;
        address submitter;
        string benchmarkType;  // "GAIA", "ARC", "Physics", "Empathy"
        uint256 score;
        string resultIpfsHash;
        bytes32 proofHash;
        uint256 timestamp;
        bool verified;
    }
    
    mapping(uint256 => BenchmarkResult) public results;
    mapping(address => uint256[]) public submitterResults;
    uint256 public resultCount;
    
    // Leaderboard tracking
    mapping(string => uint256) public bestScores;
    mapping(string => address) public bestScoreHolders;
    
    event BenchmarkSubmitted(
        uint256 indexed resultId,
        address indexed submitter,
        string benchmarkType,
        uint256 score
    );
    event BenchmarkVerified(uint256 indexed resultId);
    event NewHighScore(string indexed benchmarkType, address indexed holder, uint256 score);
    
    constructor() Ownable(msg.sender) {}
    
    /**
     * @dev Submit a benchmark result with cryptographic proof
     */
    function submitBenchmark(
        string memory benchmarkType,
        uint256 score,
        string memory resultIpfsHash,
        bytes32 proofHash
    ) external returns (uint256) {
        uint256 resultId = resultCount++;
        
        results[resultId] = BenchmarkResult({
            id: resultId,
            submitter: msg.sender,
            benchmarkType: benchmarkType,
            score: score,
            resultIpfsHash: resultIpfsHash,
            proofHash: proofHash,
            timestamp: block.timestamp,
            verified: false
        });
        
        submitterResults[msg.sender].push(resultId);
        
        emit BenchmarkSubmitted(resultId, msg.sender, benchmarkType, score);
        return resultId;
    }
    
    /**
     * @dev Verify a benchmark result
     */
    function verifyBenchmark(uint256 resultId) external onlyOwner {
        BenchmarkResult storage result = results[resultId];
        require(!result.verified, "Already verified");
        
        result.verified = true;
        
        // Check if this is a new high score
        if (result.score > bestScores[result.benchmarkType]) {
            bestScores[result.benchmarkType] = result.score;
            bestScoreHolders[result.benchmarkType] = result.submitter;
            
            emit NewHighScore(result.benchmarkType, result.submitter, result.score);
        }
        
        emit BenchmarkVerified(resultId);
    }
    
    /**
     * @dev Get benchmark result
     */
    function getResult(uint256 resultId) external view returns (BenchmarkResult memory) {
        return results[resultId];
    }
    
    /**
     * @dev Get all results for a submitter
     */
    function getSubmitterResults(address submitter) external view returns (uint256[] memory) {
        return submitterResults[submitter];
    }
    
    /**
     * @dev Get leaderboard for a benchmark type
     */
    function getLeaderboard(string memory benchmarkType)
        external
        view
        returns (address holder, uint256 score)
    {
        return (bestScoreHolders[benchmarkType], bestScores[benchmarkType]);
    }
    
    /**
     * @dev Verify cryptographic proof
     */
    function verifyProof(
        uint256 resultId,
        bytes32 expectedHash
    ) external view returns (bool) {
        return results[resultId].proofHash == expectedHash;
    }
}
