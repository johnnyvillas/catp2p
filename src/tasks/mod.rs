/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Task management functionality for distributing and executing tasks.

pub mod cpu;
pub mod gpu;
pub mod scheduler;

use crate::error::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
// Remove the following line:
// use std::time::Instant; // Removed unused Duration import

/// Task resource type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskResourceType {
    /// CPU task.
    Cpu,
    /// GPU task.
    Gpu,
    /// Memory-intensive task.
    Memory,
    /// Disk-intensive task.
    Disk,
    /// Network-intensive task.
    Network,
}

/// Task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending execution.
    Pending,
    /// Task is currently running.
    Running,
    /// Task has completed successfully.
    Completed,
    /// Task has failed.
    Failed,
    /// Task has been cancelled.
    Cancelled,
}

/// Task data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task ID.
    pub id: String,
    /// Task resource type.
    pub resource_type: TaskResourceType,
    /// Task data.
    pub data: Vec<u8>,
    /// Task status.
    pub status: TaskStatus,
    /// Task creation time.
    pub created_at: u64,
    /// Task completion time, if completed.
    pub completed_at: Option<u64>,
}

/// Task executor trait.
#[async_trait]
pub trait TaskExecutor {
    /// Executes a task.
    async fn execute(&self, task: &Task) -> Result<String, Error>;
}

/// Task manager for distributing and executing tasks.
pub struct TaskManager {
    // Will add more fields as we implement the task management functionality
}

impl TaskManager {
    /// Creates a new TaskManager.
    pub fn new() -> Self {
        Self {}
    }
    
    /// Submits a task for execution.
    pub fn submit_task(&self, _task: Task) -> Result<String, Error> {
        // Implementation will be added later
        Ok("task-id".to_string())
    }
    
    /// Cancels a task.
    pub fn cancel_task(&self, _task_id: &str) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }
    
    /// Gets the status of a task.
    pub fn get_task_status(&self, _task_id: &str) -> Result<TaskStatus, Error> {
        // Implementation will be added later
        Ok(TaskStatus::Pending)
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
