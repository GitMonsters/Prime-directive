// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title PrimeToken
 * @dev ERC-20 governance token for Prime-directive AGI system
 * Symbol: PRIME
 * Utility: Governance voting, compute payment, staking
 */
contract PrimeToken is ERC20, ERC20Burnable, Ownable {
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18; // 1 billion tokens
    
    // Token distribution addresses
    address public communityRewards;
    address public researchGrants;
    address public coreTeam;
    address public earlySupporters;
    address public liquidityPool;
    
    // Vesting tracking
    mapping(address => uint256) public vestedAmount;
    mapping(address => uint256) public vestedUntil;
    
    event TokensVested(address indexed beneficiary, uint256 amount, uint256 releaseTime);
    event RewardPaid(address indexed recipient, uint256 amount, string reason);
    
    constructor(
        address _communityRewards,
        address _researchGrants,
        address _coreTeam,
        address _earlySupporters,
        address _liquidityPool
    ) ERC20("Prime Token", "PRIME") Ownable(msg.sender) {
        communityRewards = _communityRewards;
        researchGrants = _researchGrants;
        coreTeam = _coreTeam;
        earlySupporters = _earlySupporters;
        liquidityPool = _liquidityPool;
        
        // Initial distribution
        _mint(communityRewards, (MAX_SUPPLY * 40) / 100);  // 40% Community rewards
        _mint(researchGrants, (MAX_SUPPLY * 25) / 100);     // 25% Research grants
        _mint(earlySupporters, (MAX_SUPPLY * 10) / 100);    // 10% Early supporters
        _mint(liquidityPool, (MAX_SUPPLY * 5) / 100);       // 5% Liquidity
        
        // Core team tokens vested for 4 years
        uint256 coreTeamAllocation = (MAX_SUPPLY * 20) / 100;
        _mint(address(this), coreTeamAllocation);
        vestedAmount[coreTeam] = coreTeamAllocation;
        vestedUntil[coreTeam] = block.timestamp + 4 * 365 days;
        
        emit TokensVested(coreTeam, coreTeamAllocation, vestedUntil[coreTeam]);
    }
    
    /**
     * @dev Release vested tokens
     */
    function releaseVestedTokens() external {
        require(vestedAmount[msg.sender] > 0, "No vested tokens");
        require(block.timestamp >= vestedUntil[msg.sender], "Tokens still vested");
        
        uint256 amount = vestedAmount[msg.sender];
        vestedAmount[msg.sender] = 0;
        
        _transfer(address(this), msg.sender, amount);
    }
    
    /**
     * @dev Reward compute providers
     */
    function rewardComputeProvider(address provider, uint256 gpuHours) external onlyOwner {
        uint256 reward = gpuHours * 1000 * 10**18; // 1000 PRIME per GPU-hour
        _transfer(communityRewards, provider, reward);
        emit RewardPaid(provider, reward, "Compute provision");
    }
    
    /**
     * @dev Reward benchmark submission
     */
    function rewardBenchmark(address submitter) external onlyOwner {
        uint256 reward = 500 * 10**18; // 500 PRIME per validated result
        _transfer(communityRewards, submitter, reward);
        emit RewardPaid(submitter, reward, "Benchmark submission");
    }
    
    /**
     * @dev Reward governance participation
     */
    function rewardGovernance(address voter) external onlyOwner {
        uint256 reward = 100 * 10**18; // 100 PRIME per vote
        _transfer(communityRewards, voter, reward);
        emit RewardPaid(voter, reward, "Governance participation");
    }
}
