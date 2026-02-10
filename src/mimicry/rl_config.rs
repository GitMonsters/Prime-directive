// =================================================================
// REINFORCEMENT LEARNING CONFIGURATION
// =================================================================
// Configuration module for the AgentCPM + RustyWorm RL integration.
// Defines all RL parameters, service URLs, training hyperparameters,
// and feature flags for RL vs traditional evolution.
//
// USAGE:
//   - RLConfig::default() for local development
//   - RLConfig::production() for production deployment
//   - RLConfig::from_env() for environment-based configuration
// =================================================================

use serde::{Deserialize, Serialize};

// =================================================================
// RL CONFIGURATION
// =================================================================

/// Configuration for the Reinforcement Learning optimizer integration.
/// Controls all aspects of RL-based persona evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RLConfig {
    // =========================================================
    // SERVICE CONFIGURATION
    // =========================================================
    /// URL of the AgentRL HTTP service (e.g., "http://localhost:8888")
    pub service_url: String,

    /// MongoDB connection URL for trajectory storage
    pub mongodb_url: String,

    /// MongoDB database name for trajectories
    pub database_name: String,

    /// MongoDB collection name for trajectories
    pub collection_name: String,

    /// Timeout for HTTP requests in milliseconds
    pub request_timeout_ms: u64,

    /// Maximum number of retries for failed HTTP requests
    pub max_retries: u32,

    /// Delay between retries in milliseconds
    pub retry_delay_ms: u64,

    // =========================================================
    // TRAINING HYPERPARAMETERS
    // =========================================================
    /// Batch size for RL training
    pub batch_size: usize,

    /// Minimum trajectories required before starting training
    pub min_trajectories_for_training: usize,

    /// Maximum trajectories to keep in buffer before pruning
    pub max_trajectory_buffer: usize,

    /// Importance weight threshold for trajectory pruning (0.0-1.0)
    pub importance_threshold: f64,

    /// Learning rate for RL model updates
    pub learning_rate: f64,

    /// Discount factor (gamma) for reward calculation
    pub discount_factor: f64,

    /// Entropy coefficient for exploration
    pub entropy_coefficient: f64,

    /// Type of RL loss to use ("MINIRL" or "GRPO")
    pub loss_type: RLLossType,

    // =========================================================
    // CONVERGENCE THRESHOLDS
    // =========================================================
    /// Target convergence score (0.0-1.0) at which training completes
    pub target_convergence: f64,

    /// Convergence score at which RL switches from exploration to exploitation
    pub exploitation_threshold: f64,

    /// Minimum convergence improvement per training batch to continue
    pub min_improvement_threshold: f64,

    /// Number of training iterations with no improvement before early stopping
    pub early_stopping_patience: usize,

    /// Convergence score below which aggressive learning is used
    pub aggressive_learning_threshold: f64,

    // =========================================================
    // FEATURE FLAGS
    // =========================================================
    /// Enable RL-based evolution (false = traditional evolution only)
    pub rl_enabled: bool,

    /// Fallback to traditional evolution if RL service unavailable
    pub fallback_to_traditional: bool,

    /// Enable trajectory persistence to MongoDB
    pub persist_trajectories: bool,

    /// Enable importance weighting for trajectory selection
    pub use_importance_weighting: bool,

    /// Enable async multi-turn training
    pub async_training: bool,

    /// Enable reward shaping based on convergence metrics
    pub reward_shaping: bool,

    /// Enable auto-training when buffer is full
    pub auto_train_on_buffer_full: bool,

    /// Enable debug logging for RL operations
    pub debug_logging: bool,

    // =========================================================
    // OBSERVATION PARAMETERS
    // =========================================================
    /// Minimum observations before attempting RL optimization
    pub min_observations: usize,

    /// Maximum observations to include in a single RL update
    pub max_observations_per_update: usize,

    /// Weight given to recent observations vs older ones
    pub recency_weight: f64,
}

/// Type of RL loss function to use for training
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RLLossType {
    /// MINIRL loss from AgentCPM - stable importance-weighted learning
    MINIRL,
    /// GRPO (Group Relative Policy Optimization) for faster convergence
    GRPO,
    /// Hybrid: GRPO for early training, MINIRL for refinement
    Hybrid,
}

impl std::fmt::Display for RLLossType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RLLossType::MINIRL => write!(f, "MINIRL"),
            RLLossType::GRPO => write!(f, "GRPO"),
            RLLossType::Hybrid => write!(f, "Hybrid"),
        }
    }
}

impl Default for RLConfig {
    fn default() -> Self {
        RLConfig {
            // Service configuration
            service_url: "http://localhost:8888".to_string(),
            mongodb_url: "mongodb://root:password@localhost:27017".to_string(),
            database_name: "rustyworm_rl".to_string(),
            collection_name: "trajectories".to_string(),
            request_timeout_ms: 30_000,
            max_retries: 3,
            retry_delay_ms: 1_000,

            // Training hyperparameters
            batch_size: 64,
            min_trajectories_for_training: 50,
            max_trajectory_buffer: 1000,
            importance_threshold: 0.3,
            learning_rate: 0.001,
            discount_factor: 0.99,
            entropy_coefficient: 0.01,
            loss_type: RLLossType::MINIRL,

            // Convergence thresholds
            target_convergence: 0.90,
            exploitation_threshold: 0.75,
            min_improvement_threshold: 0.001,
            early_stopping_patience: 10,
            aggressive_learning_threshold: 0.50,

            // Feature flags
            rl_enabled: true,
            fallback_to_traditional: true,
            persist_trajectories: true,
            use_importance_weighting: true,
            async_training: true,
            reward_shaping: true,
            auto_train_on_buffer_full: true,
            debug_logging: false,

            // Observation parameters
            min_observations: 3,
            max_observations_per_update: 10,
            recency_weight: 0.7,
        }
    }
}

impl RLConfig {
    /// Creates a new RLConfig with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a production-ready configuration with optimized settings
    pub fn production() -> Self {
        RLConfig {
            service_url: "http://agentrl-service:8888".to_string(),
            mongodb_url: "mongodb://root:password@mongodb:27017".to_string(),
            request_timeout_ms: 60_000,
            max_retries: 5,
            batch_size: 128,
            min_trajectories_for_training: 100,
            max_trajectory_buffer: 5000,
            target_convergence: 0.95,
            rl_enabled: true,
            fallback_to_traditional: true,
            debug_logging: false,
            ..Default::default()
        }
    }

    /// Creates a configuration optimized for fast local development
    pub fn development() -> Self {
        RLConfig {
            min_trajectories_for_training: 10,
            batch_size: 16,
            max_trajectory_buffer: 100,
            target_convergence: 0.80,
            debug_logging: true,
            ..Default::default()
        }
    }

    /// Creates a configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = RLConfig::default();

        if let Ok(url) = std::env::var("AGENTRL_SERVICE_URL") {
            config.service_url = url;
        }
        if let Ok(url) = std::env::var("MONGODB_URL") {
            config.mongodb_url = url;
        }
        if let Ok(db) = std::env::var("RUSTYWORM_RL_DB") {
            config.database_name = db;
        }
        if let Ok(enabled) = std::env::var("RL_ENABLED") {
            config.rl_enabled = enabled.to_lowercase() == "true" || enabled == "1";
        }
        if let Ok(debug) = std::env::var("RL_DEBUG") {
            config.debug_logging = debug.to_lowercase() == "true" || debug == "1";
        }
        if let Ok(target) = std::env::var("RL_TARGET_CONVERGENCE") {
            if let Ok(val) = target.parse::<f64>() {
                config.target_convergence = val.clamp(0.0, 1.0);
            }
        }
        if let Ok(batch) = std::env::var("RL_BATCH_SIZE") {
            if let Ok(val) = batch.parse::<usize>() {
                config.batch_size = val;
            }
        }
        if let Ok(loss) = std::env::var("RL_LOSS_TYPE") {
            config.loss_type = match loss.to_uppercase().as_str() {
                "GRPO" => RLLossType::GRPO,
                "HYBRID" => RLLossType::Hybrid,
                _ => RLLossType::MINIRL,
            };
        }

        config
    }

    /// Creates a configuration with RL disabled (traditional evolution only)
    pub fn traditional_only() -> Self {
        RLConfig {
            rl_enabled: false,
            ..Default::default()
        }
    }

    /// Builder method to set service URL
    pub fn with_service_url(mut self, url: &str) -> Self {
        self.service_url = url.to_string();
        self
    }

    /// Builder method to set MongoDB URL
    pub fn with_mongodb_url(mut self, url: &str) -> Self {
        self.mongodb_url = url.to_string();
        self
    }

    /// Builder method to set batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Builder method to set target convergence
    pub fn with_target_convergence(mut self, target: f64) -> Self {
        self.target_convergence = target.clamp(0.0, 1.0);
        self
    }

    /// Builder method to enable/disable RL
    pub fn with_rl_enabled(mut self, enabled: bool) -> Self {
        self.rl_enabled = enabled;
        self
    }

    /// Builder method to set loss type
    pub fn with_loss_type(mut self, loss_type: RLLossType) -> Self {
        self.loss_type = loss_type;
        self
    }

    /// Builder method to enable debug logging
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug_logging = debug;
        self
    }

    /// Validate the configuration and return any errors
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.service_url.is_empty() {
            errors.push("service_url cannot be empty".to_string());
        }
        if self.batch_size == 0 {
            errors.push("batch_size must be > 0".to_string());
        }
        if self.min_trajectories_for_training == 0 {
            errors.push("min_trajectories_for_training must be > 0".to_string());
        }
        if self.target_convergence <= 0.0 || self.target_convergence > 1.0 {
            errors.push("target_convergence must be in (0.0, 1.0]".to_string());
        }
        if self.importance_threshold < 0.0 || self.importance_threshold > 1.0 {
            errors.push("importance_threshold must be in [0.0, 1.0]".to_string());
        }
        if self.learning_rate <= 0.0 {
            errors.push("learning_rate must be > 0".to_string());
        }
        if self.discount_factor < 0.0 || self.discount_factor > 1.0 {
            errors.push("discount_factor must be in [0.0, 1.0]".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get a summary of the configuration
    pub fn summary(&self) -> String {
        format!(
            "RLConfig:\n\
             - Service: {}\n\
             - RL Enabled: {}\n\
             - Loss Type: {}\n\
             - Batch Size: {}\n\
             - Target Convergence: {:.1}%\n\
             - Min Trajectories: {}\n\
             - Fallback: {}\n\
             - Debug: {}",
            self.service_url,
            self.rl_enabled,
            self.loss_type,
            self.batch_size,
            self.target_convergence * 100.0,
            self.min_trajectories_for_training,
            self.fallback_to_traditional,
            self.debug_logging,
        )
    }
}

// =================================================================
// RL STATISTICS
// =================================================================

/// Statistics tracking for RL operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RLStatistics {
    /// Total trajectories collected
    pub total_trajectories: usize,

    /// Trajectories used in training
    pub trajectories_trained: usize,

    /// Number of training runs completed
    pub training_runs: usize,

    /// Total training time in milliseconds
    pub total_training_time_ms: u64,

    /// Average reward across trajectories
    pub avg_reward: f64,

    /// Best reward achieved
    pub best_reward: f64,

    /// Average importance weight
    pub avg_importance_weight: f64,

    /// Number of RL predictions made
    pub predictions_made: usize,

    /// Number of fallbacks to traditional evolution
    pub fallback_count: usize,

    /// Number of service errors encountered
    pub service_errors: usize,

    /// Convergence improvement from RL (vs baseline)
    pub convergence_improvement: f64,

    /// Whether RL service is currently available
    pub service_available: bool,
}

impl RLStatistics {
    /// Creates new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new trajectory collection
    pub fn record_trajectory(&mut self, reward: f64) {
        self.total_trajectories += 1;

        // Update average reward using incremental mean
        self.avg_reward =
            self.avg_reward + (reward - self.avg_reward) / self.total_trajectories as f64;

        if reward > self.best_reward {
            self.best_reward = reward;
        }
    }

    /// Record a training run
    pub fn record_training(&mut self, trajectories_used: usize, time_ms: u64) {
        self.training_runs += 1;
        self.trajectories_trained += trajectories_used;
        self.total_training_time_ms += time_ms;
    }

    /// Record a prediction
    pub fn record_prediction(&mut self) {
        self.predictions_made += 1;
    }

    /// Record a fallback to traditional evolution
    pub fn record_fallback(&mut self) {
        self.fallback_count += 1;
    }

    /// Record a service error
    pub fn record_error(&mut self) {
        self.service_errors += 1;
    }

    /// Get summary string
    pub fn summary(&self) -> String {
        format!(
            "RL Statistics:\n\
             - Trajectories: {} collected, {} trained\n\
             - Training Runs: {}\n\
             - Avg Reward: {:.4}\n\
             - Best Reward: {:.4}\n\
             - Predictions: {}\n\
             - Fallbacks: {}\n\
             - Errors: {}\n\
             - Convergence Improvement: {:.2}%",
            self.total_trajectories,
            self.trajectories_trained,
            self.training_runs,
            self.avg_reward,
            self.best_reward,
            self.predictions_made,
            self.fallback_count,
            self.service_errors,
            self.convergence_improvement * 100.0,
        )
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RLConfig::default();
        assert!(config.rl_enabled);
        assert!(config.fallback_to_traditional);
        assert_eq!(config.target_convergence, 0.90);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config() {
        let config = RLConfig::production();
        assert_eq!(config.batch_size, 128);
        assert_eq!(config.target_convergence, 0.95);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_development_config() {
        let config = RLConfig::development();
        assert_eq!(config.min_trajectories_for_training, 10);
        assert!(config.debug_logging);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_traditional_only_config() {
        let config = RLConfig::traditional_only();
        assert!(!config.rl_enabled);
    }

    #[test]
    fn test_builder_pattern() {
        let config = RLConfig::new()
            .with_service_url("http://custom:9999")
            .with_batch_size(256)
            .with_target_convergence(0.85)
            .with_debug(true);

        assert_eq!(config.service_url, "http://custom:9999");
        assert_eq!(config.batch_size, 256);
        assert_eq!(config.target_convergence, 0.85);
        assert!(config.debug_logging);
    }

    #[test]
    fn test_validation_errors() {
        let mut config = RLConfig::default();
        config.batch_size = 0;
        config.target_convergence = 1.5;

        let result = config.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2);
    }

    #[test]
    fn test_loss_type_display() {
        assert_eq!(format!("{}", RLLossType::MINIRL), "MINIRL");
        assert_eq!(format!("{}", RLLossType::GRPO), "GRPO");
        assert_eq!(format!("{}", RLLossType::Hybrid), "Hybrid");
    }

    #[test]
    fn test_rl_statistics() {
        let mut stats = RLStatistics::new();

        stats.record_trajectory(0.5);
        stats.record_trajectory(0.7);
        assert_eq!(stats.total_trajectories, 2);
        assert!((stats.avg_reward - 0.6).abs() < 0.001);
        assert_eq!(stats.best_reward, 0.7);

        stats.record_training(10, 1000);
        assert_eq!(stats.training_runs, 1);
        assert_eq!(stats.trajectories_trained, 10);

        stats.record_prediction();
        stats.record_fallback();
        stats.record_error();

        let summary = stats.summary();
        assert!(summary.contains("Trajectories"));
    }

    #[test]
    fn test_config_serialization() {
        let config = RLConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: RLConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.service_url, config.service_url);
        assert_eq!(restored.batch_size, config.batch_size);
    }

    #[test]
    fn test_config_summary() {
        let config = RLConfig::default();
        let summary = config.summary();
        assert!(summary.contains("RLConfig"));
        assert!(summary.contains("localhost:8888"));
    }
}
