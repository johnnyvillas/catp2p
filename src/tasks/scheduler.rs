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
use crate::tasks::{Task, TaskExecutor, TaskStatus, TaskResourceType};
use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};

/// A task scheduler that manages task execution.
pub struct TaskScheduler {
    cpu_executor: Arc<dyn TaskExecutor + Send + Sync>,
    gpu_executor: Option<Arc<dyn TaskExecutor + Send + Sync>>,
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    max_concurrent_tasks: usize,
}

impl TaskScheduler {
    /// Creates a new TaskScheduler with the given executors.
    pub fn new(
        cpu_executor: Arc<dyn TaskExecutor + Send + Sync>,
        gpu_executor: Option<Arc<dyn TaskExecutor + Send + Sync>>,
        max_concurrent_tasks: usize,
    ) -> Self {
        Self {
            cpu_executor,
            gpu_executor,
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            max_concurrent_tasks,
        }
    }
    
    /// Submits a task for execution.
    pub fn submit_task(&self, task: Task) -> Result<(), Error> {
        let mut queue = self.task_queue.lock().map_err(|_| {
            Error::Task("Failed to lock task queue".to_string())
        })?;
        
        queue.push_back(task);
        
        Ok(())
    }
    
    /// Starts the scheduler.
    pub async fn start(&self) -> Result<(), Error> {
        let (tx, mut rx) = mpsc::channel(100);
        
        // Spawn a task to process the queue
        let task_queue = self.task_queue.clone();
        let cpu_executor = self.cpu_executor.clone();
        let gpu_executor = self.gpu_executor.clone();
        let max_concurrent_tasks = self.max_concurrent_tasks;
        
        tokio::spawn(async move {
            let mut active_tasks = 0;
            
            loop {
                // If we have capacity, try to get a task from the queue
                if active_tasks < max_concurrent_tasks {
                    let task = {
                        let mut queue = task_queue.lock().unwrap();
                        queue.pop_front()
                    };
                    
                    if let Some(task) = task {
                        active_tasks += 1;
                        
                        // Clone the channel sender for this task
                        let task_tx = tx.clone();
                        
                        // Choose the appropriate executor based on the task type
                        let executor = match task.resources.resource_type {
                            TaskResourceType::CPU => cpu_executor.clone(),
                            TaskResourceType::GPU => {
                                if let Some(gpu_exec) = gpu_executor.clone() {
                                    gpu_exec
                                } else {
                                    // Fall back to CPU if no GPU executor is available
                                    cpu_executor.clone()
                                }
                            },
                            _ => cpu_executor.clone(), // Default to CPU for other types
                        };
                        
                        // Execute the task
                        let task_id = task.id.clone();
                        tokio::spawn(async move {
                            let result = executor.execute(&task).await;
                            
                            // Send the result back to the scheduler
                            let _ = task_tx.send((task_id, result)).await;
                        });
                    }
                }
                
                // Wait for a task to complete or a timeout
                tokio::select! {
                    Some((task_id, result)) = rx.recv() => {
                        active_tasks -= 1;
                        
                        // Process the result (in a real implementation, we would update the task status)
                        match result {
                            Ok(output) => {
                                println!("Task {} completed: {}", task_id, output);
                            },
                            Err(e) => {
                                println!("Task {} failed: {}", task_id, e);
                            }
                        }
                    }
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        // Just a timeout to prevent busy waiting
                    }
                }
            }
        });
        
        Ok(())
    }
}
