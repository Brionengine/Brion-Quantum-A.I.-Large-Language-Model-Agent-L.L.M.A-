// Task Queue for AI Agent System
// Manages and prioritizes tasks for agents

use crate::agents::agents::{AgentTask, AgentType};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use parking_lot::RwLock;
use chrono::Utc;

#[derive(Debug, Clone)]
struct PrioritizedTask {
    task: AgentTask,
}

impl PartialEq for PrioritizedTask {
    fn eq(&self, other: &Self) -> bool {
        self.task.priority == other.task.priority
    }
}

impl Eq for PrioritizedTask {}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first, then by creation time
        match other.task.priority.cmp(&self.task.priority) {
            Ordering::Equal => self.task.created_at.cmp(&other.task.created_at),
            other => other,
        }
    }
}

pub struct TaskQueue {
    tasks: Arc<RwLock<BinaryHeap<PrioritizedTask>>>,
    completed_tasks: Arc<RwLock<Vec<AgentTask>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(BinaryHeap::new())),
            completed_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_task(&self, task: AgentTask) {
        self.tasks.write().push(PrioritizedTask { task });
    }

    pub fn get_next_task(&self, agent_type: Option<AgentType>) -> Option<AgentTask> {
        let mut tasks = self.tasks.write();
        let mut temp_heap = BinaryHeap::new();
        let mut found_task = None;

        while let Some(prioritized) = tasks.pop() {
            if let Some(ref filter_type) = agent_type {
                if prioritized.task.agent_type == *filter_type {
                    found_task = Some(prioritized.task);
                    break;
                }
            } else {
                found_task = Some(prioritized.task);
                break;
            }
            temp_heap.push(prioritized);
        }

        // Put remaining tasks back
        while let Some(prioritized) = temp_heap.pop() {
            tasks.push(prioritized);
        }

        found_task
    }

    pub fn mark_completed(&self, task: AgentTask) {
        self.completed_tasks.write().push(task);
    }

    pub fn get_queue_size(&self) -> usize {
        self.tasks.read().len()
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed_tasks.read().len()
    }

    pub fn clear_completed(&self) {
        self.completed_tasks.write().clear();
    }
}

use std::sync::Arc;

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

