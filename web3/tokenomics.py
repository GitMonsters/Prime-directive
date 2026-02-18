"""
Token Economics for Prime-directive Web3 Integration

Defines PRIME token utility, distribution, and earning mechanisms.
"""

from typing import Dict, Any, List, Optional
from enum import Enum
from datetime import datetime, timedelta


class TokenDistribution(Enum):
    """Token distribution categories"""
    COMMUNITY_REWARDS = 0.40  # 40% Community rewards (compute providers)
    RESEARCH_GRANTS = 0.25    # 25% Research grants (DAO controlled)
    CORE_TEAM = 0.20          # 20% Core team (4-year vest)
    EARLY_SUPPORTERS = 0.10   # 10% Early supporters
    LIQUIDITY = 0.05          # 5% Liquidity


class PrimeTokenomics:
    """
    PRIME Token Economics System
    
    Total Supply: 1,000,000,000 PRIME
    
    Utility:
    1. Governance voting weight
    2. Compute marketplace payment
    3. Staking for node operators
    4. Research grant funding
    
    Earning Mechanisms:
    - Provide compute: 1000 PRIME per GPU-hour
    - Submit benchmarks: 500 PRIME per validated result
    - Improve models: NFT royalties in PRIME
    - Governance participation: 100 PRIME per vote
    """
    
    TOTAL_SUPPLY = 1_000_000_000  # 1 billion tokens
    DECIMALS = 18
    
    # Reward rates (in PRIME)
    COMPUTE_REWARD_PER_GPU_HOUR = 1000
    BENCHMARK_REWARD = 500
    GOVERNANCE_REWARD = 100
    MODEL_IMPROVEMENT_BASE_REWARD = 2000
    
    # Vesting parameters
    CORE_TEAM_VESTING_PERIOD_DAYS = 4 * 365  # 4 years
    
    def __init__(self):
        self.allocated_tokens: Dict[str, float] = {}
        self.earned_tokens: Dict[str, float] = {}
        self.staked_tokens: Dict[str, float] = {}
        self.vesting_schedules: Dict[str, Dict[str, Any]] = {}
        
        self._initialize_distribution()
    
    def _initialize_distribution(self):
        """Initialize token distribution"""
        self.allocated_tokens = {
            'community_rewards': self.TOTAL_SUPPLY * TokenDistribution.COMMUNITY_REWARDS.value,
            'research_grants': self.TOTAL_SUPPLY * TokenDistribution.RESEARCH_GRANTS.value,
            'core_team': self.TOTAL_SUPPLY * TokenDistribution.CORE_TEAM.value,
            'early_supporters': self.TOTAL_SUPPLY * TokenDistribution.EARLY_SUPPORTERS.value,
            'liquidity': self.TOTAL_SUPPLY * TokenDistribution.LIQUIDITY.value
        }
        
        print("Token distribution initialized:")
        for category, amount in self.allocated_tokens.items():
            percentage = (amount / self.TOTAL_SUPPLY) * 100
            print(f"  {category}: {amount:,.0f} PRIME ({percentage}%)")
    
    def calculate_compute_reward(self, gpu_hours: float) -> float:
        """
        Calculate reward for providing compute
        
        Args:
            gpu_hours: Number of GPU hours provided
            
        Returns:
            Reward amount in PRIME
        """
        reward = gpu_hours * self.COMPUTE_REWARD_PER_GPU_HOUR
        return reward
    
    def calculate_benchmark_reward(self, is_validated: bool = True) -> float:
        """
        Calculate reward for benchmark submission
        
        Args:
            is_validated: Whether the benchmark is validated
            
        Returns:
            Reward amount in PRIME
        """
        if not is_validated:
            return 0
        
        return self.BENCHMARK_REWARD
    
    def calculate_model_improvement_reward(self, improvement_score: float) -> float:
        """
        Calculate reward for model improvement
        
        Args:
            improvement_score: Score improvement (0.0 to 1.0)
            
        Returns:
            Reward amount in PRIME
        """
        # Scale reward based on improvement
        reward = self.MODEL_IMPROVEMENT_BASE_REWARD * (1 + improvement_score)
        return reward
    
    def stake_tokens(self, address: str, amount: float) -> bool:
        """
        Stake tokens for node operation
        
        Args:
            address: User address
            amount: Amount to stake
            
        Returns:
            True if successful
        """
        if amount <= 0:
            return False
        
        current_stake = self.staked_tokens.get(address, 0)
        self.staked_tokens[address] = current_stake + amount
        
        print(f"Staked {amount:,.0f} PRIME for {address}")
        return True
    
    def unstake_tokens(self, address: str, amount: float) -> bool:
        """
        Unstake tokens
        
        Args:
            address: User address
            amount: Amount to unstake
            
        Returns:
            True if successful
        """
        current_stake = self.staked_tokens.get(address, 0)
        
        if amount > current_stake:
            return False
        
        self.staked_tokens[address] = current_stake - amount
        
        print(f"Unstaked {amount:,.0f} PRIME for {address}")
        return True
    
    def create_vesting_schedule(self, address: str, amount: float,
                                start_date: Optional[datetime] = None,
                                vesting_days: int = CORE_TEAM_VESTING_PERIOD_DAYS) -> Dict[str, Any]:
        """
        Create a vesting schedule
        
        Args:
            address: Beneficiary address
            amount: Amount to vest
            start_date: Vesting start date (default: now)
            vesting_days: Vesting period in days
            
        Returns:
            Vesting schedule details
        """
        if start_date is None:
            start_date = datetime.now()
        
        end_date = start_date + timedelta(days=vesting_days)
        
        schedule = {
            'address': address,
            'total_amount': amount,
            'start_date': start_date.isoformat(),
            'end_date': end_date.isoformat(),
            'vesting_days': vesting_days,
            'claimed_amount': 0,
            'created_at': datetime.now().isoformat()
        }
        
        self.vesting_schedules[address] = schedule
        
        print(f"Vesting schedule created for {address}: {amount:,.0f} PRIME over {vesting_days} days")
        return schedule
    
    def calculate_vested_amount(self, address: str,
                               current_date: Optional[datetime] = None) -> float:
        """
        Calculate currently vested amount
        
        Args:
            address: Beneficiary address
            current_date: Current date (default: now)
            
        Returns:
            Vested amount
        """
        schedule = self.vesting_schedules.get(address)
        if not schedule:
            return 0
        
        if current_date is None:
            current_date = datetime.now()
        
        start_date = datetime.fromisoformat(schedule['start_date'])
        end_date = datetime.fromisoformat(schedule['end_date'])
        
        # Before vesting starts
        if current_date < start_date:
            return 0
        
        # After vesting ends
        if current_date >= end_date:
            return schedule['total_amount']
        
        # Linear vesting
        elapsed = (current_date - start_date).total_seconds()
        total_duration = (end_date - start_date).total_seconds()
        vesting_progress = elapsed / total_duration
        
        vested_amount = schedule['total_amount'] * vesting_progress
        return vested_amount - schedule['claimed_amount']
    
    def claim_vested_tokens(self, address: str,
                           current_date: Optional[datetime] = None) -> float:
        """
        Claim vested tokens
        
        Args:
            address: Beneficiary address
            current_date: Current date (default: now)
            
        Returns:
            Amount claimed
        """
        claimable = self.calculate_vested_amount(address, current_date)
        
        if claimable <= 0:
            return 0
        
        schedule = self.vesting_schedules[address]
        schedule['claimed_amount'] += claimable
        
        # Track earned tokens
        current_earned = self.earned_tokens.get(address, 0)
        self.earned_tokens[address] = current_earned + claimable
        
        print(f"Claimed {claimable:,.0f} vested PRIME for {address}")
        return claimable
    
    def get_token_allocation(self) -> Dict[str, float]:
        """Get current token allocation"""
        return self.allocated_tokens.copy()
    
    def get_total_staked(self) -> float:
        """Get total staked tokens"""
        return sum(self.staked_tokens.values())
    
    def get_total_earned(self) -> float:
        """Get total earned tokens"""
        return sum(self.earned_tokens.values())
    
    def get_circulating_supply(self) -> float:
        """
        Calculate circulating supply (excludes locked/vested tokens)
        
        Returns:
            Circulating supply
        """
        # Start with total supply
        circulating = self.TOTAL_SUPPLY
        
        # Subtract unvested core team tokens
        for address, schedule in self.vesting_schedules.items():
            unvested = schedule['total_amount'] - schedule['claimed_amount']
            circulating -= unvested
        
        return circulating
    
    def generate_tokenomics_report(self) -> Dict[str, Any]:
        """
        Generate comprehensive tokenomics report
        
        Returns:
            Tokenomics report
        """
        return {
            'total_supply': self.TOTAL_SUPPLY,
            'circulating_supply': self.get_circulating_supply(),
            'total_staked': self.get_total_staked(),
            'total_earned': self.get_total_earned(),
            'allocation': self.get_token_allocation(),
            'vesting_schedules': len(self.vesting_schedules),
            'reward_rates': {
                'compute_per_gpu_hour': self.COMPUTE_REWARD_PER_GPU_HOUR,
                'benchmark_submission': self.BENCHMARK_REWARD,
                'governance_participation': self.GOVERNANCE_REWARD,
                'model_improvement_base': self.MODEL_IMPROVEMENT_BASE_REWARD
            },
            'generated_at': datetime.now().isoformat()
        }


if __name__ == "__main__":
    # Example usage
    print("PRIME Token Economics")
    print("=" * 70)
    
    tokenomics = PrimeTokenomics()
    
    # Calculate rewards
    print("\n--- Reward Calculations ---")
    compute_reward = tokenomics.calculate_compute_reward(10)  # 10 GPU hours
    print(f"Compute reward (10 GPU-hours): {compute_reward:,.0f} PRIME")
    
    benchmark_reward = tokenomics.calculate_benchmark_reward(is_validated=True)
    print(f"Benchmark reward (validated): {benchmark_reward:,.0f} PRIME")
    
    improvement_reward = tokenomics.calculate_model_improvement_reward(0.15)  # 15% improvement
    print(f"Model improvement reward (15% better): {improvement_reward:,.0f} PRIME")
    
    # Staking
    print("\n--- Staking ---")
    tokenomics.stake_tokens("0x123abc", 10000)
    
    # Vesting
    print("\n--- Vesting ---")
    tokenomics.create_vesting_schedule(
        "0xCoreTeam1",
        amount=50_000_000,  # 50M tokens
        vesting_days=4 * 365
    )
    
    # Generate report
    print("\n--- Tokenomics Report ---")
    report = tokenomics.generate_tokenomics_report()
    print(json.dumps(report, indent=2))
