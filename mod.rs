// Brion Quantum AI Lab - Autonomous AI Agent System
// Perpetual Development and Optimization Engine

pub mod orchestrator;
pub mod evaluator;
pub mod version_control;
pub mod agents;
pub mod task_queue;
pub mod file_ops;
pub mod agent_impl;

pub use orchestrator::AgentOrchestrator;
pub use evaluator::ChangeEvaluator;
pub use version_control::VersionControl;
pub use agents::{Agent, AgentType, AgentTask, AgentResult};
pub use task_queue::TaskQueue;
pub use file_ops::FileOperations;

