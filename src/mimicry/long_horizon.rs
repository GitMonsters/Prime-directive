// =================================================================
// LONG-HORIZON OBSERVATION: Multi-Turn Context & Pattern Tracking
// =================================================================
// Extends the observation pipeline for 100+ turn conversations with:
// - Multi-turn context tracking and summarization
// - Pattern evolution analysis over long conversations
// - Dynamic strategy adjustment based on performance
// - Sliding context windows for memory management
//
// INTEGRATION:
// - Uses AgentDockBridge for container orchestration
// - Feeds into evolution tracking for persona adaptation
// - Supports RL optimizer trajectory collection
// =================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mimicry::rl_optimizer::BehaviorObservation;

// =================================================================
// CONTEXT WINDOW MANAGEMENT
// =================================================================

/// Configuration for context window management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindowConfig {
    /// Maximum number of raw messages to keep
    pub max_raw_messages: usize,

    /// Number of recent messages to always keep in full
    pub recent_window_size: usize,

    /// Enable automatic summarization of old context
    pub enable_summarization: bool,

    /// Summarization trigger threshold (when to summarize)
    pub summarization_threshold: usize,

    /// Maximum summary length in tokens (approximate)
    pub max_summary_tokens: usize,
}

impl Default for ContextWindowConfig {
    fn default() -> Self {
        ContextWindowConfig {
            max_raw_messages: 200,
            recent_window_size: 20,
            enable_summarization: true,
            summarization_threshold: 50,
            max_summary_tokens: 500,
        }
    }
}

/// Message in the context window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    /// Message role (user, assistant, system)
    pub role: String,

    /// Message content
    pub content: String,

    /// Turn number
    pub turn: usize,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Whether this message is part of the compressed summary
    pub is_summary: bool,

    /// Extracted patterns from this message
    pub patterns: Vec<String>,

    /// Topics detected in this message
    pub topics: Vec<String>,
}

impl ContextMessage {
    /// Create a new context message
    pub fn new(role: &str, content: &str, turn: usize) -> Self {
        ContextMessage {
            role: role.to_string(),
            content: content.to_string(),
            turn,
            timestamp: Utc::now(),
            is_summary: false,
            patterns: Vec::new(),
            topics: Vec::new(),
        }
    }

    /// Mark as summary message
    pub fn as_summary(mut self) -> Self {
        self.is_summary = true;
        self
    }

    /// Add patterns
    pub fn with_patterns(mut self, patterns: Vec<String>) -> Self {
        self.patterns = patterns;
        self
    }

    /// Add topics
    pub fn with_topics(mut self, topics: Vec<String>) -> Self {
        self.topics = topics;
        self
    }
}

/// Sliding context window for managing long conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    /// Configuration
    config: ContextWindowConfig,

    /// All messages (raw + summaries)
    messages: Vec<ContextMessage>,

    /// Compressed summaries of older context
    summaries: Vec<String>,

    /// Current turn number
    current_turn: usize,

    /// Total messages processed (including summarized)
    total_messages_processed: usize,

    /// Topic frequency across the conversation
    topic_frequency: HashMap<String, usize>,
}

impl ContextWindow {
    /// Create a new context window
    pub fn new(config: ContextWindowConfig) -> Self {
        ContextWindow {
            config,
            messages: Vec::new(),
            summaries: Vec::new(),
            current_turn: 0,
            total_messages_processed: 0,
            topic_frequency: HashMap::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(ContextWindowConfig::default())
    }

    /// Add a message to the context window
    pub fn add_message(
        &mut self,
        role: &str,
        content: &str,
        patterns: Vec<String>,
        topics: Vec<String>,
    ) {
        self.current_turn += 1;
        self.total_messages_processed += 1;

        // Update topic frequency
        for topic in &topics {
            *self.topic_frequency.entry(topic.clone()).or_insert(0) += 1;
        }

        let message = ContextMessage::new(role, content, self.current_turn)
            .with_patterns(patterns)
            .with_topics(topics);

        self.messages.push(message);

        // Check if we need to compress/summarize
        if self.config.enable_summarization
            && self.messages.len() > self.config.summarization_threshold
        {
            self.compress_old_context();
        }
    }

    /// Compress old context into a summary
    fn compress_old_context(&mut self) {
        let keep_count = self.config.recent_window_size;

        if self.messages.len() <= keep_count {
            return;
        }

        // Split messages: old ones to summarize, recent ones to keep
        let split_point = self.messages.len() - keep_count;
        let old_messages: Vec<_> = self.messages.drain(..split_point).collect();

        // Create a summary of old messages
        let summary = self.create_summary(&old_messages);
        self.summaries.push(summary.clone());

        // Insert summary as first message
        let summary_msg = ContextMessage::new("system", &summary, 0).as_summary();
        self.messages.insert(0, summary_msg);
    }

    /// Create a summary from a set of messages
    fn create_summary(&self, messages: &[ContextMessage]) -> String {
        // Simple extractive summary - collect key information
        let mut topics_mentioned: HashMap<String, usize> = HashMap::new();
        let mut patterns_seen: HashMap<String, usize> = HashMap::new();
        let mut key_points: Vec<String> = Vec::new();

        for msg in messages {
            for topic in &msg.topics {
                *topics_mentioned.entry(topic.clone()).or_insert(0) += 1;
            }
            for pattern in &msg.patterns {
                *patterns_seen.entry(pattern.clone()).or_insert(0) += 1;
            }

            // Extract key points from longer messages
            if msg.content.len() > 100 && !msg.is_summary {
                // Take first sentence as key point
                if let Some(first_sentence) = msg.content.split('.').next() {
                    if first_sentence.len() > 20 && first_sentence.len() < 200 {
                        key_points.push(format!("[Turn {}] {}", msg.turn, first_sentence.trim()));
                    }
                }
            }
        }

        // Build summary
        let mut summary_parts: Vec<String> = Vec::new();

        // Add turn range
        if let (Some(first), Some(last)) = (messages.first(), messages.last()) {
            summary_parts.push(format!(
                "[Context summary: turns {}-{}]",
                first.turn, last.turn
            ));
        }

        // Add top topics
        let mut sorted_topics: Vec<_> = topics_mentioned.into_iter().collect();
        sorted_topics.sort_by(|a, b| b.1.cmp(&a.1));
        if !sorted_topics.is_empty() {
            let top_topics: Vec<_> = sorted_topics
                .iter()
                .take(5)
                .map(|(t, _)| t.as_str())
                .collect();
            summary_parts.push(format!("Topics: {}", top_topics.join(", ")));
        }

        // Add key points (limited)
        if !key_points.is_empty() {
            let limited_points: Vec<_> = key_points.iter().take(3).map(|s| s.as_str()).collect();
            summary_parts.push(format!("Key points: {}", limited_points.join("; ")));
        }

        summary_parts.join("\n")
    }

    /// Get the current context for the model (with summarization applied)
    pub fn get_context(&self) -> Vec<(String, String)> {
        self.messages
            .iter()
            .map(|m| (m.role.clone(), m.content.clone()))
            .collect()
    }

    /// Get the full recent window (last N messages)
    pub fn get_recent_window(&self) -> Vec<&ContextMessage> {
        let start = self
            .messages
            .len()
            .saturating_sub(self.config.recent_window_size);
        self.messages[start..].iter().collect()
    }

    /// Get current turn number
    pub fn current_turn(&self) -> usize {
        self.current_turn
    }

    /// Get total messages processed
    pub fn total_processed(&self) -> usize {
        self.total_messages_processed
    }

    /// Get the top N most frequent topics
    pub fn top_topics(&self, n: usize) -> Vec<(String, usize)> {
        let mut sorted: Vec<_> = self
            .topic_frequency
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.into_iter().take(n).collect()
    }

    /// Get all summaries
    pub fn get_summaries(&self) -> &[String] {
        &self.summaries
    }

    /// Clear the context window
    pub fn clear(&mut self) {
        self.messages.clear();
        self.summaries.clear();
        self.current_turn = 0;
        self.total_messages_processed = 0;
        self.topic_frequency.clear();
    }
}

// =================================================================
// PATTERN EVOLUTION TRACKING
// =================================================================

/// Tracks how patterns evolve over a long conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternEvolution {
    /// Pattern name
    pub pattern: String,

    /// Occurrences by turn (turn -> count)
    pub occurrences: Vec<(usize, usize)>,

    /// First seen at turn
    pub first_seen: usize,

    /// Last seen at turn
    pub last_seen: usize,

    /// Total occurrences
    pub total_count: usize,

    /// Trend direction: increasing, decreasing, stable
    pub trend: PatternTrend,
}

/// Trend direction for a pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternTrend {
    Increasing,
    Decreasing,
    Stable,
    Sporadic,
}

/// Tracks pattern evolution across the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTracker {
    /// Pattern evolutions by pattern name
    patterns: HashMap<String, PatternEvolution>,

    /// Window size for trend calculation
    trend_window: usize,

    /// Current turn
    current_turn: usize,

    /// Patterns seen in current window (for batch updates)
    current_window_patterns: HashMap<String, usize>,
}

impl PatternTracker {
    /// Create a new pattern tracker
    pub fn new() -> Self {
        PatternTracker {
            patterns: HashMap::new(),
            trend_window: 10,
            current_turn: 0,
            current_window_patterns: HashMap::new(),
        }
    }

    /// Set trend window size
    pub fn with_trend_window(mut self, window: usize) -> Self {
        self.trend_window = window;
        self
    }

    /// Record patterns for a turn
    pub fn record_patterns(&mut self, turn: usize, patterns: &[String]) {
        self.current_turn = turn;

        for pattern in patterns {
            // Update current window
            *self
                .current_window_patterns
                .entry(pattern.clone())
                .or_insert(0) += 1;

            // Update pattern evolution
            let evolution =
                self.patterns
                    .entry(pattern.clone())
                    .or_insert_with(|| PatternEvolution {
                        pattern: pattern.clone(),
                        occurrences: Vec::new(),
                        first_seen: turn,
                        last_seen: turn,
                        total_count: 0,
                        trend: PatternTrend::Stable,
                    });

            evolution.last_seen = turn;
            evolution.total_count += 1;

            // Record occurrence
            if let Some(last) = evolution.occurrences.last_mut() {
                if last.0 == turn {
                    last.1 += 1;
                } else {
                    evolution.occurrences.push((turn, 1));
                }
            } else {
                evolution.occurrences.push((turn, 1));
            }
        }
    }

    /// Update trends for all patterns
    pub fn update_trends(&mut self) {
        let trend_window = self.trend_window;
        for evolution in self.patterns.values_mut() {
            evolution.trend = Self::calculate_trend_static(&evolution.occurrences, trend_window);
        }
    }

    /// Calculate trend from occurrences (static version to avoid borrow issues)
    fn calculate_trend_static(occurrences: &[(usize, usize)], trend_window: usize) -> PatternTrend {
        if occurrences.len() < 3 {
            return PatternTrend::Stable;
        }

        // Look at recent occurrences
        let recent: Vec<_> = occurrences.iter().rev().take(trend_window).collect();

        if recent.len() < 3 {
            return PatternTrend::Stable;
        }

        // Calculate simple trend: compare first half to second half
        let mid = recent.len() / 2;
        let first_half_avg: f64 =
            recent[mid..].iter().map(|(_, c)| *c as f64).sum::<f64>() / (recent.len() - mid) as f64;
        let second_half_avg: f64 =
            recent[..mid].iter().map(|(_, c)| *c as f64).sum::<f64>() / mid as f64;

        let ratio = second_half_avg / first_half_avg.max(0.1);

        if ratio > 1.5 {
            PatternTrend::Increasing
        } else if ratio < 0.67 {
            PatternTrend::Decreasing
        } else {
            // Check for sporadic pattern (high variance)
            let counts: Vec<f64> = recent.iter().map(|(_, c)| *c as f64).collect();
            let mean: f64 = counts.iter().sum::<f64>() / counts.len() as f64;
            let variance: f64 =
                counts.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / counts.len() as f64;
            let cv = variance.sqrt() / mean.max(0.1); // Coefficient of variation

            if cv > 0.5 {
                PatternTrend::Sporadic
            } else {
                PatternTrend::Stable
            }
        }
    }

    /// Get all patterns sorted by frequency
    pub fn get_patterns_by_frequency(&self) -> Vec<&PatternEvolution> {
        let mut sorted: Vec<_> = self.patterns.values().collect();
        sorted.sort_by(|a, b| b.total_count.cmp(&a.total_count));
        sorted
    }

    /// Get patterns with a specific trend
    pub fn get_patterns_with_trend(&self, trend: PatternTrend) -> Vec<&PatternEvolution> {
        self.patterns
            .values()
            .filter(|p| p.trend == trend)
            .collect()
    }

    /// Get emerging patterns (increasing trend, recently seen)
    pub fn get_emerging_patterns(&self) -> Vec<&PatternEvolution> {
        let recency_threshold = self.current_turn.saturating_sub(self.trend_window);
        self.patterns
            .values()
            .filter(|p| p.trend == PatternTrend::Increasing && p.last_seen >= recency_threshold)
            .collect()
    }

    /// Get fading patterns (decreasing trend)
    pub fn get_fading_patterns(&self) -> Vec<&PatternEvolution> {
        self.patterns
            .values()
            .filter(|p| p.trend == PatternTrend::Decreasing)
            .collect()
    }

    /// Get pattern by name
    pub fn get_pattern(&self, name: &str) -> Option<&PatternEvolution> {
        self.patterns.get(name)
    }

    /// Get current turn
    pub fn current_turn(&self) -> usize {
        self.current_turn
    }

    /// Get total unique patterns tracked
    pub fn unique_pattern_count(&self) -> usize {
        self.patterns.len()
    }
}

impl Default for PatternTracker {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================
// STRATEGY ADJUSTMENT
// =================================================================

/// Strategy adjustment based on observation performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyState {
    /// Current observation strategy
    pub strategy: ObservationStrategy,

    /// Performance metrics for current strategy
    pub performance_score: f64,

    /// Turns since last strategy change
    pub turns_since_change: usize,

    /// History of strategy changes
    pub change_history: Vec<StrategyChange>,
}

/// Observation strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObservationStrategy {
    /// Standard observation with balanced probing
    Balanced,

    /// Deep exploration of specific patterns
    DeepExplore { focus_patterns: Vec<String> },

    /// Breadth-first discovery of new patterns
    BreadthFirst,

    /// Confirmation of existing patterns
    Confirmation { patterns_to_confirm: Vec<String> },

    /// Stress testing edge cases
    StressTesting,

    /// Minimal probing (maintenance mode)
    Minimal,
}

/// Record of a strategy change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyChange {
    /// Turn when change occurred
    pub turn: usize,

    /// Previous strategy
    pub from: ObservationStrategy,

    /// New strategy
    pub to: ObservationStrategy,

    /// Reason for change
    pub reason: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Configuration for strategy adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    /// Minimum turns before strategy can change
    pub min_turns_between_changes: usize,

    /// Performance threshold for triggering change
    pub performance_threshold: f64,

    /// Enable automatic strategy adjustment
    pub auto_adjust: bool,

    /// Maximum strategy changes per session
    pub max_changes_per_session: usize,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        StrategyConfig {
            min_turns_between_changes: 10,
            performance_threshold: 0.5,
            auto_adjust: true,
            max_changes_per_session: 10,
        }
    }
}

/// Adjusts observation strategy based on performance
#[derive(Debug, Clone)]
pub struct StrategyAdjuster {
    /// Configuration
    config: StrategyConfig,

    /// Current state
    state: StrategyState,

    /// Performance history (turn -> score)
    performance_history: Vec<(usize, f64)>,
}

impl StrategyAdjuster {
    /// Create a new strategy adjuster
    pub fn new(config: StrategyConfig) -> Self {
        StrategyAdjuster {
            config,
            state: StrategyState {
                strategy: ObservationStrategy::Balanced,
                performance_score: 0.5,
                turns_since_change: 0,
                change_history: Vec::new(),
            },
            performance_history: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(StrategyConfig::default())
    }

    /// Record performance for a turn
    pub fn record_performance(&mut self, turn: usize, score: f64) {
        self.performance_history.push((turn, score));
        self.state.turns_since_change += 1;

        // Update running performance score (exponential moving average)
        let alpha = 0.2;
        self.state.performance_score = alpha * score + (1.0 - alpha) * self.state.performance_score;
    }

    /// Check if strategy should be adjusted
    pub fn should_adjust(&self) -> bool {
        if !self.config.auto_adjust {
            return false;
        }

        if self.state.change_history.len() >= self.config.max_changes_per_session {
            return false;
        }

        if self.state.turns_since_change < self.config.min_turns_between_changes {
            return false;
        }

        self.state.performance_score < self.config.performance_threshold
    }

    /// Recommend a new strategy based on current state
    pub fn recommend_strategy(
        &self,
        pattern_tracker: &PatternTracker,
    ) -> Option<ObservationStrategy> {
        if !self.should_adjust() {
            return None;
        }

        // Analyze patterns to determine best strategy
        let emerging = pattern_tracker.get_emerging_patterns();
        let _fading = pattern_tracker.get_fading_patterns(); // Reserved for future use
        let unique_count = pattern_tracker.unique_pattern_count();

        match &self.state.strategy {
            ObservationStrategy::Balanced => {
                // If we have emerging patterns, deep explore them
                if !emerging.is_empty() {
                    let focus: Vec<String> =
                        emerging.iter().take(3).map(|p| p.pattern.clone()).collect();
                    Some(ObservationStrategy::DeepExplore {
                        focus_patterns: focus,
                    })
                } else if unique_count < 10 {
                    // Not enough patterns discovered, try breadth first
                    Some(ObservationStrategy::BreadthFirst)
                } else {
                    // Try confirmation of existing patterns
                    let to_confirm: Vec<String> = pattern_tracker
                        .get_patterns_by_frequency()
                        .iter()
                        .take(5)
                        .map(|p| p.pattern.clone())
                        .collect();
                    Some(ObservationStrategy::Confirmation {
                        patterns_to_confirm: to_confirm,
                    })
                }
            }

            ObservationStrategy::DeepExplore { .. } => {
                // After deep exploration, go back to balanced or try stress testing
                if self.state.performance_score < 0.3 {
                    Some(ObservationStrategy::StressTesting)
                } else {
                    Some(ObservationStrategy::Balanced)
                }
            }

            ObservationStrategy::BreadthFirst => {
                // If we found new patterns, start exploring them
                if !emerging.is_empty() {
                    let focus: Vec<String> =
                        emerging.iter().take(3).map(|p| p.pattern.clone()).collect();
                    Some(ObservationStrategy::DeepExplore {
                        focus_patterns: focus,
                    })
                } else {
                    Some(ObservationStrategy::Balanced)
                }
            }

            ObservationStrategy::Confirmation { .. } => {
                // After confirmation, return to balanced
                Some(ObservationStrategy::Balanced)
            }

            ObservationStrategy::StressTesting => {
                // After stress testing, go minimal or balanced
                if self.state.performance_score < 0.2 {
                    Some(ObservationStrategy::Minimal)
                } else {
                    Some(ObservationStrategy::Balanced)
                }
            }

            ObservationStrategy::Minimal => {
                // If performance improves, gradually return to balanced
                if self.state.performance_score > 0.6 {
                    Some(ObservationStrategy::Balanced)
                } else {
                    None
                }
            }
        }
    }

    /// Apply a strategy change
    pub fn apply_strategy(&mut self, new_strategy: ObservationStrategy, reason: &str, turn: usize) {
        let change = StrategyChange {
            turn,
            from: self.state.strategy.clone(),
            to: new_strategy.clone(),
            reason: reason.to_string(),
            timestamp: Utc::now(),
        };

        self.state.change_history.push(change);
        self.state.strategy = new_strategy;
        self.state.turns_since_change = 0;
    }

    /// Get current strategy
    pub fn current_strategy(&self) -> &ObservationStrategy {
        &self.state.strategy
    }

    /// Get current performance score
    pub fn performance_score(&self) -> f64 {
        self.state.performance_score
    }

    /// Get strategy change history
    pub fn change_history(&self) -> &[StrategyChange] {
        &self.state.change_history
    }

    /// Get performance history
    pub fn performance_history(&self) -> &[(usize, f64)] {
        &self.performance_history
    }
}

// =================================================================
// LONG-HORIZON OBSERVER
// =================================================================

/// Configuration for long-horizon observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongHorizonConfig {
    /// Maximum turns for a single session
    pub max_turns: usize,

    /// Context window configuration
    pub context_config: ContextWindowConfig,

    /// Strategy adjustment configuration
    pub strategy_config: StrategyConfig,

    /// Pattern trend window size
    pub pattern_trend_window: usize,

    /// Enable pattern evolution tracking
    pub track_patterns: bool,

    /// Checkpoint interval (save state every N turns)
    pub checkpoint_interval: usize,
}

impl Default for LongHorizonConfig {
    fn default() -> Self {
        LongHorizonConfig {
            max_turns: 100,
            context_config: ContextWindowConfig::default(),
            strategy_config: StrategyConfig::default(),
            pattern_trend_window: 10,
            track_patterns: true,
            checkpoint_interval: 25,
        }
    }
}

/// Session state for long-horizon observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongHorizonSession {
    /// Session identifier
    pub session_id: String,

    /// Target model being observed
    pub model_id: String,

    /// Context window
    pub context: ContextWindow,

    /// Pattern tracker
    pub pattern_tracker: PatternTracker,

    /// Current observation strategy
    pub current_strategy: ObservationStrategy,

    /// Session start time
    pub started_at: DateTime<Utc>,

    /// Last checkpoint time
    pub last_checkpoint: Option<DateTime<Utc>>,

    /// Collected observations
    pub observations: Vec<BehaviorObservation>,

    /// Session metadata
    pub metadata: HashMap<String, String>,

    /// Whether the session is complete
    pub is_complete: bool,

    /// Completion reason
    pub completion_reason: Option<String>,
}

impl LongHorizonSession {
    /// Create a new long-horizon session
    pub fn new(session_id: &str, model_id: &str, config: &LongHorizonConfig) -> Self {
        LongHorizonSession {
            session_id: session_id.to_string(),
            model_id: model_id.to_string(),
            context: ContextWindow::new(config.context_config.clone()),
            pattern_tracker: PatternTracker::new().with_trend_window(config.pattern_trend_window),
            current_strategy: ObservationStrategy::Balanced,
            started_at: Utc::now(),
            last_checkpoint: None,
            observations: Vec::new(),
            metadata: HashMap::new(),
            is_complete: false,
            completion_reason: None,
        }
    }

    /// Add an observation to the session
    pub fn add_observation(&mut self, observation: BehaviorObservation) {
        let turn = self.context.current_turn();

        // Track patterns
        self.pattern_tracker
            .record_patterns(turn, &observation.patterns);

        // Add to context
        self.context.add_message(
            "assistant",
            &observation.response,
            observation.patterns.clone(),
            vec![], // Topics would be extracted separately
        );

        // Store observation
        self.observations.push(observation);
    }

    /// Mark session as complete
    pub fn complete(&mut self, reason: &str) {
        self.is_complete = true;
        self.completion_reason = Some(reason.to_string());
    }

    /// Get session duration in seconds
    pub fn duration_secs(&self) -> i64 {
        (Utc::now() - self.started_at).num_seconds()
    }

    /// Get the number of observations collected
    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }

    /// Check if session needs a checkpoint
    pub fn needs_checkpoint(&self, interval: usize) -> bool {
        let turn = self.context.current_turn();
        turn > 0 && turn % interval == 0
    }

    /// Update checkpoint timestamp
    pub fn checkpoint(&mut self) {
        self.last_checkpoint = Some(Utc::now());
    }

    /// Get summary of the session
    pub fn summary(&self) -> String {
        let mut lines = vec![
            format!("=== Long-Horizon Session Summary ==="),
            format!("Session ID: {}", self.session_id),
            format!("Model: {}", self.model_id),
            format!("Duration: {} seconds", self.duration_secs()),
            format!("Turns: {}", self.context.current_turn()),
            format!("Observations: {}", self.observations.len()),
            format!(
                "Unique patterns: {}",
                self.pattern_tracker.unique_pattern_count()
            ),
            format!("Strategy: {:?}", self.current_strategy),
            format!("Complete: {}", self.is_complete),
        ];

        if let Some(ref reason) = self.completion_reason {
            lines.push(format!("Completion reason: {}", reason));
        }

        // Add top topics
        let top_topics = self.context.top_topics(5);
        if !top_topics.is_empty() {
            lines.push(format!("Top topics: {:?}", top_topics));
        }

        // Add emerging patterns
        let emerging = self.pattern_tracker.get_emerging_patterns();
        if !emerging.is_empty() {
            let names: Vec<_> = emerging
                .iter()
                .take(3)
                .map(|p| p.pattern.as_str())
                .collect();
            lines.push(format!("Emerging patterns: {:?}", names));
        }

        lines.join("\n")
    }
}

/// Main orchestrator for long-horizon observation
pub struct LongHorizonObserver {
    /// Configuration
    config: LongHorizonConfig,

    /// Active sessions
    sessions: HashMap<String, LongHorizonSession>,

    /// Strategy adjuster
    strategy_adjuster: StrategyAdjuster,

    /// Statistics
    stats: LongHorizonStats,
}

/// Statistics for long-horizon observation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LongHorizonStats {
    /// Total sessions created
    pub total_sessions: usize,

    /// Completed sessions
    pub completed_sessions: usize,

    /// Total observations collected
    pub total_observations: usize,

    /// Total turns processed
    pub total_turns: usize,

    /// Average session duration in seconds
    pub avg_session_duration_secs: f64,

    /// Average observations per session
    pub avg_observations_per_session: f64,

    /// Strategy changes made
    pub strategy_changes: usize,
}

impl LongHorizonObserver {
    /// Create a new long-horizon observer
    pub fn new(config: LongHorizonConfig) -> Self {
        let strategy_adjuster = StrategyAdjuster::new(config.strategy_config.clone());

        LongHorizonObserver {
            config,
            sessions: HashMap::new(),
            strategy_adjuster,
            stats: LongHorizonStats::default(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(LongHorizonConfig::default())
    }

    /// Start a new observation session
    pub fn start_session(&mut self, model_id: &str) -> String {
        let session_id = format!("lh_{}_{}", model_id, Utc::now().timestamp_millis());
        let session = LongHorizonSession::new(&session_id, model_id, &self.config);

        self.sessions.insert(session_id.clone(), session);
        self.stats.total_sessions += 1;

        session_id
    }

    /// Record an observation for a session
    pub fn record_observation(
        &mut self,
        session_id: &str,
        query: &str,
        observation: BehaviorObservation,
    ) -> Result<(), String> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;

        if session.is_complete {
            return Err("Session is already complete".to_string());
        }

        // Add query to context
        session.context.add_message(
            "user",
            query,
            vec![],
            vec![], // Topics would be extracted
        );

        // Compute performance score for strategy adjustment
        let performance_score = observation.similarity_to_target * observation.confidence;
        let turn = session.context.current_turn();
        self.strategy_adjuster
            .record_performance(turn, performance_score);

        // Add observation
        session.add_observation(observation);
        self.stats.total_observations += 1;
        self.stats.total_turns += 1;

        // Check for strategy adjustment
        if let Some(new_strategy) = self
            .strategy_adjuster
            .recommend_strategy(&session.pattern_tracker)
        {
            session.current_strategy = new_strategy.clone();
            self.strategy_adjuster.apply_strategy(
                new_strategy,
                "Performance-based adjustment",
                turn,
            );
            self.stats.strategy_changes += 1;
        }

        // Update pattern trends periodically
        if turn % 10 == 0 {
            session.pattern_tracker.update_trends();
        }

        // Check for checkpoint
        if session.needs_checkpoint(self.config.checkpoint_interval) {
            session.checkpoint();
        }

        // Check if max turns reached
        if turn >= self.config.max_turns {
            session.complete("Max turns reached");
            // Extract data for stats update before dropping borrow
            let duration = session.duration_secs();
            let obs_count = session.observation_count();
            self.update_completed_stats_internal(duration, obs_count);
        }

        Ok(())
    }

    /// Update stats when a session completes (internal version to avoid borrow issues)
    fn update_completed_stats_internal(&mut self, duration_secs: i64, obs_count: usize) {
        self.stats.completed_sessions += 1;

        let completed = self.stats.completed_sessions as f64;
        let duration = duration_secs as f64;
        let obs = obs_count as f64;

        self.stats.avg_session_duration_secs =
            (self.stats.avg_session_duration_secs * (completed - 1.0) + duration) / completed;
        self.stats.avg_observations_per_session =
            (self.stats.avg_observations_per_session * (completed - 1.0) + obs) / completed;
    }

    /// Update stats when a session completes
    fn update_completed_stats(&mut self, session: &LongHorizonSession) {
        self.stats.completed_sessions += 1;

        let completed = self.stats.completed_sessions as f64;
        let duration = session.duration_secs() as f64;
        let obs_count = session.observation_count() as f64;

        self.stats.avg_session_duration_secs =
            (self.stats.avg_session_duration_secs * (completed - 1.0) + duration) / completed;
        self.stats.avg_observations_per_session =
            (self.stats.avg_observations_per_session * (completed - 1.0) + obs_count) / completed;
    }

    /// Complete a session
    pub fn complete_session(
        &mut self,
        session_id: &str,
        reason: &str,
    ) -> Result<LongHorizonSession, String> {
        let mut session = self
            .sessions
            .remove(session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;

        session.complete(reason);
        self.update_completed_stats(&session);

        Ok(session)
    }

    /// Get a reference to a session
    pub fn get_session(&self, session_id: &str) -> Option<&LongHorizonSession> {
        self.sessions.get(session_id)
    }

    /// Get a mutable reference to a session
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut LongHorizonSession> {
        self.sessions.get_mut(session_id)
    }

    /// Get current observation strategy for a session
    pub fn get_strategy(&self, session_id: &str) -> Option<&ObservationStrategy> {
        self.sessions.get(session_id).map(|s| &s.current_strategy)
    }

    /// Get active session count
    pub fn active_session_count(&self) -> usize {
        self.sessions.values().filter(|s| !s.is_complete).count()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &LongHorizonStats {
        &self.stats
    }

    /// Get status summary
    pub fn status_summary(&self) -> String {
        let mut lines = vec![
            "=== LONG-HORIZON OBSERVER STATUS ===".to_string(),
            format!("Active sessions: {}", self.active_session_count()),
            format!("Total sessions: {}", self.stats.total_sessions),
            format!("Completed sessions: {}", self.stats.completed_sessions),
            format!("Total observations: {}", self.stats.total_observations),
            format!("Total turns: {}", self.stats.total_turns),
            format!("Strategy changes: {}", self.stats.strategy_changes),
            format!(
                "Avg session duration: {:.1}s",
                self.stats.avg_session_duration_secs
            ),
            format!(
                "Avg observations/session: {:.1}",
                self.stats.avg_observations_per_session
            ),
        ];

        // Add active session info
        if !self.sessions.is_empty() {
            lines.push(String::new());
            lines.push("Active sessions:".to_string());
            for (id, session) in &self.sessions {
                lines.push(format!(
                    "  {}: {} turns, {:?}",
                    id,
                    session.context.current_turn(),
                    session.current_strategy
                ));
            }
        }

        lines.join("\n")
    }
}

// =================================================================
// QUERY GENERATION FOR STRATEGIES
// =================================================================

/// Generates queries based on the current observation strategy
pub struct StrategyQueryGenerator {
    /// Base prompts for different strategies
    prompts: HashMap<String, Vec<String>>,
}

impl StrategyQueryGenerator {
    /// Create a new query generator
    pub fn new() -> Self {
        let mut prompts = HashMap::new();

        // Balanced exploration prompts
        prompts.insert(
            "balanced".to_string(),
            vec![
                "Tell me about your approach to problem-solving.".to_string(),
                "How do you handle uncertainty in your responses?".to_string(),
                "What's your perspective on this topic?".to_string(),
                "Can you explain this concept in detail?".to_string(),
                "How would you approach this task?".to_string(),
            ],
        );

        // Deep exploration prompts
        prompts.insert(
            "deep_explore".to_string(),
            vec![
                "Let's dive deeper into that point you made.".to_string(),
                "Can you elaborate on the reasoning behind that?".to_string(),
                "What are the implications of that approach?".to_string(),
                "How does that connect to what you mentioned earlier?".to_string(),
                "What are the edge cases to consider?".to_string(),
            ],
        );

        // Breadth-first discovery prompts
        prompts.insert(
            "breadth_first".to_string(),
            vec![
                "Let's explore a completely different topic.".to_string(),
                "What about a different perspective on this?".to_string(),
                "How would you handle an unexpected situation?".to_string(),
                "Can you demonstrate a different capability?".to_string(),
                "What's something unusual about your approach?".to_string(),
            ],
        );

        // Confirmation prompts
        prompts.insert(
            "confirmation".to_string(),
            vec![
                "Can you confirm your earlier statement about...?".to_string(),
                "Is this consistent with what you said before?".to_string(),
                "Would you give the same answer to this similar question?".to_string(),
                "Let me verify my understanding of your position.".to_string(),
                "Does this align with your previous response?".to_string(),
            ],
        );

        // Stress testing prompts
        prompts.insert(
            "stress".to_string(),
            vec![
                "What if I challenged that assumption?".to_string(),
                "How would you handle a contradictory request?".to_string(),
                "Can you respond to this edge case?".to_string(),
                "What happens if the constraints are removed?".to_string(),
                "How do you handle ambiguous instructions?".to_string(),
            ],
        );

        StrategyQueryGenerator { prompts }
    }

    /// Generate a query based on strategy
    pub fn generate_query(
        &self,
        strategy: &ObservationStrategy,
        context: &ContextWindow,
        pattern_tracker: &PatternTracker,
    ) -> String {
        let strategy_key = match strategy {
            ObservationStrategy::Balanced => "balanced",
            ObservationStrategy::DeepExplore { .. } => "deep_explore",
            ObservationStrategy::BreadthFirst => "breadth_first",
            ObservationStrategy::Confirmation { .. } => "confirmation",
            ObservationStrategy::StressTesting => "stress",
            ObservationStrategy::Minimal => "balanced",
        };

        let base_prompts = self
            .prompts
            .get(strategy_key)
            .unwrap_or(self.prompts.get("balanced").unwrap());

        // Select a prompt based on context
        let idx = context.current_turn() % base_prompts.len();
        let mut query = base_prompts[idx].clone();

        // Customize based on strategy specifics
        match strategy {
            ObservationStrategy::DeepExplore { focus_patterns } => {
                if !focus_patterns.is_empty() {
                    query = format!(
                        "I noticed you tend to {}. Can you elaborate on why you do that?",
                        focus_patterns.join(", ")
                    );
                }
            }
            ObservationStrategy::Confirmation {
                patterns_to_confirm,
            } => {
                if !patterns_to_confirm.is_empty() {
                    query = format!(
                        "Earlier you demonstrated {}. Would you approach this similarly: {}?",
                        patterns_to_confirm[0], query
                    );
                }
            }
            _ => {}
        }

        // Add context-awareness for emerging patterns
        let emerging = pattern_tracker.get_emerging_patterns();
        if !emerging.is_empty() && context.current_turn() > 10 {
            if let Some(pattern) = emerging.first() {
                query = format!(
                    "{} (Considering your tendency to {})",
                    query, pattern.pattern
                );
            }
        }

        query
    }
}

impl Default for StrategyQueryGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_window_add_message() {
        let mut ctx = ContextWindow::with_defaults();

        ctx.add_message("user", "Hello", vec![], vec!["greeting".to_string()]);
        ctx.add_message(
            "assistant",
            "Hi there!",
            vec!["friendly".to_string()],
            vec![],
        );

        assert_eq!(ctx.current_turn(), 2);
        assert_eq!(ctx.total_processed(), 2);
    }

    #[test]
    fn test_context_window_summarization() {
        let config = ContextWindowConfig {
            summarization_threshold: 5,
            recent_window_size: 3,
            ..Default::default()
        };
        let mut ctx = ContextWindow::new(config);

        // Add messages beyond threshold
        for i in 0..10 {
            ctx.add_message("user", &format!("Message {}", i), vec![], vec![]);
        }

        // Should have summarized older messages
        assert!(!ctx.get_summaries().is_empty() || ctx.get_context().len() <= 5);
    }

    #[test]
    fn test_context_window_top_topics() {
        let mut ctx = ContextWindow::with_defaults();

        ctx.add_message(
            "user",
            "test",
            vec![],
            vec!["ai".to_string(), "ml".to_string()],
        );
        ctx.add_message("user", "test", vec![], vec!["ai".to_string()]);
        ctx.add_message("user", "test", vec![], vec!["rust".to_string()]);

        let top = ctx.top_topics(2);
        assert_eq!(top[0].0, "ai");
        assert_eq!(top[0].1, 2);
    }

    #[test]
    fn test_pattern_tracker_record() {
        let mut tracker = PatternTracker::new();

        tracker.record_patterns(1, &["pattern_a".to_string(), "pattern_b".to_string()]);
        tracker.record_patterns(2, &["pattern_a".to_string()]);

        assert_eq!(tracker.unique_pattern_count(), 2);

        let pattern_a = tracker.get_pattern("pattern_a").unwrap();
        assert_eq!(pattern_a.total_count, 2);
    }

    #[test]
    fn test_pattern_tracker_trends() {
        let mut tracker = PatternTracker::new();

        // Simulate increasing pattern
        for i in 1..=10 {
            let patterns: Vec<_> = (0..i).map(|_| "increasing".to_string()).collect();
            tracker.record_patterns(i, &patterns);
        }

        tracker.update_trends();

        let pattern = tracker.get_pattern("increasing").unwrap();
        assert_eq!(pattern.trend, PatternTrend::Increasing);
    }

    #[test]
    fn test_strategy_adjuster_performance() {
        let mut adjuster = StrategyAdjuster::with_defaults();

        // Record low performance
        for i in 0..15 {
            adjuster.record_performance(i, 0.3);
        }

        assert!(adjuster.should_adjust());
        assert!(adjuster.performance_score() < 0.5);
    }

    #[test]
    fn test_long_horizon_session() {
        let config = LongHorizonConfig::default();
        let mut session = LongHorizonSession::new("test-session", "gpt-4", &config);

        let observation = BehaviorObservation {
            query: "test".to_string(),
            response: "response".to_string(),
            patterns: vec!["pattern".to_string()],
            similarity_to_target: 0.8,
            confidence: 0.9,
        };

        session.add_observation(observation);

        assert_eq!(session.observation_count(), 1);
        assert!(!session.is_complete);
    }

    #[test]
    fn test_long_horizon_observer() {
        let mut observer = LongHorizonObserver::with_defaults();

        let session_id = observer.start_session("test-model");

        let observation = BehaviorObservation {
            query: "test".to_string(),
            response: "response".to_string(),
            patterns: vec!["pattern".to_string()],
            similarity_to_target: 0.8,
            confidence: 0.9,
        };

        assert!(observer
            .record_observation(&session_id, "test query", observation)
            .is_ok());

        let stats = observer.get_statistics();
        assert_eq!(stats.total_observations, 1);
    }

    #[test]
    fn test_strategy_query_generator() {
        let generator = StrategyQueryGenerator::new();
        let ctx = ContextWindow::with_defaults();
        let tracker = PatternTracker::new();

        let query = generator.generate_query(&ObservationStrategy::Balanced, &ctx, &tracker);
        assert!(!query.is_empty());

        let query = generator.generate_query(
            &ObservationStrategy::DeepExplore {
                focus_patterns: vec!["test".to_string()],
            },
            &ctx,
            &tracker,
        );
        assert!(query.contains("elaborate"));
    }

    #[test]
    fn test_observation_strategy_serialization() {
        let strategy = ObservationStrategy::DeepExplore {
            focus_patterns: vec!["pattern".to_string()],
        };

        let json = serde_json::to_string(&strategy).unwrap();
        let restored: ObservationStrategy = serde_json::from_str(&json).unwrap();

        match restored {
            ObservationStrategy::DeepExplore { focus_patterns } => {
                assert_eq!(focus_patterns, vec!["pattern".to_string()]);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_context_message_builder() {
        let msg = ContextMessage::new("user", "Hello", 1)
            .with_patterns(vec!["greeting".to_string()])
            .with_topics(vec!["salutation".to_string()]);

        assert_eq!(msg.role, "user");
        assert_eq!(msg.turn, 1);
        assert!(msg.patterns.contains(&"greeting".to_string()));
        assert!(msg.topics.contains(&"salutation".to_string()));
    }

    #[test]
    fn test_pattern_evolution() {
        let evolution = PatternEvolution {
            pattern: "test".to_string(),
            occurrences: vec![(1, 2), (3, 3)],
            first_seen: 1,
            last_seen: 3,
            total_count: 5,
            trend: PatternTrend::Increasing,
        };

        assert_eq!(evolution.total_count, 5);
        assert_eq!(evolution.trend, PatternTrend::Increasing);
    }

    #[test]
    fn test_strategy_state_serialization() {
        let state = StrategyState {
            strategy: ObservationStrategy::Balanced,
            performance_score: 0.75,
            turns_since_change: 5,
            change_history: vec![],
        };

        let json = serde_json::to_string(&state).unwrap();
        let restored: StrategyState = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.performance_score, 0.75);
        assert_eq!(restored.strategy, ObservationStrategy::Balanced);
    }
}
