// =================================================================
// CAPABILITY MODULE SYSTEM
// =================================================================
// Defines the modular capability system. Each AI model has different
// capabilities (text, vision, audio, code, reasoning). The modality
// router dispatches inputs to the correct capability handler.
//
// COMPOUND INTEGRATIONS:
// - for_profile(): auto-select capability module matching a profile
// - reconfigure_for(): clear and reload modules for a new persona
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::mimicry::profile::AiProfile;

/// Input/output modalities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modality {
    /// Natural language text processing.
    Text,
    /// Image and visual content understanding.
    Vision,
    /// Audio and speech processing.
    Audio,
    /// Video comprehension and analysis.
    Video,
    /// Source code generation, analysis, and debugging.
    Code,
    /// Logical reasoning and chain-of-thought inference.
    Reasoning,
    /// Structured function/tool calling.
    FunctionCall,
    /// Vector embedding generation.
    Embedding,
    /// User-defined modality with a custom name.
    Custom(String),
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modality::Text => write!(f, "text"),
            Modality::Vision => write!(f, "vision"),
            Modality::Audio => write!(f, "audio"),
            Modality::Video => write!(f, "video"),
            Modality::Code => write!(f, "code"),
            Modality::Reasoning => write!(f, "reasoning"),
            Modality::FunctionCall => write!(f, "function_call"),
            Modality::Embedding => write!(f, "embedding"),
            Modality::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

impl From<&str> for Modality {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "text" => Modality::Text,
            "vision" | "image" => Modality::Vision,
            "audio" | "speech" => Modality::Audio,
            "video" => Modality::Video,
            "code" | "programming" => Modality::Code,
            "reasoning" | "think" => Modality::Reasoning,
            "function_call" | "function_calling" | "tools" => Modality::FunctionCall,
            "embedding" | "embeddings" => Modality::Embedding,
            other => Modality::Custom(other.to_string()),
        }
    }
}

/// A unit of input to process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalInput {
    /// The modality type of this input.
    pub modality: Modality,
    /// The raw content payload to process.
    pub content: String,
    /// Optional key-value metadata associated with the input.
    pub metadata: HashMap<String, String>,
}

impl ModalInput {
    /// Creates a plain text input with no metadata.
    pub fn text(content: &str) -> Self {
        ModalInput {
            modality: Modality::Text,
            content: content.to_string(),
            metadata: HashMap::new(),
        }
    }

    /// Creates a code input with the specified programming language in metadata.
    pub fn code(content: &str, language: &str) -> Self {
        let mut meta = HashMap::new();
        meta.insert("language".to_string(), language.to_string());
        ModalInput {
            modality: Modality::Code,
            content: content.to_string(),
            metadata: meta,
        }
    }

    /// Creates a vision input with a description and image source path.
    pub fn vision(description: &str, source: &str) -> Self {
        let mut meta = HashMap::new();
        meta.insert("source".to_string(), source.to_string());
        ModalInput {
            modality: Modality::Vision,
            content: description.to_string(),
            metadata: meta,
        }
    }
}

/// A processed output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalOutput {
    /// The modality type of this output.
    pub modality: Modality,
    /// The generated output content.
    pub content: String,
    /// Confidence score for the output, ranging from 0.0 to 1.0.
    pub confidence: f64,
    /// Optional key-value metadata associated with the output.
    pub metadata: HashMap<String, String>,
}

/// Capability levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CapabilityLevel {
    /// No capability in this modality.
    None,
    /// Minimal, limited capability.
    Basic,
    /// Moderate competence in this modality.
    Intermediate,
    /// High proficiency with broad coverage.
    Advanced,
    /// Near-peak human-level performance.
    Expert,
    /// Exceeds typical human-level performance.
    Superhuman,
}

impl CapabilityLevel {
    /// Returns the capability level as a normalized float between 0.0 and 1.0.
    pub fn as_f64(&self) -> f64 {
        match self {
            CapabilityLevel::None => 0.0,
            CapabilityLevel::Basic => 0.2,
            CapabilityLevel::Intermediate => 0.4,
            CapabilityLevel::Advanced => 0.6,
            CapabilityLevel::Expert => 0.8,
            CapabilityLevel::Superhuman => 1.0,
        }
    }
}

/// A single capability (e.g., "code generation", "image understanding")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Human-readable name of this capability (e.g., "code-generation").
    pub name: String,
    /// The modality this capability operates on.
    pub modality: Modality,
    /// Proficiency level for this capability.
    pub level: CapabilityLevel,
    /// Free-text description of what this capability does.
    pub description: String,
    /// Names of finer-grained sub-capabilities (e.g., "debugging", "refactoring").
    pub sub_capabilities: Vec<String>,
}

impl Capability {
    /// Creates a new capability with the given name, modality, and level.
    pub fn new(name: &str, modality: Modality, level: CapabilityLevel) -> Self {
        Capability {
            name: name.to_string(),
            modality,
            level,
            description: String::new(),
            sub_capabilities: Vec::new(),
        }
    }

    /// Sets the description and returns `self` for builder chaining.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Appends a sub-capability name and returns `self` for builder chaining.
    pub fn with_sub(mut self, sub: &str) -> Self {
        self.sub_capabilities.push(sub.to_string());
        self
    }
}

/// A loadable module that provides capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityModule {
    /// Name identifier for this module.
    pub name: String,
    /// Version string of this module.
    pub version: String,
    /// The set of capabilities provided by this module.
    pub capabilities: Vec<Capability>,
    /// Whether this module has been loaded into a router.
    pub is_loaded: bool,
}

impl CapabilityModule {
    /// Creates a new empty capability module with the given name and version.
    pub fn new(name: &str, version: &str) -> Self {
        CapabilityModule {
            name: name.to_string(),
            version: version.to_string(),
            capabilities: Vec::new(),
            is_loaded: false,
        }
    }

    /// Adds a capability to this module and returns `self` for builder chaining.
    pub fn add_capability(mut self, cap: Capability) -> Self {
        self.capabilities.push(cap);
        self
    }

    /// Returns `true` if any capability in this module handles the given modality.
    pub fn supports(&self, modality: &Modality) -> bool {
        self.capabilities.iter().any(|c| &c.modality == modality)
    }

    /// Returns the highest capability level for the given modality, or `None` if unsupported.
    pub fn capability_level(&self, modality: &Modality) -> CapabilityLevel {
        self.capabilities
            .iter()
            .filter(|c| &c.modality == modality)
            .map(|c| c.level.clone())
            .max_by(|a, b| {
                a.as_f64()
                    .partial_cmp(&b.as_f64())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(CapabilityLevel::None)
    }

    // =================================================================
    // COMPOUND INTEGRATION METHODS
    // =================================================================

    /// COMPOUND: Auto-construct a capability module matching a profile.
    /// Maps known profile IDs to pre-built modules, or constructs
    /// capabilities from the profile's supported_modalities list.
    /// This is the Profile -> Capability binding path.
    pub fn for_profile(profile: &AiProfile) -> CapabilityModule {
        // First try known profiles
        match profile.id.as_str() {
            "gpt4o" => return Self::gpt4o_capabilities(),
            "claude" => return Self::claude_capabilities(),
            "o1" => return Self::o1_capabilities(),
            "gemini" => return Self::gemini_capabilities(),
            "llama" => return Self::llama_capabilities(),
            _ => {}
        }

        // Fall back: construct from profile's supported_modalities
        let mut module = CapabilityModule::new(&format!("{}-caps", profile.id), &profile.version);

        for modality_str in &profile.supported_modalities {
            let modality = Modality::from(modality_str.as_str());
            let level = if profile.response_style.verbosity > 0.7 {
                CapabilityLevel::Expert
            } else if profile.response_style.verbosity > 0.4 {
                CapabilityLevel::Advanced
            } else {
                CapabilityLevel::Intermediate
            };

            module.capabilities.push(Capability {
                name: format!("{}-processing", modality_str),
                modality,
                level,
                description: format!(
                    "Auto-generated capability for {} from profile {}",
                    modality_str, profile.id
                ),
                sub_capabilities: Vec::new(),
            });
        }

        module
    }

    /// Create the GPT-4o capability module
    pub fn gpt4o_capabilities() -> Self {
        CapabilityModule::new("gpt4o-caps", "1.0")
            .add_capability(
                Capability::new("text-generation", Modality::Text, CapabilityLevel::Expert)
                    .with_description("Natural language generation and understanding")
                    .with_sub("summarization")
                    .with_sub("translation")
                    .with_sub("question-answering")
                    .with_sub("creative-writing"),
            )
            .add_capability(
                Capability::new(
                    "image-understanding",
                    Modality::Vision,
                    CapabilityLevel::Advanced,
                )
                .with_description("Analyze and describe images")
                .with_sub("object-detection")
                .with_sub("text-extraction")
                .with_sub("scene-description"),
            )
            .add_capability(
                Capability::new(
                    "audio-processing",
                    Modality::Audio,
                    CapabilityLevel::Advanced,
                )
                .with_description("Speech recognition and audio understanding")
                .with_sub("transcription")
                .with_sub("voice-synthesis"),
            )
            .add_capability(
                Capability::new("code-generation", Modality::Code, CapabilityLevel::Expert)
                    .with_description("Write, analyze, and debug code")
                    .with_sub("debugging")
                    .with_sub("refactoring")
                    .with_sub("explanation"),
            )
            .add_capability(
                Capability::new(
                    "function-calling",
                    Modality::FunctionCall,
                    CapabilityLevel::Expert,
                )
                .with_description("Structured function/tool calling"),
            )
    }

    /// Create the Claude capability module
    pub fn claude_capabilities() -> Self {
        CapabilityModule::new("claude-caps", "1.0")
            .add_capability(
                Capability::new("text-generation", Modality::Text, CapabilityLevel::Expert)
                    .with_description("Nuanced, careful text generation")
                    .with_sub("long-form-analysis")
                    .with_sub("academic-writing")
                    .with_sub("careful-reasoning"),
            )
            .add_capability(
                Capability::new(
                    "image-understanding",
                    Modality::Vision,
                    CapabilityLevel::Advanced,
                )
                .with_description("Detailed image analysis"),
            )
            .add_capability(
                Capability::new("code-generation", Modality::Code, CapabilityLevel::Expert)
                    .with_description("Thorough code generation with explanations")
                    .with_sub("architecture-design")
                    .with_sub("code-review")
                    .with_sub("documentation"),
            )
            .add_capability(
                Capability::new("reasoning", Modality::Reasoning, CapabilityLevel::Expert)
                    .with_description("Deep analytical reasoning")
                    .with_sub("logic")
                    .with_sub("ethics")
                    .with_sub("nuance-detection"),
            )
    }

    /// Create the o1 capability module
    pub fn o1_capabilities() -> Self {
        CapabilityModule::new("o1-caps", "1.0")
            .add_capability(
                Capability::new("text-generation", Modality::Text, CapabilityLevel::Expert)
                    .with_description("Precise, deliberate text generation"),
            )
            .add_capability(
                Capability::new("code-generation", Modality::Code, CapabilityLevel::Expert)
                    .with_description("Deep code reasoning and generation")
                    .with_sub("algorithmic-thinking")
                    .with_sub("proof-construction"),
            )
            .add_capability(
                Capability::new(
                    "reasoning",
                    Modality::Reasoning,
                    CapabilityLevel::Superhuman,
                )
                .with_description("Extended chain-of-thought reasoning")
                .with_sub("mathematical-proof")
                .with_sub("logical-deduction")
                .with_sub("multi-step-planning"),
            )
    }

    /// Create the Gemini capability module
    pub fn gemini_capabilities() -> Self {
        CapabilityModule::new("gemini-caps", "1.0")
            .add_capability(
                Capability::new("text-generation", Modality::Text, CapabilityLevel::Expert)
                    .with_description("Broad-spectrum text generation"),
            )
            .add_capability(
                Capability::new(
                    "image-understanding",
                    Modality::Vision,
                    CapabilityLevel::Advanced,
                )
                .with_description("Multimodal image understanding"),
            )
            .add_capability(
                Capability::new(
                    "audio-processing",
                    Modality::Audio,
                    CapabilityLevel::Advanced,
                )
                .with_description("Audio understanding and generation"),
            )
            .add_capability(
                Capability::new(
                    "video-understanding",
                    Modality::Video,
                    CapabilityLevel::Advanced,
                )
                .with_description("Video comprehension and analysis"),
            )
            .add_capability(
                Capability::new("code-generation", Modality::Code, CapabilityLevel::Advanced)
                    .with_description("Code generation with Google ecosystem integration"),
            )
    }

    /// Create the LLaMA capability module
    pub fn llama_capabilities() -> Self {
        CapabilityModule::new("llama-caps", "1.0")
            .add_capability(
                Capability::new("text-generation", Modality::Text, CapabilityLevel::Advanced)
                    .with_description("Open-source text generation")
                    .with_sub("fine-tuning-support")
                    .with_sub("custom-deployment"),
            )
            .add_capability(
                Capability::new("code-generation", Modality::Code, CapabilityLevel::Advanced)
                    .with_description("Code generation (community fine-tuned)"),
            )
    }
}

/// The Modality Router - dispatches inputs to correct handlers
pub struct ModalityRouter {
    modules: Vec<CapabilityModule>,
    routing_table: HashMap<String, Vec<usize>>, // modality -> module indices
}

impl ModalityRouter {
    /// Creates an empty router with no loaded modules.
    pub fn new() -> Self {
        ModalityRouter {
            modules: Vec::new(),
            routing_table: HashMap::new(),
        }
    }

    /// Load a capability module into the router
    pub fn load_module(&mut self, mut module: CapabilityModule) {
        module.is_loaded = true;
        let idx = self.modules.len();

        // Update routing table
        for cap in &module.capabilities {
            let modality_key = format!("{}", cap.modality);
            self.routing_table
                .entry(modality_key)
                .or_default()
                .push(idx);
        }

        self.modules.push(module);
    }

    /// Route an input to the best available module
    pub fn route(&self, input: &ModalInput) -> Option<&CapabilityModule> {
        let key = format!("{}", input.modality);
        if let Some(indices) = self.routing_table.get(&key) {
            // Find the module with the highest capability level for this modality
            indices.iter().map(|&i| &self.modules[i]).max_by(|a, b| {
                let level_a = a.capability_level(&input.modality);
                let level_b = b.capability_level(&input.modality);
                level_a
                    .as_f64()
                    .partial_cmp(&level_b.as_f64())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        } else {
            None
        }
    }

    /// Check if a modality is supported
    pub fn supports(&self, modality: &Modality) -> bool {
        let key = format!("{}", modality);
        self.routing_table.contains_key(&key)
    }

    /// Get all supported modalities
    pub fn supported_modalities(&self) -> Vec<Modality> {
        self.modules
            .iter()
            .flat_map(|m| m.capabilities.iter().map(|c| c.modality.clone()))
            .collect::<std::collections::HashSet<Modality>>()
            .into_iter()
            .collect()
    }

    /// Get capability summary
    pub fn capability_summary(&self) -> String {
        let mut lines = vec!["Loaded Capability Modules:".to_string()];
        for module in &self.modules {
            lines.push(format!("  [{}] v{}", module.name, module.version));
            for cap in &module.capabilities {
                lines.push(format!(
                    "    - {} ({}) [{:?}]",
                    cap.name, cap.modality, cap.level
                ));
            }
        }
        lines.join("\n")
    }

    // =================================================================
    // COMPOUND INTEGRATION METHODS
    // =================================================================

    /// COMPOUND: Clear and reconfigure the router for a new persona.
    /// Loads the appropriate capability module(s) for the given profile.
    /// This is the Profile -> Router reconfiguration path.
    pub fn reconfigure_for(&mut self, profile: &AiProfile) {
        // Clear existing modules and routing table
        self.modules.clear();
        self.routing_table.clear();

        // Load the capability module appropriate for this profile
        let module = CapabilityModule::for_profile(profile);
        self.load_module(module);
    }
}

impl Default for ModalityRouter {
    fn default() -> Self {
        let mut router = ModalityRouter::new();
        router.load_module(CapabilityModule::gpt4o_capabilities());
        router.load_module(CapabilityModule::claude_capabilities());
        router
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::profile::AiProfileStore;

    #[test]
    fn test_modality_routing() {
        let router = ModalityRouter::default();

        let text_input = ModalInput::text("Hello world");
        assert!(router.route(&text_input).is_some());

        let vision_input = ModalInput::vision("a cat", "test.jpg");
        assert!(router.route(&vision_input).is_some());
    }

    #[test]
    fn test_modality_from_string() {
        assert_eq!(Modality::from("text"), Modality::Text);
        assert_eq!(Modality::from("vision"), Modality::Vision);
        assert_eq!(Modality::from("image"), Modality::Vision);
        assert_eq!(Modality::from("audio"), Modality::Audio);
        assert_eq!(Modality::from("code"), Modality::Code);
    }

    #[test]
    fn test_capability_module() {
        let module = CapabilityModule::gpt4o_capabilities();
        assert!(module.supports(&Modality::Text));
        assert!(module.supports(&Modality::Vision));
        assert!(module.supports(&Modality::Audio));
        assert!(!module.supports(&Modality::Video));
    }

    #[test]
    fn test_capability_level() {
        let module = CapabilityModule::gpt4o_capabilities();
        assert_eq!(
            module.capability_level(&Modality::Text),
            CapabilityLevel::Expert
        );
        assert_eq!(
            module.capability_level(&Modality::Video),
            CapabilityLevel::None
        );
    }

    #[test]
    fn test_capability_serialization() {
        let module = CapabilityModule::gpt4o_capabilities();
        let json = serde_json::to_string(&module).unwrap();
        let restored: CapabilityModule = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, "gpt4o-caps");
        assert!(restored.supports(&Modality::Text));
    }

    #[test]
    fn test_for_profile_known() {
        let store = AiProfileStore::default();
        let gpt4o = store.get("gpt4o").unwrap();
        let module = CapabilityModule::for_profile(gpt4o);
        assert_eq!(module.name, "gpt4o-caps");
        assert!(module.supports(&Modality::Text));
        assert!(module.supports(&Modality::Vision));
    }

    #[test]
    fn test_for_profile_unknown() {
        let profile = AiProfile::new("custom-model", "Custom Model");
        let module = CapabilityModule::for_profile(&profile);
        assert!(module.supports(&Modality::Text)); // default has text
    }

    #[test]
    fn test_reconfigure_for() {
        let store = AiProfileStore::default();
        let mut router = ModalityRouter::default();

        // Initially has gpt4o + claude modules
        assert!(router.supports(&Modality::Audio)); // gpt4o has audio

        // Reconfigure for o1 (no audio support)
        let o1 = store.get("o1").unwrap();
        router.reconfigure_for(o1);
        assert!(!router.supports(&Modality::Audio));
        assert!(router.supports(&Modality::Reasoning));
    }
}
