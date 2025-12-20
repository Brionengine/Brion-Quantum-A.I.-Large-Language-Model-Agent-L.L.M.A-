// Concrete Agent Implementations with File Modification Capabilities
// These agents can actually read, analyze, and modify website files

use crate::agents::{
    agents::{Agent, AgentType, AgentTask, AgentResult},
    file_ops::FileOperations,
    version_control::{Change, ChangeType, VersionControl},
};
use std::sync::Arc;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

pub struct EnhancedUIAgent {
    id: String,
    version_control: Option<Arc<VersionControl>>,
}

impl EnhancedUIAgent {
    pub fn new() -> Self {
        Self {
            id: format!("ui-agent-{}", Utc::now().timestamp_millis()),
            version_control: None,
        }
    }

    pub fn with_version_control(mut self, vc: Arc<VersionControl>) -> Self {
        self.version_control = Some(vc);
        self
    }

impl EnhancedUIAgent {
    pub fn new() -> Self {
        Self {
            id: format!("ui-agent-{}", Utc::now().timestamp_millis()),
        }
    }

    fn improve_css(&self, content: &str) -> String {
        let mut improved = content.to_string();
        
        // Add smooth transitions if not present
        if !improved.contains("transition:") && !improved.contains("transition ") {
            // Add to root if exists
            if improved.contains(":root {") {
                improved = improved.replace(
                    ":root {",
                    ":root {\n    --transition-smooth: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);"
                );
            }
        }
        
        // Ensure responsive design
        if !improved.contains("@media") {
            let responsive_css = "\n\n/* Responsive Design Enhancements */\n@media (max-width: 768px) {\n    /* Mobile optimizations */\n}\n";
            improved.push_str(responsive_css);
        }
        
        improved
    }

    fn improve_html(&self, content: &str) -> String {
        let mut improved = content.to_string();
        
        // Add meta tags for better UX if missing
        if !improved.contains("theme-color") {
            if let Some(head_end) = improved.find("</head>") {
                let meta_theme = "\n    <meta name=\"theme-color\" content=\"#00d4ff\">";
                improved.insert_str(head_end, meta_theme);
            }
        }
        
        // Ensure proper semantic structure
        if !improved.contains("aria-label") && improved.contains("<button") {
            improved = improved.replace(
                "<button",
                "<button aria-label=\""
            );
        }
        
        improved
    }
}

impl Agent for EnhancedUIAgent {
    fn get_type(&self) -> AgentType {
        AgentType::UIAgent
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn can_handle(&self, task: &AgentTask) -> bool {
        task.agent_type == AgentType::UIAgent
    }

    fn execute_task(&self, task: &AgentTask, base_path: &PathBuf) -> Result<AgentResult, String> {
        let target_file = task.target_file.as_ref()
            .map(|f| base_path.join(f))
            .unwrap_or_else(|| base_path.join("styles/main.css"));
        
        if !target_file.exists() {
            return Ok(AgentResult {
                task_id: task.id.clone(),
                agent_id: self.id.clone(),
                success: false,
                changes: vec![],
                message: format!("File not found: {}", target_file.display()),
                metrics: HashMap::new(),
            });
        }

        let before = FileOperations::read_file(&target_file)?;
        let after = if target_file.extension().and_then(|s| s.to_str()) == Some("css") {
            self.improve_css(&before)
        } else if target_file.extension().and_then(|s| s.to_str()) == Some("html") {
            self.improve_html(&before)
        } else {
            before.clone()
        };

        if before == after {
            return Ok(AgentResult {
                task_id: task.id.clone(),
                agent_id: self.id.clone(),
                success: true,
                changes: vec![],
                message: "No improvements needed".to_string(),
                metrics: HashMap::new(),
            });
        }

        // Create change record
        let file_path_str = target_file.strip_prefix(base_path)
            .unwrap_or(&target_file)
            .to_string_lossy()
            .to_string();
        
        let change = FileOperations::create_change(
            &self.id,
            "UIAgent",
            file_path_str.clone(),
            ChangeType::UpdateStyle,
            before.clone(),
            after.clone(),
        );

        let change_id = change.id.clone();

        // Record change in version control if available
        if let Some(ref vc) = self.version_control {
            vc.record_change(change.clone());
        }

        // Apply the change
        FileOperations::apply_change(&change, base_path)?;

        let mut metrics = HashMap::new();
        metrics.insert("lines_added".to_string(), 
            (after.lines().count() as i32 - before.lines().count() as i32) as f64);
        metrics.insert("file_size_change".to_string(), 
            (after.len() as i32 - before.len() as i32) as f64);

        Ok(AgentResult {
            task_id: task.id.clone(),
            agent_id: self.id.clone(),
            success: true,
            changes: vec![change_id],
            message: "UI improvements applied successfully".to_string(),
            metrics,
        })
    }
}

pub struct EnhancedPerformanceAgent {
    id: String,
    version_control: Option<Arc<VersionControl>>,
}

impl EnhancedPerformanceAgent {
    pub fn new() -> Self {
        Self {
            id: format!("perf-agent-{}", Utc::now().timestamp_millis()),
            version_control: None,
        }
    }

    pub fn with_version_control(mut self, vc: Arc<VersionControl>) -> Self {
        self.version_control = Some(vc);
        self
    }

impl EnhancedPerformanceAgent {
    pub fn new() -> Self {
        Self {
            id: format!("perf-agent-{}", Utc::now().timestamp_millis()),
        }
    }

    fn optimize_js(&self, content: &str) -> String {
        let mut optimized = content.to_string();
        
        // Add debouncing for scroll events
        if optimized.contains("addEventListener('scroll'") && !optimized.contains("debounce") {
            let debounce_func = r#"
// Performance: Debounce function
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}
"#;
            if let Some(pos) = optimized.find("document.addEventListener('DOMContentLoaded'") {
                optimized.insert_str(pos, debounce_func);
            }
        }
        
        // Optimize canvas animations
        if optimized.contains("requestAnimationFrame") && !optimized.contains("cancelAnimationFrame") {
            // Add pause on visibility change if not present
            if !optimized.contains("visibilitychange") {
                let visibility_opt = r#"
// Performance: Pause animations when tab is hidden
document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
        // Pause heavy animations
    }
});
"#;
                optimized.push_str(visibility_opt);
            }
        }
        
        optimized
    }

    fn optimize_html(&self, content: &str) -> String {
        let mut optimized = content.to_string();
        
        // Add lazy loading for images if not present
        if optimized.contains("<img") && !optimized.contains("loading=") {
            optimized = optimized.replace("<img", "<img loading=\"lazy\"");
        }
        
        // Add preconnect for external resources
        if optimized.contains("fonts.googleapis.com") && !optimized.contains("preconnect") {
            if let Some(head_pos) = optimized.find("<head>") {
                let preconnect = "\n    <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n    <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>";
                optimized.insert_str(head_pos + 6, preconnect);
            }
        }
        
        optimized
    }
}

impl Agent for EnhancedPerformanceAgent {
    fn get_type(&self) -> AgentType {
        AgentType::PerformanceAgent
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn can_handle(&self, task: &AgentTask) -> bool {
        task.agent_type == AgentType::PerformanceAgent
    }

    fn execute_task(&self, task: &AgentTask, base_path: &PathBuf) -> Result<AgentResult, String> {
        // Try to optimize JavaScript first
        let js_file = base_path.join("scripts/main.js");
        let mut changes = Vec::new();
        let mut metrics = HashMap::new();

        if js_file.exists() {
            let before = FileOperations::read_file(&js_file)?;
            let after = self.optimize_js(&before);
            
            if before != after {
                let change = FileOperations::create_change(
                    &self.id,
                    "PerformanceAgent",
                    "scripts/main.js".to_string(),
                    ChangeType::Optimize,
                    before.clone(),
                    after.clone(),
                );
                let change_id = change.id.clone();
                
                // Record change in version control if available
                if let Some(ref vc) = self.version_control {
                    vc.record_change(change.clone());
                }
                
                FileOperations::apply_change(&change, base_path)?;
                changes.push(change_id);
            }
        }

        // Optimize HTML
        let html_file = base_path.join("index.html");
        if html_file.exists() {
            let before = FileOperations::read_file(&html_file)?;
            let after = self.optimize_html(&before);
            
            if before != after {
                let change = FileOperations::create_change(
                    &self.id,
                    "PerformanceAgent",
                    "index.html".to_string(),
                    ChangeType::Optimize,
                    before.clone(),
                    after.clone(),
                );
                let change_id = change.id.clone();
                
                // Record change in version control if available
                if let Some(ref vc) = self.version_control {
                    vc.record_change(change.clone());
                }
                
                FileOperations::apply_change(&change, base_path)?;
                changes.push(change_id);
            }
        }

        Ok(AgentResult {
            task_id: task.id.clone(),
            agent_id: self.id.clone(),
            success: true,
            changes,
            message: format!("Performance optimizations applied: {} changes", changes.len()),
            metrics,
        })
    }
}

