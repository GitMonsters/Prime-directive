# AgentCPM + RustyWorm Integration Design

**Date**: 2026-02-10  
**Status**: Phase 1 Complete - Design Phase  
**Goal**: Full integration of AgentCPM components (AgentRL, AgentDock, AgentToLeaP) to enhance RustyWorm's evolution algorithm, multi-model scheduling, and benchmarking

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Current State Analysis](#current-state-analysis)
3. [Integration Architecture](#integration-architecture)
4. [Detailed Component Integration](#detailed-component-integration)
5. [Phase-by-Phase Implementation Plan](#phase-by-phase-implementation-plan)
6. [Technical Specifications](#technical-specifications)
7. [Success Metrics](#success-metrics)

---

## Executive Summary

### Problem Statement
RustyWorm achieves 66.7% convergence in mimicry tasks but lacks:
- Advanced reinforcement learning optimization
- Multi-model scheduling and orchestration
- Standardized benchmarking evaluation
- Long-horizon observation capabilities (100+ turns)

### Solution
Integrate three AgentCPM components into RustyWorm:
1. **AgentRL** → Enhance `EvolutionTracker` with RL-based optimization (target: 90%+ convergence)
2. **AgentDock** → Multi-model orchestration via MCP protocol
3. **AgentToLeaP** → Standardized benchmarking framework

### Expected Outcomes
- **Convergence**: 66.7% → 90%+ (35% improvement)
- **Observation Efficiency**: 5 observations → 3-4 observations (20% reduction)
- **Model Support**: Single → Multiple concurrent models
- **Benchmarking**: Custom metrics → 8+ standardized benchmarks (GAIA, HLE, XBench, etc.)

---

## Current State Analysis

### RustyWorm Architecture (10.9K LOC)

```
src/mimicry/
├── engine.rs (2,858 LOC)      # Main orchestrator
├── evolution.rs (1,198 LOC)   # Evolution tracking & drift detection
├── profile.rs (1,111 LOC)     # AI personality profiles
├── api.rs (1,401 LOC)         # LLM API client
├── cache.rs (709 LOC)         # System 1 (fast signatures)
├── analyzer.rs (806 LOC)      # Pattern detection
├── capability.rs (655 LOC)    # Model capabilities
├── persistence.rs (911 LOC)   # Data storage
├── templates.rs (1,280 LOC)   # Prompt engineering
└── mod.rs (25 LOC)
```

### Current Evolution Algorithm

**Key Components**:
- `EvolutionTracker`: Tracks convergence history, phase transitions
- `DriftDetector`: Linear trend analysis over 5-sample window
- `TrainingLoop`: Manual iteration (observe → analyze → adjust)
- `PersonalityDelta`: Fixed adjustment deltas per iteration

**Limitations**:
- Linear convergence detection (no multi-variate analysis)
- No reward shaping
- No importance weighting
- Fixed adjustment strategies (no adaptation)
- No asynchronous multi-turn training

### AgentCPM Architecture (Python-based)

```
AgentCPM/
├── AgentCPM-Explore/
│   ├── AgentRL/               # RL training framework
│   │   ├── src/rollout/       # Sampling & trajectory collection
│   │   ├── src/sampling.py    # Async sampling logic
│   │   ├── src/configs.py     # Training configurations
│   │   └── src/environments/  # Training environments
│   ├── AgentDock/             # MCP orchestration platform
│   │   ├── master/            # Central manager
│   │   ├── node/              # MCP nodes
│   │   └── docker-compose.yml
│   └── AgentToLeaP/           # Evaluation framework
│       ├── benchmarks/        # 8+ integrated benchmarks
│       ├── run_evaluation.py  # Main evaluation entry
│       └── evaluate_and_report.py
└── AgentCPM-Report/
    └── UltraRAG/              # RAG framework
```

---

## Integration Architecture

### High-Level Integration Plan

```
┌─────────────────────────────────────────────────────────────────────┐
│                          RustyWorm + AgentCPM                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │           AgentRL Bridge (Rust ↔ Python FFI)                │   │
│  │  - ReinforcementLearningOptimizer (NEW)                     │   │
│  │  - Reward models for convergence prediction                 │   │
│  │  - Importance weighting & trajectory storage                │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                              ↓                                        │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │    EvolutionTracker (ENHANCED)                              │   │
│  │  ┌────────────────────────────────────────────────────────┐ │   │
│  │  │  Before: Linear drift detection                        │ │   │
│  │  │  After:  RL-optimized personality deltas               │ │   │
│  │  │          Multi-variate convergence analysis            │ │   │
│  │  │          Trajectory-based learning                     │ │   │
│  │  └────────────────────────────────────────────────────────┘ │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                              ↓                                        │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │           AgentDock Bridge (MCP Integration)                │   │
│  │  - Multi-model scheduler                                    │   │
│  │  - Tool sandbox management                                  │   │
│  │  - Dynamic container orchestration                          │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                              ↓                                        │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │             API Observer (ENHANCED)                         │   │
│  │  - Multi-model support (Ollama, OpenAI, local models)       │   │
│  │  - MCP tool interface integration                           │   │
│  │  - Long-horizon observation (100+ turns)                    │   │
│  │  - Async/await for parallel multi-turn sampling             │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                              ↓                                        │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │         Benchmarking Engine (NEW)                           │   │
│  │  - AgentToLeaP Integration                                  │   │
│  │  - GAIA, HLE, XBench, BrowseComp support                    │   │
│  │  - Automated scoring & reporting                            │   │
│  │  - Convergence validation against standards                 │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘

        MongoDB (Trajectory Storage)    AgentDock (MCP Platform)
                  ↑                              ↑
                  └──────────────┬──────────────┘
                                 │
                    ┌────────────┴────────────┐
                    ↓                         ↓
                Local Ollama          API Models (OpenAI, etc.)
              (llama3, deepseek)      (Gpt-4, Claude, etc.)
```

### Component Mapping

| RustyWorm | AgentCPM | Function | Integration Type |
|-----------|----------|----------|-----------------|
| `EvolutionTracker` | `AgentRL` | Optimize persona convergence | FFI Bridge + Async |
| `DriftDetector` | RL reward model | Multi-variate drift analysis | Python callback |
| `API Observer` | `AgentDock` + MCP | Multi-model orchestration | HTTP/JSON RPC |
| `BehaviorAnalyzer` | AgentRL sampler | Trajectory-based pattern learning | Data format adapter |
| `PersonalityDelta` | RL policy | Adaptive adjustment strategy | Model inference |
| Custom metrics | `AgentToLeaP` | Standardized benchmarking | Python subprocess |

---

## Detailed Component Integration

### 1. AgentRL Integration (Phase 2-3: Highest Priority)

#### 1.1 Current RustyWorm Evolution

```rust
// evolution.rs (existing)
pub struct EvolutionTracker {
    pub convergence_history: Vec<f64>,
    pub phase: EvolutionPhase,
    pub drift_detector: DriftDetector,
    pub personality_delta: PersonalityDelta,  // FIXED adjustments
}

// Limited to:
// - Linear convergence detection
// - Fixed adjustment deltas
// - Sequential observation
// - No reward modeling
```

#### 1.2 AgentRL Enhancement Strategy

**Concept**: Treat persona evolution as a Markov Decision Process (MDP)

```
State: Behavioral Profile (speech_pattern, knowledge_style, reasoning_style, etc.)
Action: PersonalityDelta adjustments
Reward: Convergence metric (similarity to target model)
Environment: Target model responses + pattern analysis
```

**Integration Approach**:

```rust
// NEW: src/mimicry/rl_optimizer.rs (600-800 LOC)

pub struct ReinforcementLearningOptimizer {
    /// Python AgentRL backend (via PyO3/HTTP)
    backend: RLBackend,
    
    /// Trajectory storage for RL training
    trajectories: Vec<EvolutionTrajectory>,
    
    /// Reward model for convergence prediction
    reward_model: RewardModel,
    
    /// Importance weighting for trajectory pruning
    importance_weights: Vec<f64>,
}

pub struct EvolutionTrajectory {
    pub state: AiProfile,           // Persona before adjustment
    pub action: PersonalityDelta,   // Applied adjustment
    pub observation: BehaviorObservation,  // Target model response
    pub reward: f64,                // Convergence improvement
    pub next_state: AiProfile,      // Persona after adjustment
}

impl ReinforcementLearningOptimizer {
    /// Collect trajectory for learning
    pub async fn collect_trajectory(&mut self, traj: EvolutionTrajectory) {
        self.trajectories.push(traj);
        if self.trajectories.len() > 100 {
            self.train_on_trajectories().await;
        }
    }
    
    /// Call AgentRL for delta optimization
    pub async fn optimize_delta(
        &self,
        current_profile: &AiProfile,
        target_behavior: &BehaviorObservation,
    ) -> PersonalityDelta {
        let optimal_delta = self.backend.predict_delta(
            current_profile,
            target_behavior,
        ).await;
        optimal_delta
    }
    
    /// Train reward model on collected trajectories
    pub async fn train_on_trajectories(&mut self) {
        self.backend.train_minirl(
            &self.trajectories,
            self.importance_weights.clone(),
        ).await;
    }
}
```

**Key Improvements**:
- **Adaptive Deltas**: Instead of fixed adjustments, predict optimal δ per iteration
- **Reward Shaping**: Use AgentRL's MINIRL loss for stable learning
- **Trajectory Pruning**: Importance weighting removes poor trajectories
- **Parallel Sampling**: Async multi-turn observation
- **Off-Policy Learning**: Learn from past trajectories efficiently

#### 1.3 AgentRL Backend Options

**Option A: PyO3 FFI Bridge (Recommended)**
```rust
// Rust ↔ Python FFI via PyO3/Maturin
pyo3 = "0.21"
maturin = "0.15"

// In src/rl_optimizer.rs
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub struct RLBackend {
    python_module: Py<PyModule>,  // AgentRL Python module
}

impl RLBackend {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let module = PyModule::import(py, "agentrl_bridge")?;
            Ok(RLBackend {
                python_module: module.into(),
            })
        })
    }
    
    pub async fn predict_delta(
        &self,
        profile: &AiProfile,
        observation: &BehaviorObservation,
    ) -> PyResult<PersonalityDelta> {
        Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("profile", serde_json::to_string(profile)?)?;
            dict.set_item("observation", serde_json::to_string(observation)?)?;
            
            let result = self.python_module
                .getattr(py, "predict_delta")?
                .call(py, (dict,), None)?
                .extract(py)?;
            
            Ok(result)
        })
    }
}
```

**Option B: HTTP Microservice**
```rust
// AgentRL runs as separate HTTP service
// RustyWorm communicates via REST API

pub struct RLBackend {
    client: reqwest::Client,
    base_url: String,  // http://localhost:8888
}

impl RLBackend {
    pub async fn predict_delta(&self, ...) -> Result<PersonalityDelta> {
        let resp = self.client
            .post(&format!("{}/predict-delta", self.base_url))
            .json(&params)
            .send()
            .await?;
        
        resp.json::<PersonalityDelta>().await
    }
}
```

**Option C: Pure Rust Rewrite**
- Port AgentRL's MINIRL loss calculator to Rust
- Requires deep understanding of RL algorithms
- Best long-term maintainability
- More development effort

**Recommendation**: **Option B (HTTP microservice)** for Phase 2
- Decouples Python and Rust environments
- Easier testing and iteration
- Leverages AgentCPM's Docker infrastructure
- Can use AgentRL as-is without modification

#### 1.4 Enhanced EvolutionTracker

```rust
// src/mimicry/evolution.rs (ENHANCED)

pub struct EvolutionTracker {
    pub convergence_history: Vec<f64>,
    pub phase: EvolutionPhase,
    pub drift_detector: DriftDetector,
    
    // NEW: RL-based optimization
    pub rl_optimizer: Option<ReinforcementLearningOptimizer>,
    pub trajectory_buffer: Vec<EvolutionTrajectory>,
    pub importance_weights: Vec<f64>,
    
    // NEW: MongoDB connection for trajectory storage
    pub mongodb_client: Option<mongodb::Client>,
    pub trajectory_collection: String,
}

impl EvolutionTracker {
    /// Enhanced evolution step with RL optimization
    pub async fn evolve_with_rl(
        &mut self,
        current_profile: &mut AiProfile,
        observations: &[BehaviorObservation],
    ) -> Result<f64> {
        // 1. Analyze observations
        let target_behavior = self.analyzer.synthesize_target(observations);
        
        // 2. Get RL-optimized delta
        let delta = if let Some(rl) = &self.rl_optimizer {
            rl.optimize_delta(current_profile, &target_behavior).await?
        } else {
            self.compute_delta(observations)?  // Fallback
        };
        
        // 3. Apply delta
        let new_profile = current_profile.apply_delta(&delta);
        
        // 4. Measure convergence improvement
        let improvement = self.measure_convergence(&new_profile, &target_behavior);
        
        // 5. Collect trajectory for learning
        let trajectory = EvolutionTrajectory {
            state: current_profile.clone(),
            action: delta,
            observation: target_behavior,
            reward: improvement,
            next_state: new_profile.clone(),
        };
        
        self.trajectory_buffer.push(trajectory.clone());
        
        // 6. Store in MongoDB for persistent RL training
        if let Some(client) = &self.mongodb_client {
            self.store_trajectory_in_db(client, &trajectory).await?;
        }
        
        // 7. Train RL model periodically
        if self.trajectory_buffer.len() % 50 == 0 {
            if let Some(rl) = &mut self.rl_optimizer {
                rl.train_on_trajectories().await?;
            }
        }
        
        *current_profile = new_profile;
        
        Ok(improvement)
    }
}
```

---

### 2. AgentDock Integration (Phase 4)

#### 2.1 Current RustyWorm API Observer

```rust
// src/mimicry/api.rs (1,401 LOC)

pub struct APIObserver {
    pub client: reqwest::Client,
    pub model_name: String,
    pub base_url: String,
    // Single model only
}

// Limitations:
// - Single model support
// - No tool/MCP integration
// - No long-horizon observation (100+ turns)
// - Sequential observation only
```

#### 2.2 AgentDock Enhancement Strategy

**Goal**: Multi-model observation with MCP tool integration

```rust
// NEW: src/mimicry/agentdock_bridge.rs (800-1000 LOC)

pub struct AgentDockBridge {
    /// HTTP client for AgentDock API
    client: reqwest::Client,
    
    /// AgentDock manager URL
    manager_url: String,  // http://localhost:8080/mcpapi
    
    /// Available model configurations
    models: HashMap<String, ModelConfig>,
    
    /// Container pool (for multi-turn sessions)
    containers: HashMap<String, ContainerSession>,
}

pub struct ModelConfig {
    pub name: String,
    pub api_key: String,
    pub base_url: String,
    pub priority: u8,  // For scheduling
}

pub struct ContainerSession {
    pub container_id: String,
    pub model_config: ModelConfig,
    pub conversation_history: Vec<Message>,
    pub turn_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AgentDockBridge {
    /// Create a new MCP session for long-horizon observation
    pub async fn create_session(
        &mut self,
        model_name: &str,
        max_turns: usize,
    ) -> Result<ContainerSession> {
        // 1. Request container from AgentDock
        let container_id = self.request_container(model_name).await?;
        
        // 2. Initialize conversation
        let session = ContainerSession {
            container_id,
            model_config: self.models[model_name].clone(),
            conversation_history: vec![],
            turn_count: 0,
            created_at: chrono::Utc::now(),
        };
        
        self.containers.insert(container_id.clone(), session.clone());
        
        Ok(session)
    }
    
    /// Multi-turn observation with MCP tool support
    pub async fn observe_long_horizon(
        &mut self,
        session_id: &str,
        query: &str,
        tools: &[String],  // Available MCP tools
    ) -> Result<Vec<BehaviorObservation>> {
        let session = self.containers.get_mut(session_id)
            .ok_or("Session not found")?;
        
        let mut observations = vec![];
        
        // Multi-turn conversation loop (up to 100 turns)
        for turn in 0..100 {
            // 1. Call model via AgentDock
            let response = self.call_model_with_tools(
                session,
                query,
                tools,
            ).await?;
            
            // 2. Extract behavior patterns
            let observation = self.extract_observation(&response);
            observations.push(observation);
            
            // 3. Check for convergence or termination
            session.turn_count = turn + 1;
            if self.should_terminate(session, &observations) {
                break;
            }
        }
        
        Ok(observations)
    }
    
    /// Schedule multiple models for concurrent observation
    pub async fn observe_multi_model(
        &mut self,
        query: &str,
        models: &[&str],
    ) -> Result<HashMap<String, Vec<BehaviorObservation>>> {
        let futures = models.iter().map(|model| async {
            let session = self.create_session(model, 10).await?;
            let observations = self.observe_long_horizon(
                &session.container_id,
                query,
                &[],
            ).await?;
            Ok((model.to_string(), observations))
        });
        
        let results = futures::future::join_all(futures).await;
        
        Ok(results.into_iter().collect::<Result<_>>()?)
    }
}
```

#### 2.3 MCP Tool Integration

```rust
// Agent can call MCP tools via AgentDock

pub struct MCPToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

pub struct MCPToolResponse {
    pub tool_name: String,
    pub result: String,
    pub execution_time_ms: u64,
}

impl AgentDockBridge {
    /// Call an MCP tool via AgentDock
    pub async fn call_mcp_tool(
        &self,
        container_id: &str,
        tool: &MCPToolCall,
    ) -> Result<MCPToolResponse> {
        let mcp_request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": tool.tool_name,
                "arguments": tool.arguments,
            },
            "id": 1,
        });
        
        let response = self.client
            .post(&format!(
                "{}/container/{}/mcp",
                self.manager_url,
                container_id
            ))
            .json(&mcp_request)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        
        Ok(MCPToolResponse {
            tool_name: tool.tool_name.clone(),
            result: result.get("result").unwrap_or(&json!("")).to_string(),
            execution_time_ms: 0,
        })
    }
}
```

#### 2.4 Docker Compose Integration

```yaml
# docker-compose.yml (in project root)

version: '3.8'

services:
  # RustyWorm
  rustyworm:
    build: .
    image: rustyworm:integration
    container_name: rustyworm
    network_mode: host
    volumes:
      - ~/.rustyworm:/data/.rustyworm
      - ./observations:/workspace/observations
    environment:
      - AGENTDOCK_URL=http://localhost:8080/mcpapi
      - MONGODB_URL=mongodb://root:password@localhost:27017
      - OLLAMA_URL=http://localhost:11434
    depends_on:
      - agentdock-manager
      - mongodb
      - ollama

  # AgentDock Platform
  agentdock-manager:
    image: sailaoda/agentdock-manager:latest
    container_name: agentdock-manager
    ports:
      - "8080:8080"
    environment:
      - MONGODB_USERNAME=root
      - MONGODB_PASSWORD=password
    depends_on:
      - agentdock-mongodb

  agentdock-node-full:
    image: sailaoda/agentdock-node-full:latest
    container_name: agentdock-node-full
    ports:
      - "8004:8004"
      - "8092:8092"
    depends_on:
      - agentdock-manager

  agentdock-node-explore:
    image: sailaoda/agentdock-node-explore:latest
    container_name: agentdock-node-explore
    ports:
      - "8014:8014"
      - "8102:8102"
    depends_on:
      - agentdock-manager

  # MongoDB for trajectory storage
  mongodb:
    image: mongo:7.0
    container_name: agentdock-mongodb
    ports:
      - "27017:27017"
    environment:
      - MONGO_INITDB_ROOT_USERNAME=root
      - MONGO_INITDB_ROOT_PASSWORD=password
    volumes:
      - mongodb_data:/data/db

  # Ollama for local model inference
  ollama:
    image: ollama/ollama:latest
    container_name: ollama
    ports:
      - "11434:11434"
    volumes:
      - ~/.ollama:/root/.ollama
    environment:
      - OLLAMA_MODELS=/root/.ollama/models

  # AgentRL training service (Phase 3)
  agentrl-service:
    build:
      context: ./agentcpm-integration
      dockerfile: Dockerfile.agentrl
    container_name: agentrl-service
    ports:
      - "8888:8888"
    environment:
      - MONGODB_URL=mongodb://root:password@localhost:27017
      - AGENTDOCK_URL=http://localhost:8080/mcpapi
    depends_on:
      - mongodb
      - agentdock-manager

volumes:
  mongodb_data:
```

---

### 3. AgentToLeaP Integration (Phase 6)

#### 3.1 Benchmark Framework

```rust
// NEW: src/benchmarking/mod.rs (1200-1500 LOC)

pub struct BenchmarkingEngine {
    /// Link to AgentToLeaP Python framework
    evaluator: BenchmarkEvaluator,
    
    /// Available benchmarks
    benchmarks: HashMap<String, BenchmarkConfig>,
    
    /// Results storage
    results: HashMap<String, BenchmarkResult>,
}

pub enum BenchmarkType {
    /// General AI Assistant Ability (GAIA)
    GAIA,
    /// Human-Level Evaluation (HLE)
    HLE,
    /// Web Browsing Comprehension (BrowseComp)
    BrowseComp,
    /// Extended Bench (XBench)
    XBench,
    /// Web Navigation & QA (WebWalkerQA)
    WebWalkerQA,
    /// Frames
    Frames,
    /// SEAL (Contradictory Information Handling)
    SEAL,
}

pub struct BenchmarkResult {
    pub benchmark: BenchmarkType,
    pub model_name: String,
    pub convergence_score: f64,
    pub accuracy: f64,
    pub num_tasks: usize,
    pub successful_tasks: usize,
    pub avg_turns: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub detailed_results: Vec<TaskResult>,
}

pub struct TaskResult {
    pub task_id: String,
    pub question: String,
    pub expected_answer: String,
    pub model_answer: String,
    pub success: bool,
    pub confidence: f64,
    pub turns_used: usize,
}

impl BenchmarkingEngine {
    /// Run a specific benchmark
    pub async fn run_benchmark(
        &mut self,
        benchmark: BenchmarkType,
        model_name: &str,
        max_tasks: Option<usize>,
    ) -> Result<BenchmarkResult> {
        // Call AgentToLeaP Python evaluator
        let result = self.evaluator.run_evaluation(
            benchmark,
            model_name,
            max_tasks,
        ).await?;
        
        // Store results
        self.results.insert(
            format!("{:?}_{}", benchmark, model_name),
            result.clone()
        );
        
        Ok(result)
    }
    
    /// Run multiple benchmarks in parallel
    pub async fn run_benchmark_suite(
        &mut self,
        benchmarks: &[BenchmarkType],
        model_name: &str,
    ) -> Result<Vec<BenchmarkResult>> {
        let futures = benchmarks.iter().map(|b| {
            self.run_benchmark(*b, model_name, None)
        });
        
        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect()
    }
    
    /// Validate convergence against GAIA standard
    pub async fn validate_convergence(
        &self,
        persona: &AiProfile,
        target_model: &str,
    ) -> Result<ConvergenceValidation> {
        // Run GAIA benchmark subset
        let gaia_result = self.run_benchmark(
            BenchmarkType::GAIA,
            &persona.name,
            Some(50),  // 50 tasks
        ).await?;
        
        Ok(ConvergenceValidation {
            persona_name: persona.name.clone(),
            target_model: target_model.to_string(),
            gaia_score: gaia_result.accuracy,
            convergence_sufficient: gaia_result.accuracy > 0.85,
            benchmark_results: vec![gaia_result],
        })
    }
}
```

#### 3.2 Python Bridge for AgentToLeaP

```python
# agentcpm-integration/agentrl_bridge.py

import asyncio
from typing import Dict, List, Any
import subprocess
import json

class BenchmarkEvaluator:
    """Bridge to AgentToLeaP evaluation framework"""
    
    def __init__(self, agenttoleap_path: str):
        self.agenttoleap_path = agenttoleap_path
        self.benchmarks = {
            'GAIA': 'benchmarks/gaia',
            'HLE': 'benchmarks/hle',
            'XBench': 'benchmarks/xbench',
            'BrowseComp': 'benchmarks/browsecomp',
        }
    
    async def run_evaluation(
        self,
        benchmark: str,
        model_name: str,
        max_tasks: int = None,
    ) -> Dict[str, Any]:
        """Run AgentToLeaP evaluation"""
        
        cmd = [
            'python', f'{self.agenttoleap_path}/run_evaluation.py',
            '--benchmark', benchmark,
            '--model', model_name,
        ]
        
        if max_tasks:
            cmd.extend(['--max-tasks', str(max_tasks)])
        
        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        
        stdout, stderr = await process.communicate()
        
        if process.returncode != 0:
            raise Exception(f"Evaluation failed: {stderr.decode()}")
        
        # Parse results
        results = json.loads(stdout.decode())
        
        return results
```

---

## Phase-by-Phase Implementation Plan

### Phase 1: Research & Architecture Design ✅ **COMPLETE**

- [x] Clone AgentCPM repository
- [x] Study AgentRL implementation
- [x] Analyze AgentDock architecture
- [x] Review AgentToLeaP benchmarking
- [x] Create integration design document (this file)

**Deliverables**:
- AgentCPM repo cloned and analyzed
- Integration architecture designed
- Component mapping completed

---

### Phase 2: AgentRL Integration (HTTPService) - **NEXT**

**Timeline**: 3-4 weeks  
**Effort**: 1,200-1,500 LOC (Rust) + setup

**Tasks**:
1. Create `src/mimicry/rl_optimizer.rs` (600 LOC)
   - `ReinforcementLearningOptimizer` struct
   - `EvolutionTrajectory` data structure
   - `RLBackend` HTTP client interface
   - Trajectory collection and storage

2. Create `agentcpm-integration/agentrl_service.py` (400 LOC)
   - Flask/FastAPI wrapper around AgentRL
   - `/predict-delta` endpoint
   - `/train` endpoint for MINIRL training
   - MongoDB integration

3. Enhance `src/mimicry/evolution.rs` (300 LOC)
   - Integrate `ReinforcementLearningOptimizer`
   - Add trajectory buffer management
   - Add RL training scheduler

4. Setup MongoDB for trajectory storage
   - Docker container
   - Schema design
   - Connection pooling

5. Integration tests (200 LOC)
   - Test RL optimizer initialization
   - Test trajectory collection
   - Test HTTP communication
   - Test convergence improvement

**Success Criteria**:
- [ ] Convergence improves from 66.7% to 75%+
- [ ] RL optimizer successfully predicts deltas
- [ ] Trajectories stored in MongoDB
- [ ] 20+ integration tests passing
- [ ] API documented with examples

**Output**: `src/mimicry/rl_optimizer.rs`, `agentcpm-integration/agentrl_service.py`, enhanced evolution module

---

### Phase 3: AgentDock Integration - **AFTER PHASE 2**

**Timeline**: 2-3 weeks  
**Effort**: 1,000-1,200 LOC (Rust) + Docker setup

**Tasks**:
1. Create `src/mimicry/agentdock_bridge.rs` (800 LOC)
   - `AgentDockBridge` struct
   - Multi-model session management
   - MCP tool integration
   - Long-horizon observation (100+ turns)

2. Enhance `src/mimicry/api.rs` (200 LOC)
   - Integrate with `AgentDockBridge`
   - Multi-model support
   - Fallback strategies

3. Create Docker Compose setup (200 LOC)
   - AgentDock manager, nodes, MongoDB, Ollama
   - Network configuration
   - Volume mounting

4. Integration tests (150 LOC)
   - Multi-model observation
   - Long-horizon conversation
   - MCP tool calling

**Success Criteria**:
- [ ] Observe from 2+ models concurrently
- [ ] Long-horizon observation (50+ turns) working
- [ ] MCP tools callable via AgentDock
- [ ] Docker Compose up-and-running
- [ ] Observation efficiency improves (5→3 observations)

**Output**: `src/mimicry/agentdock_bridge.rs`, `docker-compose.yml`, enhanced API observer

---

### Phase 4: Long-Horizon Observations - **PARALLEL WITH PHASE 3**

**Timeline**: 2 weeks  
**Effort**: 600-800 LOC

**Tasks**:
1. Enhance `api.rs` for multi-turn support
   - Conversation history management
   - Context window optimization
   - Termination detection

2. Create `src/mimicry/long_horizon.rs` (400 LOC)
   - `LongHorizonObserver` struct
   - 100+ turn conversation tracking
   - Pattern extraction from conversations
   - Convergence detection

3. Integration tests (200 LOC)

**Success Criteria**:
- [ ] Support 100+ turns per observation
- [ ] Pattern extraction from conversations
- [ ] Convergence detection in conversations

**Output**: `src/mimicry/long_horizon.rs`, enhanced API observer

---

### Phase 5: AgentToLeaP Integration - **AFTER PHASES 2-3**

**Timeline**: 2-3 weeks  
**Effort**: 1,200-1,500 LOC (Rust) + 300 LOC (Python)

**Tasks**:
1. Create `src/benchmarking/mod.rs` (1,000 LOC)
   - `BenchmarkingEngine` struct
   - Support for 8+ benchmarks
   - Parallel benchmark execution
   - Results storage and analysis

2. Create `src/benchmarking/gaia.rs` (300 LOC)
   - GAIA-specific evaluation
   - Task format handling
   - Scoring logic

3. Create `agentcpm-integration/benchmark_bridge.py` (300 LOC)
   - Python-side benchmark runner
   - AgentToLeaP integration
   - Result formatting

4. Integration tests (200 LOC)

**Success Criteria**:
- [ ] Run GAIA benchmark successfully
- [ ] Support 8+ benchmarks
- [ ] Generate detailed reports
- [ ] Validate convergence against standards

**Output**: `src/benchmarking/mod.rs`, `benchmark_bridge.py`, benchmark reports

---

### Phase 6: Comprehensive Documentation - **ONGOING**

**Timeline**: 2 weeks  
**Effort**: 500 LOC docs + examples

**Deliverables**:
- [ ] `AGENTCPM_INTEGRATION.md` - Integration guide
- [ ] `AGENTRL_USAGE.md` - RL optimizer usage
- [ ] `AGENTDOCK_USAGE.md` - Multi-model scheduling
- [ ] `BENCHMARKING.md` - Benchmark framework guide
- [ ] Example scripts (5+ examples)
- [ ] API documentation

---

## Technical Specifications

### Performance Targets

| Metric | Current | Target | Improvement |
|--------|---------|--------|-------------|
| **Convergence** | 66.7% | 90%+ | +35% |
| **Observations Needed** | 5+ | 3-4 | -30% |
| **Training Time** | N/A | 2x batch | - |
| **Model Support** | 1 | 3+ | 3x |
| **Benchmarks** | Custom | 8+ | +8 |
| **Max Conversation Turns** | 20 | 100+ | 5x |

### Memory Requirements

```
RustyWorm Base:       ~200 MB
+ RL Optimizer:       ~100 MB (trajectory buffer)
+ AgentDock Bridge:   ~50 MB
+ MongoDB Connection: ~50 MB
───────────────────────────────
Total:               ~400 MB (single model)

Multi-model (3x):    ~600-800 MB
```

### Storage Requirements

```
Trajectory Storage (MongoDB):
- Per evolution step: ~2-5 KB
- 100 iterations: ~200-500 KB
- Full training run (1000 iterations): ~2-5 MB

Benchmark Results:
- Per benchmark: ~5-10 MB
- Full suite (8 benchmarks): ~40-80 MB
```

### API Specifications

#### RL Optimizer HTTP API

```
POST /predict-delta
Request:
{
  "profile": { /* AiProfile */ },
  "observation": { /* BehaviorObservation */ }
}
Response:
{
  "delta": { /* PersonalityDelta */ },
  "confidence": 0.85,
  "reasoning": "..."
}

POST /train
Request:
{
  "trajectories": [ /* EvolutionTrajectory[] */ ],
  "importance_weights": [0.9, 0.8, ...]
}
Response:
{
  "loss": 0.45,
  "training_time_ms": 1200
}
```

#### AgentDock MCP API

```
POST /node/create
Request: { "image_name": "agentdock-node-explore" }
Response: { "container_id": "abc123" }

POST /container/{id}/mcp
Request: { "jsonrpc": "2.0", "method": "tools/call", ... }
Response: { "result": "..." }
```

#### Benchmarking API

```
POST /benchmark/run
Request: {
  "benchmark": "GAIA",
  "model": "rustyworm-persona",
  "max_tasks": 50
}
Response: {
  "accuracy": 0.92,
  "successful_tasks": 46,
  "total_tasks": 50,
  "results": [...]
}
```

---

## Success Metrics

### Convergence Metrics

- **Primary**: Convergence score increases from 66.7% to 90%+
- **Secondary**: Observations needed decrease from 5+ to 3-4
- **Tertiary**: Convergence stability (lower variance)

### Performance Metrics

- **Evolution Speed**: Time per iteration ≤ 2 seconds
- **RL Training**: Batch training ≤ 5 minutes for 100 trajectories
- **Observation**: Single observation ≤ 30 seconds
- **Benchmarking**: GAIA suite ≤ 2 hours (50 tasks)

### Quality Metrics

- **Benchmark Accuracy**: GAIA score ≥ 0.85
- **Trajectory Quality**: Mean reward > 0.7
- **Model Stability**: Convergence curve smoothness > 0.8

### Developer Experience

- **Test Coverage**: ≥ 80% for new code
- **Documentation**: All public APIs documented
- **Examples**: 5+ working examples
- **Reproducibility**: All results reproducible

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| AgentRL Python incompatibility | Low | High | Test thoroughly before committing |
| MongoDB connection issues | Medium | Medium | Implement retry logic, connection pooling |
| AgentDock container overhead | Medium | Medium | Optimize resource limits, reuse containers |
| Benchmark data format mismatch | Medium | Medium | Create data format validator |
| RL training instability | Medium | High | Use MINIRL (proven stable), trajectory filtering |
| Multi-model scheduling conflicts | Low | High | Implement queue-based scheduling |

---

## Timeline Summary

```
Week 1-2:  Phase 1: Research & Design ✅ COMPLETE
Week 3-6:  Phase 2: AgentRL Integration
Week 7-8:  Phase 3: AgentDock Integration
Week 7-8:  Phase 4: Long-Horizon Observations (parallel)
Week 9-10: Phase 5: AgentToLeaP Integration
Week 11:   Phase 6: Documentation & Polish
────────────────────────────────────
Total:     11 weeks (3 months) for full integration

Fast-track (Phases 2+5 only): 6 weeks
```

---

## Next Steps

1. **Immediate** (this week):
   - [ ] Review this design document
   - [ ] Set up AgentRL HTTP service scaffold
   - [ ] Create `rl_optimizer.rs` skeleton
   - [ ] Plan MongoDB schema

2. **Short-term** (next 2 weeks):
   - [ ] Implement RL optimizer
   - [ ] Create HTTP API wrapper
   - [ ] Write trajectory storage
   - [ ] Begin integration tests

3. **Medium-term** (Weeks 3-6):
   - [ ] Complete AgentRL integration
   - [ ] Start AgentDock bridge
   - [ ] Parallel: Long-horizon observations
   - [ ] Benchmark baseline performance

4. **Long-term** (Weeks 7-11):
   - [ ] AgentToLeaP integration
   - [ ] Full documentation
   - [ ] Performance tuning
   - [ ] Release integration branch

---

## References

- **AgentCPM Repository**: https://github.com/OpenBMB/AgentCPM
- **AgentRL Documentation**: `/home/worm/AgentCPM/AgentCPM-Explore/AgentRL/README.md`
- **AgentDock Documentation**: `/home/worm/AgentCPM/AgentCPM-Explore/AgentDock/README.md`
- **AgentToLeaP Documentation**: `/home/worm/AgentCPM/AgentCPM-Explore/AgentToLeaP/README.md`
- **RustyWorm Evolution Module**: `/home/worm/Prime-directive/Prime-directive/src/mimicry/evolution.rs`

---

**Document Version**: 1.0  
**Created**: 2026-02-10  
**Last Updated**: 2026-02-10  
**Status**: Design Phase Complete - Ready for Phase 2 Implementation
