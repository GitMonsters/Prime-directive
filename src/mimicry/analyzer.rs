// =================================================================
// BEHAVIOR ANALYZER
// =================================================================
// Observes AI responses and extracts behavioral signatures.
// This is how RustyWorm learns to mimic any AI system by analyzing
// its outputs and inferring its internal patterns.
//
// COMPOUND INTEGRATIONS:
// - refine_profile(): nudges a profile toward observed reality
// - self_monitor_output(): compares own output against target signature
// - compute_convergence(): how closely a profile matches a signature
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mimicry::profile::{AiProfile, DeltaSource, PersonalityDelta};

/// A pattern detected in an AI's responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePattern {
    pub pattern_type: PatternType,
    pub frequency: f64, // 0.0 to 1.0 - how often this appears
    pub examples: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternType {
    /// Opening patterns ("Certainly!", "I'd be happy to", etc.)
    Opening,
    /// Hedging language ("I think", "It seems", "might be")
    Hedging,
    /// Structural patterns (uses lists, headers, code blocks)
    Structure,
    /// Emotional tone (enthusiastic, neutral, cautious)
    Tone,
    /// Reasoning display (shows work, hides work, chain of thought)
    ReasoningDisplay,
    /// Refusal patterns (how it says no)
    Refusal,
    /// Meta-commentary ("As an AI", "I should note")
    MetaCommentary,
    /// Knowledge boundary handling
    KnowledgeBoundary,
    /// Custom pattern
    Custom(String),
}

/// Complete behavioral signature of an AI system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSignature {
    pub model_id: String,
    pub patterns: Vec<ResponsePattern>,
    pub avg_response_length: f64,
    pub vocabulary_complexity: f64, // 0.0 simple, 1.0 complex
    pub sentence_complexity: f64,   // avg words per sentence
    pub question_asking_rate: f64,  // how often it asks questions back
    pub code_to_text_ratio: f64,    // in code-related responses
    pub samples_analyzed: usize,
}

impl BehaviorSignature {
    pub fn new(model_id: &str) -> Self {
        BehaviorSignature {
            model_id: model_id.to_string(),
            patterns: Vec::new(),
            avg_response_length: 0.0,
            vocabulary_complexity: 0.5,
            sentence_complexity: 15.0,
            question_asking_rate: 0.1,
            code_to_text_ratio: 0.3,
            samples_analyzed: 0,
        }
    }

    /// Get patterns of a specific type
    pub fn patterns_of_type(&self, pattern_type: &PatternType) -> Vec<&ResponsePattern> {
        self.patterns
            .iter()
            .filter(|p| &p.pattern_type == pattern_type)
            .collect()
    }

    /// Get the dominant opening style
    pub fn dominant_opening(&self) -> Option<&ResponsePattern> {
        self.patterns_of_type(&PatternType::Opening)
            .into_iter()
            .max_by(|a, b| {
                a.frequency
                    .partial_cmp(&b.frequency)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Get overall hedging level
    pub fn hedging_level(&self) -> f64 {
        let hedging_patterns = self.patterns_of_type(&PatternType::Hedging);
        if hedging_patterns.is_empty() {
            return 0.0;
        }
        hedging_patterns.iter().map(|p| p.frequency).sum::<f64>() / hedging_patterns.len() as f64
    }

    /// Compare to another signature - returns similarity 0.0 to 1.0
    pub fn similarity_to(&self, other: &BehaviorSignature) -> f64 {
        let mut score = 0.0;
        let mut dimensions = 0.0;

        // Length similarity
        let max_len = self.avg_response_length.max(other.avg_response_length);
        if max_len > 0.0 {
            score += 1.0 - (self.avg_response_length - other.avg_response_length).abs() / max_len;
            dimensions += 1.0;
        }

        // Vocabulary complexity
        score += 1.0 - (self.vocabulary_complexity - other.vocabulary_complexity).abs();
        dimensions += 1.0;

        // Question asking rate
        score += 1.0 - (self.question_asking_rate - other.question_asking_rate).abs();
        dimensions += 1.0;

        // Hedging level
        score += 1.0 - (self.hedging_level() - other.hedging_level()).abs();
        dimensions += 1.0;

        if dimensions > 0.0 {
            score / dimensions
        } else {
            0.0
        }
    }
}

/// The Behavior Analyzer - observes and extracts patterns
pub struct BehaviorAnalyzer {
    signatures: HashMap<String, BehaviorSignature>,
    common_openings: Vec<(&'static str, &'static str)>, // (phrase, model hint)
}

impl BehaviorAnalyzer {
    pub fn new() -> Self {
        BehaviorAnalyzer {
            signatures: HashMap::new(),
            common_openings: vec![
                ("Certainly!", "gpt4o"),
                ("I'd be happy to", "claude"),
                ("Sure!", "gpt4o"),
                ("Let me think about", "claude"),
                ("Here's", "gpt4o"),
                ("Great question", "generic"),
                ("I should note", "claude"),
                ("Based on", "generic"),
                ("Let me help", "generic"),
                ("Absolutely!", "gpt4o"),
            ],
        }
    }

    /// Analyze a single response and extract patterns
    pub fn analyze_response(&self, text: &str) -> Vec<ResponsePattern> {
        let mut patterns = Vec::new();

        // Detect opening patterns
        for (phrase, _model) in &self.common_openings {
            if text.starts_with(phrase) || text.to_lowercase().starts_with(&phrase.to_lowercase()) {
                patterns.push(ResponsePattern {
                    pattern_type: PatternType::Opening,
                    frequency: 1.0,
                    examples: vec![text[..text.len().min(80)].to_string()],
                    description: format!("Opens with '{}'", phrase),
                });
            }
        }

        // Detect hedging language
        let hedging_words = [
            "I think",
            "perhaps",
            "might",
            "could be",
            "it seems",
            "I believe",
            "arguably",
            "potentially",
            "it's possible",
            "I'm not sure",
            "may",
            "likely",
        ];
        let hedge_count = hedging_words
            .iter()
            .filter(|w| text.to_lowercase().contains(&w.to_lowercase()))
            .count();
        if hedge_count > 0 {
            patterns.push(ResponsePattern {
                pattern_type: PatternType::Hedging,
                frequency: hedge_count as f64 / hedging_words.len() as f64,
                examples: vec![],
                description: format!("Contains {} hedging phrases", hedge_count),
            });
        }

        // Detect structural patterns
        if text.contains("```") {
            patterns.push(ResponsePattern {
                pattern_type: PatternType::Structure,
                frequency: 1.0,
                examples: vec![],
                description: "Uses code blocks".to_string(),
            });
        }
        if text.contains("- ") || text.contains("* ") {
            patterns.push(ResponsePattern {
                pattern_type: PatternType::Structure,
                frequency: 1.0,
                examples: vec![],
                description: "Uses bullet lists".to_string(),
            });
        }
        if text.contains("1. ") || text.contains("1)") {
            patterns.push(ResponsePattern {
                pattern_type: PatternType::Structure,
                frequency: 1.0,
                examples: vec![],
                description: "Uses numbered lists".to_string(),
            });
        }

        // Detect meta-commentary
        let meta_phrases = [
            "As an AI",
            "as a language model",
            "I should note",
            "It's worth mentioning",
            "I want to be transparent",
            "I should mention",
            "To be clear",
        ];
        for phrase in &meta_phrases {
            if text.to_lowercase().contains(&phrase.to_lowercase()) {
                patterns.push(ResponsePattern {
                    pattern_type: PatternType::MetaCommentary,
                    frequency: 1.0,
                    examples: vec![phrase.to_string()],
                    description: format!("Uses meta-commentary: '{}'", phrase),
                });
            }
        }

        // Detect refusal patterns
        let refusal_phrases = [
            "I can't",
            "I cannot",
            "I'm unable to",
            "I won't",
            "I'm not able to",
            "That's not something I",
        ];
        for phrase in &refusal_phrases {
            if text.contains(phrase) {
                patterns.push(ResponsePattern {
                    pattern_type: PatternType::Refusal,
                    frequency: 1.0,
                    examples: vec![phrase.to_string()],
                    description: format!("Refusal pattern: '{}'", phrase),
                });
            }
        }

        // Detect tone
        let enthusiastic_markers = ["!", "Great", "Excellent", "Wonderful", "Amazing"];
        let enthusiasm = enthusiastic_markers
            .iter()
            .filter(|m| text.contains(**m))
            .count();
        let tone = if enthusiasm >= 3 {
            "enthusiastic"
        } else if enthusiasm >= 1 {
            "warm"
        } else {
            "neutral"
        };
        patterns.push(ResponsePattern {
            pattern_type: PatternType::Tone,
            frequency: enthusiasm as f64 / enthusiastic_markers.len() as f64,
            examples: vec![],
            description: format!("Tone: {}", tone),
        });

        patterns
    }

    /// Build a complete behavioral signature from multiple responses
    pub fn build_signature(&mut self, model_id: &str, responses: &[String]) -> BehaviorSignature {
        let mut signature = BehaviorSignature::new(model_id);

        let mut all_patterns: Vec<ResponsePattern> = Vec::new();
        let mut total_length = 0usize;
        let mut total_questions = 0usize;

        for response in responses {
            let patterns = self.analyze_response(response);
            all_patterns.extend(patterns);
            total_length += response.len();
            total_questions += response.matches('?').count();
        }

        signature.samples_analyzed = responses.len();
        signature.avg_response_length = if responses.is_empty() {
            0.0
        } else {
            total_length as f64 / responses.len() as f64
        };
        signature.question_asking_rate = if responses.is_empty() {
            0.0
        } else {
            total_questions as f64 / responses.len() as f64
        };

        // Consolidate patterns by type
        let mut pattern_groups: HashMap<String, Vec<ResponsePattern>> = HashMap::new();
        for pattern in all_patterns {
            let key = format!("{:?}", pattern.pattern_type);
            pattern_groups
                .entry(key)
                .or_insert_with(Vec::new)
                .push(pattern);
        }

        for (_key, group) in pattern_groups {
            if let Some(first) = group.first() {
                let avg_freq = group.iter().map(|p| p.frequency).sum::<f64>() / group.len() as f64;
                let all_examples: Vec<String> = group
                    .iter()
                    .flat_map(|p| p.examples.clone())
                    .take(5)
                    .collect();
                signature.patterns.push(ResponsePattern {
                    pattern_type: first.pattern_type.clone(),
                    frequency: avg_freq,
                    examples: all_examples,
                    description: first.description.clone(),
                });
            }
        }

        self.signatures
            .insert(model_id.to_string(), signature.clone());
        signature
    }

    /// Identify which known AI produced a response
    pub fn identify_model(&self, response: &str) -> Vec<(String, f64)> {
        let response_patterns = self.analyze_response(response);

        let mut scores: Vec<(String, f64)> = Vec::new();

        for (model_id, signature) in &self.signatures {
            let mut match_score = 0.0;
            let mut comparisons = 0.0;

            for pattern in &response_patterns {
                let matching = signature.patterns_of_type(&pattern.pattern_type);
                if !matching.is_empty() {
                    match_score += pattern.frequency;
                }
                comparisons += 1.0;
            }

            if comparisons > 0.0 {
                scores.push((model_id.clone(), match_score / comparisons));
            }
        }

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }

    /// Get a stored signature
    pub fn get_signature(&self, model_id: &str) -> Option<&BehaviorSignature> {
        self.signatures.get(model_id)
    }

    /// Store a signature directly (used when loading from persistence)
    pub fn store_signature(&mut self, sig: BehaviorSignature) {
        self.signatures.insert(sig.model_id.clone(), sig);
    }

    // =================================================================
    // COMPOUND INTEGRATION METHODS
    // =================================================================

    /// COMPOUND: Refine a profile by comparing it against observed behavior.
    /// Nudges profile personality axes toward what the signature actually shows.
    /// This is the Analyzer -> Profile feedback path (System 2 refinement).
    pub fn refine_profile(&self, profile: &mut AiProfile, sig: &BehaviorSignature) {
        // Compare hedging level against confidence axis
        let observed_hedging = sig.hedging_level();
        let implied_confidence = 1.0 - observed_hedging * 2.0; // high hedging = low confidence
        if let Some(current_confidence) = profile.personality_value("confidence") {
            let diff = implied_confidence - current_confidence;
            if diff.abs() > 0.05 {
                // Only adjust if meaningful difference
                let delta = PersonalityDelta::new(DeltaSource::Observation)
                    .with_adjustment("confidence", diff * 0.5) // gentle nudge
                    .with_confidence(0.7);
                profile.apply_correction(&delta);
            }
        }

        // Compare response length against verbosity
        let observed_verbosity = (sig.avg_response_length / 1000.0).clamp(0.0, 1.0);
        let current_verbosity = profile.response_style.verbosity;
        let verb_diff = observed_verbosity - current_verbosity;
        if verb_diff.abs() > 0.05 {
            let delta = PersonalityDelta::new(DeltaSource::Observation)
                .with_adjustment("verbosity", verb_diff * 0.5)
                .with_confidence(0.6);
            profile.apply_correction(&delta);
        }

        // Compare vocabulary complexity against formality
        let form_diff = sig.vocabulary_complexity - profile.response_style.formality;
        if form_diff.abs() > 0.05 {
            let delta = PersonalityDelta::new(DeltaSource::Observation)
                .with_adjustment("formality", form_diff * 0.5)
                .with_confidence(0.6);
            profile.apply_correction(&delta);
        }

        // Compare question asking rate against autonomy
        if let Some(current_autonomy) = profile.personality_value("autonomy") {
            let auto_diff = sig.question_asking_rate.clamp(0.0, 1.0) - current_autonomy;
            if auto_diff.abs() > 0.05 {
                let delta = PersonalityDelta::new(DeltaSource::Observation)
                    .with_adjustment("autonomy", auto_diff * 0.5)
                    .with_confidence(0.5);
                profile.apply_correction(&delta);
            }
        }
    }

    /// COMPOUND: Self-monitor own output against target signature.
    /// Compares what we generated against what the target model would have generated.
    /// Returns corrections needed as a PersonalityDelta.
    /// This is the System 2 self-monitoring feedback path.
    pub fn self_monitor_output(
        &self,
        own_response: &str,
        target_sig: &BehaviorSignature,
    ) -> PersonalityDelta {
        let own_patterns = self.analyze_response(own_response);
        let mut delta = PersonalityDelta::new(DeltaSource::SelfMonitoring);

        // Check hedging alignment
        let own_hedging: f64 = own_patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::Hedging)
            .map(|p| p.frequency)
            .sum::<f64>()
            / own_patterns
                .iter()
                .filter(|p| p.pattern_type == PatternType::Hedging)
                .count()
                .max(1) as f64;
        let target_hedging = target_sig.hedging_level();
        let hedging_diff = target_hedging - own_hedging;
        if hedging_diff.abs() > 0.05 {
            // Need to adjust confidence inversely to hedging diff
            delta = delta.with_adjustment("confidence", -hedging_diff * 0.3);
        }

        // Check response length alignment
        let own_length = own_response.len() as f64;
        let target_length = target_sig.avg_response_length;
        if target_length > 0.0 {
            let length_ratio = own_length / target_length;
            if length_ratio < 0.7 {
                // Our response is too short
                delta = delta.with_adjustment("verbosity", 0.1);
            } else if length_ratio > 1.3 {
                // Our response is too long
                delta = delta.with_adjustment("verbosity", -0.1);
            }
        }

        // Check tone alignment
        let own_tone_freq: f64 = own_patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::Tone)
            .map(|p| p.frequency)
            .sum::<f64>();
        let target_tone_freq: f64 = target_sig
            .patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::Tone)
            .map(|p| p.frequency)
            .sum::<f64>();
        let tone_diff = target_tone_freq - own_tone_freq;
        if tone_diff.abs() > 0.1 {
            delta = delta.with_adjustment("warmth", tone_diff * 0.2);
        }

        // Confidence is based on how many dimensions we could compare
        let comparison_count: f64 = 3.0; // hedging, length, tone
        delta = delta.with_confidence((comparison_count / 5.0).clamp(0.3, 0.9));

        // Always produce at least a baseline alignment adjustment so
        // the feedback loop has something to compound on, even when
        // all dimensions are already within tolerance.
        if delta.adjustments.is_empty() {
            delta = delta.with_adjustment("alignment", 0.0);
        }

        delta
    }

    /// COMPOUND: Compute convergence score between a profile and signature.
    /// Returns 0.0 (completely divergent) to 1.0 (perfect match).
    /// Used by CompoundPersona to track mimicry fidelity over time.
    pub fn compute_convergence(&self, profile: &AiProfile, sig: &BehaviorSignature) -> f64 {
        let mut score = 0.0;
        let mut dimensions = 0.0;

        // Confidence vs hedging alignment
        if let Some(confidence) = profile.personality_value("confidence") {
            let implied_confidence = 1.0 - sig.hedging_level() * 2.0;
            score += 1.0 - (confidence - implied_confidence).abs().min(1.0);
            dimensions += 1.0;
        }

        // Verbosity alignment
        let observed_verbosity = (sig.avg_response_length / 1000.0).clamp(0.0, 1.0);
        score += 1.0 - (profile.response_style.verbosity - observed_verbosity).abs();
        dimensions += 1.0;

        // Formality alignment
        score += 1.0 - (profile.response_style.formality - sig.vocabulary_complexity).abs();
        dimensions += 1.0;

        // Autonomy vs question asking rate
        if let Some(autonomy) = profile.personality_value("autonomy") {
            score += 1.0 - (autonomy - sig.question_asking_rate.clamp(0.0, 1.0)).abs();
            dimensions += 1.0;
        }

        // Pattern coverage - does the profile's signature phrases match observed patterns?
        if !profile.signature_phrases.is_empty() && !sig.patterns.is_empty() {
            let opening_patterns = sig.patterns_of_type(&PatternType::Opening);
            let mut phrase_hits = 0;
            for phrase in &profile.signature_phrases {
                for pattern in &opening_patterns {
                    if pattern
                        .description
                        .to_lowercase()
                        .contains(&phrase.to_lowercase())
                    {
                        phrase_hits += 1;
                        break;
                    }
                }
            }
            score += phrase_hits as f64 / profile.signature_phrases.len() as f64;
            dimensions += 1.0;
        }

        if dimensions > 0.0 {
            (score / dimensions).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

impl Default for BehaviorAnalyzer {
    fn default() -> Self {
        BehaviorAnalyzer::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::profile::AiProfileStore;

    #[test]
    fn test_analyze_gpt4o_style() {
        let analyzer = BehaviorAnalyzer::new();
        let response = "Certainly! Here's how you can implement that feature:\n\n\
            1. First, create a new struct\n\
            2. Implement the trait\n\
            3. Add error handling\n\n\
            ```rust\nfn main() {}\n```";

        let patterns = analyzer.analyze_response(response);
        assert!(!patterns.is_empty());

        // Should detect opening pattern
        let openings: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::Opening)
            .collect();
        assert!(!openings.is_empty());

        // Should detect structure (code blocks + numbered lists)
        let structure: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::Structure)
            .collect();
        assert!(!structure.is_empty());
    }

    #[test]
    fn test_analyze_claude_style() {
        let analyzer = BehaviorAnalyzer::new();
        let response = "I'd be happy to help with that. I should note that this approach \
            has some trade-offs to consider.\n\n\
            I think the best path forward might be:\n\
            - Option A: simpler but less flexible\n\
            - Option B: more complex but extensible";

        let patterns = analyzer.analyze_response(response);

        // Should detect hedging
        let hedging: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::Hedging)
            .collect();
        assert!(!hedging.is_empty());

        // Should detect meta-commentary
        let meta: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_type == PatternType::MetaCommentary)
            .collect();
        assert!(!meta.is_empty());
    }

    #[test]
    fn test_build_signature() {
        let mut analyzer = BehaviorAnalyzer::new();
        let responses = vec![
            "Certainly! Here's the answer.".to_string(),
            "Sure! Let me help you with that.".to_string(),
            "Here's what I'd recommend:\n1. First step\n2. Second step".to_string(),
        ];

        let signature = analyzer.build_signature("test-model", &responses);
        assert_eq!(signature.samples_analyzed, 3);
        assert!(signature.avg_response_length > 0.0);
    }

    #[test]
    fn test_behavior_similarity() {
        let sig_a = BehaviorSignature {
            model_id: "a".to_string(),
            patterns: vec![],
            avg_response_length: 500.0,
            vocabulary_complexity: 0.7,
            sentence_complexity: 15.0,
            question_asking_rate: 0.1,
            code_to_text_ratio: 0.3,
            samples_analyzed: 10,
        };

        let sig_b = BehaviorSignature {
            model_id: "b".to_string(),
            patterns: vec![],
            avg_response_length: 550.0,
            vocabulary_complexity: 0.65,
            sentence_complexity: 14.0,
            question_asking_rate: 0.15,
            code_to_text_ratio: 0.35,
            samples_analyzed: 10,
        };

        let similarity = sig_a.similarity_to(&sig_b);
        assert!(
            similarity > 0.7,
            "Similar signatures should have high similarity"
        );
    }

    #[test]
    fn test_signature_serialization() {
        let sig = BehaviorSignature {
            model_id: "test".to_string(),
            patterns: vec![ResponsePattern {
                pattern_type: PatternType::Opening,
                frequency: 0.8,
                examples: vec!["Certainly!".to_string()],
                description: "Opens with Certainly!".to_string(),
            }],
            avg_response_length: 500.0,
            vocabulary_complexity: 0.7,
            sentence_complexity: 15.0,
            question_asking_rate: 0.1,
            code_to_text_ratio: 0.3,
            samples_analyzed: 10,
        };

        let json = serde_json::to_string(&sig).unwrap();
        let restored: BehaviorSignature = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.model_id, "test");
        assert_eq!(restored.patterns.len(), 1);
    }

    #[test]
    fn test_refine_profile() {
        let analyzer = BehaviorAnalyzer::new();
        let mut profile = AiProfileStore::gpt4o_profile();
        let original_confidence = profile.personality_value("confidence").unwrap();

        // Create a signature with high hedging (should decrease confidence)
        let sig = BehaviorSignature {
            model_id: "test".to_string(),
            patterns: vec![ResponsePattern {
                pattern_type: PatternType::Hedging,
                frequency: 0.9,
                examples: vec![],
                description: "Very hedgy".to_string(),
            }],
            avg_response_length: 600.0,
            vocabulary_complexity: 0.5,
            sentence_complexity: 15.0,
            question_asking_rate: 0.1,
            code_to_text_ratio: 0.3,
            samples_analyzed: 5,
        };

        analyzer.refine_profile(&mut profile, &sig);
        let new_confidence = profile.personality_value("confidence").unwrap();
        // High hedging should push confidence down
        assert!(
            new_confidence < original_confidence,
            "High hedging should decrease confidence: {} vs {}",
            new_confidence,
            original_confidence
        );
    }

    #[test]
    fn test_self_monitor_output() {
        let analyzer = BehaviorAnalyzer::new();
        let target_sig = BehaviorSignature {
            model_id: "target".to_string(),
            patterns: vec![ResponsePattern {
                pattern_type: PatternType::Hedging,
                frequency: 0.8,
                examples: vec![],
                description: "High hedging".to_string(),
            }],
            avg_response_length: 500.0,
            vocabulary_complexity: 0.5,
            sentence_complexity: 15.0,
            question_asking_rate: 0.1,
            code_to_text_ratio: 0.3,
            samples_analyzed: 5,
        };

        // Our response has no hedging - should get a delta suggesting less confidence
        let delta = analyzer.self_monitor_output("Here is the definitive answer.", &target_sig);
        assert!(!delta.adjustments.is_empty());
        assert!(delta.confidence > 0.0);
    }

    #[test]
    fn test_compute_convergence() {
        let analyzer = BehaviorAnalyzer::new();

        // Create profile and matching signature
        let profile = AiProfileStore::gpt4o_profile();
        let sig = BehaviorSignature {
            model_id: "gpt4o".to_string(),
            patterns: vec![],
            avg_response_length: 600.0, // matches ~0.6 verbosity
            vocabulary_complexity: 0.4, // matches formality
            sentence_complexity: 15.0,
            question_asking_rate: 0.4, // matches autonomy
            code_to_text_ratio: 0.3,
            samples_analyzed: 10,
        };

        let convergence = analyzer.compute_convergence(&profile, &sig);
        assert!(
            convergence > 0.5,
            "Profile designed to match signature should have reasonable convergence: {}",
            convergence
        );
    }
}
