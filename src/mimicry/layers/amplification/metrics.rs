//! Metrics collection for amplification analysis.
//!
//! Provides detailed metrics about the amplification process
//! for monitoring, debugging, and optimization.

use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Metrics for a single iteration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationMetrics {
    /// Iteration number.
    pub iteration: u32,
    /// Confidence at this iteration.
    pub confidence: f32,
    /// Change from previous iteration.
    pub delta: f32,
    /// Amplification factor applied.
    pub factor: f32,
}

/// Aggregated metrics for the entire amplification process.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmplificationMetrics {
    /// Total number of iterations.
    pub total_iterations: u32,
    /// Initial confidence.
    pub initial_confidence: f32,
    /// Final confidence.
    pub final_confidence: f32,
    /// Peak confidence achieved.
    pub peak_confidence: f32,
    /// Minimum confidence during process.
    pub min_confidence: f32,
    /// Total amplification factor.
    pub total_amplification: f32,
    /// Average delta per iteration.
    pub average_delta: f32,
    /// Maximum delta observed.
    pub max_delta: f32,
    /// Average amplification factor.
    pub average_factor: f32,
    /// Whether the process converged.
    pub converged: bool,
    /// Processing time in microseconds.
    pub processing_time_us: u64,
    /// Per-iteration data (optional, for detailed analysis).
    pub iterations: Vec<IterationMetrics>,
}

impl AmplificationMetrics {
    /// Calculate the efficiency (how quickly we reached final value).
    pub fn efficiency(&self) -> f32 {
        if self.total_iterations == 0 || self.initial_confidence == 0.0 {
            return 0.0;
        }
        self.total_amplification / self.total_iterations as f32
    }

    /// Calculate the stability (inverse of average delta).
    pub fn stability(&self) -> f32 {
        if self.average_delta == 0.0 {
            return 1.0;
        }
        1.0 / (1.0 + self.average_delta * 10.0)
    }

    /// Get a summary string.
    pub fn summary(&self) -> String {
        format!(
            "Iterations: {}, Amplification: {:.2}x, Final: {:.4}, Converged: {}",
            self.total_iterations, self.total_amplification, self.final_confidence, self.converged
        )
    }
}

/// Collector for amplification metrics.
pub struct MetricsCollector {
    iterations: RefCell<Vec<IterationMetrics>>,
    initial: RefCell<Option<f32>>,
}

impl MetricsCollector {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        Self {
            iterations: RefCell::new(Vec::new()),
            initial: RefCell::new(None),
        }
    }

    /// Reset the collector.
    pub fn reset(&self) {
        self.iterations.borrow_mut().clear();
        *self.initial.borrow_mut() = None;
    }

    /// Set the initial confidence.
    pub fn set_initial(&self, confidence: f32) {
        *self.initial.borrow_mut() = Some(confidence);
    }

    /// Record an iteration.
    pub fn record_iteration(&self, metrics: IterationMetrics) {
        let mut iterations = self.iterations.borrow_mut();

        // Set initial from first iteration if not set
        if self.initial.borrow().is_none() && iterations.is_empty() {
            *self.initial.borrow_mut() = Some(metrics.confidence);
        }

        iterations.push(metrics);
    }

    /// Collect all metrics into an aggregated result.
    pub fn collect(&self) -> AmplificationMetrics {
        let iterations = self.iterations.borrow();

        if iterations.is_empty() {
            return AmplificationMetrics::default();
        }

        let initial = self
            .initial
            .borrow()
            .unwrap_or_else(|| iterations[0].confidence);
        let final_conf = iterations.last().map(|m| m.confidence).unwrap_or(initial);

        let peak = iterations
            .iter()
            .map(|m| m.confidence)
            .fold(0.0f32, f32::max);
        let min = iterations
            .iter()
            .map(|m| m.confidence)
            .fold(f32::INFINITY, f32::min);

        let total_delta: f32 = iterations.iter().map(|m| m.delta).sum();
        let avg_delta = total_delta / iterations.len() as f32;
        let max_delta = iterations.iter().map(|m| m.delta).fold(0.0f32, f32::max);

        let total_factor: f32 = iterations.iter().map(|m| m.factor).sum();
        let avg_factor = total_factor / iterations.len() as f32;

        let total_amplification = if initial > 0.0 {
            final_conf / initial
        } else {
            1.0
        };

        AmplificationMetrics {
            total_iterations: iterations.len() as u32,
            initial_confidence: initial,
            final_confidence: final_conf,
            peak_confidence: peak,
            min_confidence: if min.is_infinite() { 0.0 } else { min },
            total_amplification,
            average_delta: avg_delta,
            max_delta,
            average_factor: avg_factor,
            converged: iterations.last().map(|m| m.delta < 0.001).unwrap_or(false),
            processing_time_us: 0, // Set by caller
            iterations: iterations.clone(),
        }
    }

    /// Get the number of recorded iterations.
    pub fn iteration_count(&self) -> usize {
        self.iterations.borrow().len()
    }

    /// Get the current confidence.
    pub fn current_confidence(&self) -> Option<f32> {
        self.iterations.borrow().last().map(|m| m.confidence)
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Histogram bucket for distribution analysis.
#[derive(Debug, Clone)]
pub struct HistogramBucket {
    /// Lower bound of bucket.
    pub min: f32,
    /// Upper bound of bucket.
    pub max: f32,
    /// Count of values in this bucket.
    pub count: usize,
}

/// Distribution analyzer for metrics.
pub struct DistributionAnalyzer {
    values: Vec<f32>,
    bucket_count: usize,
}

impl DistributionAnalyzer {
    /// Create a new distribution analyzer.
    pub fn new(bucket_count: usize) -> Self {
        Self {
            values: Vec::new(),
            bucket_count: bucket_count.max(1),
        }
    }

    /// Add a value.
    pub fn add(&mut self, value: f32) {
        if !value.is_nan() && !value.is_infinite() {
            self.values.push(value);
        }
    }

    /// Get the histogram.
    pub fn histogram(&self) -> Vec<HistogramBucket> {
        if self.values.is_empty() {
            return Vec::new();
        }

        let min = self.values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self
            .values
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);

        if min == max {
            return vec![HistogramBucket {
                min,
                max,
                count: self.values.len(),
            }];
        }

        let range = max - min;
        let bucket_size = range / self.bucket_count as f32;

        let mut buckets: Vec<HistogramBucket> = (0..self.bucket_count)
            .map(|i| HistogramBucket {
                min: min + i as f32 * bucket_size,
                max: min + (i + 1) as f32 * bucket_size,
                count: 0,
            })
            .collect();

        for &value in &self.values {
            let bucket_idx = ((value - min) / bucket_size) as usize;
            let idx = bucket_idx.min(self.bucket_count - 1);
            buckets[idx].count += 1;
        }

        buckets
    }

    /// Get basic statistics.
    pub fn statistics(&self) -> DistributionStats {
        if self.values.is_empty() {
            return DistributionStats::default();
        }

        let sum: f32 = self.values.iter().sum();
        let mean = sum / self.values.len() as f32;

        let variance: f32 =
            self.values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / self.values.len() as f32;
        let std_dev = variance.sqrt();

        let min = self.values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self
            .values
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);

        let mut sorted = self.values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = sorted[sorted.len() / 2];

        DistributionStats {
            count: self.values.len(),
            mean,
            median,
            std_dev,
            min,
            max,
            range: max - min,
        }
    }

    /// Reset the analyzer.
    pub fn reset(&mut self) {
        self.values.clear();
    }
}

/// Statistics about a distribution.
#[derive(Debug, Clone, Default)]
pub struct DistributionStats {
    /// Number of values.
    pub count: usize,
    /// Mean value.
    pub mean: f32,
    /// Median value.
    pub median: f32,
    /// Standard deviation.
    pub std_dev: f32,
    /// Minimum value.
    pub min: f32,
    /// Maximum value.
    pub max: f32,
    /// Range (max - min).
    pub range: f32,
}

/// Metrics aggregator for multiple amplification runs.
pub struct MetricsAggregator {
    runs: Vec<AmplificationMetrics>,
    by_category: HashMap<String, Vec<AmplificationMetrics>>,
}

impl MetricsAggregator {
    /// Create a new aggregator.
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            by_category: HashMap::new(),
        }
    }

    /// Add metrics from a run.
    pub fn add(&mut self, metrics: AmplificationMetrics) {
        self.runs.push(metrics);
    }

    /// Add metrics with a category.
    pub fn add_categorized(&mut self, category: &str, metrics: AmplificationMetrics) {
        self.runs.push(metrics.clone());
        self.by_category
            .entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(metrics);
    }

    /// Get summary statistics across all runs.
    pub fn summary(&self) -> AggregatedSummary {
        if self.runs.is_empty() {
            return AggregatedSummary::default();
        }

        let total_runs = self.runs.len();
        let converged_runs = self.runs.iter().filter(|m| m.converged).count();

        let avg_iterations =
            self.runs.iter().map(|m| m.total_iterations).sum::<u32>() as f32 / total_runs as f32;
        let avg_amplification =
            self.runs.iter().map(|m| m.total_amplification).sum::<f32>() / total_runs as f32;
        let avg_time =
            self.runs.iter().map(|m| m.processing_time_us).sum::<u64>() as f32 / total_runs as f32;

        AggregatedSummary {
            total_runs,
            converged_runs,
            convergence_rate: converged_runs as f32 / total_runs as f32,
            average_iterations: avg_iterations,
            average_amplification: avg_amplification,
            average_processing_time_us: avg_time,
        }
    }

    /// Get summary for a specific category.
    pub fn category_summary(&self, category: &str) -> Option<AggregatedSummary> {
        self.by_category.get(category).map(|runs| {
            let total_runs = runs.len();
            let converged_runs = runs.iter().filter(|m| m.converged).count();

            let avg_iterations =
                runs.iter().map(|m| m.total_iterations).sum::<u32>() as f32 / total_runs as f32;
            let avg_amplification =
                runs.iter().map(|m| m.total_amplification).sum::<f32>() / total_runs as f32;
            let avg_time =
                runs.iter().map(|m| m.processing_time_us).sum::<u64>() as f32 / total_runs as f32;

            AggregatedSummary {
                total_runs,
                converged_runs,
                convergence_rate: converged_runs as f32 / total_runs as f32,
                average_iterations: avg_iterations,
                average_amplification: avg_amplification,
                average_processing_time_us: avg_time,
            }
        })
    }

    /// Get all runs.
    pub fn runs(&self) -> &[AmplificationMetrics] {
        &self.runs
    }

    /// Clear all data.
    pub fn reset(&mut self) {
        self.runs.clear();
        self.by_category.clear();
    }
}

impl Default for MetricsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of aggregated metrics.
#[derive(Debug, Clone, Default)]
pub struct AggregatedSummary {
    /// Total number of runs.
    pub total_runs: usize,
    /// Number of converged runs.
    pub converged_runs: usize,
    /// Convergence rate (0.0-1.0).
    pub convergence_rate: f32,
    /// Average iterations per run.
    pub average_iterations: f32,
    /// Average amplification factor.
    pub average_amplification: f32,
    /// Average processing time in microseconds.
    pub average_processing_time_us: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let collector = MetricsCollector::new();

        collector.record_iteration(IterationMetrics {
            iteration: 0,
            confidence: 0.5,
            delta: 0.0,
            factor: 1.1,
        });
        collector.record_iteration(IterationMetrics {
            iteration: 1,
            confidence: 0.55,
            delta: 0.05,
            factor: 1.1,
        });
        collector.record_iteration(IterationMetrics {
            iteration: 2,
            confidence: 0.58,
            delta: 0.03,
            factor: 1.05,
        });

        let metrics = collector.collect();
        assert_eq!(metrics.total_iterations, 3);
        assert!((metrics.initial_confidence - 0.5).abs() < 0.01);
        assert!((metrics.final_confidence - 0.58).abs() < 0.01);
    }

    #[test]
    fn test_distribution_analyzer() {
        let mut analyzer = DistributionAnalyzer::new(5);

        for i in 0..100 {
            analyzer.add(i as f32 / 100.0);
        }

        let stats = analyzer.statistics();
        assert_eq!(stats.count, 100);
        assert!(stats.mean > 0.4 && stats.mean < 0.6);
        assert!((stats.min - 0.0).abs() < 0.01);
        assert!((stats.max - 0.99).abs() < 0.01);
    }

    #[test]
    fn test_histogram() {
        let mut analyzer = DistributionAnalyzer::new(4);

        analyzer.add(0.1);
        analyzer.add(0.2);
        analyzer.add(0.3);
        analyzer.add(0.7);
        analyzer.add(0.8);
        analyzer.add(0.9);

        let histogram = analyzer.histogram();
        assert_eq!(histogram.len(), 4);

        // Values should be distributed across buckets
        let total_count: usize = histogram.iter().map(|b| b.count).sum();
        assert_eq!(total_count, 6);
    }

    #[test]
    fn test_metrics_aggregator() {
        let mut aggregator = MetricsAggregator::new();

        aggregator.add(AmplificationMetrics {
            total_iterations: 5,
            converged: true,
            total_amplification: 1.5,
            ..Default::default()
        });
        aggregator.add(AmplificationMetrics {
            total_iterations: 10,
            converged: true,
            total_amplification: 2.0,
            ..Default::default()
        });
        aggregator.add(AmplificationMetrics {
            total_iterations: 20,
            converged: false,
            total_amplification: 1.2,
            ..Default::default()
        });

        let summary = aggregator.summary();
        assert_eq!(summary.total_runs, 3);
        assert_eq!(summary.converged_runs, 2);
        assert!((summary.convergence_rate - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_amplification_metrics_efficiency() {
        let metrics = AmplificationMetrics {
            total_iterations: 10,
            initial_confidence: 0.5,
            final_confidence: 1.0,
            total_amplification: 2.0,
            ..Default::default()
        };

        assert!((metrics.efficiency() - 0.2).abs() < 0.01);
    }
}
