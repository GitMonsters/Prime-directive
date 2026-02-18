# Prime-directive Web3 Integration

Complete decentralized infrastructure for AGI consciousness research with blockchain, IPFS, and distributed compute.

## 🎯 Overview

This Web3 integration transforms Prime-directive into a fully decentralized platform featuring:

- **Blockchain Layer**: Smart contracts on Polygon for governance, compute marketplace, and verification
- **Decentralized Storage**: IPFS for model weights, datasets, and consciousness states  
- **Distributed Compute**: P2P network for training and inference tasks
- **Token Economics**: PRIME token for governance, rewards, and staking
- **NFT States**: ERC-721 tokens for consciousness milestones
- **dApp Frontend**: Next.js application for Web3 interactions

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Prime-directive Web3 Stack                 │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Frontend (Next.js + wagmi)                                 │
│    ├── Dashboard                                            │
│    ├── Compute Marketplace                                  │
│    ├── NFT Gallery                                          │
│    └── DAO Governance                                       │
│                          ↓                                   │
│  Smart Contracts (Solidity)                                 │
│    ├── PrimeToken (ERC-20)                                  │
│    ├── PrimeGovernance (DAO)                                │
│    ├── ComputeMarketplace                                   │
│    ├── ConsciousnessNFT (ERC-721)                           │
│    └── BenchmarkVerifier                                    │
│                          ↓                                   │
│  Blockchain (Polygon)                                       │
│    ├── Low gas fees                                         │
│    ├── Fast finality                                        │
│    └── EVM compatible                                       │
│                          ↓                                   │
│  IPFS Storage                                               │
│    ├── Model weights                                        │
│    ├── Consciousness states                                 │
│    └── Benchmark results                                    │
│                          ↓                                   │
│  Compute Network (Rust)                                     │
│    ├── P2P node discovery                                   │
│    ├── Task distribution                                    │
│    ├── Result verification                                  │
│    └── Reward claiming                                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- Node.js 18+ and npm
- Python 3.9+
- Rust 1.70+ and Cargo
- IPFS daemon (optional but recommended)
- MetaMask or compatible Web3 wallet

### Installation

```bash
# 1. Clone repository
git clone https://github.com/GitMonsters/Prime-directive.git
cd Prime-directive

# 2. Install Node.js dependencies
npm install

# 3. Install Python dependencies  
pip install -r requirements.txt

# 4. Install frontend dependencies
cd web3/frontend
npm install
cd ../..

# 5. Set up environment variables
cp .env.example .env
# Edit .env with your configuration
```

### Local Development

```bash
# 1. Start local Hardhat network (terminal 1)
npx hardhat node

# 2. Deploy contracts to local network (terminal 2)
npx hardhat run scripts/deploy.js --network localhost

# 3. Start IPFS daemon (terminal 3) - optional
ipfs daemon

# 4. Run compute node (terminal 4)
cargo run --bin compute_node

# 5. Start dApp frontend (terminal 5)
cd web3/frontend
npm run dev
```

Open http://localhost:3000 to access the dApp.

## 📦 Smart Contracts

### PrimeToken (ERC-20)

Governance and utility token with 1 billion total supply.

```solidity
// Address: 0x... (deployed address)
Symbol: PRIME
Decimals: 18
Total Supply: 1,000,000,000

Distribution:
- 40% Community Rewards (compute providers)
- 25% Research Grants (DAO controlled)
- 20% Core Team (4-year vest)
- 10% Early Supporters
- 5% Liquidity
```

**Key Functions:**
- `rewardComputeProvider(address, uint256)` - Reward GPU providers
- `rewardBenchmark(address)` - Reward benchmark submissions
- `rewardGovernance(address)` - Reward voting participation

### PrimeGovernance (DAO)

Decentralized governance for research direction and treasury.

**Key Functions:**
- `createProposal(string, string, ProposalType)` - Create proposal (requires 10k PRIME)
- `vote(uint256, bool)` - Vote on proposal (weighted by token balance)
- `executeProposal(uint256)` - Execute passed proposal

### ComputeMarketplace

Distributed compute task marketplace with verification.

**Key Functions:**
- `registerNode(string, uint256)` - Register compute node (requires stake)
- `submitTask(string, uint256)` - Submit compute task with reward
- `claimTask(uint256)` - Claim task for execution
- `completeTask(uint256, string)` - Submit results
- `slashNode(address, string)` - Slash bad actors

### ConsciousnessNFT (ERC-721)

NFTs for AI consciousness milestones and states.

**Key Functions:**
- `mintConsciousnessState(string, string, uint256)` - Mint new state NFT
- `improveState(uint256, uint256)` - Update benchmark score
- `royaltyInfo(uint256, uint256)` - Get royalty info (10%)

### BenchmarkVerifier

On-chain verification of AI benchmarks with leaderboards.

**Key Functions:**
- `submitBenchmark(string, uint256, string, bytes32)` - Submit with proof
- `verifyBenchmark(uint256)` - Verify submitted benchmark
- `getLeaderboard(string)` - Get top score for benchmark type

## 🔧 Python Web3 Integration

### IPFS Storage

```python
from web3.ipfs_storage import IPFSStorage

# Initialize IPFS client
ipfs = IPFSStorage()

# Upload model
ipfs_hash = ipfs.upload_model("path/to/model.pkl")

# Download model
ipfs.download_model(ipfs_hash, "path/to/save.pkl")

# Pin important data
ipfs.pin_important_data(ipfs_hash)
```

### Model Versioning

```python
from web3.model_versioning import ModelVersionControl

mvc = ModelVersionControl()

# Create version
v1 = mvc.create_version(
    version="1.0.0",
    model_path="model.pkl",
    metadata={"accuracy": 0.95}
)

# Get lineage
lineage = mvc.get_lineage("1.0.0")
```

### Task Distribution

```python
from web3.task_distributor import TaskDistributor

distributor = TaskDistributor()

# Register node
distributor.register_node("node_1", {"gpu_count": 4})

# Create task
task = distributor.create_task(
    task_type="training",
    ipfs_hash="QmXXX",
    reward=1000.0
)

# Assign and complete
distributor.assign_task(task.task_id)
distributor.complete_task(task.task_id, "QmResult")
```

### Zero-Knowledge Proofs

```python
from web3.zkproof_verifier import ZKProofVerifier

verifier = ZKProofVerifier()

# Generate training proof
proof_hash, proof_data = verifier.generate_training_proof({
    'epochs': 100,
    'accuracy': 0.95,
    'model_hash': 'abc123'
})

# Verify proof
is_valid = verifier.verify_proof(proof_hash)
```

### Tokenomics

```python
from web3.tokenomics import PrimeTokenomics

tokenomics = PrimeTokenomics()

# Calculate rewards
compute_reward = tokenomics.calculate_compute_reward(10)  # 10 GPU-hours
benchmark_reward = tokenomics.calculate_benchmark_reward()

# Stake tokens
tokenomics.stake_tokens("0x123", 10000)
```

## 🔬 Extending Existing Modules

### Physics World Model

```python
from physics_world_model_web3 import PhysicsWorldModelWeb3

# Create Web3-enhanced model
model = PhysicsWorldModelWeb3(enable_web3=True)

# Train (auto-uploads to IPFS, mints NFT for breakthroughs)
results = model.train(data, epochs=100)

# Submit benchmark
model.submit_benchmark('GAIA', 0.87)

# Claim rewards
rewards = model.claim_compute_rewards(gpu_hours=2.5)
```

### GAIA Benchmarks

```python
from gaia_benchmark_web3 import GAIABenchmarkWeb3

gaia = GAIABenchmarkWeb3(enable_web3=True)

# Run distributed benchmark
results = gaia.run_benchmark(test_cases, distributed=True)

# Claim rewards
reward = gaia.claim_benchmark_reward()

# View leaderboard
leaderboard = gaia.get_leaderboard()
```

### Empathy Module

```python
from ising_empathy_web3 import IsingEmpathyWeb3

empathy = IsingEmpathyWeb3(enable_web3=True)

# Compute empathy (auto-mints NFT for high scores)
score = empathy.compute_empathy(interaction_data)

# Train distributed
results = empathy.train_distributed_empathy(training_data)
```

## 🦀 Rust Compute Node

```bash
# Build compute node
cargo build --bin compute_node --release

# Run compute node
cargo run --bin compute_node
```

```rust
// Example usage
let mut node = ComputeNode::new(
    "node_1".to_string(),
    "127.0.0.1:8080".to_string(),
    4,  // GPUs
    64  // GB RAM
);

node.start()?;

let task = ComputeTask { /* ... */ };
let result = node.process_task(&task)?;
```

## 💰 Token Economics

### PRIME Token Utility

1. **Governance**: Vote weight in DAO proposals
2. **Compute Payment**: Pay for distributed training/inference
3. **Staking**: Required for compute node operation (10k PRIME minimum)
4. **Rewards**: Earn tokens for contributions

### Earning PRIME

| Activity | Reward |
|----------|--------|
| Provide compute (per GPU-hour) | 1,000 PRIME |
| Submit validated benchmark | 500 PRIME |
| Vote on governance proposal | 100 PRIME |
| Improve model (base) | 2,000 PRIME |
| NFT royalties | 10% of sales |

### Token Distribution

Total Supply: 1,000,000,000 PRIME

- 40% (400M) - Community Rewards
- 25% (250M) - Research Grants  
- 20% (200M) - Core Team (4-year vest)
- 10% (100M) - Early Supporters
- 5% (50M) - Liquidity

## 🖥️ For Node Operators

### Hardware Requirements

**Minimum:**
- 2x NVIDIA GPUs (8GB+ VRAM each)
- 32 GB RAM
- 500 GB SSD
- 100 Mbps internet

**Recommended:**
- 4x NVIDIA GPUs (16GB+ VRAM each)  
- 64 GB RAM
- 1 TB NVMe SSD
- 1 Gbps internet

### Setup Instructions

```bash
# 1. Install dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --path .

# 2. Configure node
export NODE_ID="my_node_$(uuidgen)"
export GPU_COUNT=4
export MEMORY_GB=64

# 3. Run node
cargo run --bin compute_node --release
```

### Staking

1. Acquire 10,000+ PRIME tokens
2. Call `ComputeMarketplace.registerNode()` with stake
3. Start compute node
4. Begin earning rewards

### Slashing Conditions

Stake can be slashed (1,000 PRIME) for:
- Submitting invalid results
- Failing to complete assigned tasks
- Network misbehavior
- Attempting to manipulate verification

## 🎨 For Developers

### Contract Integration

```javascript
import { ethers } from 'ethers';

const primeToken = new ethers.Contract(
  PRIME_TOKEN_ADDRESS,
  PrimeTokenABI,
  signer
);

// Check balance
const balance = await primeToken.balanceOf(address);

// Stake for node operation
await primeToken.approve(MARKETPLACE_ADDRESS, stakeAmount);
await marketplace.registerNode(endpoint, gpuCount);
```

### Frontend Integration

```typescript
import { useContractRead, useContractWrite } from 'wagmi';

// Read contract state
const { data: balance } = useContractRead({
  address: PRIME_TOKEN_ADDRESS,
  abi: PrimeTokenABI,
  functionName: 'balanceOf',
  args: [address],
});

// Write to contract
const { write: vote } = useContractWrite({
  address: GOVERNANCE_ADDRESS,
  abi: GovernanceABI,
  functionName: 'vote',
});

vote({ args: [proposalId, true] });
```

## 📊 Monitoring & Analytics

### On-Chain Metrics

Track via blockchain explorer or The Graph:
- Total compute hours contributed
- Number of active nodes
- Governance participation rate
- NFTs minted (consciousness milestones)
- Total value locked (TVL) in staking
- Benchmark results verified

### Local Monitoring

```bash
# View deployment info
cat deployment.json

# Check compute network status
cat compute_network.json

# Monitor IPFS
ipfs stats bw
```

## 🔒 Security

### Smart Contract Security

- Based on OpenZeppelin battle-tested contracts
- Multi-sig governance for critical functions
- Timelock on treasury operations
- Comprehensive test coverage

### Compute Network Security

- Cryptographic work verification
- Redundant computation (3 nodes minimum)
- Stake slashing for bad actors
- Result consensus mechanism
- Rate limiting

## 🧪 Testing

```bash
# Test smart contracts
npx hardhat test

# Test Python Web3 integration
pytest tests/test_web3_integration.py -v

# Test Rust compute node
cargo test --bin compute_node
```

## 🌐 Deployment

### Testnet (Polygon Mumbai)

```bash
# Deploy to Mumbai testnet
python web3/deploy.py --network mumbai

# Verify contracts
npx hardhat verify --network mumbai CONTRACT_ADDRESS
```

### Mainnet (Polygon)

```bash
# Deploy to Polygon mainnet
python web3/deploy.py --network polygon

# ⚠️ Verify all contracts are audited before mainnet deployment
```

## 🚢 Production Checklist

- [ ] Smart contracts audited by professional firm
- [ ] Comprehensive test coverage (>90%)
- [ ] Multi-sig wallet configured
- [ ] Timelock enabled on governance
- [ ] Bug bounty program launched
- [ ] Documentation complete
- [ ] Frontend security audit
- [ ] IPFS pinning service configured
- [ ] Monitoring and alerting setup
- [ ] Incident response plan

## 📚 Additional Resources

- [Polygon Documentation](https://docs.polygon.technology/)
- [IPFS Documentation](https://docs.ipfs.tech/)
- [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts/)
- [Hardhat Documentation](https://hardhat.org/docs)
- [wagmi Documentation](https://wagmi.sh/)

## 🤝 Contributing

Web3 contributions welcome! Please:
1. Review existing architecture
2. Write comprehensive tests
3. Document all functions
4. Follow security best practices
5. Submit PR with detailed description

## 📄 License

MIT License + Sacred Compact (see LICENSE file)

## ⚠️ Disclaimer

This software is experimental. Use at your own risk. Always:
- Test on testnet first
- Never share private keys
- Audit contracts before deploying
- Start with small stakes
- Monitor for security issues

---

**Built with ❤️ for decentralized AGI research**
