// =================================================================
// SYSTEM 1: FAST PATH CACHE
// =================================================================
// The instinctive, pattern-matching layer of the dual-process system.
// Caches compiled behavioral signatures for O(1) persona lookups,
// pre-built response templates, and fast input classification.
//
// COMPOUND INTEGRATIONS:
// - compile_from(): System 2 -> System 1 bridge
// - record_hit(): usage compounds confidence over time
// - HotSwap: instant persona switching from cached snapshots
// - InstinctiveRouter: fast modality classification without deliberation
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mimicry::analyzer::BehaviorSignature;
use crate::mimicry::capability::Modality;
use crate::mimicry::profile::AiProfileStore;

// =================================================================
// RESPONSE TEMPLATE - Pre-compiled response skeletons
// =================================================================

/// A pre-compiled response skeleton for fast generation.
/// System 2 creates these; System 1 uses them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTemplate {
    /// Patterns that trigger this template (substring matches)
    pub trigger_patterns: Vec<String>,
    /// Response skeleton with {placeholders} for variable content
    pub skeleton: String,
    /// Base confidence, compounds with usage
    pub confidence: f64,
    /// Number of times this template has been successfully used
    pub times_used: u64,
    /// Source persona ID
    pub persona_id: String,
}

impl ResponseTemplate {
    /// Creates a new response template for the given persona with trigger patterns and a skeleton.
    pub fn new(persona_id: &str, triggers: Vec<String>, skeleton: &str) -> Self {
        ResponseTemplate {
            trigger_patterns: triggers,
            skeleton: skeleton.to_string(),
            confidence: 0.5,
            times_used: 0,
            persona_id: persona_id.to_string(),
        }
    }

    /// Check if input matches any trigger pattern
    pub fn matches(&self, input: &str) -> bool {
        let lower = input.to_lowercase();
        self.trigger_patterns
            .iter()
            .any(|t| lower.contains(&t.to_lowercase()))
    }

    /// Record a successful use - compounds confidence
    pub fn record_use(&mut self) {
        self.times_used += 1;
        // Confidence grows logarithmically with usage, capped at 0.95
        self.confidence = (0.5 + (self.times_used as f64).ln() * 0.1).min(0.95);
    }
}

// =================================================================
// TONE PROFILE - Fast-lookup emotional parameters
// =================================================================

/// Pre-computed tone parameters for instant style application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneProfile {
    /// Warmth level from cold (0.0) to warm (1.0)
    pub warmth: f64,
    /// Enthusiasm level from flat (0.0) to enthusiastic (1.0)
    pub enthusiasm: f64,
    /// Formality level from casual (0.0) to formal (1.0)
    pub formality: f64,
}

impl Default for ToneProfile {
    fn default() -> Self {
        ToneProfile {
            warmth: 0.5,
            enthusiasm: 0.5,
            formality: 0.5,
        }
    }
}

// =================================================================
// STRUCTURE PREFERENCES - Fast-lookup formatting parameters
// =================================================================

/// Pre-computed structural preferences for instant formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructurePrefs {
    /// Whether the persona tends to use bullet/numbered lists
    pub uses_lists: bool,
    /// Whether the persona tends to include fenced code blocks
    pub uses_code_blocks: bool,
    /// Whether the persona tends to use markdown headers
    pub uses_headers: bool,
    /// Preferred list marker style (e.g. `"- "`, `"* "`, `"1. "`)
    pub preferred_list_marker: String,
    /// Average number of sentences per paragraph
    pub avg_paragraph_sentences: usize,
}

impl Default for StructurePrefs {
    fn default() -> Self {
        StructurePrefs {
            uses_lists: true,
            uses_code_blocks: true,
            uses_headers: false,
            preferred_list_marker: "- ".to_string(),
            avg_paragraph_sentences: 3,
        }
    }
}

// =================================================================
// CACHED SIGNATURE - Compiled form for fast matching
// =================================================================

/// Compiled form of a BehaviorSignature optimized for System 1 speed.
/// Pre-computes commonly needed values to avoid recalculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSignature {
    /// The model identifier this signature was compiled from
    pub model_id: String,
    /// Pre-computed opening phrases with probabilities
    pub opening_phrases: Vec<(String, f64)>,
    /// Pre-computed hedging level
    pub hedging_level: f64,
    /// Pre-computed tone profile
    pub tone: ToneProfile,
    /// Pre-computed structure preferences
    pub structure: StructurePrefs,
    /// Number of source samples this was compiled from
    pub source_samples: usize,
    /// Cache hit count - compounds with confidence
    pub hit_count: u64,
    /// Confidence in this cached entry (compounds with usage)
    pub confidence: f64,
}

impl CachedSignature {
    /// COMPOUND: Compile from a full BehaviorSignature (System 2 -> System 1 bridge).
    /// Extracts and pre-computes the most frequently needed values for instant access.
    pub fn compile_from(sig: &BehaviorSignature) -> Self {
        use crate::mimicry::analyzer::PatternType;

        // Extract opening phrases with their frequencies
        let opening_phrases: Vec<(String, f64)> = sig
            .patterns_of_type(&PatternType::Opening)
            .iter()
            .flat_map(|p| {
                p.examples
                    .iter()
                    .map(|ex| (ex.clone(), p.frequency))
                    .collect::<Vec<_>>()
            })
            .collect();

        // Pre-compute hedging level
        let hedging_level = sig.hedging_level();

        // Extract tone from tone patterns
        let tone_patterns = sig.patterns_of_type(&PatternType::Tone);
        let enthusiasm = tone_patterns.iter().map(|p| p.frequency).sum::<f64>()
            / tone_patterns.len().max(1) as f64;

        let tone = ToneProfile {
            warmth: if enthusiasm > 0.3 { 0.7 } else { 0.4 },
            enthusiasm,
            formality: sig.vocabulary_complexity,
        };

        // Extract structure preferences from structure patterns
        let structure_patterns = sig.patterns_of_type(&PatternType::Structure);
        let uses_code = structure_patterns
            .iter()
            .any(|p| p.description.contains("code"));
        let uses_lists = structure_patterns
            .iter()
            .any(|p| p.description.contains("list"));
        let uses_numbered = structure_patterns
            .iter()
            .any(|p| p.description.contains("numbered"));

        let structure = StructurePrefs {
            uses_lists,
            uses_code_blocks: uses_code,
            uses_headers: false,
            preferred_list_marker: if uses_numbered {
                "1. ".to_string()
            } else {
                "- ".to_string()
            },
            avg_paragraph_sentences: if sig.avg_response_length > 500.0 {
                4
            } else {
                2
            },
        };

        CachedSignature {
            model_id: sig.model_id.clone(),
            opening_phrases,
            hedging_level,
            tone,
            structure,
            source_samples: sig.samples_analyzed,
            hit_count: 0,
            confidence: 0.5,
        }
    }

    /// Record a cache hit - compounds confidence over time
    pub fn record_hit(&mut self) {
        self.hit_count += 1;
        self.confidence = (0.5 + (self.hit_count as f64).ln() * 0.08).min(0.95);
    }
}

// =================================================================
// SIGNATURE CACHE - O(1) persona lookup
// =================================================================

/// HashMap-based O(1) persona cache for System 1 fast path.
/// Stores pre-compiled CachedSignatures keyed by model_id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureCache {
    cache: HashMap<String, CachedSignature>,
    /// Total cache lookups (for statistics)
    pub total_lookups: u64,
    /// Total cache hits
    pub total_hits: u64,
}

impl SignatureCache {
    /// Creates a new empty `SignatureCache` with zeroed statistics.
    pub fn new() -> Self {
        SignatureCache {
            cache: HashMap::new(),
            total_lookups: 0,
            total_hits: 0,
        }
    }

    /// O(1) lookup of a cached signature
    pub fn lookup(&mut self, model_id: &str) -> Option<&CachedSignature> {
        self.total_lookups += 1;
        if self.cache.contains_key(model_id) {
            self.total_hits += 1;
            // Record the hit
            if let Some(cached) = self.cache.get_mut(model_id) {
                cached.record_hit();
            }
            self.cache.get(model_id)
        } else {
            None
        }
    }

    /// COMPOUND: Compile a BehaviorSignature and cache it (System 2 -> System 1 bridge)
    pub fn compile_from(&mut self, sig: &BehaviorSignature) {
        let cached = CachedSignature::compile_from(sig);
        self.cache.insert(sig.model_id.clone(), cached);
    }

    /// Warm up the cache by compiling all known profiles from a store.
    /// Creates synthetic signatures from profile data for initial cache population.
    pub fn warm_up(&mut self, store: &AiProfileStore) {
        for id in store.ids() {
            if let Some(profile) = store.get(&id) {
                // Create a synthetic signature from profile metadata
                let mut sig = BehaviorSignature::new(&id);
                sig.avg_response_length = profile.response_style.verbosity * 1000.0;
                sig.vocabulary_complexity = profile.response_style.formality;
                sig.question_asking_rate = profile.personality_value("autonomy").unwrap_or(0.3);
                sig.samples_analyzed = 0; // synthetic

                // Add opening patterns from signature phrases
                for phrase in &profile.signature_phrases {
                    sig.patterns
                        .push(crate::mimicry::analyzer::ResponsePattern {
                            pattern_type: crate::mimicry::analyzer::PatternType::Opening,
                            frequency: 0.7,
                            examples: vec![phrase.clone()],
                            description: format!("Signature phrase: {}", phrase),
                        });
                }

                self.compile_from(&sig);
            }
        }
    }

    /// Get cache statistics
    pub fn hit_rate(&self) -> f64 {
        if self.total_lookups > 0 {
            self.total_hits as f64 / self.total_lookups as f64
        } else {
            0.0
        }
    }

    /// Number of cached entries
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Check if a model is cached
    pub fn contains(&self, model_id: &str) -> bool {
        self.cache.contains_key(model_id)
    }
}

impl Default for SignatureCache {
    fn default() -> Self {
        SignatureCache::new()
    }
}

// =================================================================
// HOT SWAP - Instant persona switching
// =================================================================

/// Enables instant persona switching by keeping pre-loaded snapshots.
/// No deserialization overhead - just swap the pointer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotSwap {
    /// Pre-loaded persona snapshots ready for instant switch
    preloaded: HashMap<String, HotSwapEntry>,
    /// Currently active persona ID
    current_id: Option<String>,
}

/// A pre-loaded persona snapshot entry ready for instant switching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotSwapEntry {
    /// Identifier of the persona this snapshot represents
    pub persona_id: String,
    /// Serialized snapshot data (JSON) for the CompoundPersona
    pub snapshot_json: String,
    /// When this was preloaded (iteration count)
    pub preloaded_at: u64,
    /// Number of times switched to
    pub switch_count: u64,
}

impl HotSwap {
    /// Creates a new `HotSwap` with no preloaded personas.
    pub fn new() -> Self {
        HotSwap {
            preloaded: HashMap::new(),
            current_id: None,
        }
    }

    /// Preload a persona snapshot for instant switching
    pub fn preload(&mut self, persona_id: &str, snapshot_json: String, iteration: u64) {
        self.preloaded.insert(
            persona_id.to_string(),
            HotSwapEntry {
                persona_id: persona_id.to_string(),
                snapshot_json,
                preloaded_at: iteration,
                switch_count: 0,
            },
        );
    }

    /// Switch to a pre-loaded persona. Returns the snapshot JSON if available.
    pub fn switch_to(&mut self, persona_id: &str) -> Option<&str> {
        if let Some(entry) = self.preloaded.get_mut(persona_id) {
            entry.switch_count += 1;
            self.current_id = Some(persona_id.to_string());
            Some(&entry.snapshot_json)
        } else {
            None
        }
    }

    /// Get the currently active persona ID
    pub fn current(&self) -> Option<&str> {
        self.current_id.as_deref()
    }

    /// List all preloaded persona IDs
    pub fn preloaded_ids(&self) -> Vec<String> {
        self.preloaded.keys().cloned().collect()
    }

    /// Check if a persona is preloaded
    pub fn is_preloaded(&self, persona_id: &str) -> bool {
        self.preloaded.contains_key(persona_id)
    }
}

impl Default for HotSwap {
    fn default() -> Self {
        HotSwap::new()
    }
}

// =================================================================
// INSTINCTIVE ROUTER - Fast input classification
// =================================================================

/// Fast input classification without full System 2 deliberation.
/// Uses keyword lists to quickly determine input modality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstinctiveRouter {
    /// Keyword -> modality mappings for fast classification
    keyword_map: Vec<(Vec<String>, Modality, f64)>, // (keywords, modality, base_confidence)
    /// Minimum confidence threshold for routing decisions
    pub confidence_threshold: f64,
}

impl InstinctiveRouter {
    /// Creates a new router pre-populated with default keyword-to-modality mappings.
    pub fn new() -> Self {
        InstinctiveRouter {
            keyword_map: vec![
                (
                    vec![
                        "code".to_string(),
                        "function".to_string(),
                        "implement".to_string(),
                        "debug".to_string(),
                        "compile".to_string(),
                        "rust".to_string(),
                        "python".to_string(),
                        "javascript".to_string(),
                        "error".to_string(),
                        "bug".to_string(),
                        "```".to_string(),
                        "fn ".to_string(),
                        "def ".to_string(),
                        "class ".to_string(),
                    ],
                    Modality::Code,
                    0.7,
                ),
                (
                    vec![
                        "image".to_string(),
                        "picture".to_string(),
                        "photo".to_string(),
                        "screenshot".to_string(),
                        "visual".to_string(),
                        "see".to_string(),
                        "look at".to_string(),
                        "diagram".to_string(),
                    ],
                    Modality::Vision,
                    0.6,
                ),
                (
                    vec![
                        "think".to_string(),
                        "reason".to_string(),
                        "prove".to_string(),
                        "logic".to_string(),
                        "analyze".to_string(),
                        "step by step".to_string(),
                        "why".to_string(),
                        "explain".to_string(),
                        "derive".to_string(),
                    ],
                    Modality::Reasoning,
                    0.5,
                ),
                (
                    vec![
                        "audio".to_string(),
                        "sound".to_string(),
                        "music".to_string(),
                        "voice".to_string(),
                        "speech".to_string(),
                        "listen".to_string(),
                        "hear".to_string(),
                        "podcast".to_string(),
                    ],
                    Modality::Audio,
                    0.6,
                ),
                (
                    vec![
                        "call".to_string(),
                        "execute".to_string(),
                        "tool".to_string(),
                        "api".to_string(),
                        "endpoint".to_string(),
                        "invoke".to_string(),
                    ],
                    Modality::FunctionCall,
                    0.5,
                ),
            ],
            confidence_threshold: 0.4,
        }
    }

    /// Fast-classify an input string into a modality with confidence.
    /// Returns (modality, confidence) or Text as default.
    pub fn classify(&self, input: &str) -> (Modality, f64) {
        let lower = input.to_lowercase();
        let mut best_modality = Modality::Text;
        let mut best_score = 0.0;

        for (keywords, modality, base_confidence) in &self.keyword_map {
            let hits = keywords
                .iter()
                .filter(|k| lower.contains(k.as_str()))
                .count();

            if hits > 0 {
                // Score scales with base_confidence and number of keyword hits.
                // Each hit adds weight, with mild diminishing returns via sqrt.
                let score = base_confidence * (hits as f64).sqrt() * 0.4;

                if score > best_score {
                    best_score = score;
                    best_modality = modality.clone();
                }
            }
        }

        // Default to text with baseline confidence
        if best_score < self.confidence_threshold {
            (Modality::Text, 0.8) // Text is always the safe default
        } else {
            (best_modality, best_score.min(0.95))
        }
    }
}

impl Default for InstinctiveRouter {
    fn default() -> Self {
        InstinctiveRouter::new()
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_template_match() {
        let template = ResponseTemplate::new(
            "gpt4o",
            vec!["hello".to_string(), "hi".to_string()],
            "Certainly! {response}",
        );
        assert!(template.matches("Hello there!"));
        assert!(template.matches("hi how are you"));
        assert!(!template.matches("goodbye"));
    }

    #[test]
    fn test_response_template_compound_confidence() {
        let mut template = ResponseTemplate::new("test", vec![], "test");
        assert_eq!(template.confidence, 0.5);

        for _ in 0..10 {
            template.record_use();
        }
        assert!(
            template.confidence > 0.5,
            "Confidence should increase with usage"
        );
        assert!(template.confidence <= 0.95, "Confidence should be capped");
    }

    #[test]
    fn test_cached_signature_compile() {
        let mut sig = BehaviorSignature::new("test-model");
        sig.avg_response_length = 500.0;
        sig.vocabulary_complexity = 0.7;
        sig.samples_analyzed = 5;

        let cached = CachedSignature::compile_from(&sig);
        assert_eq!(cached.model_id, "test-model");
        assert_eq!(cached.source_samples, 5);
        assert!(cached.confidence == 0.5); // initial confidence
    }

    #[test]
    fn test_cached_signature_compound_hits() {
        let sig = BehaviorSignature::new("test");
        let mut cached = CachedSignature::compile_from(&sig);

        for _ in 0..20 {
            cached.record_hit();
        }
        assert!(
            cached.confidence > 0.5,
            "Confidence should compound with hits"
        );
        assert!(cached.hit_count == 20);
    }

    #[test]
    fn test_signature_cache_lookup() {
        let mut cache = SignatureCache::new();
        let sig = BehaviorSignature::new("gpt4o");
        cache.compile_from(&sig);

        assert!(cache.contains("gpt4o"));
        assert!(!cache.contains("unknown"));

        let result = cache.lookup("gpt4o");
        assert!(result.is_some());
        assert_eq!(cache.total_hits, 1);
        assert_eq!(cache.total_lookups, 1);

        let _ = cache.lookup("unknown");
        assert_eq!(cache.total_hits, 1);
        assert_eq!(cache.total_lookups, 2);
    }

    #[test]
    fn test_signature_cache_warm_up() {
        let store = AiProfileStore::default();
        let mut cache = SignatureCache::new();
        cache.warm_up(&store);

        assert!(cache.size() > 0);
        assert!(cache.contains("gpt4o"));
        assert!(cache.contains("claude"));
        assert!(cache.contains("rustyworm"));
    }

    #[test]
    fn test_hot_swap() {
        let mut hot_swap = HotSwap::new();

        hot_swap.preload("gpt4o", r#"{"profile":"gpt4o"}"#.to_string(), 0);
        hot_swap.preload("claude", r#"{"profile":"claude"}"#.to_string(), 0);

        assert!(hot_swap.is_preloaded("gpt4o"));
        assert!(!hot_swap.is_preloaded("unknown"));

        let json = hot_swap.switch_to("gpt4o");
        assert!(json.is_some());
        assert_eq!(hot_swap.current(), Some("gpt4o"));

        let json = hot_swap.switch_to("claude");
        assert!(json.is_some());
        assert_eq!(hot_swap.current(), Some("claude"));
    }

    #[test]
    fn test_instinctive_router_code() {
        let router = InstinctiveRouter::new();
        let (modality, confidence) = router.classify("Can you help me debug this rust function?");
        assert_eq!(modality, Modality::Code);
        assert!(confidence > 0.0);
    }

    #[test]
    fn test_instinctive_router_text_default() {
        let router = InstinctiveRouter::new();
        let (modality, _confidence) = router.classify("Tell me about the weather today");
        assert_eq!(modality, Modality::Text);
    }

    #[test]
    fn test_instinctive_router_reasoning() {
        let router = InstinctiveRouter::new();
        let (modality, _) =
            router.classify("Can you prove this theorem step by step and explain the logic?");
        assert_eq!(modality, Modality::Reasoning);
    }

    #[test]
    fn test_cache_serialization() {
        let mut cache = SignatureCache::new();
        let sig = BehaviorSignature::new("test");
        cache.compile_from(&sig);

        let json = serde_json::to_string(&cache).unwrap();
        let restored: SignatureCache = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.size(), 1);
        assert!(restored.contains("test"));
    }

    #[test]
    fn test_hot_swap_serialization() {
        let mut hs = HotSwap::new();
        hs.preload("test", "{}".to_string(), 0);

        let json = serde_json::to_string(&hs).unwrap();
        let restored: HotSwap = serde_json::from_str(&json).unwrap();
        assert!(restored.is_preloaded("test"));
    }
}
