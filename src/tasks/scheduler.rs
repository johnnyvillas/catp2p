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

//! Task scheduling functionality.

use crate::error::Error;
use crate::tasks::{Task, TaskExecutor, TaskResourceType}; // Removed unused TaskStatus import
// Removed unused async_trait import
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration; // Removed unused Instant import

/// Task scheduler for distributing tasks to executors.
#[allow(dead_code)]
pub struct TaskScheduler {
    cpu_executor: Option<Arc<dyn TaskExecutor + Send + Sync>>,
    gpu_executor: Option<Arc<dyn TaskExecutor + Send + Sync>>,
    pending_tasks: Arc<Mutex<Vec<Task>>>,
    running_tasks: Arc<Mutex<HashMap<String, Task>>>,
    completed_tasks: Arc<Mutex<HashMap<String, Task>>>,
    max_concurrent_tasks: usize,
    task_timeout: Duration,
}

impl TaskScheduler {
    /// Creates a new TaskScheduler.
    pub fn new(max_concurrent_tasks: usize, task_timeout: Duration) -> Self {
        Self {
            cpu_executor: None,
            gpu_executor: None,
            pending_tasks: Arc::new(Mutex::new(Vec::new())),
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            completed_tasks: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_tasks,
            task_timeout,
        }
    }
    
    /// Sets the CPU executor.
    pub fn set_cpu_executor(&mut self, executor: Arc<dyn TaskExecutor + Send + Sync>) {
        self.cpu_executor = Some(executor);
    }
    
    /// Sets the GPU executor.
    pub fn set_gpu_executor(&mut self, executor: Arc<dyn TaskExecutor + Send + Sync>) {
        self.gpu_executor = Some(executor);
    }
    
    /// Schedules a task for execution.
    pub async fn schedule_task(&self, task: Task) -> Result<(), Error> {
        let mut pending_tasks = self.pending_tasks.lock().await;
        pending_tasks.push(task);
        Ok(())
    }
    
    /// Starts the scheduler.
    pub async fn start(&self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }
    
    /// Stops the scheduler.
    pub async fn stop(&self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }
    
    /// Gets the appropriate executor for a task.
    #[allow(dead_code)]
    fn get_executor_for_task(&self, task: &Task) -> Option<Arc<dyn TaskExecutor + Send + Sync>> {
        match task.resource_type {
            TaskResourceType::Cpu | TaskResourceType::Memory | TaskResourceType::Disk => self.cpu_executor.clone(),
            TaskResourceType::Gpu => self.gpu_executor.clone(),
            TaskResourceType::Network => None, // Network tasks are handled differently
        }
    }
}
