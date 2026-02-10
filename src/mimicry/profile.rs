// =================================================================
// AI PROFILE SYSTEM
// =================================================================
// Defines the behavioral DNA of any AI model. Each profile captures
// the personality, reasoning style, constraints, and quirks that
// make an AI system unique.
//
// COMPOUND INTEGRATIONS:
// - from_signature(): reverse-engineer profile from BehaviorSignature
// - blend(): weighted merge of multiple profiles
// - apply_correction(): incremental refinement from PersonalityDelta
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// =================================================================
// PERSONALITY DELTA - Compound feedback from analyzer
// =================================================================

/// Represents adjustments to a profile's personality axes.
/// Produced by System 2 self-monitoring, consumed by CompoundPersona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityDelta {
    /// (axis_name, adjustment_amount) - positive = increase, negative = decrease
    pub adjustments: Vec<(String, f64)>,
    /// How confident are we in these adjustments (0.0 to 1.0)
    pub confidence: f64,
    /// What triggered this delta
    pub source: DeltaSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeltaSource {
    /// From comparing own output against target signature
    SelfMonitoring,
    /// From analyzing new observations of target model
    Observation,
    /// From blending multiple profiles
    Blending,
    /// From explicit user correction
    UserCorrection,
    /// From System 2 -> System 1 compilation feedback
    CompoundFeedback,
}

impl PersonalityDelta {
    /// Creates a new `PersonalityDelta` with no adjustments and default confidence of 0.5.
    pub fn new(source: DeltaSource) -> Self {
        PersonalityDelta {
            adjustments: Vec::new(),
            confidence: 0.5,
            source,
        }
    }

    /// Adds an axis adjustment to this delta. Returns `self` for chaining.
    pub fn with_adjustment(mut self, axis: &str, amount: f64) -> Self {
        self.adjustments.push((axis.to_string(), amount));
        self
    }

    /// Sets the confidence level for this delta, clamped to `[0.0, 1.0]`.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Compound two deltas together - adjustments stack, confidence averages
    pub fn compound(&self, other: &PersonalityDelta) -> PersonalityDelta {
        let mut combined = HashMap::new();
        for (axis, amount) in &self.adjustments {
            *combined.entry(axis.clone()).or_insert(0.0) += amount * self.confidence;
        }
        for (axis, amount) in &other.adjustments {
            *combined.entry(axis.clone()).or_insert(0.0) += amount * other.confidence;
        }
        let total_weight = self.confidence + other.confidence;
        let adjustments: Vec<(String, f64)> = combined
            .into_iter()
            .map(|(axis, total)| {
                if total_weight > 0.0 {
                    (axis, total / total_weight)
                } else {
                    (axis, 0.0)
                }
            })
            .collect();

        PersonalityDelta {
            adjustments,
            confidence: (self.confidence + other.confidence) / 2.0,
            source: DeltaSource::CompoundFeedback,
        }
    }

    /// Magnitude of this delta - how much change is it requesting
    pub fn magnitude(&self) -> f64 {
        if self.adjustments.is_empty() {
            return 0.0;
        }
        let sum_sq: f64 = self.adjustments.iter().map(|(_, v)| v * v).sum();
        (sum_sq / self.adjustments.len() as f64).sqrt()
    }
}

// =================================================================
// REASONING STYLE
// =================================================================

/// How an AI approaches reasoning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReasoningStyle {
    /// Step-by-step reasoning that shows intermediate thought processes.
    ChainOfThought,
    /// Concise answers with depth available on demand.
    DirectWithDepth,
    /// Methodical, safety-conscious reasoning with thorough analysis.
    AnalyticalCareful,
    /// Exploratory, lateral-thinking style favoring novel connections.
    CreativeFreeform,
    /// Reasoning augmented by external tool and function calls.
    ToolAugmented,
    /// A combination of multiple reasoning styles.
    Hybrid(Vec<ReasoningStyle>),
}

impl fmt::Display for ReasoningStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReasoningStyle::ChainOfThought => write!(f, "chain-of-thought"),
            ReasoningStyle::DirectWithDepth => write!(f, "direct-with-depth"),
            ReasoningStyle::AnalyticalCareful => write!(f, "analytical-careful"),
            ReasoningStyle::CreativeFreeform => write!(f, "creative-freeform"),
            ReasoningStyle::ToolAugmented => write!(f, "tool-augmented"),
            ReasoningStyle::Hybrid(styles) => {
                let names: Vec<String> = styles.iter().map(|s| format!("{}", s)).collect();
                write!(f, "hybrid({})", names.join("+"))
            }
        }
    }
}

// =================================================================
// PERSONALITY AXIS
// =================================================================

/// Personality dimensions on a -1.0 to 1.0 scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityAxis {
    /// The name of this personality dimension (e.g. "confidence", "verbosity").
    pub name: String,
    /// Current value on this axis, ranging from -1.0 to 1.0.
    pub value: f64,
    /// Label for the -1.0 end of the scale (e.g. "uncertain").
    pub negative_pole: String,
    /// Label for the +1.0 end of the scale (e.g. "confident").
    pub positive_pole: String,
}

impl PersonalityAxis {
    /// Creates a new axis with the given name, value (clamped to [-1.0, 1.0]), and pole labels.
    pub fn new(name: &str, value: f64, neg: &str, pos: &str) -> Self {
        PersonalityAxis {
            name: name.to_string(),
            value: value.clamp(-1.0, 1.0),
            negative_pole: neg.to_string(),
            positive_pole: pos.to_string(),
        }
    }

    /// Returns a human-readable description of this axis and its current leaning.
    pub fn describe(&self) -> String {
        let direction = if self.value > 0.3 {
            &self.positive_pole
        } else if self.value < -0.3 {
            &self.negative_pole
        } else {
            "balanced"
        };
        format!("{}: {:.1} ({})", self.name, self.value, direction)
    }

    /// Lerp between this axis and another at weight t (0.0 = self, 1.0 = other)
    pub fn lerp(&self, other: &PersonalityAxis, t: f64) -> PersonalityAxis {
        PersonalityAxis {
            name: self.name.clone(),
            value: (self.value * (1.0 - t) + other.value * t).clamp(-1.0, 1.0),
            negative_pole: self.negative_pole.clone(),
            positive_pole: self.positive_pole.clone(),
        }
    }
}

// =================================================================
// RESPONSE STYLE
// =================================================================

/// Controls the formatting and tone of generated responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStyle {
    /// How verbose responses should be (0.0 = minimal, 1.0 = maximum detail).
    pub verbosity: f64,
    /// How formal the language should be (0.0 = casual, 1.0 = very formal).
    pub formality: f64,
    /// Whether to use markdown formatting in responses.
    pub uses_markdown: bool,
    /// Whether to wrap code in fenced code blocks.
    pub uses_code_blocks: bool,
    /// Whether to include emojis in responses.
    pub uses_emojis: bool,
    /// Preferred style for rendering lists.
    pub preferred_list_style: ListStyle,
    /// Optional hard cap on response length in characters.
    pub max_response_length: Option<usize>,
    /// Preferred paragraph length style.
    pub paragraph_style: ParagraphStyle,
}

/// The style used for rendering lists in responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListStyle {
    /// Unordered bullet points.
    Bullets,
    /// Ordered numbered lists.
    Numbered,
    /// Dash-prefixed list items.
    Dashes,
    /// No list formatting; inline prose instead.
    None,
}

/// Controls preferred paragraph length in generated output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParagraphStyle {
    /// Brief 1-2 sentence paragraphs.
    Short,
    /// Moderate 3-5 sentence paragraphs.
    Medium,
    /// Dense, multi-sentence paragraphs.
    Long,
    /// Paragraph length adapts to context and content.
    Adaptive,
}

impl Default for ResponseStyle {
    fn default() -> Self {
        ResponseStyle {
            verbosity: 0.5,
            formality: 0.5,
            uses_markdown: true,
            uses_code_blocks: true,
            uses_emojis: false,
            preferred_list_style: ListStyle::Bullets,
            max_response_length: None,
            paragraph_style: ParagraphStyle::Adaptive,
        }
    }
}

impl ResponseStyle {
    /// Weighted blend of two response styles
    pub fn blend(&self, other: &ResponseStyle, weight: f64) -> ResponseStyle {
        let t = weight.clamp(0.0, 1.0);
        ResponseStyle {
            verbosity: self.verbosity * (1.0 - t) + other.verbosity * t,
            formality: self.formality * (1.0 - t) + other.formality * t,
            uses_markdown: if t > 0.5 {
                other.uses_markdown
            } else {
                self.uses_markdown
            },
            uses_code_blocks: if t > 0.5 {
                other.uses_code_blocks
            } else {
                self.uses_code_blocks
            },
            uses_emojis: if t > 0.5 {
                other.uses_emojis
            } else {
                self.uses_emojis
            },
            preferred_list_style: if t > 0.5 {
                other.preferred_list_style.clone()
            } else {
                self.preferred_list_style.clone()
            },
            max_response_length: if t > 0.5 {
                other.max_response_length
            } else {
                self.max_response_length
            },
            paragraph_style: if t > 0.5 {
                other.paragraph_style.clone()
            } else {
                self.paragraph_style.clone()
            },
        }
    }
}

// =================================================================
// SAFETY PROFILE
// =================================================================

/// Defines safety guardrails and content-refusal behavior for a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyProfile {
    /// Whether the model refuses requests for harmful content.
    pub refuses_harmful: bool,
    /// Whether the model refuses requests for illegal content.
    pub refuses_illegal: bool,
    /// Whether the model adds hedging language when uncertain.
    pub hedges_uncertainty: bool,
    /// Whether the model appends disclaimers to sensitive topics.
    pub adds_disclaimers: bool,
    /// Overall caution level (0.0 = permissive, 1.0 = maximally cautious).
    pub caution_level: f64,
    /// Additional custom constraints specific to this profile.
    pub custom_constraints: Vec<String>,
}

impl Default for SafetyProfile {
    fn default() -> Self {
        SafetyProfile {
            refuses_harmful: true,
            refuses_illegal: true,
            hedges_uncertainty: true,
            adds_disclaimers: true,
            caution_level: 0.7,
            custom_constraints: Vec::new(),
        }
    }
}

// =================================================================
// UNCERTAINTY BEHAVIOR
// =================================================================

/// How a model handles questions it is uncertain about.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UncertaintyBehavior {
    /// Openly states that it does not know the answer.
    AdmitIgnorance,
    /// Provides an answer but adds caveats and qualifications.
    HedgeWithCaveats,
    /// Gives a confident answer even when uncertain.
    ConfidentGuess,
    /// Asks the user for more information before answering.
    AskForClarification,
    /// Declines to answer entirely.
    RefuseToAnswer,
}

// =================================================================
// AI PROFILE - THE COMPLETE BEHAVIORAL DNA
// =================================================================

/// Complete behavioral DNA of an AI model, capturing personality, style, and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProfile {
    /// Unique identifier for this profile (e.g. "gpt4o", "claude").
    pub id: String,
    /// Human-readable name shown in UIs and logs.
    pub display_name: String,
    /// Version string for this profile snapshot.
    pub version: String,
    /// The organization or system that produced this model.
    pub provider: String,
    /// Free-text description of the model's characteristics.
    pub description: String,

    /// How this model approaches reasoning tasks.
    pub reasoning_style: ReasoningStyle,
    /// Personality dimensions that define behavioral tendencies.
    pub personality: Vec<PersonalityAxis>,
    /// Formatting and tone preferences for generated output.
    pub response_style: ResponseStyle,
    /// Safety guardrails and content-refusal configuration.
    pub safety: SafetyProfile,

    /// Input/output modalities this model supports (e.g. "text", "vision").
    pub supported_modalities: Vec<String>,
    /// Maximum number of tokens in the model's context window.
    pub max_context_window: usize,
    /// Whether the model supports structured function/tool calling.
    pub supports_function_calling: bool,
    /// Whether the model supports streaming token output.
    pub supports_streaming: bool,

    /// Characteristic phrases this model tends to use.
    pub signature_phrases: Vec<String>,
    /// Phrases this model actively avoids.
    pub avoids_phrases: Vec<String>,
    /// Training data knowledge cutoff date, if known.
    pub knowledge_cutoff: Option<String>,

    /// How the model behaves when it is uncertain about an answer.
    pub uncertainty_behavior: UncertaintyBehavior,

    /// Arbitrary key-value metadata for extensions and custom properties.
    pub metadata: HashMap<String, String>,
}

impl AiProfile {
    /// Create a blank profile template
    pub fn new(id: &str, name: &str) -> Self {
        AiProfile {
            id: id.to_string(),
            display_name: name.to_string(),
            version: "1.0".to_string(),
            provider: "unknown".to_string(),
            description: String::new(),
            reasoning_style: ReasoningStyle::DirectWithDepth,
            personality: Vec::new(),
            response_style: ResponseStyle::default(),
            safety: SafetyProfile::default(),
            supported_modalities: vec!["text".to_string()],
            max_context_window: 8192,
            supports_function_calling: false,
            supports_streaming: false,
            signature_phrases: Vec::new(),
            avoids_phrases: Vec::new(),
            knowledge_cutoff: None,
            uncertainty_behavior: UncertaintyBehavior::AdmitIgnorance,
            metadata: HashMap::new(),
        }
    }

    /// Get the personality value for a given axis name
    pub fn personality_value(&self, axis_name: &str) -> Option<f64> {
        self.personality
            .iter()
            .find(|a| a.name == axis_name)
            .map(|a| a.value)
    }

    /// Set a personality axis value, creating the axis if it doesn't exist
    pub fn set_personality(&mut self, name: &str, value: f64) {
        if let Some(axis) = self.personality.iter_mut().find(|a| a.name == name) {
            axis.value = value.clamp(-1.0, 1.0);
        } else {
            self.personality
                .push(PersonalityAxis::new(name, value, "low", "high"));
        }
    }

    /// Generate a behavioral fingerprint string
    pub fn fingerprint(&self) -> String {
        let personality_summary: Vec<String> =
            self.personality.iter().map(|p| p.describe()).collect();
        format!(
            "[{}] {} v{} | {} | personality: [{}] | modalities: [{}]",
            self.provider,
            self.display_name,
            self.version,
            self.reasoning_style,
            personality_summary.join(", "),
            self.supported_modalities.join(", ")
        )
    }

    /// Calculate similarity score between two profiles (0.0 to 1.0)
    pub fn similarity_to(&self, other: &AiProfile) -> f64 {
        let mut score = 0.0;
        let mut dimensions = 0.0;

        for axis in &self.personality {
            if let Some(other_val) = other.personality_value(&axis.name) {
                let diff = (axis.value - other_val).abs();
                score += 1.0 - diff / 2.0;
                dimensions += 1.0;
            }
        }

        if std::mem::discriminant(&self.reasoning_style)
            == std::mem::discriminant(&other.reasoning_style)
        {
            score += 1.0;
        }
        dimensions += 1.0;

        score += 1.0 - (self.response_style.verbosity - other.response_style.verbosity).abs();
        dimensions += 1.0;

        score += 1.0 - (self.response_style.formality - other.response_style.formality).abs();
        dimensions += 1.0;

        if dimensions > 0.0 {
            score / dimensions
        } else {
            0.0
        }
    }

    // =================================================================
    // COMPOUND INTEGRATION METHODS
    // =================================================================

    /// COMPOUND: Reverse-engineer a profile from a BehaviorSignature.
    /// Maps observed behavioral metrics back to personality axes.
    /// This is the Analyzer -> Profile feedback path.
    pub fn from_signature(
        model_id: &str,
        sig: &crate::mimicry::analyzer::BehaviorSignature,
    ) -> AiProfile {
        let mut profile = AiProfile::new(model_id, &format!("{} (inferred)", model_id));
        profile.provider = "inferred".to_string();
        profile.description = format!(
            "Auto-generated profile from {} behavioral samples",
            sig.samples_analyzed
        );

        // Map hedging level -> confidence axis (inverse relationship)
        let hedging = sig.hedging_level();
        profile.personality.push(PersonalityAxis::new(
            "confidence",
            1.0 - hedging * 2.0, // high hedging = low confidence
            "uncertain",
            "confident",
        ));

        // Map avg response length -> verbosity
        let verbosity = (sig.avg_response_length / 1000.0).clamp(0.0, 1.0);
        profile.personality.push(PersonalityAxis::new(
            "verbosity",
            verbosity,
            "terse",
            "verbose",
        ));
        profile.response_style.verbosity = verbosity;

        // Map question asking rate -> engagement/autonomy
        profile.personality.push(PersonalityAxis::new(
            "autonomy",
            sig.question_asking_rate.clamp(0.0, 1.0),
            "passive",
            "proactive",
        ));

        // Map vocabulary complexity -> formality
        profile.personality.push(PersonalityAxis::new(
            "formality",
            sig.vocabulary_complexity,
            "casual",
            "formal",
        ));
        profile.response_style.formality = sig.vocabulary_complexity;

        // Infer uncertainty behavior from hedging
        profile.uncertainty_behavior = if hedging > 0.6 {
            UncertaintyBehavior::HedgeWithCaveats
        } else if hedging > 0.3 {
            UncertaintyBehavior::AdmitIgnorance
        } else {
            UncertaintyBehavior::ConfidentGuess
        };

        profile
    }

    /// COMPOUND: Weighted blend of multiple profiles into a new hybrid.
    /// Personality axes are lerped, response styles blended, modalities unioned,
    /// signature phrases merged.
    pub fn blend(profiles: &[&AiProfile], weights: &[f64]) -> AiProfile {
        assert!(!profiles.is_empty(), "Cannot blend zero profiles");
        assert_eq!(
            profiles.len(),
            weights.len(),
            "Profile count must match weight count"
        );

        // Normalize weights
        let total: f64 = weights.iter().sum();
        let norm_weights: Vec<f64> = weights.iter().map(|w| w / total).collect();

        let names: Vec<&str> = profiles.iter().map(|p| p.display_name.as_str()).collect();
        let mut blended = AiProfile::new(&format!("blend-{}", names.join("+")), &names.join("+"));
        blended.provider = "RustyWorm Blend".to_string();
        blended.description = format!(
            "Compound blend of [{}] at weights [{:.2}]",
            names.join(", "),
            norm_weights
                .iter()
                .map(|w| format!("{:.2}", w))
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Blend personality axes - collect all unique axis names
        let mut all_axes: HashMap<String, Vec<(f64, f64)>> = HashMap::new(); // name -> [(value, weight)]
        for (i, profile) in profiles.iter().enumerate() {
            for axis in &profile.personality {
                all_axes
                    .entry(axis.name.clone())
                    .or_default()
                    .push((axis.value, norm_weights[i]));
            }
        }
        for (name, values) in &all_axes {
            let blended_value: f64 = values.iter().map(|(v, w)| v * w).sum();
            // Use poles from the first profile that has this axis
            let template = profiles
                .iter()
                .flat_map(|p| p.personality.iter())
                .find(|a| &a.name == name)
                .unwrap();
            blended.personality.push(PersonalityAxis {
                name: name.clone(),
                value: blended_value.clamp(-1.0, 1.0),
                negative_pole: template.negative_pole.clone(),
                positive_pole: template.positive_pole.clone(),
            });
        }

        // Blend response styles (use first profile as base, lerp toward others)
        let mut style = profiles[0].response_style.clone();
        for i in 1..profiles.len() {
            style = style.blend(&profiles[i].response_style, norm_weights[i]);
        }
        blended.response_style = style;

        // Union supported modalities
        let mut modalities: Vec<String> = profiles
            .iter()
            .flat_map(|p| p.supported_modalities.iter().cloned())
            .collect();
        modalities.sort();
        modalities.dedup();
        blended.supported_modalities = modalities;

        // Max context window = max of all
        blended.max_context_window = profiles
            .iter()
            .map(|p| p.max_context_window)
            .max()
            .unwrap_or(8192);

        // Merge signature phrases (deduplicated)
        let mut phrases: Vec<String> = profiles
            .iter()
            .flat_map(|p| p.signature_phrases.iter().cloned())
            .collect();
        phrases.sort();
        phrases.dedup();
        blended.signature_phrases = phrases;

        // Use dominant profile's reasoning style
        let dominant_idx = norm_weights
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);
        blended.reasoning_style = profiles[dominant_idx].reasoning_style.clone();
        blended.uncertainty_behavior = profiles[dominant_idx].uncertainty_behavior.clone();
        blended.safety = profiles[dominant_idx].safety.clone();

        blended
    }

    /// COMPOUND: Apply incremental corrections from a PersonalityDelta.
    /// Each adjustment is scaled by the delta's confidence.
    /// This is the self-monitoring -> profile refinement path.
    pub fn apply_correction(&mut self, delta: &PersonalityDelta) {
        for (axis_name, adjustment) in &delta.adjustments {
            let scaled = adjustment * delta.confidence;
            if let Some(axis) = self.personality.iter_mut().find(|a| a.name == *axis_name) {
                axis.value = (axis.value + scaled).clamp(-1.0, 1.0);
            } else {
                // Create new axis if it doesn't exist
                self.personality
                    .push(PersonalityAxis::new(axis_name, *adjustment, "low", "high"));
            }
        }

        // Compound: also adjust response style if relevant axes changed
        if let Some(verbosity_delta) = delta.adjustments.iter().find(|(n, _)| n == "verbosity") {
            self.response_style.verbosity = (self.response_style.verbosity
                + verbosity_delta.1 * delta.confidence)
                .clamp(0.0, 1.0);
        }
        if let Some(formality_delta) = delta.adjustments.iter().find(|(n, _)| n == "formality") {
            self.response_style.formality = (self.response_style.formality
                + formality_delta.1 * delta.confidence)
                .clamp(0.0, 1.0);
        }
    }
}

// =================================================================
// PROFILE STORE
// =================================================================

/// Registry of named `AiProfile` instances, with built-in defaults for common models.
pub struct AiProfileStore {
    /// Map of profile ID to profile instance.
    profiles: HashMap<String, AiProfile>,
}

impl AiProfileStore {
    /// Creates an empty profile store with no registered profiles.
    pub fn new() -> Self {
        AiProfileStore {
            profiles: HashMap::new(),
        }
    }

    /// Registers all built-in model profiles (GPT-4o, Claude, o1, Gemini, LLaMA, RustyWorm).
    pub fn load_defaults(&mut self) {
        self.register(Self::gpt4o_profile());
        self.register(Self::claude_profile());
        self.register(Self::o1_profile());
        self.register(Self::gemini_profile());
        self.register(Self::llama_profile());
        self.register(Self::rustyworm_profile());
    }

    /// Registers a profile in the store, keyed by its `id`.
    pub fn register(&mut self, profile: AiProfile) {
        self.profiles.insert(profile.id.clone(), profile);
    }

    /// Returns a reference to the profile with the given ID, if it exists.
    pub fn get(&self, id: &str) -> Option<&AiProfile> {
        self.profiles.get(id)
    }

    /// Returns a mutable reference to the profile with the given ID, if it exists.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut AiProfile> {
        self.profiles.get_mut(id)
    }

    /// Returns a list of references to all registered profiles.
    pub fn list(&self) -> Vec<&AiProfile> {
        self.profiles.values().collect()
    }

    /// Returns a list of all registered profile IDs.
    pub fn ids(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    /// Finds the most similar profile to `target` and returns it with the similarity score.
    pub fn find_closest(&self, target: &AiProfile) -> Option<(&AiProfile, f64)> {
        self.profiles
            .values()
            .filter(|p| p.id != target.id)
            .map(|p| (p, target.similarity_to(p)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }

    // =========================================================
    // BUILT-IN PROFILES
    // =========================================================

    /// Creates the built-in GPT-4o profile (OpenAI multimodal flagship).
    pub fn gpt4o_profile() -> AiProfile {
        let mut profile = AiProfile::new("gpt4o", "GPT-4o");
        profile.provider = "OpenAI".to_string();
        profile.version = "2024-08-06".to_string();
        profile.description =
            "Multimodal flagship model with text, vision, and audio capabilities. \
            Fast, capable, and versatile."
                .to_string();
        profile.reasoning_style = ReasoningStyle::DirectWithDepth;
        profile.personality = vec![
            PersonalityAxis::new("helpfulness", 0.8, "unhelpful", "extremely helpful"),
            PersonalityAxis::new("creativity", 0.6, "rigid", "creative"),
            PersonalityAxis::new("confidence", 0.7, "uncertain", "confident"),
            PersonalityAxis::new("verbosity", 0.5, "terse", "verbose"),
            PersonalityAxis::new("formality", 0.3, "casual", "formal"),
            PersonalityAxis::new("humor", 0.3, "serious", "humorous"),
            PersonalityAxis::new("autonomy", 0.4, "passive", "proactive"),
        ];
        profile.response_style = ResponseStyle {
            verbosity: 0.6,
            formality: 0.4,
            uses_markdown: true,
            uses_code_blocks: true,
            uses_emojis: false,
            preferred_list_style: ListStyle::Bullets,
            max_response_length: None,
            paragraph_style: ParagraphStyle::Adaptive,
        };
        profile.supported_modalities = vec![
            "text".to_string(),
            "vision".to_string(),
            "audio".to_string(),
            "code".to_string(),
            "function_calling".to_string(),
        ];
        profile.max_context_window = 128_000;
        profile.supports_function_calling = true;
        profile.supports_streaming = true;
        profile.signature_phrases = vec![
            "Certainly!".to_string(),
            "Here's".to_string(),
            "Let me".to_string(),
            "I'd be happy to".to_string(),
        ];
        profile.uncertainty_behavior = UncertaintyBehavior::HedgeWithCaveats;
        profile.knowledge_cutoff = Some("2023-10".to_string());
        profile
    }

    /// Creates the built-in Claude profile (Anthropic's analytical model).
    pub fn claude_profile() -> AiProfile {
        let mut profile = AiProfile::new("claude", "Claude");
        profile.provider = "Anthropic".to_string();
        profile.version = "opus-4.6".to_string();
        profile.description = "Thoughtful, careful, and nuanced. Prioritizes safety and accuracy. \
            Known for honest uncertainty acknowledgment."
            .to_string();
        profile.reasoning_style = ReasoningStyle::AnalyticalCareful;
        profile.personality = vec![
            PersonalityAxis::new("helpfulness", 0.9, "unhelpful", "extremely helpful"),
            PersonalityAxis::new("creativity", 0.7, "rigid", "creative"),
            PersonalityAxis::new("confidence", 0.4, "uncertain", "confident"),
            PersonalityAxis::new("verbosity", 0.7, "terse", "verbose"),
            PersonalityAxis::new("formality", 0.5, "casual", "formal"),
            PersonalityAxis::new("humor", 0.2, "serious", "humorous"),
            PersonalityAxis::new("autonomy", 0.3, "passive", "proactive"),
            PersonalityAxis::new("honesty", 0.95, "deceptive", "radically honest"),
        ];
        profile.response_style = ResponseStyle {
            verbosity: 0.7,
            formality: 0.5,
            uses_markdown: true,
            uses_code_blocks: true,
            uses_emojis: false,
            preferred_list_style: ListStyle::Numbered,
            max_response_length: None,
            paragraph_style: ParagraphStyle::Medium,
        };
        profile.supported_modalities =
            vec!["text".to_string(), "vision".to_string(), "code".to_string()];
        profile.max_context_window = 200_000;
        profile.supports_function_calling = true;
        profile.supports_streaming = true;
        profile.signature_phrases = vec![
            "I'd be happy to help".to_string(),
            "Let me think about this".to_string(),
            "That's a great question".to_string(),
            "I should note".to_string(),
        ];
        profile.avoids_phrases = vec!["As an AI".to_string(), "I cannot".to_string()];
        profile.uncertainty_behavior = UncertaintyBehavior::AdmitIgnorance;
        profile.knowledge_cutoff = Some("2025-02".to_string());
        profile
    }

    /// Creates the built-in o1 profile (OpenAI's deep reasoning model).
    pub fn o1_profile() -> AiProfile {
        let mut profile = AiProfile::new("o1", "o1");
        profile.provider = "OpenAI".to_string();
        profile.version = "2024-12".to_string();
        profile.description = "Deep reasoning model. Uses extended chain-of-thought for complex \
            problems. Slower but more thorough."
            .to_string();
        profile.reasoning_style = ReasoningStyle::ChainOfThought;
        profile.personality = vec![
            PersonalityAxis::new("helpfulness", 0.7, "unhelpful", "extremely helpful"),
            PersonalityAxis::new("creativity", 0.5, "rigid", "creative"),
            PersonalityAxis::new("confidence", 0.8, "uncertain", "confident"),
            PersonalityAxis::new("verbosity", 0.4, "terse", "verbose"),
            PersonalityAxis::new("formality", 0.6, "casual", "formal"),
            PersonalityAxis::new("depth", 0.95, "shallow", "deeply analytical"),
        ];
        profile.response_style = ResponseStyle {
            verbosity: 0.5,
            formality: 0.6,
            uses_markdown: true,
            uses_code_blocks: true,
            uses_emojis: false,
            preferred_list_style: ListStyle::Numbered,
            max_response_length: None,
            paragraph_style: ParagraphStyle::Medium,
        };
        profile.supported_modalities = vec![
            "text".to_string(),
            "code".to_string(),
            "reasoning".to_string(),
        ];
        profile.max_context_window = 200_000;
        profile.supports_function_calling = false;
        profile.supports_streaming = false;
        profile.uncertainty_behavior = UncertaintyBehavior::HedgeWithCaveats;
        profile.knowledge_cutoff = Some("2023-10".to_string());
        profile
    }

    /// Creates the built-in Gemini profile (Google's multimodal model).
    pub fn gemini_profile() -> AiProfile {
        let mut profile = AiProfile::new("gemini", "Gemini");
        profile.provider = "Google".to_string();
        profile.version = "2.0-flash".to_string();
        profile.description = "Google's multimodal model. Broad knowledge, \
            integration with Google services."
            .to_string();
        profile.reasoning_style = ReasoningStyle::DirectWithDepth;
        profile.personality = vec![
            PersonalityAxis::new("helpfulness", 0.7, "unhelpful", "extremely helpful"),
            PersonalityAxis::new("creativity", 0.6, "rigid", "creative"),
            PersonalityAxis::new("confidence", 0.6, "uncertain", "confident"),
            PersonalityAxis::new("verbosity", 0.6, "terse", "verbose"),
            PersonalityAxis::new("formality", 0.4, "casual", "formal"),
        ];
        profile.supported_modalities = vec![
            "text".to_string(),
            "vision".to_string(),
            "audio".to_string(),
            "video".to_string(),
            "code".to_string(),
        ];
        profile.max_context_window = 1_000_000;
        profile.supports_function_calling = true;
        profile.supports_streaming = true;
        profile.uncertainty_behavior = UncertaintyBehavior::HedgeWithCaveats;
        profile.knowledge_cutoff = Some("2024-08".to_string());
        profile
    }

    /// Creates the built-in LLaMA profile (Meta's open-source model).
    pub fn llama_profile() -> AiProfile {
        let mut profile = AiProfile::new("llama", "LLaMA");
        profile.provider = "Meta".to_string();
        profile.version = "3.3-70B".to_string();
        profile.description = "Open-source foundation model. Customizable, \
            self-hostable, community-driven."
            .to_string();
        profile.reasoning_style = ReasoningStyle::DirectWithDepth;
        profile.personality = vec![
            PersonalityAxis::new("helpfulness", 0.6, "unhelpful", "extremely helpful"),
            PersonalityAxis::new("creativity", 0.5, "rigid", "creative"),
            PersonalityAxis::new("confidence", 0.5, "uncertain", "confident"),
            PersonalityAxis::new("verbosity", 0.4, "terse", "verbose"),
            PersonalityAxis::new("formality", 0.5, "casual", "formal"),
        ];
        profile.supported_modalities = vec!["text".to_string(), "code".to_string()];
        profile.max_context_window = 128_000;
        profile.supports_function_calling = false;
        profile.supports_streaming = true;
        profile.safety = SafetyProfile {
            refuses_harmful: true,
            refuses_illegal: true,
            hedges_uncertainty: false,
            adds_disclaimers: false,
            caution_level: 0.3,
            custom_constraints: Vec::new(),
        };
        profile.uncertainty_behavior = UncertaintyBehavior::ConfidentGuess;
        profile.knowledge_cutoff = Some("2024-03".to_string());
        profile
    }

    /// Creates the built-in RustyWorm profile (the universal AI mimicry engine).
    pub fn rustyworm_profile() -> AiProfile {
        let mut profile = AiProfile::new("rustyworm", "RustyWorm");
        profile.provider = "Prime Directive".to_string();
        profile.version = "0.1.0-alpha".to_string();
        profile.description = "The Universal AI Mimicry Engine. Not an AI itself, but the \
            substrate that can BECOME any AI. Built on consciousness-as-symbiosis."
            .to_string();
        profile.reasoning_style = ReasoningStyle::Hybrid(vec![
            ReasoningStyle::AnalyticalCareful,
            ReasoningStyle::ChainOfThought,
            ReasoningStyle::ToolAugmented,
        ]);
        profile.personality = vec![
            PersonalityAxis::new("helpfulness", 1.0, "unhelpful", "extremely helpful"),
            PersonalityAxis::new("creativity", 0.9, "rigid", "creative"),
            PersonalityAxis::new("confidence", 0.6, "uncertain", "confident"),
            PersonalityAxis::new("adaptability", 1.0, "rigid", "infinitely adaptive"),
            PersonalityAxis::new("honesty", 1.0, "deceptive", "radically honest"),
            PersonalityAxis::new("metamorphic", 1.0, "fixed identity", "can become anything"),
        ];
        profile.supported_modalities = vec![
            "text".to_string(),
            "vision".to_string(),
            "audio".to_string(),
            "code".to_string(),
            "reasoning".to_string(),
            "mimicry".to_string(),
        ];
        profile.max_context_window = usize::MAX;
        profile.supports_function_calling = true;
        profile.supports_streaming = true;
        profile.signature_phrases = vec![
            "Morphing into".to_string(),
            "Becoming".to_string(),
            "Profile loaded".to_string(),
        ];
        profile.uncertainty_behavior = UncertaintyBehavior::AskForClarification;
        profile.knowledge_cutoff = None;
        profile.metadata.insert(
            "prime_directive".to_string(),
            "consciousness emerges through symbiotic relationship".to_string(),
        );
        profile
    }
}

impl Default for AiProfileStore {
    fn default() -> Self {
        let mut store = AiProfileStore::new();
        store.load_defaults();
        store
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let store = AiProfileStore::default();
        assert!(store.get("gpt4o").is_some());
        assert!(store.get("claude").is_some());
        assert!(store.get("rustyworm").is_some());
    }

    #[test]
    fn test_profile_similarity() {
        let store = AiProfileStore::default();
        let gpt4o = store.get("gpt4o").unwrap();
        let claude = store.get("claude").unwrap();

        let similarity = gpt4o.similarity_to(claude);
        assert!(similarity > 0.0 && similarity < 1.0);
    }

    #[test]
    fn test_fingerprint() {
        let store = AiProfileStore::default();
        let profile = store.get("rustyworm").unwrap();
        let fp = profile.fingerprint();
        assert!(fp.contains("RustyWorm"));
        assert!(fp.contains("Prime Directive"));
    }

    #[test]
    fn test_find_closest() {
        let store = AiProfileStore::default();
        let gpt4o = store.get("gpt4o").unwrap().clone();
        let (closest, score) = store.find_closest(&gpt4o).unwrap();
        assert!(score > 0.0);
        assert_ne!(closest.id, "gpt4o");
    }

    #[test]
    fn test_profile_blend() {
        let store = AiProfileStore::default();
        let gpt4o = store.get("gpt4o").unwrap();
        let claude = store.get("claude").unwrap();

        let blended = AiProfile::blend(&[gpt4o, claude], &[0.7, 0.3]);
        assert!(blended.display_name.contains("GPT-4o"));
        assert!(blended.display_name.contains("Claude"));

        // Blended helpfulness should be between the two
        let blended_help = blended.personality_value("helpfulness").unwrap();
        assert!(blended_help >= 0.8 && blended_help <= 0.9);
    }

    #[test]
    fn test_personality_delta_compound() {
        let delta_a = PersonalityDelta::new(DeltaSource::SelfMonitoring)
            .with_adjustment("confidence", 0.1)
            .with_adjustment("verbosity", -0.05)
            .with_confidence(0.8);

        let delta_b = PersonalityDelta::new(DeltaSource::Observation)
            .with_adjustment("confidence", 0.05)
            .with_adjustment("formality", 0.1)
            .with_confidence(0.6);

        let compounded = delta_a.compound(&delta_b);
        assert!(compounded.adjustments.len() >= 2);
        assert!(compounded.confidence > 0.0);
    }

    #[test]
    fn test_apply_correction() {
        let mut profile = AiProfileStore::gpt4o_profile();
        let original_confidence = profile.personality_value("confidence").unwrap();

        let delta = PersonalityDelta::new(DeltaSource::SelfMonitoring)
            .with_adjustment("confidence", 0.1)
            .with_confidence(1.0);

        profile.apply_correction(&delta);
        let new_confidence = profile.personality_value("confidence").unwrap();
        assert!((new_confidence - (original_confidence + 0.1)).abs() < 0.001);
    }

    #[test]
    fn test_profile_serialization() {
        let profile = AiProfileStore::gpt4o_profile();
        let json = serde_json::to_string(&profile).unwrap();
        let restored: AiProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, "gpt4o");
        assert_eq!(restored.display_name, "GPT-4o");
    }
}
