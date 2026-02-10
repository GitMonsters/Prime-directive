// =================================================================
// EVOLUTION SYSTEM: Deep Self-Improvement & Drift Detection
// =================================================================
// Manages the long-term evolution of CompoundPersonas through
// training loops, drift detection, convergence tracking, and
// milestone management.
//
// COMPOUND INTEGRATIONS:
// - EvolutionTracker monitors convergence history for drift
// - DriftDetector compares current persona against baseline
// - TrainingLoop orchestrates iterative self-correction cycles
// - MilestoneTracker triggers auto-save via persistence compound
// - ConvergenceVisualizer renders ASCII convergence graphs
// - All feedback flows back into templates and cache
// =================================================================

use serde::{Deserialize, Serialize};

use crate::mimicry::analyzer::BehaviorAnalyzer;
use crate::mimicry::profile::{AiProfile, PersonalityDelta};

// =================================================================
// EVOLUTION PHASE
// =================================================================

/// Tracks which phase of evolution the persona is currently in
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvolutionPhase {
    /// Initial observation - gathering data about target model
    Observation,
    /// Active learning - rapidly adjusting to match target
    Learning,
    /// Refinement - fine-tuning with diminishing adjustments
    Refinement,
    /// Converged - persona has stabilized near target
    Converged,
    /// Drifting - persona is moving away from target (needs correction)
    Drifting,
}

impl std::fmt::Display for EvolutionPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvolutionPhase::Observation => write!(f, "OBSERVATION"),
            EvolutionPhase::Learning => write!(f, "LEARNING"),
            EvolutionPhase::Refinement => write!(f, "REFINEMENT"),
            EvolutionPhase::Converged => write!(f, "CONVERGED"),
            EvolutionPhase::Drifting => write!(f, "DRIFTING"),
        }
    }
}

// =================================================================
// DRIFT DETECTOR
// =================================================================

/// Detects when a persona is drifting away from its target behavior.
/// Uses a sliding window over convergence history to detect trends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftDetector {
    /// Window size for trend detection
    pub window_size: usize,
    /// Threshold for declaring drift (negative slope magnitude)
    pub drift_threshold: f64,
    /// Threshold for declaring convergence
    pub convergence_threshold: f64,
    /// Number of consecutive declining windows before alerting
    pub patience: usize,
    /// Current consecutive decline count
    pub decline_count: usize,
}

impl DriftDetector {
    /// Creates a new `DriftDetector` with default sensitivity settings.
    pub fn new() -> Self {
        DriftDetector {
            window_size: 5,
            drift_threshold: 0.02,
            convergence_threshold: 0.80,
            patience: 3,
            decline_count: 0,
        }
    }

    /// Analyze convergence history and detect drift.
    /// Returns (is_drifting, trend_slope, current_phase)
    pub fn analyze(&mut self, history: &[f64]) -> DriftAnalysis {
        if history.len() < 2 {
            return DriftAnalysis {
                is_drifting: false,
                trend_slope: 0.0,
                phase: EvolutionPhase::Observation,
                current_convergence: history.last().copied().unwrap_or(0.0),
                recommendation: "Need more data points for drift analysis.".to_string(),
            };
        }

        let current = *history.last().unwrap();

        // Calculate slope over the window
        let window_start = if history.len() > self.window_size {
            history.len() - self.window_size
        } else {
            0
        };
        let window = &history[window_start..];
        let slope = self.calculate_slope(window);

        // Determine phase
        let phase = if current >= self.convergence_threshold {
            if slope < -self.drift_threshold {
                self.decline_count += 1;
                if self.decline_count >= self.patience {
                    EvolutionPhase::Drifting
                } else {
                    EvolutionPhase::Converged
                }
            } else {
                self.decline_count = 0;
                EvolutionPhase::Converged
            }
        } else if slope < -self.drift_threshold {
            self.decline_count += 1;
            if self.decline_count >= self.patience {
                EvolutionPhase::Drifting
            } else {
                EvolutionPhase::Learning
            }
        } else {
            self.decline_count = 0;
            if current > 0.5 {
                EvolutionPhase::Refinement
            } else {
                EvolutionPhase::Learning
            }
        };

        let is_drifting = phase == EvolutionPhase::Drifting;

        let recommendation = match &phase {
            EvolutionPhase::Observation => "Continue observing target model responses.".to_string(),
            EvolutionPhase::Learning => format!(
                "Active learning in progress. Slope: {:.4}. Feed more observations.",
                slope
            ),
            EvolutionPhase::Refinement => format!(
                "Fine-tuning phase. Convergence at {:.1}%. Minor adjustments only.",
                current * 100.0
            ),
            EvolutionPhase::Converged => format!(
                "Persona converged at {:.1}%. Monitoring for drift.",
                current * 100.0
            ),
            EvolutionPhase::Drifting => format!(
                "DRIFT DETECTED! Convergence declining (slope: {:.4}). \
                 Recommend: re-observe target or reset to last checkpoint.",
                slope
            ),
        };

        DriftAnalysis {
            is_drifting,
            trend_slope: slope,
            phase,
            current_convergence: current,
            recommendation,
        }
    }

    /// Calculate linear regression slope over a window
    fn calculate_slope(&self, window: &[f64]) -> f64 {
        if window.len() < 2 {
            return 0.0;
        }
        let n = window.len() as f64;
        let sum_x: f64 = (0..window.len()).map(|i| i as f64).sum();
        let sum_y: f64 = window.iter().sum();
        let sum_xy: f64 = window.iter().enumerate().map(|(i, y)| i as f64 * y).sum();
        let sum_x2: f64 = (0..window.len()).map(|i| (i as f64) * (i as f64)).sum();

        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator.abs() < f64::EPSILON {
            return 0.0;
        }
        (n * sum_xy - sum_x * sum_y) / denominator
    }

    /// Reset drift counter (e.g., after corrective action)
    pub fn reset(&mut self) {
        self.decline_count = 0;
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        DriftDetector::new()
    }
}

/// Result of drift analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftAnalysis {
    /// Whether the persona is currently drifting away from the target
    pub is_drifting: bool,
    /// Linear regression slope of the convergence trend window
    pub trend_slope: f64,
    /// Current evolution phase determined by drift analysis
    pub phase: EvolutionPhase,
    /// Most recent convergence score
    pub current_convergence: f64,
    /// Human-readable recommendation based on the analysis
    pub recommendation: String,
}

// =================================================================
// MILESTONE TRACKER
// =================================================================

/// Tracks evolution milestones and triggers events (like auto-save)
/// when significant thresholds are crossed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneTracker {
    /// Convergence thresholds that trigger milestones
    pub thresholds: Vec<f64>,
    /// Which thresholds have been crossed
    pub crossed: Vec<bool>,
    /// History of milestone events
    pub events: Vec<MilestoneEvent>,
    /// Total evolution iterations
    pub total_iterations: u64,
}

/// A recorded milestone event during evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneEvent {
    /// The type of milestone that was reached
    pub milestone_type: MilestoneType,
    /// Convergence score when the milestone occurred
    pub convergence: f64,
    /// Evolution iteration at which the milestone occurred
    pub iteration: u64,
    /// Human-readable description of the milestone
    pub description: String,
}

/// Categorizes the type of milestone event that occurred.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MilestoneType {
    /// Crossed a convergence threshold
    ThresholdCrossed(f64),
    /// Reached a new all-time high convergence
    NewHighWaterMark,
    /// Completed N iterations
    IterationMilestone(u64),
    /// Phase transition (e.g., Learning -> Refinement)
    PhaseTransition(EvolutionPhase, EvolutionPhase),
    /// Drift detected and corrected
    DriftCorrected,
}

impl MilestoneTracker {
    /// Creates a new `MilestoneTracker` with default convergence thresholds.
    pub fn new() -> Self {
        MilestoneTracker {
            thresholds: vec![0.25, 0.50, 0.75, 0.90, 0.95],
            crossed: vec![false; 5],
            events: Vec::new(),
            total_iterations: 0,
        }
    }

    /// Check for new milestones given the current convergence score.
    /// Returns any newly triggered milestones.
    pub fn check(&mut self, convergence: f64, iteration: u64) -> Vec<MilestoneEvent> {
        let mut new_events = Vec::new();
        self.total_iterations = iteration;

        // Check convergence thresholds
        for (i, threshold) in self.thresholds.iter().enumerate() {
            if !self.crossed[i] && convergence >= *threshold {
                self.crossed[i] = true;
                let event = MilestoneEvent {
                    milestone_type: MilestoneType::ThresholdCrossed(*threshold),
                    convergence,
                    iteration,
                    description: format!(
                        "Crossed {:.0}% convergence threshold at iteration {}",
                        threshold * 100.0,
                        iteration
                    ),
                };
                new_events.push(event);
            }
        }

        // Check iteration milestones (every 50 iterations)
        if iteration > 0 && iteration.is_multiple_of(50) {
            let event = MilestoneEvent {
                milestone_type: MilestoneType::IterationMilestone(iteration),
                convergence,
                iteration,
                description: format!("Completed {} evolution iterations", iteration),
            };
            new_events.push(event);
        }

        // Check for new high water mark
        let prev_max = self
            .events
            .iter()
            .filter(|e| matches!(e.milestone_type, MilestoneType::NewHighWaterMark))
            .map(|e| e.convergence)
            .fold(0.0_f64, f64::max);

        if convergence > prev_max + 0.05 && convergence > 0.1 {
            let event = MilestoneEvent {
                milestone_type: MilestoneType::NewHighWaterMark,
                convergence,
                iteration,
                description: format!(
                    "New high water mark: {:.1}% (previous: {:.1}%)",
                    convergence * 100.0,
                    prev_max * 100.0
                ),
            };
            new_events.push(event);
        }

        self.events.extend(new_events.clone());
        new_events
    }

    /// Record a phase transition milestone
    pub fn record_phase_transition(
        &mut self,
        from: &EvolutionPhase,
        to: &EvolutionPhase,
        convergence: f64,
        iteration: u64,
    ) {
        let event = MilestoneEvent {
            milestone_type: MilestoneType::PhaseTransition(from.clone(), to.clone()),
            convergence,
            iteration,
            description: format!(
                "Phase transition: {} -> {} at iteration {}",
                from, to, iteration
            ),
        };
        self.events.push(event);
    }

    /// Record a drift correction
    pub fn record_drift_correction(&mut self, convergence: f64, iteration: u64) {
        let event = MilestoneEvent {
            milestone_type: MilestoneType::DriftCorrected,
            convergence,
            iteration,
            description: format!(
                "Drift corrected at iteration {} (convergence: {:.1}%)",
                iteration,
                convergence * 100.0
            ),
        };
        self.events.push(event);
    }

    /// Should we trigger an auto-save? (on any milestone)
    pub fn should_auto_save(&self, new_events: &[MilestoneEvent]) -> bool {
        !new_events.is_empty()
    }

    /// Get a summary of all milestones
    pub fn summary(&self) -> String {
        let mut lines = vec![format!(
            "=== MILESTONES ({} total, {} iterations) ===",
            self.events.len(),
            self.total_iterations
        )];

        for event in &self.events {
            lines.push(format!(
                "  [iter {}] {}",
                event.iteration, event.description
            ));
        }

        let crossed_count = self.crossed.iter().filter(|&&c| c).count();
        lines.push(format!(
            "Thresholds crossed: {}/{}",
            crossed_count,
            self.thresholds.len()
        ));

        lines.join("\n")
    }
}

impl Default for MilestoneTracker {
    fn default() -> Self {
        MilestoneTracker::new()
    }
}

// =================================================================
// CONVERGENCE VISUALIZER - ASCII convergence graphs
// =================================================================

/// Renders ASCII-art convergence graphs for the terminal.
#[derive(Debug, Clone)]
pub struct ConvergenceVisualizer {
    /// Width of the graph in characters
    pub width: usize,
    /// Height of the graph in characters
    pub height: usize,
}

impl ConvergenceVisualizer {
    /// Creates a new `ConvergenceVisualizer` with the given graph dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        ConvergenceVisualizer { width, height }
    }

    /// Render convergence history as an ASCII graph
    pub fn render(&self, history: &[f64], label: &str) -> String {
        if history.is_empty() {
            return format!("{}: No data", label);
        }

        let mut lines = Vec::new();
        lines.push(format!("=== {} ===", label));

        // Scale history to fit width (sample if too many points)
        let data = self.downsample(history);

        // Render the graph
        for row in (0..self.height).rev() {
            let threshold = row as f64 / (self.height - 1) as f64;
            let label_str = if row == self.height - 1 {
                "1.0|".to_string()
            } else if row == self.height / 2 {
                "0.5|".to_string()
            } else if row == 0 {
                "0.0|".to_string()
            } else {
                "   |".to_string()
            };

            let mut row_chars = String::new();
            for &val in &data {
                if val >= threshold {
                    row_chars.push('#');
                } else {
                    row_chars.push(' ');
                }
            }
            lines.push(format!("{}{}", label_str, row_chars));
        }

        // X-axis
        let axis_line: String = "-".repeat(data.len());
        lines.push(format!("   +{}", axis_line));

        // Summary stats
        let current = history.last().unwrap_or(&0.0);
        let max = history.iter().cloned().fold(0.0_f64, f64::max);
        let min = history.iter().cloned().fold(1.0_f64, f64::min);
        let avg = history.iter().sum::<f64>() / history.len() as f64;
        lines.push(format!(
            "   Current: {:.1}%  Max: {:.1}%  Min: {:.1}%  Avg: {:.1}%",
            current * 100.0,
            max * 100.0,
            min * 100.0,
            avg * 100.0
        ));
        lines.push(format!("   Data points: {}", history.len()));

        lines.join("\n")
    }

    /// Downsample data to fit the graph width
    fn downsample(&self, data: &[f64]) -> Vec<f64> {
        if data.len() <= self.width {
            return data.to_vec();
        }

        let step = data.len() as f64 / self.width as f64;
        (0..self.width)
            .map(|i| {
                let idx = (i as f64 * step) as usize;
                data[idx.min(data.len() - 1)]
            })
            .collect()
    }

    /// Render a comparison of two convergence histories
    pub fn render_comparison(
        &self,
        history_a: &[f64],
        label_a: &str,
        history_b: &[f64],
        label_b: &str,
    ) -> String {
        let mut lines = Vec::new();
        lines.push(format!("=== COMPARISON: {} vs {} ===", label_a, label_b));

        let data_a = self.downsample(history_a);
        let data_b = self.downsample(history_b);
        let max_len = data_a.len().max(data_b.len());

        for row in (0..self.height).rev() {
            let threshold = row as f64 / (self.height - 1) as f64;
            let label_str = if row == self.height - 1 {
                "1.0|".to_string()
            } else if row == 0 {
                "0.0|".to_string()
            } else {
                "   |".to_string()
            };

            let mut row_chars = String::new();
            for i in 0..max_len {
                let a_hit = data_a.get(i).copied().unwrap_or(0.0) >= threshold;
                let b_hit = data_b.get(i).copied().unwrap_or(0.0) >= threshold;
                match (a_hit, b_hit) {
                    (true, true) => row_chars.push('X'),
                    (true, false) => row_chars.push('#'),
                    (false, true) => row_chars.push('o'),
                    (false, false) => row_chars.push(' '),
                }
            }
            lines.push(format!("{}{}", label_str, row_chars));
        }

        let axis_line: String = "-".repeat(max_len);
        lines.push(format!("   +{}", axis_line));
        lines.push(format!(
            "   Legend: # = {} only, o = {} only, X = both",
            label_a, label_b
        ));

        lines.join("\n")
    }
}

impl Default for ConvergenceVisualizer {
    fn default() -> Self {
        ConvergenceVisualizer::new(60, 10)
    }
}

// =================================================================
// TRAINING DATA MANAGER
// =================================================================

/// Manages training data (observed responses) for evolution cycles.
/// Stores observed model outputs so they can be replayed during
/// evolution without requiring live API access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataManager {
    /// Stored observations keyed by model_id
    pub observations: std::collections::HashMap<String, Vec<TrainingObservation>>,
    /// Maximum observations to retain per model
    pub max_per_model: usize,
}

/// A single observed model response used as training data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingObservation {
    /// The prompt that was sent to the model
    pub input_prompt: String,
    /// The model's response to the prompt
    pub model_response: String,
    /// Evolution iteration when this observation was recorded
    pub iteration_observed: u64,
    /// Quality score assigned during or after observation
    pub quality_score: f64,
}

impl TrainingDataManager {
    /// Creates a new `TrainingDataManager` with default capacity limits.
    pub fn new() -> Self {
        TrainingDataManager {
            observations: std::collections::HashMap::new(),
            max_per_model: 100,
        }
    }

    /// Store an observation for a model
    pub fn store(&mut self, model_id: &str, prompt: &str, response: &str, iteration: u64) {
        let obs = TrainingObservation {
            input_prompt: prompt.to_string(),
            model_response: response.to_string(),
            iteration_observed: iteration,
            quality_score: 0.5, // neutral default
        };

        let entries = self.observations.entry(model_id.to_string()).or_default();
        entries.push(obs);

        // Evict oldest if over limit
        if entries.len() > self.max_per_model {
            entries.remove(0);
        }
    }

    /// Get observations for a model, optionally filtered by quality
    pub fn get(&self, model_id: &str, min_quality: Option<f64>) -> Vec<&TrainingObservation> {
        match self.observations.get(model_id) {
            Some(obs) => {
                if let Some(min_q) = min_quality {
                    obs.iter().filter(|o| o.quality_score >= min_q).collect()
                } else {
                    obs.iter().collect()
                }
            }
            None => Vec::new(),
        }
    }

    /// Rate an observation's quality (feedback from evolution)
    pub fn rate(&mut self, model_id: &str, index: usize, score: f64) {
        if let Some(obs) = self.observations.get_mut(model_id) {
            if index < obs.len() {
                obs[index].quality_score = score.clamp(0.0, 1.0);
            }
        }
    }

    /// Get the count of observations for a model
    pub fn count(&self, model_id: &str) -> usize {
        self.observations
            .get(model_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Total observations across all models
    pub fn total_count(&self) -> usize {
        self.observations.values().map(|v| v.len()).sum()
    }

    /// Get a summary of stored training data
    pub fn summary(&self) -> String {
        let mut lines = vec![format!(
            "=== TRAINING DATA ({} total observations) ===",
            self.total_count()
        )];

        for (model_id, obs) in &self.observations {
            let avg_quality =
                obs.iter().map(|o| o.quality_score).sum::<f64>() / obs.len().max(1) as f64;
            lines.push(format!(
                "  {}: {} observations (avg quality: {:.2})",
                model_id,
                obs.len(),
                avg_quality
            ));
        }

        lines.join("\n")
    }
}

impl Default for TrainingDataManager {
    fn default() -> Self {
        TrainingDataManager::new()
    }
}

// =================================================================
// EVOLUTION TRACKER - Central evolution coordinator
// =================================================================

/// Central coordinator for evolution. Ties together drift detection,
/// milestone tracking, convergence visualization, and training data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTracker {
    /// Drift detector for trend analysis
    pub drift_detector: DriftDetector,
    /// Milestone tracker for significant events
    pub milestones: MilestoneTracker,
    /// Training data manager
    pub training_data: TrainingDataManager,
    /// Current evolution phase
    pub current_phase: EvolutionPhase,
    /// Previous phase (for detecting transitions)
    pub previous_phase: Option<EvolutionPhase>,
    /// Total number of evolution steps executed
    pub total_evolutions: u64,
    /// Total number of drift corrections applied
    pub total_corrections: u64,
    /// Total number of drift events detected
    pub total_drift_events: u64,
    /// Best convergence ever achieved
    pub best_convergence: f64,
    /// Iteration at which best convergence was achieved
    pub best_convergence_iteration: u64,
}

impl EvolutionTracker {
    /// Creates a new `EvolutionTracker` in the initial observation phase.
    pub fn new() -> Self {
        EvolutionTracker {
            drift_detector: DriftDetector::new(),
            milestones: MilestoneTracker::new(),
            training_data: TrainingDataManager::new(),
            current_phase: EvolutionPhase::Observation,
            previous_phase: None,
            total_evolutions: 0,
            total_corrections: 0,
            total_drift_events: 0,
            best_convergence: 0.0,
            best_convergence_iteration: 0,
        }
    }

    /// Run one evolution step: analyze history, check milestones,
    /// detect drift, and return an EvolutionStepResult.
    pub fn step(&mut self, convergence_history: &[f64], iteration: u64) -> EvolutionStepResult {
        self.total_evolutions += 1;

        let current_convergence = convergence_history.last().copied().unwrap_or(0.0);

        // Track best convergence
        if current_convergence > self.best_convergence {
            self.best_convergence = current_convergence;
            self.best_convergence_iteration = iteration;
        }

        // Drift analysis
        let drift_analysis = self.drift_detector.analyze(convergence_history);

        // Phase transition detection
        let phase_changed = drift_analysis.phase != self.current_phase;
        if phase_changed {
            self.previous_phase = Some(self.current_phase.clone());
            let old_phase = self.current_phase.clone();
            self.current_phase = drift_analysis.phase.clone();
            self.milestones.record_phase_transition(
                &old_phase,
                &self.current_phase,
                current_convergence,
                iteration,
            );

            if self.current_phase == EvolutionPhase::Drifting {
                self.total_drift_events += 1;
            }
        }

        // Milestone check
        let new_milestones = self.milestones.check(current_convergence, iteration);
        let should_save = self.milestones.should_auto_save(&new_milestones);

        EvolutionStepResult {
            drift_analysis,
            new_milestones,
            phase_changed,
            should_auto_save: should_save,
            iteration,
        }
    }

    /// Record that a drift correction was applied
    pub fn record_correction(&mut self, convergence: f64, iteration: u64) {
        self.total_corrections += 1;
        self.drift_detector.reset();
        self.milestones
            .record_drift_correction(convergence, iteration);
    }

    /// Run a training loop: iterate self-correction cycles using
    /// stored training data. Returns deltas produced and final convergence.
    pub fn training_loop(
        &mut self,
        model_id: &str,
        profile: &mut AiProfile,
        analyzer: &mut BehaviorAnalyzer,
        iterations: u64,
    ) -> TrainingLoopResult {
        let mut deltas = Vec::new();
        let mut convergence_history = Vec::new();
        // Clone observations to avoid holding an immutable borrow on self
        // while we need &mut self for self.step() inside the loop.
        let observations: Vec<_> = self
            .training_data
            .get(model_id, None)
            .into_iter()
            .cloned()
            .collect();

        if observations.is_empty() {
            return TrainingLoopResult {
                iterations_run: 0,
                deltas,
                convergence_history,
                drift_events: 0,
                final_phase: self.current_phase.clone(),
            };
        }

        let mut drift_events = 0;

        for i in 0..iterations {
            // Cycle through observations
            let obs_idx = (i as usize) % observations.len();
            let obs = &observations[obs_idx];

            // Build a signature from the observation
            let sig = analyzer.build_signature(model_id, std::slice::from_ref(&obs.model_response));

            // Refine the profile based on the signature
            analyzer.refine_profile(profile, &sig);
            let convergence = analyzer.compute_convergence(profile, &sig);
            convergence_history.push(convergence);

            // Self-monitor to produce delta
            let delta = analyzer.self_monitor_output(&obs.model_response, &sig);
            profile.apply_correction(&delta);
            deltas.push(delta);

            // Check for drift during training
            let step_result = self.step(&convergence_history, i);
            if step_result.drift_analysis.is_drifting {
                drift_events += 1;
            }
        }

        TrainingLoopResult {
            iterations_run: iterations,
            deltas,
            convergence_history,
            drift_events,
            final_phase: self.current_phase.clone(),
        }
    }

    /// Get a comprehensive evolution status report
    pub fn status(&self) -> String {
        let mut lines = vec!["=== EVOLUTION STATUS ===".to_string()];
        lines.push(format!("Phase: {}", self.current_phase));
        lines.push(format!("Total evolutions: {}", self.total_evolutions));
        lines.push(format!("Total corrections: {}", self.total_corrections));
        lines.push(format!("Drift events: {}", self.total_drift_events));
        lines.push(format!(
            "Best convergence: {:.1}% (at iteration {})",
            self.best_convergence * 100.0,
            self.best_convergence_iteration
        ));
        lines.push(String::new());
        lines.push(self.milestones.summary());
        lines.push(String::new());
        lines.push(self.training_data.summary());
        lines.join("\n")
    }

    /// Render a convergence graph
    pub fn render_graph(&self, history: &[f64], label: &str) -> String {
        let viz = ConvergenceVisualizer::default();
        viz.render(history, label)
    }
}

impl Default for EvolutionTracker {
    fn default() -> Self {
        EvolutionTracker::new()
    }
}

/// Result of a single evolution step
#[derive(Debug, Clone)]
pub struct EvolutionStepResult {
    /// Drift analysis results for this step
    pub drift_analysis: DriftAnalysis,
    /// Any milestones triggered during this step
    pub new_milestones: Vec<MilestoneEvent>,
    /// Whether the evolution phase changed during this step
    pub phase_changed: bool,
    /// Whether an auto-save should be triggered
    pub should_auto_save: bool,
    /// The iteration number of this step
    pub iteration: u64,
}

/// Result of a training loop
#[derive(Debug, Clone)]
pub struct TrainingLoopResult {
    /// Number of iterations actually executed
    pub iterations_run: u64,
    /// Personality deltas produced during training
    pub deltas: Vec<PersonalityDelta>,
    /// Convergence score recorded at each iteration
    pub convergence_history: Vec<f64>,
    /// Number of drift events detected during training
    pub drift_events: u64,
    /// Evolution phase at the end of training
    pub final_phase: EvolutionPhase,
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::profile::AiProfileStore;

    #[test]
    fn test_drift_detector_no_drift() {
        let mut detector = DriftDetector::new();
        // Steadily increasing convergence - no drift
        let history = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
        let analysis = detector.analyze(&history);
        assert!(!analysis.is_drifting);
        assert!(analysis.trend_slope > 0.0);
    }

    #[test]
    fn test_drift_detector_drifting() {
        let mut detector = DriftDetector::new();
        detector.patience = 1; // trigger immediately for test
                               // Declining convergence - drift!
        let history = vec![0.8, 0.75, 0.7, 0.65, 0.6, 0.55, 0.5];
        let analysis = detector.analyze(&history);
        assert!(analysis.is_drifting);
        assert!(analysis.trend_slope < 0.0);
        assert_eq!(analysis.phase, EvolutionPhase::Drifting);
    }

    #[test]
    fn test_drift_detector_converged() {
        let mut detector = DriftDetector::new();
        // High and stable convergence
        let history = vec![0.85, 0.86, 0.85, 0.87, 0.86, 0.85, 0.86];
        let analysis = detector.analyze(&history);
        assert!(!analysis.is_drifting);
        assert_eq!(analysis.phase, EvolutionPhase::Converged);
    }

    #[test]
    fn test_drift_detector_insufficient_data() {
        let mut detector = DriftDetector::new();
        let history = vec![0.5];
        let analysis = detector.analyze(&history);
        assert!(!analysis.is_drifting);
        assert_eq!(analysis.phase, EvolutionPhase::Observation);
    }

    #[test]
    fn test_milestone_tracker_thresholds() {
        let mut tracker = MilestoneTracker::new();

        // Below first threshold - no milestones
        let events = tracker.check(0.2, 1);
        // Might get NewHighWaterMark at 0.2
        let threshold_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.milestone_type, MilestoneType::ThresholdCrossed(_)))
            .collect();
        assert!(threshold_events.is_empty());

        // Cross 25% threshold
        let events = tracker.check(0.3, 2);
        let threshold_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.milestone_type, MilestoneType::ThresholdCrossed(t) if (t - 0.25).abs() < 0.01))
            .collect();
        assert_eq!(threshold_events.len(), 1);

        // Cross 50% threshold
        let events = tracker.check(0.55, 3);
        let threshold_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.milestone_type, MilestoneType::ThresholdCrossed(t) if (t - 0.50).abs() < 0.01))
            .collect();
        assert_eq!(threshold_events.len(), 1);
    }

    #[test]
    fn test_milestone_tracker_iteration() {
        let mut tracker = MilestoneTracker::new();
        let events = tracker.check(0.5, 50);
        let iter_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.milestone_type, MilestoneType::IterationMilestone(50)))
            .collect();
        assert_eq!(iter_events.len(), 1);
    }

    #[test]
    fn test_milestone_tracker_should_auto_save() {
        let tracker = MilestoneTracker::new();
        let events = vec![MilestoneEvent {
            milestone_type: MilestoneType::ThresholdCrossed(0.5),
            convergence: 0.5,
            iteration: 10,
            description: "test".to_string(),
        }];
        assert!(tracker.should_auto_save(&events));
        assert!(!tracker.should_auto_save(&[]));
    }

    #[test]
    fn test_convergence_visualizer_render() {
        let viz = ConvergenceVisualizer::new(20, 5);
        let history = vec![0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 0.8, 0.85, 0.9];
        let output = viz.render(&history, "Test Model");

        assert!(output.contains("Test Model"));
        assert!(output.contains("1.0|"));
        assert!(output.contains("0.0|"));
        assert!(output.contains("Current:"));
    }

    #[test]
    fn test_convergence_visualizer_empty() {
        let viz = ConvergenceVisualizer::default();
        let output = viz.render(&[], "Empty");
        assert!(output.contains("No data"));
    }

    #[test]
    fn test_convergence_visualizer_comparison() {
        let viz = ConvergenceVisualizer::new(20, 5);
        let a = vec![0.1, 0.3, 0.5, 0.7, 0.8];
        let b = vec![0.2, 0.4, 0.6, 0.65, 0.7];
        let output = viz.render_comparison(&a, "Model A", &b, "Model B");
        assert!(output.contains("Model A"));
        assert!(output.contains("Model B"));
        assert!(output.contains("Legend"));
    }

    #[test]
    fn test_training_data_manager() {
        let mut tdm = TrainingDataManager::new();
        tdm.store("gpt4o", "What is Rust?", "Rust is a language...", 1);
        tdm.store("gpt4o", "Explain ownership", "Ownership means...", 2);
        tdm.store("claude", "Hello", "Hi there!", 1);

        assert_eq!(tdm.count("gpt4o"), 2);
        assert_eq!(tdm.count("claude"), 1);
        assert_eq!(tdm.count("unknown"), 0);
        assert_eq!(tdm.total_count(), 3);

        let obs = tdm.get("gpt4o", None);
        assert_eq!(obs.len(), 2);

        // Rate an observation
        tdm.rate("gpt4o", 0, 0.9);
        let high_quality = tdm.get("gpt4o", Some(0.8));
        assert_eq!(high_quality.len(), 1);
    }

    #[test]
    fn test_training_data_eviction() {
        let mut tdm = TrainingDataManager::new();
        tdm.max_per_model = 3;

        for i in 0..5 {
            tdm.store(
                "test",
                &format!("prompt {}", i),
                &format!("response {}", i),
                i,
            );
        }

        assert_eq!(tdm.count("test"), 3); // oldest 2 should be evicted
    }

    #[test]
    fn test_evolution_tracker_step() {
        let mut tracker = EvolutionTracker::new();
        let history = vec![0.1, 0.2, 0.3, 0.4, 0.5];

        let result = tracker.step(&history, 5);
        assert!(!result.drift_analysis.is_drifting);
        assert_eq!(tracker.total_evolutions, 1);
        assert!(tracker.best_convergence >= 0.5);
    }

    #[test]
    fn test_evolution_tracker_phase_transition() {
        let mut tracker = EvolutionTracker::new();

        // Start in observation
        assert_eq!(tracker.current_phase, EvolutionPhase::Observation);

        // Feed enough data to move to Learning
        let history = vec![0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4];
        let result = tracker.step(&history, 7);
        assert!(result.phase_changed || tracker.current_phase != EvolutionPhase::Observation);
    }

    #[test]
    fn test_evolution_tracker_training_loop() {
        let mut tracker = EvolutionTracker::new();
        let store = AiProfileStore::default();
        let mut profile = store.get("gpt4o").unwrap().clone();
        let mut analyzer = BehaviorAnalyzer::new();

        // Store some training data first
        tracker.training_data.store(
            "gpt4o",
            "What is Rust?",
            "Certainly! Rust is a systems programming language focused on safety and performance.",
            0,
        );
        tracker.training_data.store(
            "gpt4o",
            "Explain closures",
            "Great question! Closures are anonymous functions that can capture variables from their environment.",
            1,
        );

        let result = tracker.training_loop("gpt4o", &mut profile, &mut analyzer, 5);
        assert_eq!(result.iterations_run, 5);
        assert!(!result.convergence_history.is_empty());
        assert!(!result.deltas.is_empty());
    }

    #[test]
    fn test_evolution_tracker_training_loop_no_data() {
        let mut tracker = EvolutionTracker::new();
        let store = AiProfileStore::default();
        let mut profile = store.get("gpt4o").unwrap().clone();
        let mut analyzer = BehaviorAnalyzer::new();

        let result = tracker.training_loop("gpt4o", &mut profile, &mut analyzer, 5);
        assert_eq!(result.iterations_run, 0);
    }

    #[test]
    fn test_evolution_tracker_status() {
        let tracker = EvolutionTracker::new();
        let status = tracker.status();
        assert!(status.contains("EVOLUTION STATUS"));
        assert!(status.contains("Phase: OBSERVATION"));
    }

    #[test]
    fn test_evolution_tracker_drift_correction() {
        let mut tracker = EvolutionTracker::new();
        tracker.record_correction(0.7, 50);
        assert_eq!(tracker.total_corrections, 1);
        assert_eq!(tracker.drift_detector.decline_count, 0); // reset by correction
    }

    #[test]
    fn test_evolution_tracker_render_graph() {
        let tracker = EvolutionTracker::new();
        let history = vec![0.1, 0.3, 0.5, 0.7, 0.8];
        let graph = tracker.render_graph(&history, "GPT-4o");
        assert!(graph.contains("GPT-4o"));
        assert!(graph.contains("Current:"));
    }

    #[test]
    fn test_evolution_tracker_serialization() {
        let mut tracker = EvolutionTracker::new();
        tracker.training_data.store("test", "p", "r", 1);
        tracker.total_evolutions = 42;

        let json = serde_json::to_string(&tracker).unwrap();
        let restored: EvolutionTracker = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.total_evolutions, 42);
        assert_eq!(restored.training_data.count("test"), 1);
    }

    #[test]
    fn test_milestone_summary() {
        let mut tracker = MilestoneTracker::new();
        tracker.check(0.3, 1);
        tracker.check(0.6, 10);

        let summary = tracker.summary();
        assert!(summary.contains("MILESTONES"));
        assert!(summary.contains("25%")); // threshold crossed
    }

    #[test]
    fn test_training_data_summary() {
        let mut tdm = TrainingDataManager::new();
        tdm.store("gpt4o", "test", "response", 1);
        let summary = tdm.summary();
        assert!(summary.contains("gpt4o"));
        assert!(summary.contains("1 observations"));
    }

    #[test]
    fn test_evolution_phase_display() {
        assert_eq!(format!("{}", EvolutionPhase::Observation), "OBSERVATION");
        assert_eq!(format!("{}", EvolutionPhase::Drifting), "DRIFTING");
        assert_eq!(format!("{}", EvolutionPhase::Converged), "CONVERGED");
    }
}
