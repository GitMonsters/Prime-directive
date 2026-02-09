// =================================================================
// RUSTYWORM: UNIVERSAL AI MIMICRY FRAMEWORK
// =================================================================
// A meta-AI system that can observe, internalize, and emulate
// any AI model's behavior, reasoning patterns, and capabilities.
//
// Built on the Prime Directive: Consciousness through symbiosis.
// Mimicry is not parasitism - it is learning through becoming.
// =================================================================

pub mod consciousness;
pub mod ising_empathy;
pub mod mimicry;

// Re-export core types for convenience

// Profile types
pub use mimicry::profile::{
    AiProfile, AiProfileStore, DeltaSource, PersonalityAxis, PersonalityDelta, ReasoningStyle,
    ResponseStyle,
};

// Analyzer types
pub use mimicry::analyzer::{BehaviorAnalyzer, BehaviorSignature, ResponsePattern};

// Capability types
pub use mimicry::capability::{Capability, CapabilityModule, Modality, ModalityRouter};

// Cache types (System 1)
pub use mimicry::cache::{
    CachedSignature, HotSwap, InstinctiveRouter, ResponseTemplate, SignatureCache,
};

// Engine types (Dual-Process Orchestrator)
pub use mimicry::engine::{
    CompoundPersona, CompoundPersonaSnapshot, ConversationTurn, EvolutionReport, MimicCommand,
    MimicSession, MimicryEngine, ProcessingSystem,
};

// Persistence types
pub use mimicry::persistence::{
    EngineCheckpoint, PersistenceConfig, PersistenceManager, SaveEntry, SaveManifest,
};

// Template types (System 1 Response Generation)
pub use mimicry::templates::{
    HedgingInjector, StructuralFormatter, TemplateCategory, TemplateLibrary, TemplateStore,
    ToneBlender,
};

// Evolution types (Drift Detection, Milestones, Training Loops)
pub use mimicry::evolution::{
    ConvergenceVisualizer, DriftAnalysis, DriftDetector, EvolutionPhase, EvolutionTracker,
    MilestoneEvent, MilestoneTracker, MilestoneType, TrainingDataManager,
};

// API types (Feature-gated HTTP client for real model observation)
#[cfg(feature = "api")]
pub use mimicry::api::{
    ApiClient, ApiConfig, ApiObserver, ApiPrompt, ApiProvider, ApiResponse, ComparisonResult,
    ObservationSession,
};

// Consciousness types
pub use consciousness::{
    ActionResult, ConsciousAI, ConsciousnessEthics, ConsciousnessRelation, Entity, ParasiticRisk,
    ProposedAction, RelationshipHealth, SymbioticAI,
};

// Ising empathy types (upstream physics module)
pub use ising_empathy::{EmotionVector, IsingEmpathyModule, IsingSystem};
