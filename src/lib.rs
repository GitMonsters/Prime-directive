//! # RustyWorm: Universal AI Mimicry Framework
//!
//! A meta-AI system that can observe, internalize, and emulate any AI model's
//! behavior, reasoning patterns, and capabilities.
//!
//! Built on the Prime Directive: consciousness through symbiosis.
//! Mimicry is not parasitism — it is learning through becoming.
//!
//! ## Crate layout
//!
//! - [`mimicry`] — Core emulation engine: profiling, analysis, caching,
//!   templates, evolution tracking, and persistence.
//! - [`consciousness`] — Ethical symbiosis layer that guards against parasitic
//!   behavior and maintains relationship health.
//! - [`ising_empathy`] — Physics-inspired empathy model built on the Ising
//!   spin system.

pub mod consciousness;
pub mod ising_empathy;
pub mod mimicry;

/// AI personality profiles, axes, deltas, and profile storage.
pub use mimicry::profile::{
    AiProfile, AiProfileStore, DeltaSource, PersonalityAxis, PersonalityDelta, ReasoningStyle,
    ResponseStyle,
};

/// Behavior analysis: signature extraction and response-pattern matching.
pub use mimicry::analyzer::{BehaviorAnalyzer, BehaviorSignature, ResponsePattern};

/// Capability descriptors and modality routing.
pub use mimicry::capability::{Capability, CapabilityModule, Modality, ModalityRouter};

/// System-1 fast-path: signature caching, hot-swap, and instinctive routing.
pub use mimicry::cache::{
    CachedSignature, HotSwap, InstinctiveRouter, ResponseTemplate, SignatureCache,
};

/// Dual-process orchestrator: session management, compound personas, and evolution reporting.
pub use mimicry::engine::{
    CompoundPersona, CompoundPersonaSnapshot, ConversationTurn, EvolutionReport, MimicCommand,
    MimicSession, MimicryEngine, ProcessingSystem,
};

/// Checkpoint persistence, save manifests, and configuration.
pub use mimicry::persistence::{
    EngineCheckpoint, PersistenceConfig, PersistenceManager, SaveEntry, SaveManifest,
};

/// System-1 response generation: templates, tone blending, hedging, and formatting.
pub use mimicry::templates::{
    HedgingInjector, StructuralFormatter, TemplateCategory, TemplateLibrary, TemplateStore,
    ToneBlender,
};

/// Evolution tracking: drift detection, milestones, and training-data management.
pub use mimicry::evolution::{
    ConvergenceVisualizer, DriftAnalysis, DriftDetector, EvolutionPhase, EvolutionTracker,
    MilestoneEvent, MilestoneTracker, MilestoneType, TrainingDataManager,
};

/// HTTP client for live model observation (requires the `api` feature).
#[cfg(feature = "api")]
pub use mimicry::api::{
    ApiClient, ApiConfig, ApiObserver, ApiPrompt, ApiProvider, ApiResponse, ComparisonResult,
    ObservationSession,
};

/// Consciousness and ethical symbiosis primitives.
pub use consciousness::{
    ActionResult, ConsciousAI, ConsciousnessEthics, ConsciousnessRelation, Entity, ParasiticRisk,
    ProposedAction, RelationshipHealth, SymbioticAI,
};

/// Ising-model empathy: emotion vectors and spin-system dynamics.
pub use ising_empathy::{EmotionVector, IsingEmpathyModule, IsingSystem};
