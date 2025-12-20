// File Operations for AI Agents
// Handles reading, writing, and modifying website files

use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use crate::agents::version_control::{Change, ChangeType};
use chrono::Utc;
use uuid::Uuid;

pub struct FileOperations;

impl FileOperations {
    pub fn read_file(path: &Path) -> Result<String, String> {
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))
    }

    pub fn write_file(path: &Path, content: &str) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
        }
        
        let mut file = fs::File::create(path)
            .map_err(|e| format!("Failed to create file {}: {}", path.display(), e))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write file {}: {}", path.display(), e))?;
        
        Ok(())
    }

    pub fn create_change(
        agent_id: &str,
        agent_type: &str,
        file_path: String,
        change_type: ChangeType,
        before: String,
        after: String,
    ) -> Change {
        Change {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            agent_id: agent_id.to_string(),
            agent_type: agent_type.to_string(),
            file_path,
            change_type,
            before,
            after,
            metadata: std::collections::HashMap::new(),
            evaluation_score: None,
        }
    }

    pub fn apply_change(change: &Change, base_path: &PathBuf) -> Result<(), String> {
        let file_path = base_path.join(&change.file_path);
        
        match change.change_type {
            ChangeType::Create | ChangeType::Modify | ChangeType::Optimize | 
            ChangeType::AddFeature | ChangeType::UpdateContent | ChangeType::UpdateStyle => {
                Self::write_file(&file_path, &change.after)?;
            }
            ChangeType::Delete => {
                if file_path.exists() {
                    fs::remove_file(&file_path)
                        .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
                }
            }
            ChangeType::AddImage | ChangeType::AddModule => {
                // For images and modules, we'd handle them differently
                // This is a placeholder
                Self::write_file(&file_path, &change.after)?;
            }
        }
        
        Ok(())
    }

    pub fn rollback_change(change: &Change, base_path: &PathBuf) -> Result<(), String> {
        let file_path = base_path.join(&change.file_path);
        
        match change.change_type {
            ChangeType::Create => {
                // Delete the file that was created
                if file_path.exists() {
                    fs::remove_file(&file_path)
                        .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
                }
            }
            ChangeType::Modify | ChangeType::Optimize | ChangeType::AddFeature |
            ChangeType::UpdateContent | ChangeType::UpdateStyle => {
                // Restore the previous content
                Self::write_file(&file_path, &change.before)?;
            }
            ChangeType::Delete => {
                // Restore the deleted file
                Self::write_file(&file_path, &change.before)?;
            }
            ChangeType::AddImage | ChangeType::AddModule => {
                // Remove the added image/module
                if file_path.exists() {
                    fs::remove_file(&file_path)
                        .map_err(|e| format!("Failed to remove file {}: {}", file_path.display(), e))?;
                }
            }
        }
        
        Ok(())
    }
}

