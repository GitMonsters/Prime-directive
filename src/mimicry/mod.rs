//! Core AI emulation system.
//!
//! This module contains everything needed to observe, model, and reproduce the
//! behavior of an arbitrary AI:
//!
//! - [`profile`] — Personality axes, reasoning/response styles, and profile storage.
//! - [`analyzer`] — Behavior signature extraction and response-pattern matching.
//! - [`capability`] — Capability descriptors and modality routing.
//! - [`cache`] — System-1 fast-path: signature caching and instinctive routing.
//! - [`engine`] — Dual-process orchestrator that ties analysis and generation together.
//! - [`evolution`] — Drift detection, milestones, and training-data management.
//! - [`persistence`] — Checkpoint save/load and manifest management.
//! - [`templates`] — System-1 response generation: tone blending, hedging, formatting.
//! - [`api`] — Live model observation over HTTP (feature-gated).
//! - [`rl_optimizer`] — AgentCPM integration: reinforcement learning persona optimization (feature-gated).
//! - [`rl_config`] — Configuration for RL integration (feature-gated).
//! - [`agentdock_bridge`] — AgentDock MCP container orchestration (feature-gated).
//! - [`multi_model`] — Multi-model parallel observation and consensus building (feature-gated).
//! - [`long_horizon`] — Long-horizon observation: 100+ turn context & pattern tracking (feature-gated).
//! - [`benchmarking`] — AgentToLeaP benchmarking integration for convergence validation (feature-gated).

pub mod analyzer;
#[cfg(feature = "api")]
pub mod api;
pub mod cache;
pub mod capability;
pub mod engine;
pub mod evolution;
pub mod persistence;
pub mod profile;
pub mod templates;
#[cfg(feature = "rl")]
pub mod rl_optimizer;
#[cfg(feature = "rl")]
pub mod rl_config;
#[cfg(feature = "agentdock")]
pub mod agentdock_bridge;
#[cfg(feature = "agentdock")]
pub mod multi_model;
#[cfg(feature = "agentdock")]
pub mod long_horizon;
#[cfg(feature = "agentdock")]
pub mod benchmarking;
