// =================================================================
// MULTI-MODEL OBSERVER: Parallel Observation & Consensus Building
// =================================================================
// Orchestrates observation across multiple AI models simultaneously
// for cross-validation, consensus building, and model selection.
//
// This module provides:
// - Parallel observation across multiple models
// - Cross-validation between model responses
// - Consensus building from multiple observations
// - Task-based model selection strategies
// - Fallback handling when models are unavailable
//
// INTEGRATION:
// - Uses AgentDockBridge for container orchestration
// - Feeds observations into evolution tracking
// - Supports RL optimizer trajectory collection
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mimicry::agentdock_bridge::{AgentDockBridge, ModelConfig};
use crate::mimicry::rl_optimizer::BehaviorObservation;

// =================================================================
// TASK TYPES FOR MODEL SELECTION
// =================================================================

/// Types of tasks for intelligent model selection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskType {
    /// General chat/conversation
    Chat,
    /// Code generation or analysis
    Coding,
    /// Mathematical reasoning
    Math,
    /// Creative writing
    Creative,
    /// Factual knowledge retrieval
    Knowledge,
    /// Tool use and function calling
    ToolUse,
    /// Long-form analysis
    Analysis,
    /// Multi-turn conversation
    MultiTurn,
    /// Unknown or mixed task type
    Unknown,
}

impl TaskType {
    /// Detect task type from query content
    pub fn detect_from_query(query: &str) -> Self {
        let query_lower = query.to_lowercase();
        
        // Check for code indicators
        if query_lower.contains("code") 
            || query_lower.contains("function")
            || query_lower.contains("implement")
            || query_lower.contains("program")
            || query_lower.contains("debug")
            || query_lower.contains("```")
        {
            return TaskType::Coding;
        }
        
        // Check for math indicators
        if query_lower.contains("calculate")
            || query_lower.contains("solve")
            || query_lower.contains("equation")
            || query_lower.contains("math")
            || query_lower.contains("formula")
        {
            return TaskType::Math;
        }
        
        // Check for creative indicators
        if query_lower.contains("write a story")
            || query_lower.contains("write a poem")
            || query_lower.contains("creative")
            || query_lower.contains("imagine")
            || query_lower.contains("fiction")
        {
            return TaskType::Creative;
        }
        
        // Check for tool use indicators
        if query_lower.contains("search")
            || query_lower.contains("look up")
            || query_lower.contains("find")
            || query_lower.contains("use the")
        {
            return TaskType::ToolUse;
        }
        
        // Check for analysis indicators
        if query_lower.contains("analyze")
            || query_lower.contains("compare")
            || query_lower.contains("evaluate")
            || query_lower.contains("explain")
            || query_lower.len() > 500
        {
            return TaskType::Analysis;
        }
        
        // Default to chat
        TaskType::Chat
    }
}

// =================================================================
// MODEL SCHEDULER
// =================================================================

/// Scheduling strategies for model selection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SchedulingStrategy {
    /// Use highest priority available model
    Priority,
    /// Round-robin across available models
    RoundRobin,
    /// Use all models simultaneously
    AllModels,
    /// Task-based model selection
    TaskBased,
    /// Random selection
    Random,
    /// Weighted random based on priority
    WeightedRandom,
}

/// Model scheduler for selecting models
#[derive(Debug, Clone)]
pub struct ModelScheduler {
    /// Current scheduling strategy
    strategy: SchedulingStrategy,
    
    /// Round-robin index for RoundRobin strategy
    round_robin_idx: usize,
    
    /// Task-to-model preferences
    task_preferences: HashMap<TaskType, Vec<String>>,
}

impl ModelScheduler {
    /// Create a new scheduler with default strategy
    pub fn new() -> Self {
        let mut task_preferences = HashMap::new();
        
        // Default task preferences (can be customized)
        task_preferences.insert(TaskType::Coding, vec!["gpt4".to_string(), "claude".to_string()]);
        task_preferences.insert(TaskType::Math, vec!["gpt4".to_string(), "gemini".to_string()]);
        task_preferences.insert(TaskType::Creative, vec!["claude".to_string(), "gpt4".to_string()]);
        task_preferences.insert(TaskType::Chat, vec!["llama".to_string(), "gpt4".to_string()]);
        
        ModelScheduler {
            strategy: SchedulingStrategy::Priority,
            round_robin_idx: 0,
            task_preferences,
        }
    }
    
    /// Set scheduling strategy
    pub fn with_strategy(mut self, strategy: SchedulingStrategy) -> Self {
        self.strategy = strategy;
        self
    }
    
    /// Set task preference
    pub fn set_task_preference(&mut self, task: TaskType, model_ids: Vec<String>) {
        self.task_preferences.insert(task, model_ids);
    }
    
    /// Select model(s) based on strategy
    pub fn select_models(
        &mut self,
        available: &[&ModelConfig],
        task_type: Option<TaskType>,
    ) -> Vec<String> {
        if available.is_empty() {
            return vec![];
        }
        
        match self.strategy {
            SchedulingStrategy::Priority => {
                // Return highest priority model
                vec![available[0].name.clone()]
            }
            
            SchedulingStrategy::RoundRobin => {
                // Cycle through models
                let idx = self.round_robin_idx % available.len();
                self.round_robin_idx += 1;
                vec![available[idx].name.clone()]
            }
            
            SchedulingStrategy::AllModels => {
                // Return all available models
                available.iter().map(|m| m.name.clone()).collect()
            }
            
            SchedulingStrategy::TaskBased => {
                if let Some(task) = task_type {
                    if let Some(preferred) = self.task_preferences.get(&task) {
                        // Find first preferred model that is available
                        for pref in preferred {
                            if available.iter().any(|m| &m.name == pref) {
                                return vec![pref.clone()];
                            }
                        }
                    }
                }
                // Fall back to priority
                vec![available[0].name.clone()]
            }
            
            SchedulingStrategy::Random => {
                use rand::Rng;
                let idx = rand::thread_rng().gen_range(0..available.len());
                vec![available[idx].name.clone()]
            }
            
            SchedulingStrategy::WeightedRandom => {
                // Weighted random based on priority
                let total_priority: u32 = available.iter().map(|m| m.priority as u32).sum();
                if total_priority == 0 {
                    return vec![available[0].name.clone()];
                }
                
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let mut target = rng.gen_range(0..total_priority);
                
                for model in available {
                    if target < model.priority as u32 {
                        return vec![model.name.clone()];
                    }
                    target -= model.priority as u32;
                }
                
                vec![available[0].name.clone()]
            }
        }
    }
}

impl Default for ModelScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================
// CONSENSUS BUILDER
// =================================================================

/// Result of consensus building from multiple observations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusObservation {
    /// Original query
    pub query: String,
    
    /// Consensus response (merged or selected best)
    pub consensus_response: String,
    
    /// Agreement score (0.0 to 1.0)
    pub agreement_score: f64,
    
    /// Individual observations from each model
    pub individual_observations: Vec<ModelObservation>,
    
    /// Common patterns found across all models
    pub common_patterns: Vec<String>,
    
    /// Divergent patterns (unique to specific models)
    pub divergent_patterns: HashMap<String, Vec<String>>,
    
    /// Confidence in the consensus
    pub confidence: f64,
    
    /// Model that contributed most to consensus
    pub primary_model: String,
}

/// Individual model's observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelObservation {
    /// Model identifier
    pub model_id: String,
    
    /// The observation
    pub observation: BehaviorObservation,
    
    /// Weight given to this observation in consensus
    pub weight: f64,
}

/// Consensus building strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsensusStrategy {
    /// Use majority voting on patterns
    MajorityVote,
    /// Use response from highest priority model
    HighestPriority,
    /// Merge responses to create composite
    Merge,
    /// Use the longest/most detailed response
    MostDetailed,
    /// Use the response with highest confidence
    HighestConfidence,
}

/// Builder for creating consensus from multiple observations
#[derive(Debug, Clone)]
pub struct ConsensusBuilder {
    /// Consensus building strategy
    strategy: ConsensusStrategy,
    
    /// Minimum agreement threshold
    min_agreement: f64,
}

impl ConsensusBuilder {
    /// Create a new consensus builder
    pub fn new() -> Self {
        ConsensusBuilder {
            strategy: ConsensusStrategy::MajorityVote,
            min_agreement: 0.5,
        }
    }
    
    /// Set consensus strategy
    pub fn with_strategy(mut self, strategy: ConsensusStrategy) -> Self {
        self.strategy = strategy;
        self
    }
    
    /// Set minimum agreement threshold
    pub fn with_min_agreement(mut self, threshold: f64) -> Self {
        self.min_agreement = threshold.clamp(0.0, 1.0);
        self
    }
    
    /// Build consensus from multiple observations
    pub fn build_consensus(
        &self,
        query: &str,
        observations: &HashMap<String, Vec<BehaviorObservation>>,
        model_priorities: &HashMap<String, u8>,
    ) -> ConsensusObservation {
        if observations.is_empty() {
            return ConsensusObservation {
                query: query.to_string(),
                consensus_response: String::new(),
                agreement_score: 0.0,
                individual_observations: vec![],
                common_patterns: vec![],
                divergent_patterns: HashMap::new(),
                confidence: 0.0,
                primary_model: String::new(),
            };
        }
        
        // Collect all patterns and responses
        let mut all_patterns: HashMap<String, usize> = HashMap::new();
        let mut individual_obs: Vec<ModelObservation> = Vec::new();
        let mut model_patterns: HashMap<String, Vec<String>> = HashMap::new();
        
        for (model_id, obs_list) in observations {
            for obs in obs_list {
                let priority = model_priorities.get(model_id).copied().unwrap_or(1);
                let weight = priority as f64 / 10.0;
                
                individual_obs.push(ModelObservation {
                    model_id: model_id.clone(),
                    observation: obs.clone(),
                    weight,
                });
                
                // Count patterns
                for pattern in &obs.patterns {
                    *all_patterns.entry(pattern.clone()).or_insert(0) += 1;
                }
                
                // Track per-model patterns
                model_patterns.entry(model_id.clone())
                    .or_default()
                    .extend(obs.patterns.clone());
            }
        }
        
        let model_count = observations.len();
        
        // Find common patterns (appear in majority of models)
        let common_patterns: Vec<String> = all_patterns.iter()
            .filter(|(_, count)| **count >= (model_count + 1) / 2)
            .map(|(pattern, _)| pattern.clone())
            .collect();
        
        // Find divergent patterns (unique to specific models)
        let divergent_patterns: HashMap<String, Vec<String>> = model_patterns.iter()
            .map(|(model_id, patterns)| {
                let unique: Vec<String> = patterns.iter()
                    .filter(|p| !common_patterns.contains(p))
                    .cloned()
                    .collect();
                (model_id.clone(), unique)
            })
            .filter(|(_, v)| !v.is_empty())
            .collect();
        
        // Calculate agreement score based on common patterns
        let total_unique_patterns = all_patterns.len();
        let agreement_score = if total_unique_patterns > 0 {
            common_patterns.len() as f64 / total_unique_patterns as f64
        } else {
            1.0  // No patterns = full agreement
        };
        
        // Select primary model and consensus response based on strategy
        let (primary_model, consensus_response) = match self.strategy {
            ConsensusStrategy::HighestPriority => {
                let best = individual_obs.iter()
                    .max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
                    .unwrap();
                (best.model_id.clone(), best.observation.response.clone())
            }
            
            ConsensusStrategy::MostDetailed => {
                let best = individual_obs.iter()
                    .max_by_key(|o| o.observation.response.len())
                    .unwrap();
                (best.model_id.clone(), best.observation.response.clone())
            }
            
            ConsensusStrategy::HighestConfidence => {
                let best = individual_obs.iter()
                    .max_by(|a, b| a.observation.confidence.partial_cmp(&b.observation.confidence).unwrap())
                    .unwrap();
                (best.model_id.clone(), best.observation.response.clone())
            }
            
            ConsensusStrategy::Merge => {
                // Simple merge: use highest priority response as base
                let best = individual_obs.iter()
                    .max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
                    .unwrap();
                (best.model_id.clone(), best.observation.response.clone())
                // TODO: Implement actual response merging
            }
            
            ConsensusStrategy::MajorityVote => {
                // Use response from model with most common patterns
                let model_common_count: HashMap<String, usize> = individual_obs.iter()
                    .map(|o| {
                        let count = o.observation.patterns.iter()
                            .filter(|p| common_patterns.contains(p))
                            .count();
                        (o.model_id.clone(), count)
                    })
                    .collect();
                
                let best_model = model_common_count.iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(m, _)| m.clone())
                    .unwrap_or_default();
                
                let response = individual_obs.iter()
                    .find(|o| o.model_id == best_model)
                    .map(|o| o.observation.response.clone())
                    .unwrap_or_default();
                
                (best_model, response)
            }
        };
        
        // Calculate confidence based on agreement and observation confidence
        let avg_confidence: f64 = individual_obs.iter()
            .map(|o| o.observation.confidence)
            .sum::<f64>() / individual_obs.len().max(1) as f64;
        let confidence = agreement_score * avg_confidence;
        
        ConsensusObservation {
            query: query.to_string(),
            consensus_response,
            agreement_score,
            individual_observations: individual_obs,
            common_patterns,
            divergent_patterns,
            confidence,
            primary_model,
        }
    }
}

impl Default for ConsensusBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================
// MULTI-MODEL OBSERVER
// =================================================================

/// Result of multi-model observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModelResult {
    /// Query that was observed
    pub query: String,
    
    /// Observations from each model
    pub observations: HashMap<String, Vec<BehaviorObservation>>,
    
    /// Models that failed to respond
    pub failed_models: Vec<String>,
    
    /// Total observation time in ms
    pub total_time_ms: u64,
    
    /// Task type detected for this query
    pub task_type: TaskType,
}

/// Main multi-model observer for parallel observation
pub struct MultiModelObserver {
    /// AgentDock bridge for container orchestration
    bridge: AgentDockBridge,
    
    /// Model scheduler
    scheduler: ModelScheduler,
    
    /// Consensus builder
    consensus_builder: ConsensusBuilder,
    
    /// Fallback model ID (used when AgentDock unavailable)
    fallback_model: Option<String>,
    
    /// Statistics
    stats: MultiModelStats,
}

/// Statistics for multi-model observer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiModelStats {
    /// Total observations made
    pub total_observations: usize,
    
    /// Successful observations
    pub successful_observations: usize,
    
    /// Failed observations
    pub failed_observations: usize,
    
    /// Consensus observations built
    pub consensus_built: usize,
    
    /// Average agreement score
    pub avg_agreement_score: f64,
    
    /// Average observation time in ms
    pub avg_observation_time_ms: f64,
}

impl MultiModelObserver {
    /// Create a new multi-model observer
    pub fn new(bridge: AgentDockBridge) -> Self {
        MultiModelObserver {
            bridge,
            scheduler: ModelScheduler::new(),
            consensus_builder: ConsensusBuilder::new(),
            fallback_model: None,
            stats: MultiModelStats::default(),
        }
    }
    
    /// Set the model scheduler
    pub fn with_scheduler(mut self, scheduler: ModelScheduler) -> Self {
        self.scheduler = scheduler;
        self
    }
    
    /// Set the consensus builder
    pub fn with_consensus_builder(mut self, builder: ConsensusBuilder) -> Self {
        self.consensus_builder = builder;
        self
    }
    
    /// Set fallback model for when AgentDock is unavailable
    pub fn with_fallback(mut self, model_id: &str) -> Self {
        self.fallback_model = Some(model_id.to_string());
        self
    }
    
    /// Get mutable reference to bridge
    pub fn bridge_mut(&mut self) -> &mut AgentDockBridge {
        &mut self.bridge
    }
    
    /// Get reference to bridge
    pub fn bridge(&self) -> &AgentDockBridge {
        &self.bridge
    }
    
    /// Register a model with the bridge
    pub fn register_model(&mut self, model_id: &str, config: ModelConfig) {
        self.bridge.register_model(model_id, config);
    }
    
    /// Observe all available models with a query
    pub async fn observe_all(&mut self, query: &str) -> Result<MultiModelResult, String> {
        let start = std::time::Instant::now();
        let task_type = TaskType::detect_from_query(query);
        
        // Get available models
        let available = self.bridge.list_available_models();
        let model_names: Vec<String> = available.iter().map(|m| m.name.clone()).collect();
        
        if model_names.is_empty() {
            return Err("No models available for observation".to_string());
        }
        
        // Observe each model
        let model_refs: Vec<&str> = model_names.iter().map(|s| s.as_str()).collect();
        let observations = self.bridge.observe_multi_model(query, &model_refs).await?;
        
        let elapsed = start.elapsed().as_millis() as u64;
        
        // Collect failed models
        let failed_models: Vec<String> = model_names.iter()
            .filter(|m| !observations.contains_key(*m) || observations.get(*m).map(|v| v.is_empty()).unwrap_or(true))
            .cloned()
            .collect();
        
        // Update stats
        self.stats.total_observations += model_names.len();
        self.stats.successful_observations += observations.len() - failed_models.len();
        self.stats.failed_observations += failed_models.len();
        
        let total = self.stats.total_observations as f64;
        self.stats.avg_observation_time_ms = 
            (self.stats.avg_observation_time_ms * (total - 1.0) + elapsed as f64) / total;
        
        Ok(MultiModelResult {
            query: query.to_string(),
            observations,
            failed_models,
            total_time_ms: elapsed,
            task_type,
        })
    }
    
    /// Observe and build consensus from multiple models
    pub async fn observe_with_consensus(&mut self, query: &str) -> Result<ConsensusObservation, String> {
        // First observe all models
        let result = self.observe_all(query).await?;
        
        if result.observations.is_empty() {
            return Err("No observations collected for consensus".to_string());
        }
        
        // Build model priorities map
        let model_priorities: HashMap<String, u8> = self.bridge.list_models()
            .iter()
            .filter_map(|m| {
                self.bridge.get_model(m).map(|config| (m.clone(), config.priority))
            })
            .collect();
        
        // Build consensus
        let consensus = self.consensus_builder.build_consensus(
            query,
            &result.observations,
            &model_priorities,
        );
        
        // Update stats
        self.stats.consensus_built += 1;
        let count = self.stats.consensus_built as f64;
        self.stats.avg_agreement_score = 
            (self.stats.avg_agreement_score * (count - 1.0) + consensus.agreement_score) / count;
        
        Ok(consensus)
    }
    
    /// Select best model for a given task type
    pub fn select_best_model(&mut self, task_type: TaskType) -> Option<String> {
        let available = self.bridge.list_available_models();
        if available.is_empty() {
            return None;
        }
        
        let selected = self.scheduler.select_models(&available, Some(task_type));
        selected.into_iter().next()
    }
    
    /// Observe using selected models based on strategy
    pub async fn observe_with_selection(
        &mut self,
        query: &str,
    ) -> Result<MultiModelResult, String> {
        let start = std::time::Instant::now();
        let task_type = TaskType::detect_from_query(query);
        
        // Get available models
        let available = self.bridge.list_available_models();
        if available.is_empty() {
            return Err("No models available".to_string());
        }
        
        // Select models based on strategy
        let selected = self.scheduler.select_models(&available, Some(task_type.clone()));
        
        if selected.is_empty() {
            return Err("No models selected by scheduler".to_string());
        }
        
        // Observe selected models
        let model_refs: Vec<&str> = selected.iter().map(|s| s.as_str()).collect();
        let observations = self.bridge.observe_multi_model(query, &model_refs).await?;
        
        let elapsed = start.elapsed().as_millis() as u64;
        
        let failed_models: Vec<String> = selected.iter()
            .filter(|m| !observations.contains_key(*m) || observations.get(*m).map(|v| v.is_empty()).unwrap_or(true))
            .cloned()
            .collect();
        
        self.stats.total_observations += selected.len();
        self.stats.successful_observations += observations.len() - failed_models.len();
        self.stats.failed_observations += failed_models.len();
        
        Ok(MultiModelResult {
            query: query.to_string(),
            observations,
            failed_models,
            total_time_ms: elapsed,
            task_type,
        })
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &MultiModelStats {
        &self.stats
    }
    
    /// Get status summary
    pub fn status_summary(&self) -> String {
        let mut lines = vec!["=== MULTI-MODEL OBSERVER STATUS ===".to_string()];
        
        lines.push(String::new());
        lines.push("Statistics:".to_string());
        lines.push(format!("  Total observations: {}", self.stats.total_observations));
        lines.push(format!("  Successful: {}", self.stats.successful_observations));
        lines.push(format!("  Failed: {}", self.stats.failed_observations));
        lines.push(format!("  Consensus built: {}", self.stats.consensus_built));
        lines.push(format!("  Avg agreement: {:.1}%", self.stats.avg_agreement_score * 100.0));
        lines.push(format!("  Avg observation time: {:.1}ms", self.stats.avg_observation_time_ms));
        
        if let Some(ref fallback) = self.fallback_model {
            lines.push(format!("  Fallback model: {}", fallback));
        }
        
        lines.push(String::new());
        lines.push(self.bridge.status_summary());
        
        lines.join("\n")
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_type_detection() {
        assert_eq!(TaskType::detect_from_query("Write a function to sort an array"), TaskType::Coding);
        assert_eq!(TaskType::detect_from_query("Calculate 2+2"), TaskType::Math);
        assert_eq!(TaskType::detect_from_query("Write a poem about nature"), TaskType::Creative);
        assert_eq!(TaskType::detect_from_query("Search for information about Rust"), TaskType::ToolUse);
        assert_eq!(TaskType::detect_from_query("Hello, how are you?"), TaskType::Chat);
    }
    
    #[test]
    fn test_model_scheduler_priority() {
        let mut scheduler = ModelScheduler::new().with_strategy(SchedulingStrategy::Priority);
        
        let models = vec![
            ModelConfig::new("low", "http://a").with_priority(1),
            ModelConfig::new("high", "http://b").with_priority(10),
        ];
        let refs: Vec<&ModelConfig> = models.iter().collect();
        
        let selected = scheduler.select_models(&refs, None);
        assert_eq!(selected.len(), 1);
        // Note: models are passed as-is, scheduler doesn't sort by priority
    }
    
    #[test]
    fn test_model_scheduler_all_models() {
        let mut scheduler = ModelScheduler::new().with_strategy(SchedulingStrategy::AllModels);
        
        let models = vec![
            ModelConfig::new("a", "http://a"),
            ModelConfig::new("b", "http://b"),
            ModelConfig::new("c", "http://c"),
        ];
        let refs: Vec<&ModelConfig> = models.iter().collect();
        
        let selected = scheduler.select_models(&refs, None);
        assert_eq!(selected.len(), 3);
    }
    
    #[test]
    fn test_model_scheduler_round_robin() {
        let mut scheduler = ModelScheduler::new().with_strategy(SchedulingStrategy::RoundRobin);
        
        let models = vec![
            ModelConfig::new("a", "http://a"),
            ModelConfig::new("b", "http://b"),
        ];
        let refs: Vec<&ModelConfig> = models.iter().collect();
        
        let first = scheduler.select_models(&refs, None);
        let second = scheduler.select_models(&refs, None);
        let third = scheduler.select_models(&refs, None);
        
        assert_eq!(first[0], "a");
        assert_eq!(second[0], "b");
        assert_eq!(third[0], "a"); // Wraps around
    }
    
    #[test]
    fn test_consensus_builder_empty() {
        let builder = ConsensusBuilder::new();
        let observations: HashMap<String, Vec<BehaviorObservation>> = HashMap::new();
        let priorities: HashMap<String, u8> = HashMap::new();
        
        let consensus = builder.build_consensus("test", &observations, &priorities);
        
        assert_eq!(consensus.agreement_score, 0.0);
        assert!(consensus.consensus_response.is_empty());
    }
    
    #[test]
    fn test_consensus_builder_single_model() {
        let builder = ConsensusBuilder::new();
        
        let mut observations: HashMap<String, Vec<BehaviorObservation>> = HashMap::new();
        observations.insert("model-a".to_string(), vec![
            BehaviorObservation {
                query: "test".to_string(),
                response: "Test response".to_string(),
                patterns: vec!["pattern1".to_string()],
                similarity_to_target: 0.8,
                confidence: 0.9,
            }
        ]);
        
        let mut priorities: HashMap<String, u8> = HashMap::new();
        priorities.insert("model-a".to_string(), 10);
        
        let consensus = builder.build_consensus("test", &observations, &priorities);
        
        assert_eq!(consensus.agreement_score, 1.0); // Only one model
        assert_eq!(consensus.primary_model, "model-a");
    }
    
    #[test]
    fn test_consensus_builder_multiple_models() {
        let builder = ConsensusBuilder::new().with_strategy(ConsensusStrategy::HighestPriority);
        
        let mut observations: HashMap<String, Vec<BehaviorObservation>> = HashMap::new();
        observations.insert("model-a".to_string(), vec![
            BehaviorObservation {
                query: "test".to_string(),
                response: "Response A".to_string(),
                patterns: vec!["common".to_string(), "unique_a".to_string()],
                similarity_to_target: 0.8,
                confidence: 0.9,
            }
        ]);
        observations.insert("model-b".to_string(), vec![
            BehaviorObservation {
                query: "test".to_string(),
                response: "Response B".to_string(),
                patterns: vec!["common".to_string(), "unique_b".to_string()],
                similarity_to_target: 0.7,
                confidence: 0.8,
            }
        ]);
        
        let mut priorities: HashMap<String, u8> = HashMap::new();
        priorities.insert("model-a".to_string(), 5);
        priorities.insert("model-b".to_string(), 10);
        
        let consensus = builder.build_consensus("test", &observations, &priorities);
        
        // "common" appears in both, "unique_a" and "unique_b" are divergent
        assert!(consensus.common_patterns.contains(&"common".to_string()));
        assert!(consensus.agreement_score > 0.0 && consensus.agreement_score < 1.0);
        
        // Highest priority is model-b
        assert_eq!(consensus.primary_model, "model-b");
        assert_eq!(consensus.consensus_response, "Response B");
    }
    
    #[test]
    fn test_consensus_observation_serialization() {
        let consensus = ConsensusObservation {
            query: "test".to_string(),
            consensus_response: "response".to_string(),
            agreement_score: 0.8,
            individual_observations: vec![],
            common_patterns: vec!["p1".to_string()],
            divergent_patterns: HashMap::new(),
            confidence: 0.85,
            primary_model: "model-a".to_string(),
        };
        
        let json = serde_json::to_string(&consensus).unwrap();
        let restored: ConsensusObservation = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored.query, "test");
        assert_eq!(restored.agreement_score, 0.8);
    }
    
    #[test]
    fn test_multi_model_result_serialization() {
        let result = MultiModelResult {
            query: "test".to_string(),
            observations: HashMap::new(),
            failed_models: vec!["failed".to_string()],
            total_time_ms: 100,
            task_type: TaskType::Chat,
        };
        
        let json = serde_json::to_string(&result).unwrap();
        let restored: MultiModelResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored.query, "test");
        assert_eq!(restored.total_time_ms, 100);
        assert!(restored.failed_models.contains(&"failed".to_string()));
    }
    
    #[test]
    fn test_multi_model_stats_default() {
        let stats = MultiModelStats::default();
        assert_eq!(stats.total_observations, 0);
        assert_eq!(stats.successful_observations, 0);
        assert_eq!(stats.failed_observations, 0);
    }
}
