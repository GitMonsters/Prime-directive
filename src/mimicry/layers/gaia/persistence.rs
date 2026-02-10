//! GAIA pattern persistence for saving and loading learned patterns.
//!
//! This module provides save/load functionality for GAIA patterns,
//! enabling learned intuition to persist across sessions.

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::pattern::{Pattern, PatternId, PatternMemory};
use super::{GaiaError, GaiaResult};
use crate::mimicry::layers::layer::Domain;

/// Serializable pattern data for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternData {
    /// Pattern identifier.
    pub id: PatternId,
    /// Domain this pattern belongs to.
    pub domain: Domain,
    /// The pattern fingerprint (feature vector).
    pub fingerprint: Vec<f32>,
    /// Weight representing importance/reliability.
    pub weight: f32,
    /// Success rate from reinforcement.
    pub success_rate: f32,
    /// Number of times this pattern has been activated.
    pub activation_count: u64,
    /// Cross-links to related patterns.
    pub cross_links: Vec<PatternId>,
}

impl From<&Pattern> for PatternData {
    fn from(pattern: &Pattern) -> Self {
        Self {
            id: pattern.id().to_string(),
            domain: pattern.domain(),
            fingerprint: pattern.fingerprint().to_vec(),
            weight: pattern.weight(),
            success_rate: pattern.success_rate(),
            activation_count: pattern.activation_count(),
            cross_links: pattern.cross_links().to_vec(),
        }
    }
}

impl From<PatternData> for Pattern {
    fn from(data: PatternData) -> Self {
        let mut pattern = Pattern::new(&data.id, data.domain)
            .with_fingerprint(data.fingerprint)
            .with_weight(data.weight);

        // Add cross-links using builder pattern
        for link in data.cross_links {
            pattern = pattern.with_cross_link(link);
        }

        pattern
    }
}

/// Snapshot of the entire GAIA pattern memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaiaSnapshot {
    /// Version for compatibility.
    pub version: String,
    /// Timestamp when snapshot was created.
    pub created_at: String,
    /// All patterns in memory.
    pub patterns: Vec<PatternData>,
    /// Statistics about the pattern memory.
    pub stats: SnapshotStats,
}

/// Statistics included in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotStats {
    /// Total number of patterns.
    pub total_patterns: usize,
    /// Patterns per domain.
    pub patterns_by_domain: HashMap<String, usize>,
    /// Average pattern weight.
    pub average_weight: f32,
    /// Average success rate.
    pub average_success_rate: f32,
    /// Total cross-links.
    pub total_cross_links: usize,
}

/// Persistence manager for GAIA patterns.
pub struct GaiaPersistence {
    /// Base directory for persistence.
    base_dir: PathBuf,
}

impl GaiaPersistence {
    /// Create a new persistence manager with the given base directory.
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Create with default directory (~/.rustyworm/gaia/).
    pub fn default_location() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let base = PathBuf::from(home).join(".rustyworm").join("gaia");
        Self::new(base)
    }

    /// Initialize the persistence directory.
    pub fn initialize(&self) -> GaiaResult<()> {
        fs::create_dir_all(&self.base_dir).map_err(|e| {
            GaiaError::ConfigError(format!("Failed to create GAIA directory: {}", e))
        })?;
        Ok(())
    }

    /// Save pattern memory to disk.
    pub fn save(&self, name: &str, memory: &PatternMemory) -> GaiaResult<PathBuf> {
        self.initialize()?;

        let patterns: Vec<PatternData> = memory
            .all_patterns()
            .iter()
            .map(|p| PatternData::from(p))
            .collect();

        let stats = self.compute_stats(&patterns);

        let snapshot = GaiaSnapshot {
            version: "1.0.0".to_string(),
            created_at: chrono_now(),
            patterns,
            stats,
        };

        let json = serde_json::to_string_pretty(&snapshot)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to serialize patterns: {}", e)))?;

        let path = self.base_dir.join(format!("{}.gaia.json", name));
        let mut file = fs::File::create(&path)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to create file: {}", e)))?;
        file.write_all(json.as_bytes())
            .map_err(|e| GaiaError::ConfigError(format!("Failed to write file: {}", e)))?;

        Ok(path)
    }

    /// Load pattern memory from disk.
    pub fn load(&self, name: &str) -> GaiaResult<PatternMemory> {
        let path = self.base_dir.join(format!("{}.gaia.json", name));

        if !path.exists() {
            return Err(GaiaError::PatternNotFound(format!(
                "No saved patterns found: {}",
                path.display()
            )));
        }

        let mut file = fs::File::open(&path)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to open file: {}", e)))?;

        let mut json = String::new();
        file.read_to_string(&mut json)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to read file: {}", e)))?;

        let snapshot: GaiaSnapshot = serde_json::from_str(&json)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to parse patterns: {}", e)))?;

        let memory = PatternMemory::new();
        for data in snapshot.patterns {
            let pattern: Pattern = data.into();
            // Ignore registration errors for individual patterns
            let _ = memory.register(pattern);
        }

        Ok(memory)
    }

    /// List all saved pattern snapshots.
    pub fn list(&self) -> GaiaResult<Vec<String>> {
        if !self.base_dir.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.base_dir)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to read directory: {}", e)))?;

        let names: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.ends_with(".gaia.json") {
                    Some(name.trim_end_matches(".gaia.json").to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(names)
    }

    /// Delete a saved pattern snapshot.
    pub fn delete(&self, name: &str) -> GaiaResult<()> {
        let path = self.base_dir.join(format!("{}.gaia.json", name));

        if !path.exists() {
            return Err(GaiaError::PatternNotFound(format!(
                "No saved patterns found: {}",
                name
            )));
        }

        fs::remove_file(&path)
            .map_err(|e| GaiaError::ConfigError(format!("Failed to delete file: {}", e)))?;

        Ok(())
    }

    /// Get summary of saved patterns.
    pub fn summary(&self) -> GaiaResult<String> {
        let names = self.list()?;

        if names.is_empty() {
            return Ok("No saved GAIA patterns.".to_string());
        }

        let mut lines = vec![format!("GAIA Patterns ({}):", names.len())];

        for name in &names {
            let path = self.base_dir.join(format!("{}.gaia.json", name));
            if let Ok(metadata) = fs::metadata(&path) {
                let size = metadata.len();
                lines.push(format!("  {} ({} bytes)", name, size));
            } else {
                lines.push(format!("  {}", name));
            }
        }

        Ok(lines.join("\n"))
    }

    /// Compute statistics for a set of patterns.
    fn compute_stats(&self, patterns: &[PatternData]) -> SnapshotStats {
        let mut patterns_by_domain: HashMap<String, usize> = HashMap::new();
        let mut total_weight = 0.0f32;
        let mut total_success = 0.0f32;
        let mut total_links = 0usize;

        for pattern in patterns {
            *patterns_by_domain
                .entry(format!("{:?}", pattern.domain))
                .or_insert(0) += 1;
            total_weight += pattern.weight;
            total_success += pattern.success_rate;
            total_links += pattern.cross_links.len();
        }

        let n = patterns.len().max(1) as f32;

        SnapshotStats {
            total_patterns: patterns.len(),
            patterns_by_domain,
            average_weight: total_weight / n,
            average_success_rate: total_success / n,
            total_cross_links: total_links,
        }
    }
}

impl Default for GaiaPersistence {
    fn default() -> Self {
        Self::default_location()
    }
}

/// Get current timestamp as string.
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_data_conversion() {
        let pattern = Pattern::new("test", Domain::Physics)
            .with_fingerprint(vec![0.5, 0.5])
            .with_weight(1.0);

        let data = PatternData::from(&pattern);
        assert_eq!(data.id, "test");
        assert_eq!(data.domain, Domain::Physics);

        let restored: Pattern = data.into();
        assert_eq!(restored.id(), "test");
    }

    #[test]
    fn test_snapshot_stats() {
        let persistence = GaiaPersistence::new("/tmp/test_gaia");

        let patterns = vec![
            PatternData {
                id: "p1".to_string(),
                domain: Domain::Physics,
                fingerprint: vec![0.5],
                weight: 1.0,
                success_rate: 0.8,
                activation_count: 10,
                cross_links: vec!["p2".to_string()],
            },
            PatternData {
                id: "p2".to_string(),
                domain: Domain::Language,
                fingerprint: vec![0.5],
                weight: 0.8,
                success_rate: 0.7,
                activation_count: 5,
                cross_links: vec![],
            },
        ];

        let stats = persistence.compute_stats(&patterns);
        assert_eq!(stats.total_patterns, 2);
        assert_eq!(stats.total_cross_links, 1);
    }
}
