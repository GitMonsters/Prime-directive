// =================================================================
// REINFORCEMENT LEARNING OPTIMIZER
// =================================================================
// Integrates AgentCPM's AgentRL framework into RustyWorm's evolution
// system for advanced persona convergence optimization.
//
// This module manages:
// - Trajectory collection for learning
// - Communication with AgentRL HTTP service
// - Reward modeling and importance weighting
// - MongoDB-based persistent trajectory storage
// - Async multi-turn training support
// =================================================================

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::mimicry::profile::AiProfile;
use crate::mimicry::profile::PersonalityDelta;

// =================================================================
// DATA STRUCTURES
// =================================================================

/// Represents a behavior observation (model response and patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorObservation {
    /// Query sent to the model
    pub query: String,
    
    /// Response from the model
    pub response: String,
    
    /// Detected behavior patterns
    pub patterns: Vec<String>,
    
    /// Similarity score to target behavior (0.0 to 1.0)
    pub similarity_to_target: f64,
    
    /// Confidence in this observation (0.0 to 1.0)
    pub confidence: f64,
}

impl Default for BehaviorObservation {
    fn default() -> Self {
        BehaviorObservation {
            query: String::new(),
            response: String::new(),
            patterns: vec![],
            similarity_to_target: 0.0,
            confidence: 0.0,
        }
    }
}

/// Represents a single evolution trajectory for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTrajectory {
    /// Unique trajectory ID
    pub id: String,
    
    /// Behavioral state before applying delta
    pub state: AiProfile,
    
    /// Action taken (personality adjustment)
    pub action: PersonalityDelta,
    
    /// Observed behavior after adjustment
    pub observation: BehaviorObservation,
    
    /// Convergence reward (improvement metric)
    pub reward: f64,
    
    /// Behavioral state after applying action
    pub next_state: AiProfile,
    
    /// Timestamp of trajectory collection
    pub timestamp: DateTime<Utc>,
    
    /// Whether this trajectory was used in training
    pub used_in_training: bool,
    
    /// Importance weight for this trajectory (for importance weighting)
    pub importance_weight: f64,
}

impl EvolutionTrajectory {
    /// Creates a new evolution trajectory
    pub fn new(
        state: AiProfile,
        action: PersonalityDelta,
        observation: BehaviorObservation,
        reward: f64,
        next_state: AiProfile,
    ) -> Self {
        use uuid::Uuid;
        
        EvolutionTrajectory {
            id: Uuid::new_v4().to_string(),
            state,
            action,
            observation,
            reward,
            next_state,
            timestamp: Utc::now(),
            used_in_training: false,
            importance_weight: 1.0,
        }
    }
}

/// Configuration for RL optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RLOptimizerConfig {
    /// AgentRL service URL (e.g., http://localhost:8888)
    pub service_url: String,
    
    /// MongoDB connection URL for trajectory storage
    pub mongodb_url: String,
    
    /// Database name for trajectories
    pub database_name: String,
    
    /// Trajectory collection name
    pub collection_name: String,
    
    /// Batch size for RL training
    pub batch_size: usize,
    
    /// Minimum trajectories before starting training
    pub min_trajectories_for_training: usize,
    
    /// Importance weight threshold (prune below this)
    pub importance_threshold: f64,
    
    /// Timeout for HTTP requests (ms)
    pub request_timeout_ms: u64,
}

impl Default for RLOptimizerConfig {
    fn default() -> Self {
        RLOptimizerConfig {
            service_url: "http://localhost:8888".to_string(),
            mongodb_url: "mongodb://root:password@localhost:27017".to_string(),
            database_name: "rustyworm_rl".to_string(),
            collection_name: "trajectories".to_string(),
            batch_size: 64,
            min_trajectories_for_training: 50,
            importance_threshold: 0.3,
            request_timeout_ms: 30000,
        }
    }
}

/// HTTP request to AgentRL service for delta prediction
#[derive(Debug, Serialize)]
pub struct RLPredictDeltaRequest {
    pub profile: AiProfile,
    pub observation: BehaviorObservation,
}

/// HTTP response from AgentRL service
#[derive(Debug, Deserialize)]
pub struct RLPredictDeltaResponse {
    pub delta: PersonalityDelta,
    pub confidence: f64,
    pub reasoning: String,
}

/// HTTP request for RL training
#[derive(Debug, Serialize)]
pub struct RLTrainingRequest {
    pub trajectories: Vec<EvolutionTrajectory>,
    pub importance_weights: Vec<f64>,
    pub loss_type: String,  // "MINIRL" or "GRPO"
}

/// HTTP response from RL training
#[derive(Debug, Deserialize)]
pub struct RLTrainingResponse {
    pub loss: f64,
    pub training_time_ms: u64,
    pub num_trajectories_used: usize,
}

// =================================================================
// REINFORCEMENT LEARNING OPTIMIZER
// =================================================================

/// Main RL optimizer for persona evolution
pub struct ReinforcementLearningOptimizer {
    /// Configuration
    config: RLOptimizerConfig,
    
    /// HTTP client for communication with AgentRL service
    http_client: reqwest::Client,
    
    /// In-memory trajectory buffer
    trajectories: Vec<EvolutionTrajectory>,
    
    /// Trajectory storage backend (MongoDB) - not currently used but reserved
    #[allow(dead_code)]
    mongodb_client: Option<String>,
    
    /// Statistics
    stats: OptimzerStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimzerStatistics {
    /// Total trajectories collected
    pub total_trajectories: usize,
    
    /// Trajectories used in training
    pub trajectories_in_training: usize,
    
    /// Number of training runs
    pub training_runs: usize,
    
    /// Average reward
    pub avg_reward: f64,
    
    /// Average importance weight
    pub avg_importance_weight: f64,
}

impl Default for OptimzerStatistics {
    fn default() -> Self {
        OptimzerStatistics {
            total_trajectories: 0,
            trajectories_in_training: 0,
            training_runs: 0,
            avg_reward: 0.0,
            avg_importance_weight: 1.0,
        }
    }
}

impl ReinforcementLearningOptimizer {
    /// Creates a new RL optimizer with default config
    pub fn new(config: RLOptimizerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let http_client = reqwest::Client::new();
        
        // Store MongoDB URL for future async connections
        // We use async MongoDB client in actual usage, so we just store the URL
        let _mongodb_url = config.mongodb_url.clone();
        
        Ok(ReinforcementLearningOptimizer {
            config,
            http_client,
            trajectories: Vec::new(),
            mongodb_client: Some(_mongodb_url),
            stats: OptimzerStatistics::default(),
        })
    }
    
    /// Health check - verify AgentRL service is accessible
    pub async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!("{}/health", self.config.service_url);
        let response = self.http_client
            .get(&url)
            .timeout(std::time::Duration::from_millis(self.config.request_timeout_ms))
            .send()
            .await?;
        
        Ok(response.status().is_success())
    }
    
    /// Predict optimal PersonalityDelta using RL model
    pub async fn predict_delta(
        &self,
        profile: &AiProfile,
        observation: &BehaviorObservation,
    ) -> Result<PersonalityDelta, Box<dyn std::error::Error>> {
        let request = RLPredictDeltaRequest {
            profile: profile.clone(),
            observation: observation.clone(),
        };
        
        let url = format!("{}/predict-delta", self.config.service_url);
        let response = self.http_client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_millis(self.config.request_timeout_ms))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("AgentRL service error: {}", response.status()).into());
        }
        
        let result: RLPredictDeltaResponse = response.json().await?;
        Ok(result.delta)
    }
    
    /// Collect a trajectory for learning
    pub fn collect_trajectory(&mut self, trajectory: EvolutionTrajectory) {
        self.trajectories.push(trajectory.clone());
        self.stats.total_trajectories += 1;
        
        // Update average reward
        let sum: f64 = self.trajectories.iter().map(|t| t.reward).sum();
        self.stats.avg_reward = sum / self.trajectories.len() as f64;
        
        // Store in MongoDB if available (async would be used in production)
        if let Some(_) = &self.mongodb_client {
            // MongoDB storage would be async in production
            // For now, trajectories are stored in memory
        }
        
        // Auto-train if enough trajectories collected
        if self.trajectories.len() >= self.config.min_trajectories_for_training {
            // This would be called periodically in the actual evolution loop
        }
    }
    
    /// Get current trajectory buffer
    pub fn get_trajectories(&self) -> &[EvolutionTrajectory] {
        &self.trajectories
    }
    
    /// Calculate importance weights for trajectories using reward-based weighting
    pub fn calculate_importance_weights(&self) -> Vec<f64> {
        if self.trajectories.is_empty() {
            return vec![];
        }
        
        // Use reward-based importance: higher reward = higher weight
        let min_reward = self.trajectories.iter()
            .map(|t| t.reward)
            .fold(f64::INFINITY, f64::min);
        let max_reward = self.trajectories.iter()
            .map(|t| t.reward)
            .fold(f64::NEG_INFINITY, f64::max);
        
        let range = max_reward - min_reward;
        
        let weights: Vec<f64> = self.trajectories.iter().map(|t| {
            if range > 0.0 {
                (t.reward - min_reward) / range
            } else {
                1.0
            }
        }).collect();
        
        // Normalize
        let sum: f64 = weights.iter().sum();
        let normalized: Vec<f64> = weights.iter()
            .map(|w| w / sum)
            .collect();
        
        normalized
    }
    
    /// Prune trajectories with low importance weights
    pub fn prune_trajectories(&mut self) {
        let weights = self.calculate_importance_weights();
        
        let threshold = self.config.importance_threshold;
        
        // Keep only trajectories with importance weight >= threshold
        let mut new_trajectories = Vec::new();
        for (i, trajectory) in self.trajectories.iter().enumerate() {
            if i < weights.len() && weights[i] >= threshold {
                new_trajectories.push(trajectory.clone());
            }
        }
        self.trajectories = new_trajectories;
    }
    
    /// Train RL model on collected trajectories
    pub async fn train_on_trajectories(
        &mut self,
        loss_type: &str,  // "MINIRL" or "GRPO"
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.trajectories.len() < self.config.min_trajectories_for_training {
            return Err("Not enough trajectories for training".into());
        }
        
        let weights = self.calculate_importance_weights();
        
        let request = RLTrainingRequest {
            trajectories: self.trajectories.clone(),
            importance_weights: weights,
            loss_type: loss_type.to_string(),
        };
        
        let url = format!("{}/train", self.config.service_url);
        let response = self.http_client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_millis(self.config.request_timeout_ms))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("RL training failed: {}", response.status()).into());
        }
        
        let result: RLTrainingResponse = response.json().await?;
        
        // Update statistics
        self.stats.training_runs += 1;
        self.stats.trajectories_in_training += result.num_trajectories_used;
        
        println!(
            "[RL] Training completed: loss={:.4}, time={}ms, trajectories_used={}",
            result.loss, result.training_time_ms, result.num_trajectories_used
        );
        
        Ok(())
    }
    
    /// Store trajectory in MongoDB (async would be used in production)
    #[allow(dead_code)]
    fn store_trajectory_async(
        &self,
        _mongodb_url: &str,
        _trajectory: EvolutionTrajectory,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // MongoDB integration would be implemented here
        // For now, trajectories are stored in memory
        Ok(())
    }
    
    /// Get optimizer statistics
    pub fn get_statistics(&self) -> OptimzerStatistics {
        self.stats.clone()
    }
    
    /// Clear trajectory buffer (after successful training)
    pub fn clear_trajectories(&mut self) {
        self.trajectories.clear();
    }
    
    /// Get current service URL
    pub fn service_url(&self) -> &str {
        &self.config.service_url
    }
}

// =================================================================
// REWARD MODEL
// =================================================================

/// Simple reward model for convergence prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardModel {
    /// Mean reward from trajectories
    pub mean_reward: f64,
    
    /// Standard deviation of rewards
    pub std_reward: f64,
    
    /// Number of samples used to estimate statistics
    pub sample_count: usize,
}

impl RewardModel {
    /// Create a new reward model from trajectories
    pub fn from_trajectories(trajectories: &[EvolutionTrajectory]) -> Self {
        if trajectories.is_empty() {
            return RewardModel {
                mean_reward: 0.0,
                std_reward: 0.0,
                sample_count: 0,
            };
        }
        
        let mean_reward: f64 = trajectories.iter()
            .map(|t| t.reward)
            .sum::<f64>() / trajectories.len() as f64;
        
        let variance: f64 = trajectories.iter()
            .map(|t| (t.reward - mean_reward).powi(2))
            .sum::<f64>() / trajectories.len() as f64;
        
        let std_reward = variance.sqrt();
        
        RewardModel {
            mean_reward,
            std_reward,
            sample_count: trajectories.len(),
        }
    }
    
    /// Predict normalized reward for new observation
    pub fn predict_reward(&self, reward: f64) -> f64 {
        if self.std_reward > 0.0 {
            (reward - self.mean_reward) / self.std_reward
        } else {
            reward
        }
    }
}

// =================================================================
// TESTS (for integration testing)
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::profile::DeltaSource;
    
    #[test]
    fn test_trajectory_creation() {
        let profile = AiProfile::new("test_id", "test");
        
        let mut delta = PersonalityDelta::new(DeltaSource::Observation);
        delta.adjustments.push(("speech_pattern".to_string(), 0.1));
        delta.confidence = 0.8;
        
        let observation = BehaviorObservation {
            query: "test".to_string(),
            response: "response".to_string(),
            patterns: vec![],
            similarity_to_target: 0.7,
            confidence: 0.8,
        };
        
        let profile_after = AiProfile::new("test_id", "test");
        
        let trajectory = EvolutionTrajectory::new(
            profile,
            delta,
            observation,
            0.7,
            profile_after,
        );
        
        assert_eq!(trajectory.reward, 0.7);
        assert_eq!(trajectory.importance_weight, 1.0);
    }
    
    #[test]
    fn test_reward_model() {
        let profile = AiProfile::new("test_id", "test");
        let mut delta = PersonalityDelta::new(DeltaSource::Observation);
        delta.adjustments.push(("speech_pattern".to_string(), 0.1));
        
        let observation = BehaviorObservation::default();
        
        let trajectories = vec![
            EvolutionTrajectory::new(
                profile.clone(),
                delta.clone(),
                observation.clone(),
                0.5,
                profile.clone(),
            ),
            EvolutionTrajectory::new(
                profile.clone(),
                delta.clone(),
                observation.clone(),
                0.7,
                profile.clone(),
            ),
            EvolutionTrajectory::new(
                profile.clone(),
                delta.clone(),
                observation.clone(),
                0.9,
                profile.clone(),
            ),
        ];
        
        let model = RewardModel::from_trajectories(&trajectories);
        assert!(model.mean_reward > 0.6 && model.mean_reward < 0.8);
        assert!(model.std_reward > 0.1);
    }
}
