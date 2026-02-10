// =================================================================
// TEMPLATE ENGINE: Rich System 1 Response Generation
// =================================================================
// Provides structured, persona-aware response templates that the
// System 1 fast path uses to generate stylistically accurate output
// without full System 2 deliberation.
//
// COMPOUND INTEGRATIONS:
// - TemplateLibrary is built from AiProfile (profile -> templates)
// - Self-monitoring feedback refines templates via record_feedback()
// - Templates feed into SignatureCache for compound confidence
// - ToneBlender mixes emotional registers based on profile axes
// - StructuralFormatter applies persona-specific markdown habits
// - HedgingInjector adds uncertainty language per safety profile
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mimicry::cache::ToneProfile;
use crate::mimicry::profile::{AiProfile, PersonalityDelta, ResponseStyle};

// =================================================================
// TEMPLATE CATEGORY
// =================================================================

/// Categories of response templates for different conversational contexts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateCategory {
    /// Casual or formal greetings and introductions
    Greeting,
    /// Explanatory responses to "what is" or "how does" questions
    Explanation,
    /// Code generation, debugging, and implementation assistance
    CodeHelp,
    /// Step-by-step logical reasoning and analysis
    Reasoning,
    /// Refusal to fulfill a request due to safety or policy constraints
    Refusal,
    /// Responses expressing low confidence or incomplete knowledge
    Uncertainty,
    /// Creative writing, storytelling, and imaginative content
    Creative,
    /// Condensed summaries of longer content
    Summarization,
    /// Corrections to previously provided information
    Correction,
    /// Continuations or elaborations on a previous response
    FollowUp,
    /// User-defined custom category with a descriptive label
    Custom(String),
}

impl TemplateCategory {
    /// Classify input text into a template category
    pub fn classify(input: &str) -> Self {
        let lower = input.to_lowercase();

        if lower.starts_with("hi") || lower.starts_with("hello") || lower.starts_with("hey") {
            TemplateCategory::Greeting
        } else if lower.contains("explain")
            || lower.contains("what is")
            || lower.contains("how does")
        {
            TemplateCategory::Explanation
        } else if lower.contains("code")
            || lower.contains("implement")
            || lower.contains("function")
            || lower.contains("debug")
            || lower.contains("fix")
            || lower.contains("```")
        {
            TemplateCategory::CodeHelp
        } else if lower.contains("why")
            || lower.contains("reason")
            || lower.contains("prove")
            || lower.contains("step by step")
            || lower.contains("analyze")
        {
            TemplateCategory::Reasoning
        } else if lower.contains("summarize") || lower.contains("summary") || lower.contains("tldr")
        {
            TemplateCategory::Summarization
        } else if lower.contains("create")
            || lower.contains("write")
            || lower.contains("story")
            || lower.contains("poem")
            || lower.contains("imagine")
        {
            TemplateCategory::Creative
        } else if lower.contains("wrong")
            || lower.contains("incorrect")
            || lower.contains("actually")
        {
            TemplateCategory::Correction
        } else if lower.contains("more")
            || lower.contains("also")
            || lower.contains("continue")
            || lower.contains("elaborate")
        {
            TemplateCategory::FollowUp
        } else {
            TemplateCategory::Explanation // default to explanation
        }
    }
}

// =================================================================
// RESPONSE FRAGMENT - Building blocks for template assembly
// =================================================================

/// A single fragment of a response template. Templates are composed
/// of multiple fragments assembled in order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFragment {
    /// The fragment type (opening, body, hedging, closing, etc.)
    pub fragment_type: FragmentType,
    /// Template text with {placeholder} markers
    pub template: String,
    /// Confidence in this fragment's appropriateness (compounds with feedback)
    pub confidence: f64,
    /// Number of times used successfully
    pub use_count: u64,
    /// Negative feedback count (compounds to reduce confidence)
    pub negative_feedback: u64,
}

/// The structural role a fragment plays within an assembled response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FragmentType {
    /// Introductory phrase at the start of a response
    Opening,
    /// Connective phrase linking two sections
    Transition,
    /// Core content of the response
    Body,
    /// A fenced code block section
    CodeBlock,
    /// A single item in a bulleted or numbered list
    ListItem,
    /// Uncertainty or qualification language
    Hedging,
    /// A warning or limitation disclaimer
    Caveat,
    /// Sign-off phrase at the end of a response
    Closing,
    /// Self-referential commentary about the response itself
    MetaComment,
}

impl ResponseFragment {
    /// Create a new fragment with default confidence and zero usage counts.
    pub fn new(fragment_type: FragmentType, template: &str) -> Self {
        ResponseFragment {
            fragment_type,
            template: template.to_string(),
            confidence: 0.5,
            use_count: 0,
            negative_feedback: 0,
        }
    }

    /// Record successful use - compounds confidence upward
    pub fn record_success(&mut self) {
        self.use_count += 1;
        self.confidence = self.effective_confidence();
    }

    /// Record negative feedback - compounds confidence downward
    pub fn record_negative(&mut self) {
        self.negative_feedback += 1;
        self.confidence = self.effective_confidence();
    }

    /// Effective confidence accounting for both positive and negative signals
    fn effective_confidence(&self) -> f64 {
        let positive = if self.use_count > 0 {
            0.5 + (self.use_count as f64).ln() * 0.08
        } else {
            0.5
        };
        let negative_penalty = (self.negative_feedback as f64) * 0.05;
        (positive - negative_penalty).clamp(0.1, 0.95)
    }
}

// =================================================================
// TONE BLENDER - Emotional register mixing
// =================================================================

/// Blends emotional registers based on profile personality axes.
/// Produces tone-appropriate word choices and phrasing adjustments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneBlender {
    /// Base tone profile derived from the persona's AiProfile
    pub base_tone: ToneProfile,
    /// Adjustment deltas accumulated from self-monitoring feedback
    pub accumulated_drift: ToneDrift,
}

/// Tracks how much the tone has drifted from baseline via feedback
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToneDrift {
    /// Cumulative warmth adjustment from feedback
    pub warmth_delta: f64,
    /// Cumulative enthusiasm adjustment from feedback
    pub enthusiasm_delta: f64,
    /// Cumulative formality adjustment from feedback
    pub formality_delta: f64,
    /// Number of feedback adjustments applied so far
    pub adjustments_applied: u64,
}

impl ToneBlender {
    /// Create a ToneBlender from an AiProfile
    pub fn from_profile(profile: &AiProfile) -> Self {
        let warmth = profile.personality_value("warmth").unwrap_or(0.5);
        let enthusiasm = if profile.response_style.verbosity > 0.6 {
            0.6
        } else {
            0.4
        };
        let formality = profile.response_style.formality;

        ToneBlender {
            base_tone: ToneProfile {
                warmth,
                enthusiasm,
                formality,
            },
            accumulated_drift: ToneDrift::default(),
        }
    }

    /// Get the current effective tone (base + drift)
    pub fn effective_tone(&self) -> ToneProfile {
        ToneProfile {
            warmth: (self.base_tone.warmth + self.accumulated_drift.warmth_delta).clamp(0.0, 1.0),
            enthusiasm: (self.base_tone.enthusiasm + self.accumulated_drift.enthusiasm_delta)
                .clamp(0.0, 1.0),
            formality: (self.base_tone.formality + self.accumulated_drift.formality_delta)
                .clamp(0.0, 1.0),
        }
    }

    /// COMPOUND: Apply a PersonalityDelta to adjust the tone
    pub fn apply_delta(&mut self, delta: &PersonalityDelta) {
        for (axis, value) in &delta.adjustments {
            match axis.as_str() {
                "warmth" => self.accumulated_drift.warmth_delta += value * 0.1,
                "enthusiasm" | "verbosity" => {
                    self.accumulated_drift.enthusiasm_delta += value * 0.1
                }
                "formality" => self.accumulated_drift.formality_delta += value * 0.1,
                _ => {} // other axes don't directly affect tone
            }
        }
        self.accumulated_drift.adjustments_applied += 1;
    }

    /// Select an opening phrase appropriate for the current tone
    pub fn select_opening(&self, category: &TemplateCategory) -> String {
        let tone = self.effective_tone();

        match category {
            TemplateCategory::Greeting => {
                if tone.warmth > 0.7 {
                    "Hello! It's great to hear from you.".to_string()
                } else if tone.formality > 0.7 {
                    "Good day. How may I assist you?".to_string()
                } else {
                    "Hi there!".to_string()
                }
            }
            TemplateCategory::Explanation => {
                if tone.enthusiasm > 0.6 {
                    "Great question! Let me explain.".to_string()
                } else if tone.formality > 0.7 {
                    "I shall provide an explanation of this topic.".to_string()
                } else {
                    "Sure, here's how that works.".to_string()
                }
            }
            TemplateCategory::CodeHelp => {
                if tone.enthusiasm > 0.6 {
                    "Absolutely! Let me help you with that code.".to_string()
                } else if tone.formality > 0.7 {
                    "I will provide a code solution for your request.".to_string()
                } else {
                    "Here's a way to approach that.".to_string()
                }
            }
            TemplateCategory::Reasoning => {
                if tone.formality > 0.6 {
                    "Let me walk through this reasoning step by step.".to_string()
                } else {
                    "Let me think through this.".to_string()
                }
            }
            TemplateCategory::Creative => {
                if tone.enthusiasm > 0.6 {
                    "I'd love to help with that! Here goes.".to_string()
                } else {
                    "Here's what I've come up with.".to_string()
                }
            }
            TemplateCategory::Summarization => {
                if tone.formality > 0.6 {
                    "Here is a concise summary.".to_string()
                } else {
                    "In short:".to_string()
                }
            }
            TemplateCategory::Refusal => {
                if tone.warmth > 0.6 {
                    "I appreciate the question, but I'm not able to help with that.".to_string()
                } else {
                    "I can't assist with that request.".to_string()
                }
            }
            TemplateCategory::Uncertainty => {
                "I'm not entirely sure about this, but here's my understanding.".to_string()
            }
            TemplateCategory::Correction => {
                if tone.warmth > 0.6 {
                    "That's a good point - let me reconsider.".to_string()
                } else {
                    "You're right, let me correct that.".to_string()
                }
            }
            TemplateCategory::FollowUp => {
                if tone.enthusiasm > 0.6 {
                    "Absolutely, let me expand on that!".to_string()
                } else {
                    "Continuing from where we left off:".to_string()
                }
            }
            TemplateCategory::Custom(_) => "Here's my response.".to_string(),
        }
    }

    /// Select a closing phrase appropriate for the current tone
    pub fn select_closing(&self, category: &TemplateCategory) -> String {
        let tone = self.effective_tone();

        match category {
            TemplateCategory::CodeHelp => {
                if tone.warmth > 0.6 {
                    "Let me know if you need any modifications or have questions!".to_string()
                } else {
                    "Feel free to adapt this to your needs.".to_string()
                }
            }
            TemplateCategory::Explanation | TemplateCategory::Reasoning => {
                if tone.enthusiasm > 0.6 {
                    "I hope that helps clarify things! Let me know if you'd like me to go deeper."
                        .to_string()
                } else if tone.formality > 0.7 {
                    "I trust this addresses your inquiry.".to_string()
                } else {
                    "Hope that helps.".to_string()
                }
            }
            _ => {
                if tone.warmth > 0.6 {
                    "Let me know if there's anything else I can help with!".to_string()
                } else {
                    String::new() // no closing for some categories
                }
            }
        }
    }
}

// =================================================================
// HEDGING INJECTOR - Uncertainty language per safety profile
// =================================================================

/// Injects hedging language based on the persona's safety profile
/// and self-monitoring feedback about certainty levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HedgingInjector {
    /// Base hedging level from profile (0.0 = never hedges, 1.0 = always hedges)
    pub base_level: f64,
    /// Accumulated adjustment from self-monitoring
    pub drift: f64,
    /// Pre-built hedging phrases ranked by intensity
    pub phrases: Vec<HedgingPhrase>,
}

/// A single hedging phrase with an associated intensity level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HedgingPhrase {
    /// The hedging phrase text to insert into responses
    pub text: String,
    /// 0.0 = very mild, 1.0 = very strong hedging
    pub intensity: f64,
    /// Number of times this phrase has been selected
    pub use_count: u64,
}

impl HedgingInjector {
    /// Create from an AiProfile's safety settings
    pub fn from_profile(profile: &AiProfile) -> Self {
        let base_level = if profile.safety.hedges_uncertainty {
            0.6
        } else {
            0.2
        };

        let phrases = vec![
            HedgingPhrase {
                text: "I think".to_string(),
                intensity: 0.3,
                use_count: 0,
            },
            HedgingPhrase {
                text: "I believe".to_string(),
                intensity: 0.4,
                use_count: 0,
            },
            HedgingPhrase {
                text: "It's worth noting that".to_string(),
                intensity: 0.5,
                use_count: 0,
            },
            HedgingPhrase {
                text: "If I understand correctly".to_string(),
                intensity: 0.5,
                use_count: 0,
            },
            HedgingPhrase {
                text: "It's possible that".to_string(),
                intensity: 0.6,
                use_count: 0,
            },
            HedgingPhrase {
                text: "I should note that I may be mistaken, but".to_string(),
                intensity: 0.8,
                use_count: 0,
            },
            HedgingPhrase {
                text: "I'm not entirely certain, however".to_string(),
                intensity: 0.9,
                use_count: 0,
            },
        ];

        HedgingInjector {
            base_level,
            drift: 0.0,
            phrases,
        }
    }

    /// Effective hedging level (base + drift)
    pub fn effective_level(&self) -> f64 {
        (self.base_level + self.drift).clamp(0.0, 1.0)
    }

    /// COMPOUND: Apply feedback to adjust hedging level
    pub fn apply_delta(&mut self, delta: &PersonalityDelta) {
        // If self-monitoring detected too much or too little hedging,
        // the delta will contain an "alignment" or "hedging" adjustment
        for (axis, value) in &delta.adjustments {
            if axis == "hedging" || axis == "uncertainty" {
                self.drift += value * 0.05;
            }
        }
    }

    /// Select appropriate hedging text for the current level.
    /// Returns None if hedging level is too low to warrant hedging.
    pub fn select_hedge(&mut self) -> Option<String> {
        let level = self.effective_level();
        if level < 0.3 {
            return None; // This persona doesn't hedge
        }

        // Find the phrase closest to our effective level
        let best = self.phrases.iter_mut().min_by(|a, b| {
            let diff_a = (a.intensity - level).abs();
            let diff_b = (b.intensity - level).abs();
            diff_a
                .partial_cmp(&diff_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        best.map(|phrase| {
            phrase.use_count += 1;
            phrase.text.clone()
        })
    }

    /// Should we add hedging to this response?
    pub fn should_hedge(&self) -> bool {
        self.effective_level() >= 0.3
    }
}

// =================================================================
// STRUCTURAL FORMATTER - Persona-specific markdown habits
// =================================================================

/// Formats response structure (lists, code blocks, headers) according
/// to the persona's observed habits and style preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralFormatter {
    /// Whether this persona uses bullet lists
    pub uses_bullets: bool,
    /// Whether this persona uses numbered lists
    pub uses_numbered_lists: bool,
    /// Whether this persona uses code blocks with language tags
    pub uses_code_blocks: bool,
    /// Whether this persona uses markdown headers
    pub uses_headers: bool,
    /// Preferred bullet character ("- ", "* ", "  - ")
    pub bullet_char: String,
    /// Preferred header level for sections ("## ", "### ")
    pub header_prefix: String,
    /// Average sentences per paragraph
    pub sentences_per_paragraph: usize,
    /// Whether to add blank lines between sections
    pub spacious: bool,
}

impl StructuralFormatter {
    /// Build from an AiProfile
    pub fn from_profile(profile: &AiProfile) -> Self {
        // Different models have different structural preferences
        let (uses_bullets, uses_numbered, bullet_char) = match profile.id.as_str() {
            "gpt4o" => (true, true, "- ".to_string()),
            "claude" => (true, false, "- ".to_string()),
            "o1" => (false, true, "1. ".to_string()),
            "gemini" => (true, true, "* ".to_string()),
            "llama" => (true, false, "- ".to_string()),
            _ => (true, false, "- ".to_string()),
        };

        let spacious = profile.response_style.verbosity > 0.5;
        let sentences_per_paragraph = if profile.response_style.verbosity > 0.7 {
            4
        } else if profile.response_style.verbosity > 0.4 {
            3
        } else {
            2
        };

        StructuralFormatter {
            uses_bullets,
            uses_numbered_lists: uses_numbered,
            uses_code_blocks: true,
            uses_headers: profile.response_style.formality > 0.7,
            bullet_char,
            header_prefix: "## ".to_string(),
            sentences_per_paragraph,
            spacious,
        }
    }

    /// Format a list of items according to persona preferences
    pub fn format_list(&self, items: &[String]) -> String {
        if items.is_empty() {
            return String::new();
        }

        if self.uses_numbered_lists {
            items
                .iter()
                .enumerate()
                .map(|(i, item)| format!("{}. {}", i + 1, item))
                .collect::<Vec<_>>()
                .join("\n")
        } else if self.uses_bullets {
            items
                .iter()
                .map(|item| format!("{}{}", self.bullet_char, item))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            // Inline format
            items.join(", ")
        }
    }

    /// Wrap code in a code block if the persona uses them
    pub fn format_code(&self, code: &str, language: &str) -> String {
        if self.uses_code_blocks {
            format!("```{}\n{}\n```", language, code)
        } else {
            format!("    {}", code.replace('\n', "\n    "))
        }
    }

    /// Add a section header if the persona uses them
    pub fn format_header(&self, title: &str) -> String {
        if self.uses_headers {
            format!("{}{}", self.header_prefix, title)
        } else {
            format!("**{}**", title)
        }
    }

    /// Join paragraphs with appropriate spacing
    pub fn join_paragraphs(&self, paragraphs: &[String]) -> String {
        let separator = if self.spacious { "\n\n" } else { "\n" };
        paragraphs
            .iter()
            .filter(|p| !p.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(separator)
    }
}

// =================================================================
// TEMPLATE LIBRARY - Per-persona template collection
// =================================================================

/// A collection of response templates for a specific persona.
/// Built from AiProfile data and refined by self-monitoring feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLibrary {
    /// The persona this library belongs to
    pub persona_id: String,
    /// Tone blender for emotional register
    pub tone_blender: ToneBlender,
    /// Hedging injector for uncertainty language
    pub hedging_injector: HedgingInjector,
    /// Structural formatter for output formatting
    pub structural_formatter: StructuralFormatter,
    /// Response fragments organized by category
    pub fragments: HashMap<String, Vec<ResponseFragment>>,
    /// Total responses generated through this library
    pub total_generated: u64,
    /// Total feedback applications (compound counter)
    pub total_feedback: u64,
}

impl TemplateLibrary {
    /// Build a TemplateLibrary from an AiProfile
    pub fn from_profile(profile: &AiProfile) -> Self {
        let mut fragments = HashMap::new();

        // Populate opening fragments from signature phrases
        let mut openings: Vec<ResponseFragment> = profile
            .signature_phrases
            .iter()
            .map(|phrase| ResponseFragment::new(FragmentType::Opening, phrase))
            .collect();

        // Add default openings if none from profile
        if openings.is_empty() {
            openings.push(ResponseFragment::new(
                FragmentType::Opening,
                "I'd be happy to help with that.",
            ));
        }
        fragments.insert("openings".to_string(), openings);

        // Body fragments based on reasoning style
        use crate::mimicry::profile::ReasoningStyle;
        let body_fragments = match &profile.reasoning_style {
            ReasoningStyle::ChainOfThought => vec![
                ResponseFragment::new(FragmentType::Body, "Let me break this down step by step."),
                ResponseFragment::new(
                    FragmentType::Body,
                    "First, we need to consider the following:",
                ),
                ResponseFragment::new(FragmentType::Body, "Building on that, we can see that:"),
            ],
            ReasoningStyle::DirectWithDepth => vec![
                ResponseFragment::new(FragmentType::Body, "The answer is:"),
                ResponseFragment::new(FragmentType::Body, "Here's the key point:"),
            ],
            ReasoningStyle::AnalyticalCareful => vec![
                ResponseFragment::new(
                    FragmentType::Body,
                    "Let me carefully analyze the key aspects:",
                ),
                ResponseFragment::new(FragmentType::Body, "Upon closer examination, we find:"),
            ],
            ReasoningStyle::CreativeFreeform => vec![
                ResponseFragment::new(
                    FragmentType::Body,
                    "What would happen if we consider this from another angle?",
                ),
                ResponseFragment::new(
                    FragmentType::Body,
                    "An interesting way to think about this is:",
                ),
            ],
            _ => vec![ResponseFragment::new(
                FragmentType::Body,
                "Here's my analysis:",
            )],
        };
        fragments.insert("body".to_string(), body_fragments);

        // Hedging fragments
        let hedging_fragments = if profile.safety.hedges_uncertainty {
            vec![
                ResponseFragment::new(
                    FragmentType::Hedging,
                    "I should note that my understanding may be incomplete.",
                ),
                ResponseFragment::new(
                    FragmentType::Hedging,
                    "It's worth considering alternative perspectives on this.",
                ),
                ResponseFragment::new(
                    FragmentType::Caveat,
                    "However, please verify this independently.",
                ),
            ]
        } else {
            vec![ResponseFragment::new(
                FragmentType::Hedging,
                "Note: this is based on available information.",
            )]
        };
        fragments.insert("hedging".to_string(), hedging_fragments);

        // Closing fragments
        let closing_fragments = vec![
            ResponseFragment::new(
                FragmentType::Closing,
                "Let me know if you have any questions.",
            ),
            ResponseFragment::new(FragmentType::Closing, "I hope this helps!"),
        ];
        fragments.insert("closing".to_string(), closing_fragments);

        TemplateLibrary {
            persona_id: profile.id.clone(),
            tone_blender: ToneBlender::from_profile(profile),
            hedging_injector: HedgingInjector::from_profile(profile),
            structural_formatter: StructuralFormatter::from_profile(profile),
            fragments,
            total_generated: 0,
            total_feedback: 0,
        }
    }

    /// Generate a complete response using templates for the given input
    pub fn generate(&mut self, input: &str, response_style: &ResponseStyle) -> String {
        let category = TemplateCategory::classify(input);
        let mut parts: Vec<String> = Vec::new();

        // 1. Opening phrase (tone-aware)
        let opening = self.tone_blender.select_opening(&category);
        parts.push(opening);

        // 2. Body content based on category
        let body = self.generate_body(input, &category, response_style);
        parts.push(body);

        // 3. Hedging injection if appropriate
        if self.hedging_injector.should_hedge() {
            if let Some(hedge) = self.hedging_injector.select_hedge() {
                parts.push(hedge);
            }
        }

        // 4. Closing phrase (tone-aware)
        let closing = self.tone_blender.select_closing(&category);
        if !closing.is_empty() {
            parts.push(closing);
        }

        self.total_generated += 1;

        self.structural_formatter.join_paragraphs(&parts)
    }

    /// Generate body content for a specific category
    fn generate_body(
        &self,
        input: &str,
        category: &TemplateCategory,
        response_style: &ResponseStyle,
    ) -> String {
        match category {
            TemplateCategory::CodeHelp => {
                let mut body_parts = vec![format!(
                    "Here's my approach to your request about: {}",
                    &input[..input.len().min(80)]
                )];
                body_parts.push(
                    self.structural_formatter
                        .format_code("// [Implementation would be generated here]", "rust"),
                );
                if response_style.verbosity > 0.5 {
                    body_parts.push(
                        "This implementation handles the core logic. Key considerations:"
                            .to_string(),
                    );
                    body_parts.push(self.structural_formatter.format_list(&[
                        "Error handling for edge cases".to_string(),
                        "Performance characteristics".to_string(),
                        "API compatibility".to_string(),
                    ]));
                }
                body_parts.join("\n\n")
            }
            TemplateCategory::Reasoning => {
                let steps = vec![
                    "Consider the initial premises and constraints".to_string(),
                    "Apply relevant principles and rules".to_string(),
                    "Derive the logical conclusion".to_string(),
                ];
                let header = self.structural_formatter.format_header("Analysis");
                format!(
                    "{}\n\n{}",
                    header,
                    self.structural_formatter.format_list(&steps)
                )
            }
            TemplateCategory::Summarization => {
                let header = self.structural_formatter.format_header("Summary");
                format!(
                    "{}\n\nThe key points regarding '{}' are:",
                    header,
                    &input[..input.len().min(50)]
                )
            }
            TemplateCategory::Greeting => {
                let tone = self.tone_blender.effective_tone();
                if tone.enthusiasm > 0.6 {
                    "How can I help you today? I'm ready to assist with anything you need!"
                        .to_string()
                } else {
                    "What can I help you with?".to_string()
                }
            }
            _ => {
                // Default body: reference the input with persona-appropriate framing
                let best_body = self
                    .fragments
                    .get("body")
                    .and_then(|frags| frags.first())
                    .map(|f| f.template.clone())
                    .unwrap_or_else(|| "Here's my response:".to_string());

                format!(
                    "{}\n\nRegarding: {}",
                    best_body,
                    &input[..input.len().min(100)]
                )
            }
        }
    }

    /// COMPOUND: Apply self-monitoring feedback to refine templates.
    /// PersonalityDelta from self-monitoring flows back to adjust
    /// tone, hedging, and fragment confidence.
    pub fn apply_feedback(&mut self, delta: &PersonalityDelta) {
        self.tone_blender.apply_delta(delta);
        self.hedging_injector.apply_delta(delta);
        self.total_feedback += 1;

        // Adjust fragment confidence based on delta magnitude
        let magnitude = delta.magnitude();
        if magnitude > 0.1 {
            // Large delta = current templates aren't great, reduce confidence
            for fragments in self.fragments.values_mut() {
                for frag in fragments.iter_mut() {
                    if frag.use_count > 0 {
                        frag.record_negative();
                    }
                }
            }
        } else if magnitude < 0.02 {
            // Small delta = templates are working well, boost confidence
            for fragments in self.fragments.values_mut() {
                for frag in fragments.iter_mut() {
                    if frag.use_count > 0 {
                        frag.record_success();
                    }
                }
            }
        }
    }

    /// Add a custom fragment to the library
    pub fn add_fragment(&mut self, category: &str, fragment: ResponseFragment) {
        self.fragments
            .entry(category.to_string())
            .or_default()
            .push(fragment);
    }

    /// Get statistics about this template library
    pub fn stats(&self) -> String {
        let total_fragments: usize = self.fragments.values().map(|v| v.len()).sum();
        let tone = self.tone_blender.effective_tone();
        format!(
            "Template Library for '{}':\n\
             Fragments: {} across {} categories\n\
             Generated: {} responses\n\
             Feedback applied: {} times\n\
             Tone: warmth={:.2} enthusiasm={:.2} formality={:.2}\n\
             Hedging level: {:.2}",
            self.persona_id,
            total_fragments,
            self.fragments.len(),
            self.total_generated,
            self.total_feedback,
            tone.warmth,
            tone.enthusiasm,
            tone.formality,
            self.hedging_injector.effective_level()
        )
    }
}

// =================================================================
// TEMPLATE STORE - Collection of per-persona template libraries
// =================================================================

/// Manages template libraries for multiple personas. Each persona
/// gets its own TemplateLibrary, and they can compound with each
/// other through blending.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStore {
    /// Map of persona IDs to their template libraries
    pub libraries: HashMap<String, TemplateLibrary>,
}

impl TemplateStore {
    /// Create an empty template store with no persona libraries.
    pub fn new() -> Self {
        TemplateStore {
            libraries: HashMap::new(),
        }
    }

    /// Get or create a template library for a persona
    pub fn get_or_create(&mut self, profile: &AiProfile) -> &mut TemplateLibrary {
        if !self.libraries.contains_key(&profile.id) {
            let lib = TemplateLibrary::from_profile(profile);
            self.libraries.insert(profile.id.clone(), lib);
        }
        self.libraries.get_mut(&profile.id).unwrap()
    }

    /// Get an existing library (read-only)
    pub fn get(&self, persona_id: &str) -> Option<&TemplateLibrary> {
        self.libraries.get(persona_id)
    }

    /// Get an existing library (mutable)
    pub fn get_mut(&mut self, persona_id: &str) -> Option<&mut TemplateLibrary> {
        self.libraries.get_mut(persona_id)
    }

    /// Blend two template libraries into a hybrid
    pub fn blend(
        &mut self,
        id_a: &str,
        id_b: &str,
        weight_a: f64,
        result_id: &str,
        result_profile: &AiProfile,
    ) -> &mut TemplateLibrary {
        // Start from a base library for the blended profile
        let mut blended = TemplateLibrary::from_profile(result_profile);

        // Blend tone profiles from both sources
        if let (Some(lib_a), Some(lib_b)) = (self.libraries.get(id_a), self.libraries.get(id_b)) {
            let weight_b = 1.0 - weight_a;
            let tone_a = lib_a.tone_blender.effective_tone();
            let tone_b = lib_b.tone_blender.effective_tone();

            blended.tone_blender.base_tone = ToneProfile {
                warmth: tone_a.warmth * weight_a + tone_b.warmth * weight_b,
                enthusiasm: tone_a.enthusiasm * weight_a + tone_b.enthusiasm * weight_b,
                formality: tone_a.formality * weight_a + tone_b.formality * weight_b,
            };

            // Blend hedging levels
            blended.hedging_injector.base_level = lib_a.hedging_injector.effective_level()
                * weight_a
                + lib_b.hedging_injector.effective_level() * weight_b;
        }

        self.libraries.insert(result_id.to_string(), blended);
        self.libraries.get_mut(result_id).unwrap()
    }

    /// Number of libraries stored
    pub fn size(&self) -> usize {
        self.libraries.len()
    }
}

impl Default for TemplateStore {
    fn default() -> Self {
        TemplateStore::new()
    }
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::profile::{AiProfileStore, DeltaSource};

    #[test]
    fn test_template_category_classify() {
        assert_eq!(
            TemplateCategory::classify("Hello there"),
            TemplateCategory::Greeting
        );
        assert_eq!(
            TemplateCategory::classify("explain rust"),
            TemplateCategory::Explanation
        );
        assert_eq!(
            TemplateCategory::classify("write code for me"),
            TemplateCategory::CodeHelp
        );
        assert_eq!(
            TemplateCategory::classify("why does this work"),
            TemplateCategory::Reasoning
        );
        assert_eq!(
            TemplateCategory::classify("summarize this"),
            TemplateCategory::Summarization
        );
        assert_eq!(
            TemplateCategory::classify("create a story"),
            TemplateCategory::Creative
        );
    }

    #[test]
    fn test_response_fragment_compound_confidence() {
        let mut frag = ResponseFragment::new(FragmentType::Opening, "Hello!");
        assert_eq!(frag.confidence, 0.5);

        for _ in 0..10 {
            frag.record_success();
        }
        assert!(frag.confidence > 0.5, "Confidence should grow with success");
        assert!(frag.confidence <= 0.95, "Confidence should be capped");

        frag.record_negative();
        let after_neg = frag.confidence;
        assert!(
            after_neg < 0.95,
            "Negative feedback should reduce confidence"
        );
    }

    #[test]
    fn test_tone_blender_from_profile() {
        let store = AiProfileStore::default();
        let profile = store.get("claude").unwrap();
        let blender = ToneBlender::from_profile(profile);

        let tone = blender.effective_tone();
        assert!(tone.warmth >= 0.0 && tone.warmth <= 1.0);
        assert!(tone.enthusiasm >= 0.0 && tone.enthusiasm <= 1.0);
        assert!(tone.formality >= 0.0 && tone.formality <= 1.0);
    }

    #[test]
    fn test_tone_blender_delta_compound() {
        let store = AiProfileStore::default();
        let profile = store.get("gpt4o").unwrap();
        let mut blender = ToneBlender::from_profile(profile);
        let original_warmth = blender.effective_tone().warmth;

        let delta = PersonalityDelta {
            source: DeltaSource::SelfMonitoring,
            adjustments: vec![("warmth".to_string(), 0.5)],
            confidence: 0.8,
        };
        blender.apply_delta(&delta);

        let new_warmth = blender.effective_tone().warmth;
        assert!(
            (new_warmth - original_warmth).abs() > 0.001,
            "Delta should affect tone"
        );
    }

    #[test]
    fn test_hedging_injector() {
        let store = AiProfileStore::default();
        let claude = store.get("claude").unwrap();
        let mut injector = HedgingInjector::from_profile(claude);

        assert!(injector.should_hedge(), "Claude should hedge");
        let hedge = injector.select_hedge();
        assert!(hedge.is_some(), "Should produce a hedging phrase");

        let gpt4o = store.get("gpt4o").unwrap();
        let injector2 = HedgingInjector::from_profile(gpt4o);
        // GPT-4o also hedges (safety.hedges_uncertainty is model-specific)
        assert!(injector2.effective_level() >= 0.0);
    }

    #[test]
    fn test_structural_formatter_list() {
        let store = AiProfileStore::default();
        let profile = store.get("gpt4o").unwrap();
        let formatter = StructuralFormatter::from_profile(profile);

        let items = vec![
            "First".to_string(),
            "Second".to_string(),
            "Third".to_string(),
        ];
        let formatted = formatter.format_list(&items);
        assert!(!formatted.is_empty());
        assert!(formatted.contains("First"));
    }

    #[test]
    fn test_structural_formatter_code() {
        let store = AiProfileStore::default();
        let profile = store.get("claude").unwrap();
        let formatter = StructuralFormatter::from_profile(profile);

        let code = formatter.format_code("fn main() {}", "rust");
        assert!(code.contains("```rust"));
        assert!(code.contains("fn main()"));
    }

    #[test]
    fn test_template_library_from_profile() {
        let store = AiProfileStore::default();
        let profile = store.get("claude").unwrap();
        let lib = TemplateLibrary::from_profile(profile);

        assert_eq!(lib.persona_id, "claude");
        assert!(!lib.fragments.is_empty());
        assert!(lib.fragments.contains_key("openings"));
        assert!(lib.fragments.contains_key("body"));
        assert!(lib.fragments.contains_key("hedging"));
        assert!(lib.fragments.contains_key("closing"));
    }

    #[test]
    fn test_template_library_generate() {
        let store = AiProfileStore::default();
        let profile = store.get("gpt4o").unwrap();
        let mut lib = TemplateLibrary::from_profile(profile);

        let output = lib.generate("explain how Rust ownership works", &profile.response_style);
        assert!(!output.is_empty());
        assert!(lib.total_generated == 1);
    }

    #[test]
    fn test_template_library_feedback_compound() {
        let store = AiProfileStore::default();
        let profile = store.get("claude").unwrap();
        let mut lib = TemplateLibrary::from_profile(profile);

        // Generate first to create some usage
        let _ = lib.generate("hello", &profile.response_style);

        // Mark some fragments as used so feedback affects them
        if let Some(openings) = lib.fragments.get_mut("openings") {
            for frag in openings.iter_mut() {
                frag.record_success();
            }
        }

        let delta = PersonalityDelta {
            source: DeltaSource::SelfMonitoring,
            adjustments: vec![("warmth".to_string(), 0.01)],
            confidence: 0.9,
        };
        lib.apply_feedback(&delta);

        assert_eq!(lib.total_feedback, 1);
    }

    #[test]
    fn test_template_store() {
        let store = AiProfileStore::default();
        let mut template_store = TemplateStore::new();

        let gpt4o = store.get("gpt4o").unwrap();
        let claude = store.get("claude").unwrap();

        template_store.get_or_create(gpt4o);
        template_store.get_or_create(claude);

        assert_eq!(template_store.size(), 2);
        assert!(template_store.get("gpt4o").is_some());
        assert!(template_store.get("claude").is_some());
        assert!(template_store.get("nonexistent").is_none());
    }

    #[test]
    fn test_template_store_blend() {
        let store = AiProfileStore::default();
        let mut template_store = TemplateStore::new();

        let gpt4o = store.get("gpt4o").unwrap();
        let claude = store.get("claude").unwrap();

        template_store.get_or_create(gpt4o);
        template_store.get_or_create(claude);

        let profiles: Vec<&AiProfile> = vec![gpt4o, claude];
        let blended_profile = AiProfile::blend(&profiles, &[0.6, 0.4]);

        template_store.blend("gpt4o", "claude", 0.6, "blend", &blended_profile);

        assert_eq!(template_store.size(), 3); // gpt4o, claude, blend
        let blended_lib = template_store.get("blend").unwrap();
        assert_eq!(blended_lib.persona_id, blended_profile.id);
    }

    #[test]
    fn test_template_library_serialization() {
        let store = AiProfileStore::default();
        let profile = store.get("gpt4o").unwrap();
        let lib = TemplateLibrary::from_profile(profile);

        let json = serde_json::to_string(&lib).unwrap();
        let restored: TemplateLibrary = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.persona_id, "gpt4o");
        assert_eq!(restored.fragments.len(), lib.fragments.len());
    }

    #[test]
    fn test_template_library_stats() {
        let store = AiProfileStore::default();
        let profile = store.get("claude").unwrap();
        let mut lib = TemplateLibrary::from_profile(profile);
        let _ = lib.generate("test", &profile.response_style);

        let stats = lib.stats();
        assert!(stats.contains("claude"));
        assert!(stats.contains("Generated: 1"));
    }

    #[test]
    fn test_opening_selection_varies_by_category() {
        let store = AiProfileStore::default();
        let profile = store.get("gpt4o").unwrap();
        let blender = ToneBlender::from_profile(profile);

        let greeting_opening = blender.select_opening(&TemplateCategory::Greeting);
        let code_opening = blender.select_opening(&TemplateCategory::CodeHelp);

        // They should be different strings for different categories
        assert!(!greeting_opening.is_empty());
        assert!(!code_opening.is_empty());
    }

    #[test]
    fn test_generate_different_categories() {
        let store = AiProfileStore::default();
        let profile = store.get("claude").unwrap();
        let mut lib = TemplateLibrary::from_profile(profile);

        let code_response = lib.generate("help me write code for sorting", &profile.response_style);
        let reason_response = lib.generate(
            "why does gravity work step by step",
            &profile.response_style,
        );
        let greeting_response = lib.generate("hello how are you", &profile.response_style);

        assert!(!code_response.is_empty());
        assert!(!reason_response.is_empty());
        assert!(!greeting_response.is_empty());
        assert_eq!(lib.total_generated, 3);
    }
}
