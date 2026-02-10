//! Convergence detection for amplification.
//!
//! Determines when the amplification process has stabilized
//! and should terminate.

use std::cell::RefCell;

use serde::{Deserialize, Serialize};

/// Configuration for convergence detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceConfig {
    /// Threshold for considering the process converged.
    /// When delta falls below this, we're converged.
    pub threshold: f32,

    /// Number of consecutive iterations below threshold required.
    pub stable_iterations: u32,

    /// Divergence threshold (when delta exceeds this, we're diverging).
    pub divergence_threshold: f32,

    /// Maximum rate of increase before triggering divergence.
    pub max_growth_rate: f32,

    /// Window size for moving average calculation.
    pub window_size: usize,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            threshold: 0.001,
            stable_iterations: 3,
            divergence_threshold: 100.0,
            max_growth_rate: 2.0,
            window_size: 5,
        }
    }
}

impl ConvergenceConfig {
    /// Strict convergence (lower threshold).
    pub fn strict() -> Self {
        Self {
            threshold: 0.0001,
            stable_iterations: 5,
            ..Default::default()
        }
    }

    /// Relaxed convergence (higher threshold).
    pub fn relaxed() -> Self {
        Self {
            threshold: 0.01,
            stable_iterations: 2,
            ..Default::default()
        }
    }
}

/// Status of convergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergenceStatus {
    /// Still converging.
    InProgress,
    /// Successfully converged.
    Converged,
    /// Diverging (unbounded growth).
    Diverging,
}

/// Internal state for convergence tracking.
struct ConvergenceState {
    consecutive_stable: u32,
    history: Vec<f32>,
    last_value: Option<f32>,
    max_seen: f32,
}

impl ConvergenceState {
    fn new() -> Self {
        Self {
            consecutive_stable: 0,
            history: Vec::new(),
            last_value: None,
            max_seen: 0.0,
        }
    }

    fn reset(&mut self) {
        self.consecutive_stable = 0;
        self.history.clear();
        self.last_value = None;
        self.max_seen = 0.0;
    }
}

/// Detector for convergence in iterative processes.
pub struct ConvergenceDetector {
    config: ConvergenceConfig,
    state: RefCell<ConvergenceState>,
}

impl ConvergenceDetector {
    /// Create a new convergence detector.
    pub fn new(config: ConvergenceConfig) -> Self {
        Self {
            config,
            state: RefCell::new(ConvergenceState::new()),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(ConvergenceConfig::default())
    }

    /// Get the configuration.
    pub fn config(&self) -> &ConvergenceConfig {
        &self.config
    }

    /// Reset the detector state.
    pub fn reset(&self) {
        self.state.borrow_mut().reset();
    }

    /// Check convergence status given old and new values.
    pub fn check(&self, old_value: f32, new_value: f32, _iteration: u32) -> ConvergenceStatus {
        let mut state = self.state.borrow_mut();

        // Calculate delta
        let delta = (new_value - old_value).abs();

        // Update history
        state.history.push(delta);
        if state.history.len() > self.config.window_size {
            state.history.remove(0);
        }

        // Track maximum
        state.max_seen = state.max_seen.max(new_value);

        // Check for divergence
        if new_value.is_infinite() || new_value.is_nan() {
            return ConvergenceStatus::Diverging;
        }

        if new_value > self.config.divergence_threshold {
            return ConvergenceStatus::Diverging;
        }

        // Check growth rate
        if let Some(last) = state.last_value {
            if last > 0.0 {
                let growth = new_value / last;
                if growth > self.config.max_growth_rate {
                    return ConvergenceStatus::Diverging;
                }
            }
        }

        // Update last value
        state.last_value = Some(new_value);

        // Check for convergence
        if delta < self.config.threshold {
            state.consecutive_stable += 1;
            if state.consecutive_stable >= self.config.stable_iterations {
                return ConvergenceStatus::Converged;
            }
        } else {
            state.consecutive_stable = 0;
        }

        ConvergenceStatus::InProgress
    }

    /// Calculate the moving average of deltas.
    pub fn moving_average_delta(&self) -> f32 {
        let state = self.state.borrow();
        if state.history.is_empty() {
            return 0.0;
        }
        state.history.iter().sum::<f32>() / state.history.len() as f32
    }

    /// Get the maximum value seen.
    pub fn max_seen(&self) -> f32 {
        self.state.borrow().max_seen
    }

    /// Get the number of consecutive stable iterations.
    pub fn consecutive_stable(&self) -> u32 {
        self.state.borrow().consecutive_stable
    }

    /// Check if currently stable (but not yet converged).
    pub fn is_stable(&self) -> bool {
        self.state.borrow().consecutive_stable > 0
    }

    /// Get the trend (positive = increasing, negative = decreasing).
    pub fn trend(&self) -> f32 {
        let state = self.state.borrow();
        if state.history.len() < 2 {
            return 0.0;
        }

        let recent = &state.history[state.history.len().saturating_sub(3)..];
        if recent.len() < 2 {
            return 0.0;
        }

        let first_half: f32 = recent[..recent.len() / 2].iter().sum();
        let second_half: f32 = recent[recent.len() / 2..].iter().sum();

        second_half - first_half
    }
}

impl Default for ConvergenceDetector {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Statistics about convergence.
#[derive(Debug, Clone, Default)]
pub struct ConvergenceStats {
    /// Number of iterations to converge.
    pub iterations: u32,
    /// Final delta value.
    pub final_delta: f32,
    /// Whether convergence was achieved.
    pub converged: bool,
    /// Whether divergence was detected.
    pub diverged: bool,
    /// Average delta over all iterations.
    pub average_delta: f32,
    /// Maximum delta observed.
    pub max_delta: f32,
    /// Minimum delta observed.
    pub min_delta: f32,
}

/// Convergence analyzer for post-hoc analysis.
pub struct ConvergenceAnalyzer {
    deltas: Vec<f32>,
    values: Vec<f32>,
}

impl ConvergenceAnalyzer {
    /// Create a new analyzer.
    pub fn new() -> Self {
        Self {
            deltas: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Record a value.
    pub fn record(&mut self, value: f32) {
        if let Some(&last) = self.values.last() {
            self.deltas.push((value - last).abs());
        }
        self.values.push(value);
    }

    /// Analyze the recorded data.
    pub fn analyze(&self) -> ConvergenceStats {
        if self.deltas.is_empty() {
            return ConvergenceStats::default();
        }

        let sum: f32 = self.deltas.iter().sum();
        let avg = sum / self.deltas.len() as f32;
        let max = self.deltas.iter().cloned().fold(0.0f32, f32::max);
        let min = self.deltas.iter().cloned().fold(f32::INFINITY, f32::min);
        let final_delta = *self.deltas.last().unwrap_or(&0.0);

        ConvergenceStats {
            iterations: self.values.len() as u32,
            final_delta,
            converged: final_delta < 0.001,
            diverged: self.values.last().map(|v| v.is_infinite()).unwrap_or(false),
            average_delta: avg,
            max_delta: max,
            min_delta: if min.is_infinite() { 0.0 } else { min },
        }
    }

    /// Reset the analyzer.
    pub fn reset(&mut self) {
        self.deltas.clear();
        self.values.clear();
    }
}

impl Default for ConvergenceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_detector_creation() {
        let detector = ConvergenceDetector::with_defaults();
        assert_eq!(detector.config().threshold, 0.001);
    }

    #[test]
    fn test_convergence_detection() {
        let config = ConvergenceConfig {
            threshold: 0.01,
            stable_iterations: 2,
            ..Default::default()
        };
        let detector = ConvergenceDetector::new(config);

        // Not converged yet
        assert_eq!(detector.check(1.0, 1.1, 0), ConvergenceStatus::InProgress);

        // Small change
        assert_eq!(detector.check(1.1, 1.105, 1), ConvergenceStatus::InProgress);

        // Another small change - should converge
        assert_eq!(
            detector.check(1.105, 1.108, 2),
            ConvergenceStatus::Converged
        );
    }

    #[test]
    fn test_divergence_detection() {
        let detector = ConvergenceDetector::with_defaults();

        // Normal change
        assert_eq!(detector.check(1.0, 1.5, 0), ConvergenceStatus::InProgress);

        // Diverging (infinite)
        assert_eq!(
            detector.check(1.5, f32::INFINITY, 1),
            ConvergenceStatus::Diverging
        );
    }

    #[test]
    fn test_divergence_by_threshold() {
        let config = ConvergenceConfig {
            divergence_threshold: 10.0,
            ..Default::default()
        };
        let detector = ConvergenceDetector::new(config);

        // Normal
        assert_eq!(detector.check(1.0, 5.0, 0), ConvergenceStatus::InProgress);

        // Over threshold
        assert_eq!(detector.check(5.0, 15.0, 1), ConvergenceStatus::Diverging);
    }

    #[test]
    fn test_moving_average() {
        let detector = ConvergenceDetector::with_defaults();

        detector.check(1.0, 1.1, 0);
        detector.check(1.1, 1.15, 1);
        detector.check(1.15, 1.17, 2);

        let avg = detector.moving_average_delta();
        assert!(avg > 0.0);
    }

    #[test]
    fn test_reset() {
        let detector = ConvergenceDetector::with_defaults();

        detector.check(1.0, 1.1, 0);
        assert!(detector.moving_average_delta() > 0.0);

        detector.reset();
        assert_eq!(detector.moving_average_delta(), 0.0);
        assert_eq!(detector.consecutive_stable(), 0);
    }

    #[test]
    fn test_analyzer() {
        let mut analyzer = ConvergenceAnalyzer::new();

        analyzer.record(1.0);
        analyzer.record(1.1);
        analyzer.record(1.15);
        analyzer.record(1.155);
        analyzer.record(1.156);

        let stats = analyzer.analyze();
        assert_eq!(stats.iterations, 5);
        assert!(stats.final_delta < 0.01);
    }
}
