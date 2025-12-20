// Version Control System for AI Agent Changes
// Tracks all modifications with full rollback capability

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub agent_type: String,
    pub file_path: String,
    pub change_type: ChangeType,
    pub before: String,
    pub after: String,
    pub metadata: HashMap<String, String>,
    pub evaluation_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Create,
    Modify,
    Delete,
    Optimize,
    AddFeature,
    UpdateContent,
    UpdateStyle,
    AddImage,
    AddModule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSnapshot {
    pub version_id: String,
    pub timestamp: DateTime<Utc>,
    pub changes: Vec<String>, // Change IDs
    pub total_files_changed: usize,
    pub description: String,
}

pub struct VersionControl {
    changes: Arc<RwLock<HashMap<String, Change>>>,
    versions: Arc<RwLock<Vec<VersionSnapshot>>>,
    current_version: Arc<RwLock<String>>,
    base_path: PathBuf,
}

impl VersionControl {
    pub fn new(base_path: PathBuf) -> Self {
        let initial_version = format!("v{}", Utc::now().timestamp());
        
        Self {
            changes: Arc::new(RwLock::new(HashMap::new())),
            versions: Arc::new(RwLock::new(Vec::new())),
            current_version: Arc::new(RwLock::new(initial_version)),
            base_path,
        }
    }

    pub fn record_change(&self, change: Change) -> String {
        let change_id = change.id.clone();
        self.changes.write().insert(change_id.clone(), change);
        change_id
    }

    pub fn get_change(&self, change_id: &str) -> Option<Change> {
        self.changes.read().get(change_id).cloned()
    }

    pub fn create_snapshot(&self, description: String) -> String {
        let version_id = format!("v{}", Utc::now().timestamp_millis());
        let changes: Vec<String> = self.changes.read()
            .values()
            .filter(|c| c.evaluation_score.is_none() || c.evaluation_score.unwrap() > 0.5)
            .map(|c| c.id.clone())
            .collect();
        
        let snapshot = VersionSnapshot {
            version_id: version_id.clone(),
            timestamp: Utc::now(),
            total_files_changed: changes.len(),
            changes,
            description,
        };
        
        self.versions.write().push(snapshot);
        *self.current_version.write() = version_id.clone();
        version_id
    }

    pub fn rollback_to_version(&self, version_id: &str) -> Result<Vec<Change>, String> {
        let versions = self.versions.read();
        let version = versions.iter()
            .find(|v| v.version_id == version_id)
            .ok_or_else(|| format!("Version {} not found", version_id))?;
        
        let changes_to_rollback: Vec<Change> = version.changes.iter()
            .filter_map(|change_id| self.get_change(change_id))
            .collect();
        
        Ok(changes_to_rollback)
    }

    pub fn rollback_change(&self, change_id: &str) -> Result<Change, String> {
        let change = self.changes.read()
            .get(change_id)
            .cloned()
            .ok_or_else(|| format!("Change {} not found", change_id))?;
        
        Ok(change)
    }

    pub fn get_all_changes(&self) -> Vec<Change> {
        self.changes.read().values().cloned().collect()
    }

    pub fn get_recent_changes(&self, limit: usize) -> Vec<Change> {
        let mut changes: Vec<Change> = self.changes.read().values().cloned().collect();
        changes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        changes.into_iter().take(limit).collect()
    }

    pub fn get_current_version(&self) -> String {
        self.current_version.read().clone()
    }

    pub fn get_version_history(&self) -> Vec<VersionSnapshot> {
        self.versions.read().clone()
    }
}

