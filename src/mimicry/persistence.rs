// =================================================================
// PERSISTENCE MODULE: File-Based Compound State Management
// =================================================================
// Saves and loads the entire RustyWorm state tree to/from disk.
// Every type in the system is serde-serializable; this module
// provides the file I/O layer and directory management.
//
// COMPOUND INTEGRATIONS:
// - save_persona() / load_persona(): full CompoundPersonaSnapshot
// - save_profile() / load_profile(): AiProfile import/export
// - save_session() / load_session(): MimicSession with history
// - save_engine_state() / load_engine_state(): entire engine checkpoint
// - auto_save(): triggered by evolution milestones
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::mimicry::engine::CompoundPersonaSnapshot;
use crate::mimicry::profile::AiProfile;

// =================================================================
// PERSISTENCE CONFIG
// =================================================================

/// Configuration for the persistence layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Base directory for all saved data
    pub base_dir: PathBuf,
    /// Sub-directory for persona snapshots
    pub personas_dir: String,
    /// Sub-directory for profile definitions
    pub profiles_dir: String,
    /// Sub-directory for session logs
    pub sessions_dir: String,
    /// Sub-directory for engine checkpoints
    pub checkpoints_dir: String,
    /// Whether to auto-save on evolution milestones
    pub auto_save_enabled: bool,
    /// Auto-save every N compound iterations
    pub auto_save_interval: u64,
    /// Pretty-print JSON output
    pub pretty_print: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        PersistenceConfig {
            base_dir: PathBuf::from(".rustyworm"),
            personas_dir: "personas".to_string(),
            profiles_dir: "profiles".to_string(),
            sessions_dir: "sessions".to_string(),
            checkpoints_dir: "checkpoints".to_string(),
            auto_save_enabled: true,
            auto_save_interval: 10,
            pretty_print: true,
        }
    }
}

impl PersistenceConfig {
    /// Returns the full path to the personas directory.
    pub fn personas_path(&self) -> PathBuf {
        self.base_dir.join(&self.personas_dir)
    }
    /// Returns the full path to the profiles directory.
    pub fn profiles_path(&self) -> PathBuf {
        self.base_dir.join(&self.profiles_dir)
    }
    /// Returns the full path to the sessions directory.
    pub fn sessions_path(&self) -> PathBuf {
        self.base_dir.join(&self.sessions_dir)
    }
    /// Returns the full path to the checkpoints directory.
    pub fn checkpoints_path(&self) -> PathBuf {
        self.base_dir.join(&self.checkpoints_dir)
    }
}

// =================================================================
// SAVE MANIFEST - Index of all saved artifacts
// =================================================================

/// Tracks all saved artifacts for quick listing without reading each file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveManifest {
    /// All saved persona snapshot entries.
    pub personas: Vec<SaveEntry>,
    /// All saved profile entries.
    pub profiles: Vec<SaveEntry>,
    /// All saved session entries.
    pub sessions: Vec<SaveEntry>,
    /// All saved engine checkpoint entries.
    pub checkpoints: Vec<SaveEntry>,
}

/// A single entry in the save manifest, representing one saved artifact on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveEntry {
    /// Human-readable name identifying this artifact.
    pub name: String,
    /// On-disk filename for this artifact.
    pub filename: String,
    /// Timestamp string indicating when this artifact was saved.
    pub saved_at: String,
    /// Size of the serialized artifact in bytes.
    pub size_bytes: u64,
    /// Arbitrary key-value metadata associated with this artifact.
    pub metadata: HashMap<String, String>,
}

impl SaveManifest {
    /// Creates an empty save manifest with no entries.
    pub fn new() -> Self {
        SaveManifest {
            personas: Vec::new(),
            profiles: Vec::new(),
            sessions: Vec::new(),
            checkpoints: Vec::new(),
        }
    }

    fn add_entry(entries: &mut Vec<SaveEntry>, entry: SaveEntry) {
        // Replace existing entry with same name
        entries.retain(|e| e.name != entry.name);
        entries.push(entry);
    }
}

impl Default for SaveManifest {
    fn default() -> Self {
        SaveManifest::new()
    }
}

// =================================================================
// ENGINE STATE CHECKPOINT
// =================================================================

/// A complete serializable checkpoint of the MimicryEngine state.
/// Used for full save/restore of the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineCheckpoint {
    /// All registered AI profiles at the time of the checkpoint.
    pub profiles: Vec<AiProfile>,
    /// Serialized behavior signature cache entries.
    pub cached_signatures: Vec<String>, // serialized CachedSignatures
    /// Map of snapshot name to serialized snapshot JSON.
    pub saved_snapshots: HashMap<String, String>,
    /// Hot-swap queue entries as (id, serialized JSON) pairs.
    pub hot_swap_entries: Vec<(String, String)>, // (id, json)
    /// ID of the active persona at checkpoint time, if any.
    pub active_persona_id: Option<String>,
    /// The compound iteration count when this checkpoint was created.
    pub checkpoint_iteration: u64,
}

// =================================================================
// PERSISTENCE MANAGER
// =================================================================

/// The main persistence manager. Handles all file I/O for RustyWorm.
pub struct PersistenceManager {
    /// Configuration controlling directory layout and serialization options.
    pub config: PersistenceConfig,
    /// The in-memory manifest tracking all saved artifacts.
    pub manifest: SaveManifest,
    initialized: bool,
}

impl PersistenceManager {
    /// Creates a new persistence manager with the given configuration.
    pub fn new(config: PersistenceConfig) -> Self {
        PersistenceManager {
            config,
            manifest: SaveManifest::new(),
            initialized: false,
        }
    }

    /// Creates a new persistence manager using the default configuration.
    pub fn with_default_config() -> Self {
        PersistenceManager::new(PersistenceConfig::default())
    }

    /// Initialize the directory structure, loading the manifest if it exists
    pub fn initialize(&mut self) -> Result<String, String> {
        let dirs = [
            self.config.personas_path(),
            self.config.profiles_path(),
            self.config.sessions_path(),
            self.config.checkpoints_path(),
        ];

        let mut created = Vec::new();
        for dir in &dirs {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| format!("Failed to create {}: {}", dir.display(), e))?;
                created.push(dir.display().to_string());
            }
        }

        // Load manifest if it exists
        let manifest_path = self.config.base_dir.join("manifest.json");
        if manifest_path.exists() {
            let data = fs::read_to_string(&manifest_path)
                .map_err(|e| format!("Failed to read manifest: {}", e))?;
            self.manifest = serde_json::from_str(&data)
                .map_err(|e| format!("Failed to parse manifest: {}", e))?;
        }

        self.initialized = true;

        if created.is_empty() {
            Ok(format!(
                "Persistence initialized at {}",
                self.config.base_dir.display()
            ))
        } else {
            Ok(format!(
                "Persistence initialized at {} (created {} directories)",
                self.config.base_dir.display(),
                created.len()
            ))
        }
    }

    /// Ensure initialized
    fn ensure_init(&mut self) -> Result<(), String> {
        if !self.initialized {
            self.initialize()?;
        }
        Ok(())
    }

    /// Save the manifest to disk
    fn save_manifest(&self) -> Result<(), String> {
        let path = self.config.base_dir.join("manifest.json");
        let json = if self.config.pretty_print {
            serde_json::to_string_pretty(&self.manifest)
        } else {
            serde_json::to_string(&self.manifest)
        }
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

        fs::write(&path, &json).map_err(|e| format!("Failed to write manifest: {}", e))?;
        Ok(())
    }

    /// Serialize to JSON string
    fn to_json<T: Serialize>(&self, value: &T) -> Result<String, String> {
        if self.config.pretty_print {
            serde_json::to_string_pretty(value)
        } else {
            serde_json::to_string(value)
        }
        .map_err(|e| format!("Serialization error: {}", e))
    }

    // =================================================================
    // PERSONA SAVE/LOAD
    // =================================================================

    /// Save a CompoundPersonaSnapshot to disk
    pub fn save_persona(
        &mut self,
        name: &str,
        snapshot: &CompoundPersonaSnapshot,
    ) -> Result<String, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.personas_path().join(&filename);
        let json = self.to_json(snapshot)?;
        let size = json.len() as u64;

        fs::write(&path, &json)
            .map_err(|e| format!("Failed to write persona '{}': {}", name, e))?;

        let mut meta = HashMap::new();
        meta.insert(
            "convergence".to_string(),
            format!("{:.3}", snapshot.convergence_score),
        );
        meta.insert(
            "iterations".to_string(),
            snapshot.compound_iterations.to_string(),
        );
        meta.insert("profile_id".to_string(), snapshot.profile.id.clone());

        SaveManifest::add_entry(
            &mut self.manifest.personas,
            SaveEntry {
                name: name.to_string(),
                filename: filename.clone(),
                saved_at: timestamp(),
                size_bytes: size,
                metadata: meta,
            },
        );
        self.save_manifest()?;

        Ok(format!(
            "Saved persona '{}' -> {} ({} bytes, convergence: {:.1}%)",
            name,
            path.display(),
            size,
            snapshot.convergence_score * 100.0
        ))
    }

    /// Load a CompoundPersonaSnapshot from disk
    pub fn load_persona(&mut self, name: &str) -> Result<CompoundPersonaSnapshot, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.personas_path().join(&filename);

        if !path.exists() {
            // Try finding by manifest entry
            if let Some(entry) = self.manifest.personas.iter().find(|e| e.name == name) {
                let alt_path = self.config.personas_path().join(&entry.filename);
                if alt_path.exists() {
                    let data = fs::read_to_string(&alt_path)
                        .map_err(|e| format!("Failed to read persona '{}': {}", name, e))?;
                    return serde_json::from_str(&data)
                        .map_err(|e| format!("Failed to parse persona '{}': {}", name, e));
                }
            }
            return Err(format!(
                "Persona '{}' not found at {}",
                name,
                path.display()
            ));
        }

        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read persona '{}': {}", name, e))?;
        serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse persona '{}': {}", name, e))
    }

    /// List all saved personas
    pub fn list_personas(&mut self) -> Result<Vec<SaveEntry>, String> {
        self.ensure_init()?;
        Ok(self.manifest.personas.clone())
    }

    /// Delete a saved persona
    pub fn delete_persona(&mut self, name: &str) -> Result<String, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.personas_path().join(&filename);

        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete persona '{}': {}", name, e))?;
        }

        self.manifest.personas.retain(|e| e.name != name);
        self.save_manifest()?;

        Ok(format!("Deleted persona '{}'", name))
    }

    // =================================================================
    // PROFILE SAVE/LOAD (Import/Export)
    // =================================================================

    /// Save/export an AiProfile to disk
    pub fn save_profile(&mut self, profile: &AiProfile) -> Result<String, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(&profile.id));
        let path = self.config.profiles_path().join(&filename);
        let json = self.to_json(profile)?;
        let size = json.len() as u64;

        fs::write(&path, &json)
            .map_err(|e| format!("Failed to write profile '{}': {}", profile.id, e))?;

        let mut meta = HashMap::new();
        meta.insert("provider".to_string(), profile.provider.clone());
        meta.insert("version".to_string(), profile.version.clone());
        meta.insert(
            "modalities".to_string(),
            profile.supported_modalities.join(","),
        );

        SaveManifest::add_entry(
            &mut self.manifest.profiles,
            SaveEntry {
                name: profile.id.clone(),
                filename: filename.clone(),
                saved_at: timestamp(),
                size_bytes: size,
                metadata: meta,
            },
        );
        self.save_manifest()?;

        Ok(format!(
            "Exported profile '{}' ({}) -> {} ({} bytes)",
            profile.id,
            profile.display_name,
            path.display(),
            size
        ))
    }

    /// Load/import an AiProfile from disk
    pub fn load_profile(&mut self, name: &str) -> Result<AiProfile, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.profiles_path().join(&filename);

        if !path.exists() {
            return Err(format!(
                "Profile '{}' not found at {}",
                name,
                path.display()
            ));
        }

        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read profile '{}': {}", name, e))?;
        serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse profile '{}': {}", name, e))
    }

    /// Load all profiles from the profiles directory
    pub fn load_all_profiles(&mut self) -> Result<Vec<AiProfile>, String> {
        self.ensure_init()?;

        let dir = self.config.profiles_path();
        let mut profiles = Vec::new();

        if !dir.exists() {
            return Ok(profiles);
        }

        let entries =
            fs::read_dir(&dir).map_err(|e| format!("Failed to read profiles dir: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                match fs::read_to_string(&path) {
                    Ok(data) => match serde_json::from_str::<AiProfile>(&data) {
                        Ok(profile) => profiles.push(profile),
                        Err(e) => {
                            eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Warning: Failed to read {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(profiles)
    }

    /// List all saved profiles
    pub fn list_profiles(&mut self) -> Result<Vec<SaveEntry>, String> {
        self.ensure_init()?;
        Ok(self.manifest.profiles.clone())
    }

    // =================================================================
    // SESSION SAVE/LOAD
    // =================================================================

    /// Save serialized session data to disk
    pub fn save_session(&mut self, name: &str, session_json: &str) -> Result<String, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.sessions_path().join(&filename);
        let size = session_json.len() as u64;

        fs::write(&path, session_json)
            .map_err(|e| format!("Failed to write session '{}': {}", name, e))?;

        SaveManifest::add_entry(
            &mut self.manifest.sessions,
            SaveEntry {
                name: name.to_string(),
                filename,
                saved_at: timestamp(),
                size_bytes: size,
                metadata: HashMap::new(),
            },
        );
        self.save_manifest()?;

        Ok(format!(
            "Saved session '{}' -> {} ({} bytes)",
            name,
            path.display(),
            size
        ))
    }

    /// Load session data from disk
    pub fn load_session(&mut self, name: &str) -> Result<String, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.sessions_path().join(&filename);

        if !path.exists() {
            return Err(format!(
                "Session '{}' not found at {}",
                name,
                path.display()
            ));
        }

        fs::read_to_string(&path).map_err(|e| format!("Failed to read session '{}': {}", name, e))
    }

    // =================================================================
    // ENGINE CHECKPOINT SAVE/LOAD
    // =================================================================

    /// Save a full engine checkpoint
    pub fn save_checkpoint(
        &mut self,
        name: &str,
        checkpoint: &EngineCheckpoint,
    ) -> Result<String, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.checkpoints_path().join(&filename);
        let json = self.to_json(checkpoint)?;
        let size = json.len() as u64;

        fs::write(&path, &json)
            .map_err(|e| format!("Failed to write checkpoint '{}': {}", name, e))?;

        let mut meta = HashMap::new();
        meta.insert(
            "profiles".to_string(),
            checkpoint.profiles.len().to_string(),
        );
        meta.insert(
            "iteration".to_string(),
            checkpoint.checkpoint_iteration.to_string(),
        );

        SaveManifest::add_entry(
            &mut self.manifest.checkpoints,
            SaveEntry {
                name: name.to_string(),
                filename,
                saved_at: timestamp(),
                size_bytes: size,
                metadata: meta,
            },
        );
        self.save_manifest()?;

        Ok(format!(
            "Saved checkpoint '{}' -> {} ({} bytes)",
            name,
            path.display(),
            size
        ))
    }

    /// Load a full engine checkpoint
    pub fn load_checkpoint(&mut self, name: &str) -> Result<EngineCheckpoint, String> {
        self.ensure_init()?;

        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.config.checkpoints_path().join(&filename);

        if !path.exists() {
            return Err(format!(
                "Checkpoint '{}' not found at {}",
                name,
                path.display()
            ));
        }

        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read checkpoint '{}': {}", name, e))?;
        serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse checkpoint '{}': {}", name, e))
    }

    // =================================================================
    // UTILITY METHODS
    // =================================================================

    /// Get a summary of all saved data
    pub fn summary(&mut self) -> Result<String, String> {
        self.ensure_init()?;

        let mut lines = vec![format!(
            "=== PERSISTENCE SUMMARY ({}) ===",
            self.config.base_dir.display()
        )];

        lines.push(format!("Personas: {}", self.manifest.personas.len()));
        for entry in &self.manifest.personas {
            let conv = entry
                .metadata
                .get("convergence")
                .cloned()
                .unwrap_or_default();
            lines.push(format!(
                "  {} (convergence: {}, {} bytes, saved: {})",
                entry.name, conv, entry.size_bytes, entry.saved_at
            ));
        }

        lines.push(format!("Profiles: {}", self.manifest.profiles.len()));
        for entry in &self.manifest.profiles {
            let provider = entry.metadata.get("provider").cloned().unwrap_or_default();
            lines.push(format!(
                "  {} ({}, {} bytes)",
                entry.name, provider, entry.size_bytes
            ));
        }

        lines.push(format!("Sessions: {}", self.manifest.sessions.len()));
        lines.push(format!("Checkpoints: {}", self.manifest.checkpoints.len()));

        // Total disk usage
        let total_bytes: u64 = self
            .manifest
            .personas
            .iter()
            .chain(self.manifest.profiles.iter())
            .chain(self.manifest.sessions.iter())
            .chain(self.manifest.checkpoints.iter())
            .map(|e| e.size_bytes)
            .sum();
        lines.push(format!("Total disk usage: {} bytes", total_bytes));

        Ok(lines.join("\n"))
    }

    /// Check if auto-save should trigger based on iteration count
    pub fn should_auto_save(&self, iteration: u64) -> bool {
        self.config.auto_save_enabled
            && iteration > 0
            && iteration % self.config.auto_save_interval == 0
    }

    /// Import a profile from an arbitrary file path
    pub fn import_profile_from(&mut self, path: &Path) -> Result<AiProfile, String> {
        self.ensure_init()?;

        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }

        let data = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let profile: AiProfile = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse profile from {}: {}", path.display(), e))?;

        // Also save a copy in our profiles directory
        self.save_profile(&profile)?;

        Ok(profile)
    }
}

impl Default for PersistenceManager {
    fn default() -> Self {
        PersistenceManager::with_default_config()
    }
}

// =================================================================
// HELPERS
// =================================================================

/// Sanitize a name for use as a filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Simple timestamp string (no external deps)
fn timestamp() -> String {
    // Use a simple monotonic counter since we don't have chrono
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("ts-{}", secs)
}

// =================================================================
// TESTS
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mimicry::analyzer::BehaviorSignature;
    use crate::mimicry::capability::CapabilityModule;
    use crate::mimicry::profile::AiProfileStore;
    use std::env;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn test_config() -> PersistenceConfig {
        let mut config = PersistenceConfig::default();
        // Use a unique temp directory per test to avoid parallel test conflicts
        let unique_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = env::temp_dir().join(format!(
            "rustyworm-test-{}-{}",
            std::process::id(),
            unique_id
        ));
        config.base_dir = dir;
        config
    }

    fn cleanup(config: &PersistenceConfig) {
        let _ = fs::remove_dir_all(&config.base_dir);
    }

    #[test]
    fn test_persistence_init() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());
        let result = pm.initialize();
        assert!(result.is_ok());
        assert!(config.personas_path().exists());
        assert!(config.profiles_path().exists());
        cleanup(&config);
    }

    #[test]
    fn test_save_load_persona() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());

        let profile = AiProfileStore::gpt4o_profile();
        let snapshot = CompoundPersonaSnapshot {
            profile,
            signature: BehaviorSignature::new("gpt4o"),
            capabilities: CapabilityModule::gpt4o_capabilities(),
            convergence_score: 0.75,
            compound_iterations: 42,
            created_at: "test".to_string(),
            last_updated: "test".to_string(),
        };

        let save_result = pm.save_persona("test-gpt4o", &snapshot);
        assert!(save_result.is_ok(), "Save failed: {:?}", save_result);

        let loaded = pm.load_persona("test-gpt4o");
        assert!(loaded.is_ok(), "Load failed: {:?}", loaded);
        let loaded = loaded.unwrap();
        assert_eq!(loaded.profile.id, "gpt4o");
        assert_eq!(loaded.convergence_score, 0.75);

        cleanup(&config);
    }

    #[test]
    fn test_save_load_profile() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());

        let profile = AiProfileStore::claude_profile();
        let save_result = pm.save_profile(&profile);
        assert!(save_result.is_ok());

        let loaded = pm.load_profile("claude");
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap().display_name, "Claude");

        cleanup(&config);
    }

    #[test]
    fn test_load_all_profiles() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());

        pm.save_profile(&AiProfileStore::gpt4o_profile()).unwrap();
        pm.save_profile(&AiProfileStore::claude_profile()).unwrap();

        let profiles = pm.load_all_profiles().unwrap();
        assert_eq!(profiles.len(), 2);

        cleanup(&config);
    }

    #[test]
    fn test_save_load_checkpoint() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());

        let checkpoint = EngineCheckpoint {
            profiles: vec![AiProfileStore::gpt4o_profile()],
            cached_signatures: vec!["{}".to_string()],
            saved_snapshots: HashMap::new(),
            hot_swap_entries: vec![],
            active_persona_id: Some("gpt4o".to_string()),
            checkpoint_iteration: 100,
        };

        let save_result = pm.save_checkpoint("test-checkpoint", &checkpoint);
        assert!(save_result.is_ok());

        let loaded = pm.load_checkpoint("test-checkpoint");
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap().checkpoint_iteration, 100);

        cleanup(&config);
    }

    #[test]
    fn test_delete_persona() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());

        let profile = AiProfileStore::gpt4o_profile();
        let snapshot = CompoundPersonaSnapshot {
            profile,
            signature: BehaviorSignature::new("gpt4o"),
            capabilities: CapabilityModule::gpt4o_capabilities(),
            convergence_score: 0.5,
            compound_iterations: 10,
            created_at: "test".to_string(),
            last_updated: "test".to_string(),
        };
        pm.save_persona("to-delete", &snapshot).unwrap();
        assert!(pm.load_persona("to-delete").is_ok());

        pm.delete_persona("to-delete").unwrap();
        assert!(pm.load_persona("to-delete").is_err());

        cleanup(&config);
    }

    #[test]
    fn test_auto_save_trigger() {
        let pm = PersistenceManager::with_default_config();
        assert!(!pm.should_auto_save(0));
        assert!(pm.should_auto_save(10));
        assert!(!pm.should_auto_save(7));
        assert!(pm.should_auto_save(20));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("gpt4o"), "gpt4o");
        assert_eq!(sanitize_filename("gpt-4o"), "gpt-4o");
        assert_eq!(sanitize_filename("my model/test"), "my_model_test");
        assert_eq!(sanitize_filename("hello world!"), "hello_world_");
    }

    #[test]
    fn test_manifest_serialization() {
        let mut manifest = SaveManifest::new();
        SaveManifest::add_entry(
            &mut manifest.personas,
            SaveEntry {
                name: "test".to_string(),
                filename: "test.json".to_string(),
                saved_at: "now".to_string(),
                size_bytes: 100,
                metadata: HashMap::new(),
            },
        );

        let json = serde_json::to_string(&manifest).unwrap();
        let restored: SaveManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.personas.len(), 1);
    }

    #[test]
    fn test_summary() {
        let config = test_config();
        let mut pm = PersistenceManager::new(config.clone());

        let result = pm.summary();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("PERSISTENCE SUMMARY"));

        cleanup(&config);
    }
}
