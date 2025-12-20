// AI Agent Orchestrator
// Manages all agents and coordinates continuous improvement

use crate::agents::{
    agents::{Agent, AgentType, AgentTask, AgentResult},
    evaluator::ChangeEvaluator,
    version_control::{VersionControl, Change, ChangeType},
    task_queue::TaskQueue,
};
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::{interval, Duration};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;
use log::{info, warn, error};

pub struct AgentOrchestrator {
    agents: Arc<RwLock<HashMap<AgentType, Vec<Box<dyn Agent + Send + Sync>>>>>,
    version_control: Arc<VersionControl>,
    evaluator: Arc<ChangeEvaluator>,
    task_queue: Arc<TaskQueue>,
    base_path: PathBuf,
    is_running: Arc<RwLock<bool>>,
    stats: Arc<RwLock<OrchestratorStats>>,
}

#[derive(Debug, Clone, Default)]
pub struct OrchestratorStats {
    pub total_tasks_executed: usize,
    pub successful_changes: usize,
    pub rolled_back_changes: usize,
    pub agents_active: usize,
    pub last_activity: Option<chrono::DateTime<Utc>>,
}

impl AgentOrchestrator {
    pub fn new(base_path: PathBuf) -> Self {
        let version_control = Arc::new(VersionControl::new(base_path.clone()));
        let evaluator = Arc::new(ChangeEvaluator::new());
        let task_queue = Arc::new(TaskQueue::new());

        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            version_control,
            evaluator,
            task_queue,
            base_path,
            is_running: Arc::new(RwLock::new(false)),
            stats: Arc::new(RwLock::new(OrchestratorStats::default())),
        }
    }

    pub fn register_agent(&self, agent: Box<dyn Agent + Send + Sync>) {
        let agent_type = agent.get_type();
        self.agents.write()
            .entry(agent_type)
            .or_insert_with(Vec::new)
            .push(agent);
        
        let mut stats = self.stats.write();
        stats.agents_active = self.agents.read().values().map(|v| v.len()).sum();
    }

    pub fn start_continuous_improvement(self: Arc<Self>) {
        *self.is_running.write() = true;
        let orchestrator = Arc::clone(&self);
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Check every 30 seconds
            
            loop {
                interval.tick().await;
                
                if !*orchestrator.is_running.read() {
                    break;
                }

                // Generate tasks automatically
                orchestrator.generate_improvement_tasks().await;
                
                // Process tasks
                orchestrator.process_task_queue().await;
            }
        });
    }

    pub fn stop(&self) {
        *self.is_running.write() = false;
    }

    async fn generate_improvement_tasks(&self) {
        // Automatically generate tasks for continuous improvement
        let task_types = vec![
            (AgentType::PerformanceAgent, "Optimize page load performance", 7),
            (AgentType::UIAgent, "Improve user interface aesthetics", 6),
            (AgentType::ContentAgent, "Update and enhance content", 5),
            (AgentType::FeatureAgent, "Add new interactive features", 8),
            (AgentType::AccessibilityAgent, "Enhance accessibility", 6),
            (AgentType::SEOAgent, "Improve SEO optimization", 5),
        ];

        for (agent_type, description, priority) in task_types {
            let task = AgentTask {
                id: Uuid::new_v4().to_string(),
                agent_type: agent_type.clone(),
                priority,
                description: description.to_string(),
                target_file: None,
                parameters: HashMap::new(),
                created_at: Utc::now(),
            };

            self.task_queue.add_task(task);
        }
    }

    async fn process_task_queue(&self) {
        let agents = self.agents.read();
        
        for (agent_type, agent_list) in agents.iter() {
            if agent_list.is_empty() {
                continue;
            }

            // Get next task for this agent type
            if let Some(task) = self.task_queue.get_next_task(Some(agent_type.clone())) {
                // Select an agent (round-robin or based on availability)
                if let Some(agent) = agent_list.first() {
                    match self.execute_task_with_agent(agent.as_ref(), &task).await {
                        Ok(result) => {
                            info!("Task {} completed by agent {}", task.id, result.agent_id);
                            self.task_queue.mark_completed(task);
                            
                            let mut stats = self.stats.write();
                            stats.total_tasks_executed += 1;
                            if result.success {
                                stats.successful_changes += result.changes.len();
                            }
                            stats.last_activity = Some(Utc::now());
                        }
                        Err(e) => {
                            error!("Task {} failed: {}", task.id, e);
                        }
                    }
                }
            }
        }
    }

    async fn execute_task_with_agent(
        &self,
        agent: &dyn Agent,
        task: &AgentTask,
    ) -> Result<AgentResult, String> {
        let result = agent.execute_task(task, &self.base_path)?;

        // Record and evaluate changes
        for change_id in &result.changes {
            // Get the change from the agent (agents should store changes temporarily)
            // For now, we'll create a placeholder evaluation
            // In a full implementation, agents would return full Change objects
            
            // The change should already be recorded by the agent via version control
            // We just need to evaluate it
            if let Some(change) = self.version_control.get_change(change_id) {
                // Evaluate the change
                let evaluation = self.evaluator.evaluate_change(&change);
                
                // Update change with evaluation score
                let mut updated_change = change.clone();
                updated_change.evaluation_score = Some(evaluation.overall_score);
                self.version_control.record_change(updated_change.clone());

                // Decide whether to keep or rollback
                if !evaluation.should_keep {
                    warn!("Change {} scored below threshold ({:.2}), rolling back", 
                        change_id, evaluation.overall_score);
                    self.rollback_change(change_id)?;
                    
                    let mut stats = self.stats.write();
                    stats.rolled_back_changes += 1;
                } else {
                    info!("Change {} approved with score {:.2}", 
                        change_id, evaluation.overall_score);
                }
            }
        }

        Ok(result)
    }

    pub fn rollback_change(&self, change_id: &str) -> Result<(), String> {
        let change = self.version_control.rollback_change(change_id)?;
        
        // Restore the file to its previous state
        use crate::agents::file_ops::FileOperations;
        FileOperations::rollback_change(&change, &self.base_path)?;
        
        info!("Rolled back change {} in file {}", change_id, change.file_path);
        
        Ok(())
    }

    pub fn get_stats(&self) -> OrchestratorStats {
        self.stats.read().clone()
    }

    pub fn get_version_control(&self) -> Arc<VersionControl> {
        self.version_control.clone()
    }

    pub fn get_task_queue(&self) -> Arc<TaskQueue> {
        self.task_queue.clone()
    }
}


