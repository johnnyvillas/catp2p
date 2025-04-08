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

//! CPU task execution functionality.

use crate::error::Error;
use crate::tasks::{Task, TaskExecutor}; 
use async_trait::async_trait;
// Remove unused import
// use rayon::prelude::*;
use std::time::Instant;

/// A CPU task executor.
pub struct CpuTaskExecutor {
    // Number of CPU cores to use
    cpu_cores: usize,
}

impl CpuTaskExecutor {
    /// Creates a new CpuTaskExecutor with the given number of CPU cores.
    pub fn new(cpu_cores: usize) -> Self {
        let available_cores = rayon::current_num_threads();
        let cores_to_use = cpu_cores.min(available_cores);
        
        Self {
            cpu_cores: cores_to_use,
        }
    }
    
    /// Creates a new CpuTaskExecutor using all available CPU cores.
    pub fn new_with_all_cores() -> Self {
        Self {
            cpu_cores: rayon::current_num_threads(),
        }
    }
}

#[async_trait]
impl TaskExecutor for CpuTaskExecutor {
    async fn execute(&self, _task: &Task) -> Result<String, Error> {
        // This is a placeholder implementation
        // In a real implementation, we would parse the task data and execute the actual computation
        
        let start_time = Instant::now();
        let cpu_cores = self.cpu_cores; // Clone the value to move into the closure
        
        // Simulate CPU-intensive work
        let result = tokio::task::spawn_blocking(move || {
            // Create a parallel iterator with the specified number of threads
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(cpu_cores)
                .build()
                .map_err(|e| Error::Task(format!("Failed to create thread pool: {}", e)))?;
            
            let result = pool.install(|| {
                // Simulate some CPU-intensive work
                let mut sum = 0;
                for i in 0..1_000_000 {
                    sum += i;
                }
                Ok::<_, Error>(sum.to_string())
            });
            
            result
        }).await.map_err(|e| Error::Task(format!("Task execution failed: {}", e)))??;
        
        let elapsed = start_time.elapsed();
        
        Ok(format!("Result: {}, Time: {:?}", result, elapsed))
    }
}
