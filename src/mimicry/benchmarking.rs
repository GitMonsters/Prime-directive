// =================================================================
// BENCHMARKING: AgentToLeaP Integration for Persona Validation
// =================================================================
// Integrates AgentToLeaP benchmarking framework for automated scoring
// and convergence validation. Supports 9+ benchmarks including GAIA,
// HLE, BrowseComp, Frames, and more.
//
// This module provides:
// - Benchmark suite definition and configuration
// - Task execution and response collection
// - Automated scoring and metrics calculation
// - Convergence validation against baseline
// - Report generation and analysis
//
// BENCHMARKS SUPPORTED:
// - GAIA (General AI Assistants)
// - HLE (text subset)
// - BrowseComp (EN/ZH)
// - Frames
// - WebWalkerQA
// - SEAL-0
// - XBench-DeepSearch
// =================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =================================================================
// BENCHMARK TYPES
// =================================================================

/// Types of benchmarks supported
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BenchmarkType {
    /// GAIA - General AI Assistants benchmark
    Gaia,
    /// GAIA text-only subset
    GaiaText,
    /// HLE text subset (2158 tasks)
    HleText,
    /// BrowseComp English
    BrowseCompEn,
    /// BrowseComp Chinese
    BrowseCompZh,
    /// Frames - factuality and multi-hop reasoning
    Frames,
    /// WebWalkerQA - web navigation tasks
    WebWalkerQa,
    /// SEAL-0 - contradictory information reasoning
    Seal0,
    /// XBench-DeepSearch
    XBench,
    /// Custom benchmark
    Custom(String),
}

impl BenchmarkType {
    /// Get the benchmark directory name
    pub fn directory_name(&self) -> &str {
        match self {
            BenchmarkType::Gaia => "gaia",
            BenchmarkType::GaiaText => "gaia_text",
            BenchmarkType::HleText => "hle_text",
            BenchmarkType::BrowseCompEn => "browsecomp",
            BenchmarkType::BrowseCompZh => "browsecomp_zh",
            BenchmarkType::Frames => "frames",
            BenchmarkType::WebWalkerQa => "webwalker",
            BenchmarkType::Seal0 => "seal-0",
            BenchmarkType::XBench => "xbench",
            BenchmarkType::Custom(name) => name,
        }
    }

    /// Get a human-readable name
    pub fn display_name(&self) -> &str {
        match self {
            BenchmarkType::Gaia => "GAIA Validation",
            BenchmarkType::GaiaText => "GAIA Text-103",
            BenchmarkType::HleText => "HLE Text-2158",
            BenchmarkType::BrowseCompEn => "BrowseComp English",
            BenchmarkType::BrowseCompZh => "BrowseComp Chinese",
            BenchmarkType::Frames => "Frames",
            BenchmarkType::WebWalkerQa => "WebWalkerQA",
            BenchmarkType::Seal0 => "SEAL-0",
            BenchmarkType::XBench => "XBench-DeepSearch",
            BenchmarkType::Custom(name) => name,
        }
    }

    /// List all standard benchmark types
    pub fn all_standard() -> Vec<BenchmarkType> {
        vec![
            BenchmarkType::Gaia,
            BenchmarkType::GaiaText,
            BenchmarkType::HleText,
            BenchmarkType::BrowseCompEn,
            BenchmarkType::BrowseCompZh,
            BenchmarkType::Frames,
            BenchmarkType::WebWalkerQa,
            BenchmarkType::Seal0,
            BenchmarkType::XBench,
        ]
    }
}

// =================================================================
// BENCHMARK TASK
// =================================================================

/// A single benchmark task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTask {
    /// Unique task identifier
    pub task_id: String,

    /// The question or instruction
    pub question: String,

    /// Reference answer (for scoring)
    pub reference_answer: String,

    /// Optional file attachments
    pub file_names: Vec<String>,

    /// Benchmark type
    pub benchmark_type: BenchmarkType,

    /// Task difficulty (if known)
    pub difficulty: Option<TaskDifficulty>,

    /// Task metadata
    pub metadata: HashMap<String, String>,
}

/// Task difficulty levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl BenchmarkTask {
    /// Create a new benchmark task
    pub fn new(
        task_id: &str,
        question: &str,
        reference_answer: &str,
        benchmark_type: BenchmarkType,
    ) -> Self {
        BenchmarkTask {
            task_id: task_id.to_string(),
            question: question.to_string(),
            reference_answer: reference_answer.to_string(),
            file_names: Vec::new(),
            benchmark_type,
            difficulty: None,
            metadata: HashMap::new(),
        }
    }

    /// Add file attachments
    pub fn with_files(mut self, files: Vec<String>) -> Self {
        self.file_names = files;
        self
    }

    /// Set difficulty
    pub fn with_difficulty(mut self, difficulty: TaskDifficulty) -> Self {
        self.difficulty = Some(difficulty);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

// =================================================================
// TASK RESULT
// =================================================================

/// Result of executing a benchmark task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,

    /// Model's response
    pub model_response: String,

    /// Extracted final answer
    pub extracted_answer: String,

    /// Whether the answer is correct
    pub is_correct: bool,

    /// Similarity score to reference (0.0 to 1.0)
    pub similarity_score: f64,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Number of reasoning steps
    pub reasoning_steps: usize,

    /// Error message if failed
    pub error: Option<String>,

    /// Reasoning trajectory
    pub trajectory: Vec<ReasoningStep>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// A single reasoning step in the trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step number
    pub step: usize,

    /// Action type (e.g., "search", "calculate", "analyze")
    pub action: String,

    /// Input to the action
    pub input: String,

    /// Output from the action
    pub output: String,

    /// Confidence in this step
    pub confidence: f64,
}

impl TaskResult {
    /// Create a successful result
    pub fn success(
        task_id: &str,
        model_response: &str,
        extracted_answer: &str,
        is_correct: bool,
        similarity_score: f64,
        execution_time_ms: u64,
    ) -> Self {
        TaskResult {
            task_id: task_id.to_string(),
            model_response: model_response.to_string(),
            extracted_answer: extracted_answer.to_string(),
            is_correct,
            similarity_score,
            execution_time_ms,
            reasoning_steps: 0,
            error: None,
            trajectory: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Create a failed result
    pub fn failure(task_id: &str, error: &str) -> Self {
        TaskResult {
            task_id: task_id.to_string(),
            model_response: String::new(),
            extracted_answer: String::new(),
            is_correct: false,
            similarity_score: 0.0,
            execution_time_ms: 0,
            reasoning_steps: 0,
            error: Some(error.to_string()),
            trajectory: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Add reasoning trajectory
    pub fn with_trajectory(mut self, trajectory: Vec<ReasoningStep>) -> Self {
        self.reasoning_steps = trajectory.len();
        self.trajectory = trajectory;
        self
    }
}

// =================================================================
// BENCHMARK SUITE
// =================================================================

/// Configuration for a benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuiteConfig {
    /// Suite name
    pub name: String,

    /// Benchmarks to include
    pub benchmarks: Vec<BenchmarkType>,

    /// Maximum tasks per benchmark (0 = all)
    pub max_tasks_per_benchmark: usize,

    /// Timeout per task in seconds
    pub task_timeout_secs: u64,

    /// Number of parallel workers
    pub parallel_workers: usize,

    /// Retry failed tasks
    pub retry_failed: bool,

    /// Maximum retries per task
    pub max_retries: usize,

    /// Enable detailed trajectory logging
    pub log_trajectories: bool,

    /// Base directory for benchmarks
    pub benchmark_base_dir: String,

    /// Output directory for results
    pub output_dir: String,
}

impl Default for BenchmarkSuiteConfig {
    fn default() -> Self {
        BenchmarkSuiteConfig {
            name: "default".to_string(),
            benchmarks: vec![BenchmarkType::GaiaText],
            max_tasks_per_benchmark: 0,
            task_timeout_secs: 300,
            parallel_workers: 4,
            retry_failed: true,
            max_retries: 2,
            log_trajectories: true,
            benchmark_base_dir: "./benchmarks".to_string(),
            output_dir: "./benchmark_results".to_string(),
        }
    }
}

impl BenchmarkSuiteConfig {
    /// Create a config for quick validation
    pub fn quick_validation() -> Self {
        BenchmarkSuiteConfig {
            name: "quick_validation".to_string(),
            benchmarks: vec![BenchmarkType::GaiaText],
            max_tasks_per_benchmark: 10,
            task_timeout_secs: 60,
            parallel_workers: 2,
            retry_failed: false,
            max_retries: 0,
            log_trajectories: false,
            ..Default::default()
        }
    }

    /// Create a config for full benchmark run
    pub fn full_benchmark() -> Self {
        BenchmarkSuiteConfig {
            name: "full_benchmark".to_string(),
            benchmarks: BenchmarkType::all_standard(),
            max_tasks_per_benchmark: 0,
            task_timeout_secs: 600,
            parallel_workers: 8,
            retry_failed: true,
            max_retries: 3,
            log_trajectories: true,
            ..Default::default()
        }
    }

    /// Create a config for convergence testing
    pub fn convergence_test() -> Self {
        BenchmarkSuiteConfig {
            name: "convergence_test".to_string(),
            benchmarks: vec![
                BenchmarkType::GaiaText,
                BenchmarkType::Frames,
                BenchmarkType::HleText,
            ],
            max_tasks_per_benchmark: 50,
            task_timeout_secs: 300,
            parallel_workers: 4,
            retry_failed: true,
            max_retries: 2,
            log_trajectories: true,
            ..Default::default()
        }
    }
}

// =================================================================
// BENCHMARK METRICS
// =================================================================

/// Metrics for a single benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    /// Benchmark type
    pub benchmark_type: BenchmarkType,

    /// Total tasks attempted
    pub total_tasks: usize,

    /// Tasks completed successfully
    pub completed_tasks: usize,

    /// Correct answers
    pub correct_answers: usize,

    /// Accuracy (correct / total)
    pub accuracy: f64,

    /// Average similarity score
    pub avg_similarity: f64,

    /// Average execution time in ms
    pub avg_execution_time_ms: f64,

    /// Average reasoning steps
    pub avg_reasoning_steps: f64,

    /// Tasks by difficulty
    pub by_difficulty: HashMap<String, DifficultyMetrics>,

    /// Error breakdown
    pub error_counts: HashMap<String, usize>,
}

/// Metrics for a specific difficulty level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DifficultyMetrics {
    pub total: usize,
    pub correct: usize,
    pub accuracy: f64,
}

impl BenchmarkMetrics {
    /// Create empty metrics
    pub fn new(benchmark_type: BenchmarkType) -> Self {
        BenchmarkMetrics {
            benchmark_type,
            total_tasks: 0,
            completed_tasks: 0,
            correct_answers: 0,
            accuracy: 0.0,
            avg_similarity: 0.0,
            avg_execution_time_ms: 0.0,
            avg_reasoning_steps: 0.0,
            by_difficulty: HashMap::new(),
            error_counts: HashMap::new(),
        }
    }

    /// Calculate metrics from results
    pub fn calculate(benchmark_type: BenchmarkType, results: &[TaskResult]) -> Self {
        let total_tasks = results.len();
        if total_tasks == 0 {
            return Self::new(benchmark_type);
        }

        let completed_tasks = results.iter().filter(|r| r.error.is_none()).count();
        let correct_answers = results.iter().filter(|r| r.is_correct).count();

        let accuracy = correct_answers as f64 / total_tasks as f64;

        let total_similarity: f64 = results.iter().map(|r| r.similarity_score).sum();
        let avg_similarity = total_similarity / total_tasks as f64;

        let total_time: u64 = results.iter().map(|r| r.execution_time_ms).sum();
        let avg_execution_time_ms = total_time as f64 / total_tasks as f64;

        let total_steps: usize = results.iter().map(|r| r.reasoning_steps).sum();
        let avg_reasoning_steps = total_steps as f64 / total_tasks as f64;

        // Count errors
        let mut error_counts = HashMap::new();
        for result in results {
            if let Some(ref error) = result.error {
                let error_type = Self::classify_error(error);
                *error_counts.entry(error_type).or_insert(0) += 1;
            }
        }

        BenchmarkMetrics {
            benchmark_type,
            total_tasks,
            completed_tasks,
            correct_answers,
            accuracy,
            avg_similarity,
            avg_execution_time_ms,
            avg_reasoning_steps,
            by_difficulty: HashMap::new(),
            error_counts,
        }
    }

    /// Classify an error message into a category
    fn classify_error(error: &str) -> String {
        let error_lower = error.to_lowercase();
        if error_lower.contains("timeout") {
            "Timeout".to_string()
        } else if error_lower.contains("rate limit") {
            "RateLimit".to_string()
        } else if error_lower.contains("connection") || error_lower.contains("network") {
            "Network".to_string()
        } else if error_lower.contains("parse") || error_lower.contains("format") {
            "ParseError".to_string()
        } else if error_lower.contains("not found") || error_lower.contains("404") {
            "NotFound".to_string()
        } else {
            "Other".to_string()
        }
    }
}

// =================================================================
// CONVERGENCE VALIDATION
// =================================================================

/// Configuration for convergence validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceConfig {
    /// Target accuracy to achieve
    pub target_accuracy: f64,

    /// Baseline accuracy (e.g., original model)
    pub baseline_accuracy: f64,

    /// Minimum improvement required
    pub min_improvement: f64,

    /// Acceptable variance from target
    pub acceptable_variance: f64,

    /// Benchmarks to use for validation
    pub validation_benchmarks: Vec<BenchmarkType>,

    /// Minimum tasks required per benchmark
    pub min_tasks_per_benchmark: usize,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        ConvergenceConfig {
            target_accuracy: 0.90,
            baseline_accuracy: 0.667, // Current 66.7%
            min_improvement: 0.10,
            acceptable_variance: 0.05,
            validation_benchmarks: vec![BenchmarkType::GaiaText, BenchmarkType::Frames],
            min_tasks_per_benchmark: 20,
        }
    }
}

/// Result of convergence validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceResult {
    /// Overall convergence score (0.0 to 1.0)
    pub convergence_score: f64,

    /// Whether target was achieved
    pub target_achieved: bool,

    /// Improvement over baseline
    pub improvement: f64,

    /// Per-benchmark results
    pub benchmark_results: HashMap<String, BenchmarkConvergence>,

    /// Recommendations for improvement
    pub recommendations: Vec<String>,

    /// Validation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Convergence result for a single benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConvergence {
    pub benchmark_name: String,
    pub accuracy: f64,
    pub target: f64,
    pub achieved: bool,
    pub gap: f64,
}

impl ConvergenceResult {
    /// Calculate convergence from metrics
    pub fn calculate(
        config: &ConvergenceConfig,
        metrics: &HashMap<BenchmarkType, BenchmarkMetrics>,
    ) -> Self {
        let mut benchmark_results = HashMap::new();
        let mut total_accuracy = 0.0;
        let mut count = 0;

        for benchmark in &config.validation_benchmarks {
            if let Some(m) = metrics.get(benchmark) {
                let achieved = m.accuracy >= config.target_accuracy - config.acceptable_variance;
                let gap = config.target_accuracy - m.accuracy;

                benchmark_results.insert(
                    benchmark.display_name().to_string(),
                    BenchmarkConvergence {
                        benchmark_name: benchmark.display_name().to_string(),
                        accuracy: m.accuracy,
                        target: config.target_accuracy,
                        achieved,
                        gap: gap.max(0.0),
                    },
                );

                total_accuracy += m.accuracy;
                count += 1;
            }
        }

        let avg_accuracy = if count > 0 {
            total_accuracy / count as f64
        } else {
            0.0
        };
        let improvement = avg_accuracy - config.baseline_accuracy;
        let target_achieved = avg_accuracy >= config.target_accuracy - config.acceptable_variance;

        // Calculate convergence score (normalized 0-1)
        let convergence_score = (avg_accuracy / config.target_accuracy).min(1.0);

        // Generate recommendations
        let mut recommendations = Vec::new();

        if !target_achieved {
            recommendations.push(format!(
                "Overall accuracy ({:.1}%) is below target ({:.1}%). Consider:",
                avg_accuracy * 100.0,
                config.target_accuracy * 100.0
            ));

            for (name, result) in &benchmark_results {
                if !result.achieved {
                    recommendations.push(format!(
                        "  - {} needs {:.1}% improvement (current: {:.1}%)",
                        name,
                        result.gap * 100.0,
                        result.accuracy * 100.0
                    ));
                }
            }

            recommendations
                .push("  - Increase observation turns for pattern refinement".to_string());
            recommendations.push("  - Enable RL optimization for targeted improvement".to_string());
        } else {
            recommendations.push(format!(
                "Target achieved! Accuracy: {:.1}%, Improvement: +{:.1}%",
                avg_accuracy * 100.0,
                improvement * 100.0
            ));
        }

        ConvergenceResult {
            convergence_score,
            target_achieved,
            improvement,
            benchmark_results,
            recommendations,
            timestamp: Utc::now(),
        }
    }
}

// =================================================================
// ANSWER SCORING
// =================================================================

/// Scoring utilities for comparing answers
pub struct AnswerScorer;

impl AnswerScorer {
    /// Score answer against reference
    pub fn score(model_answer: &str, reference: &str) -> (bool, f64) {
        let model_normalized = Self::normalize(model_answer);
        let ref_normalized = Self::normalize(reference);

        // Exact match
        if model_normalized == ref_normalized {
            return (true, 1.0);
        }

        // Fuzzy match
        let similarity = Self::calculate_similarity(&model_normalized, &ref_normalized);

        // Consider correct if very high similarity
        let is_correct = similarity >= 0.95;

        (is_correct, similarity)
    }

    /// Normalize an answer for comparison
    fn normalize(answer: &str) -> String {
        answer
            .to_lowercase()
            .trim()
            .replace("the ", "")
            .replace("a ", "")
            .replace("an ", "")
            .replace(",", "")
            .replace(".", "")
            .replace("  ", " ")
    }

    /// Calculate similarity between two strings
    fn calculate_similarity(a: &str, b: &str) -> f64 {
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        // Simple token overlap similarity
        let tokens_a: std::collections::HashSet<_> = a.split_whitespace().collect();
        let tokens_b: std::collections::HashSet<_> = b.split_whitespace().collect();

        let intersection = tokens_a.intersection(&tokens_b).count();
        let union = tokens_a.union(&tokens_b).count();

        if union == 0 {
            return 0.0;
        }

        intersection as f64 / union as f64
    }

    /// Extract final answer from model response
    pub fn extract_answer(response: &str) -> String {
        // Common patterns for final answers
        let patterns = [
            "final answer:",
            "the answer is:",
            "the answer is ",
            "answer:",
            "answer is ",
            "therefore,",
            "so the answer is",
            "result:",
        ];

        let response_lower = response.to_lowercase();

        for pattern in patterns {
            if let Some(idx) = response_lower.find(pattern) {
                let start = idx + pattern.len();
                let remaining = &response[start..];

                // Take until end of sentence or end of string
                let end = remaining.find('.').unwrap_or(remaining.len());
                let answer = remaining[..end].trim();

                if !answer.is_empty() {
                    return answer.to_string();
                }
            }
        }

        // Fallback: return last sentence
        if let Some(last_period) = response.rfind('.') {
            if let Some(second_last) = response[..last_period].rfind('.') {
                return response[second_last + 1..last_period].trim().to_string();
            }
        }

        // Last resort: return truncated response
        response.chars().take(100).collect()
    }
}

// =================================================================
// BENCHMARK RUNNER
// =================================================================

/// Main benchmark runner
pub struct BenchmarkRunner {
    /// Configuration
    config: BenchmarkSuiteConfig,

    /// Loaded tasks by benchmark
    tasks: HashMap<BenchmarkType, Vec<BenchmarkTask>>,

    /// Results by benchmark
    results: HashMap<BenchmarkType, Vec<TaskResult>>,

    /// Metrics by benchmark
    metrics: HashMap<BenchmarkType, BenchmarkMetrics>,

    /// Statistics
    stats: RunnerStats,
}

/// Runner statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunnerStats {
    pub total_tasks_loaded: usize,
    pub total_tasks_executed: usize,
    pub total_correct: usize,
    pub total_errors: usize,
    pub total_execution_time_ms: u64,
    pub runs_completed: usize,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(config: BenchmarkSuiteConfig) -> Self {
        BenchmarkRunner {
            config,
            tasks: HashMap::new(),
            results: HashMap::new(),
            metrics: HashMap::new(),
            stats: RunnerStats::default(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(BenchmarkSuiteConfig::default())
    }

    /// Create for quick validation
    pub fn quick_validation() -> Self {
        Self::new(BenchmarkSuiteConfig::quick_validation())
    }

    /// Load tasks for a benchmark (stub - would load from JSONL)
    pub fn load_tasks(&mut self, benchmark: BenchmarkType) -> Result<usize, String> {
        // In a real implementation, this would load from JSONL files
        // For now, create sample tasks for testing

        let sample_tasks = self.create_sample_tasks(&benchmark);
        let count = sample_tasks.len();

        self.tasks.insert(benchmark, sample_tasks);
        self.stats.total_tasks_loaded += count;

        Ok(count)
    }

    /// Create sample tasks for testing (stub implementation)
    fn create_sample_tasks(&self, benchmark: &BenchmarkType) -> Vec<BenchmarkTask> {
        let limit = if self.config.max_tasks_per_benchmark > 0 {
            self.config.max_tasks_per_benchmark
        } else {
            10 // Default for sample
        };

        (0..limit)
            .map(|i| {
                BenchmarkTask::new(
                    &format!("{}_{}", benchmark.directory_name(), i),
                    &format!("Sample question {} for {}", i, benchmark.display_name()),
                    &format!("Sample answer {}", i),
                    benchmark.clone(),
                )
            })
            .collect()
    }

    /// Execute a single task with a model response
    pub fn execute_task(&mut self, task: &BenchmarkTask, model_response: &str) -> TaskResult {
        let start = std::time::Instant::now();

        // Extract answer
        let extracted_answer = AnswerScorer::extract_answer(model_response);

        // Score against reference
        let (is_correct, similarity) =
            AnswerScorer::score(&extracted_answer, &task.reference_answer);

        let execution_time = start.elapsed().as_millis() as u64;

        self.stats.total_tasks_executed += 1;
        self.stats.total_execution_time_ms += execution_time;
        if is_correct {
            self.stats.total_correct += 1;
        }

        TaskResult::success(
            &task.task_id,
            model_response,
            &extracted_answer,
            is_correct,
            similarity,
            execution_time,
        )
    }

    /// Add a result for a benchmark
    pub fn add_result(&mut self, benchmark: BenchmarkType, result: TaskResult) {
        self.results.entry(benchmark).or_default().push(result);
    }

    /// Calculate metrics for all benchmarks
    pub fn calculate_all_metrics(&mut self) {
        for (benchmark, results) in &self.results {
            let metrics = BenchmarkMetrics::calculate(benchmark.clone(), results);
            self.metrics.insert(benchmark.clone(), metrics);
        }
    }

    /// Get metrics for a benchmark
    pub fn get_metrics(&self, benchmark: &BenchmarkType) -> Option<&BenchmarkMetrics> {
        self.metrics.get(benchmark)
    }

    /// Get all metrics
    pub fn get_all_metrics(&self) -> &HashMap<BenchmarkType, BenchmarkMetrics> {
        &self.metrics
    }

    /// Validate convergence
    pub fn validate_convergence(&self, config: &ConvergenceConfig) -> ConvergenceResult {
        ConvergenceResult::calculate(config, &self.metrics)
    }

    /// Get statistics
    pub fn get_stats(&self) -> &RunnerStats {
        &self.stats
    }

    /// Generate a summary report
    pub fn summary_report(&self) -> String {
        let mut lines = vec![
            "=== BENCHMARK SUMMARY REPORT ===".to_string(),
            format!("Suite: {}", self.config.name),
            format!("Benchmarks: {}", self.config.benchmarks.len()),
            format!("Total tasks loaded: {}", self.stats.total_tasks_loaded),
            format!("Total tasks executed: {}", self.stats.total_tasks_executed),
            format!("Total correct: {}", self.stats.total_correct),
            format!(
                "Overall accuracy: {:.1}%",
                if self.stats.total_tasks_executed > 0 {
                    self.stats.total_correct as f64 / self.stats.total_tasks_executed as f64 * 100.0
                } else {
                    0.0
                }
            ),
            String::new(),
            "Per-benchmark results:".to_string(),
        ];

        for (benchmark, metrics) in &self.metrics {
            lines.push(format!(
                "  {}: {:.1}% accuracy ({}/{} correct)",
                benchmark.display_name(),
                metrics.accuracy * 100.0,
                metrics.correct_answers,
                metrics.total_tasks,
            ));
        }

        if !self.metrics.is_empty() {
            lines.push(String::new());

            let config = ConvergenceConfig::default();
            let convergence = self.validate_convergence(&config);

            lines.push("Convergence validation:".to_string());
            lines.push(format!(
                "  Score: {:.1}%",
                convergence.convergence_score * 100.0
            ));
            lines.push(format!(
                "  Target achieved: {}",
                convergence.target_achieved
            ));
            lines.push(format!(
                "  Improvement: {:+.1}%",
                convergence.improvement * 100.0
            ));

            lines.push(String::new());
            lines.push("Recommendations:".to_string());
            for rec in &convergence.recommendations {
                lines.push(format!("  {}", rec));
            }
        }

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
    fn test_benchmark_type_directory() {
        assert_eq!(BenchmarkType::Gaia.directory_name(), "gaia");
        assert_eq!(BenchmarkType::GaiaText.directory_name(), "gaia_text");
        assert_eq!(BenchmarkType::HleText.directory_name(), "hle_text");
    }

    #[test]
    fn test_benchmark_type_all_standard() {
        let all = BenchmarkType::all_standard();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn test_benchmark_task_creation() {
        let task = BenchmarkTask::new("task_1", "What is 2+2?", "4", BenchmarkType::GaiaText)
            .with_difficulty(TaskDifficulty::Easy);

        assert_eq!(task.task_id, "task_1");
        assert_eq!(task.difficulty, Some(TaskDifficulty::Easy));
    }

    #[test]
    fn test_task_result_success() {
        let result = TaskResult::success("task_1", "The answer is 4", "4", true, 1.0, 100);

        assert!(result.is_correct);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_task_result_failure() {
        let result = TaskResult::failure("task_1", "Timeout error");

        assert!(!result.is_correct);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_answer_scorer_exact_match() {
        let (is_correct, similarity) = AnswerScorer::score("Paris", "Paris");
        assert!(is_correct);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_answer_scorer_normalized_match() {
        let (is_correct, similarity) = AnswerScorer::score("The Paris", "paris");
        assert!(is_correct);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_answer_scorer_partial_match() {
        let (is_correct, similarity) = AnswerScorer::score("Paris is the capital", "Paris");
        // Partial match should have lower similarity
        assert!(similarity > 0.0 && similarity < 1.0);
    }

    #[test]
    fn test_answer_scorer_extract() {
        let response = "After analyzing the data, the final answer: 42";
        let extracted = AnswerScorer::extract_answer(response);
        assert_eq!(extracted, "42");
    }

    #[test]
    fn test_benchmark_metrics_calculate() {
        let results = vec![
            TaskResult::success("1", "response", "answer", true, 1.0, 100),
            TaskResult::success("2", "response", "answer", true, 0.9, 150),
            TaskResult::success("3", "response", "wrong", false, 0.5, 200),
        ];

        let metrics = BenchmarkMetrics::calculate(BenchmarkType::GaiaText, &results);

        assert_eq!(metrics.total_tasks, 3);
        assert_eq!(metrics.correct_answers, 2);
        assert!((metrics.accuracy - 0.667).abs() < 0.01);
    }

    #[test]
    fn test_convergence_result() {
        let mut metrics = HashMap::new();
        metrics.insert(
            BenchmarkType::GaiaText,
            BenchmarkMetrics {
                benchmark_type: BenchmarkType::GaiaText,
                total_tasks: 100,
                completed_tasks: 100,
                correct_answers: 80, // 80% accuracy - below target
                accuracy: 0.80,
                avg_similarity: 0.85,
                avg_execution_time_ms: 150.0,
                avg_reasoning_steps: 3.0,
                by_difficulty: HashMap::new(),
                error_counts: HashMap::new(),
            },
        );

        let config = ConvergenceConfig {
            target_accuracy: 0.90,
            baseline_accuracy: 0.667,
            acceptable_variance: 0.05,
            validation_benchmarks: vec![BenchmarkType::GaiaText], // Only test GaiaText
            ..Default::default()
        };

        let result = ConvergenceResult::calculate(&config, &metrics);

        // 80% is below 90% - 5% = 85%, so target not achieved
        assert!(!result.target_achieved);
        assert!(result.improvement > 0.10); // 80% - 66.7% = 13.3%
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_benchmark_runner_creation() {
        let runner = BenchmarkRunner::quick_validation();
        assert_eq!(runner.config.name, "quick_validation");
        assert_eq!(runner.config.max_tasks_per_benchmark, 10);
    }

    #[test]
    fn test_benchmark_runner_load_tasks() {
        let mut runner = BenchmarkRunner::quick_validation();
        let count = runner.load_tasks(BenchmarkType::GaiaText).unwrap();

        assert_eq!(count, 10);
        assert_eq!(runner.stats.total_tasks_loaded, 10);
    }

    #[test]
    fn test_benchmark_runner_execute_task() {
        let mut runner = BenchmarkRunner::quick_validation();

        let task = BenchmarkTask::new("test_1", "What is 2+2?", "4", BenchmarkType::GaiaText);

        let result = runner.execute_task(&task, "The answer is 4");

        assert!(result.is_correct);
        assert_eq!(runner.stats.total_tasks_executed, 1);
        assert_eq!(runner.stats.total_correct, 1);
    }

    #[test]
    fn test_benchmark_suite_config_presets() {
        let quick = BenchmarkSuiteConfig::quick_validation();
        assert_eq!(quick.max_tasks_per_benchmark, 10);
        assert!(!quick.retry_failed);

        let full = BenchmarkSuiteConfig::full_benchmark();
        assert_eq!(full.max_tasks_per_benchmark, 0);
        assert!(full.retry_failed);

        let convergence = BenchmarkSuiteConfig::convergence_test();
        assert_eq!(convergence.max_tasks_per_benchmark, 50);
    }

    #[test]
    fn test_reasoning_step() {
        let step = ReasoningStep {
            step: 1,
            action: "search".to_string(),
            input: "query".to_string(),
            output: "results".to_string(),
            confidence: 0.9,
        };

        let json = serde_json::to_string(&step).unwrap();
        let restored: ReasoningStep = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.step, 1);
        assert_eq!(restored.action, "search");
    }

    #[test]
    fn test_difficulty_metrics() {
        let metrics = DifficultyMetrics {
            total: 10,
            correct: 8,
            accuracy: 0.8,
        };

        assert_eq!(metrics.accuracy, 0.8);
    }
}
