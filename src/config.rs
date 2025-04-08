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

//! Configuration for the CatP2P library.

use serde::{Deserialize, Serialize};

/// Resource allocation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceMode {
    /// Light mode - minimal resource usage.
    Light,
    /// Medium mode - balanced resource usage.
    Medium,
    /// High performance mode - maximum resource usage.
    HighPerformance,
    /// Custom mode with user-defined resource limits.
    Custom,
}

/// Resource limits for custom mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU usage limit (0.0 - 1.0).
    pub cpu_limit: f32,
    /// Memory usage limit in bytes.
    pub memory_limit: u64,
    /// GPU usage limit (0.0 - 1.0), if GPU is available.
    pub gpu_limit: Option<f32>,
    /// Storage usage limit in bytes.
    pub storage_limit: u64,
}

/// Network configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Port to listen on.
    pub port: u16,
    /// Bootstrap nodes to connect to.
    pub bootstrap_nodes: Vec<String>,
    /// Whether to enable NAT traversal.
    pub enable_nat_traversal: bool,
    /// Maximum number of connections.
    pub max_connections: usize,
}

/// Storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Path to the database directory.
    pub db_path: String,
    /// Maximum size of the database in bytes.
    pub max_size: u64,
}

/// Task configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// Maximum number of concurrent tasks.
    pub max_concurrent_tasks: usize,
    /// Task timeout in seconds.
    pub task_timeout: u64,
}

/// Main configuration for the CatP2P library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Resource allocation mode.
    pub resource_mode: ResourceMode,
    /// Custom resource limits (only used if resource_mode is Custom).
    pub resource_limits: Option<ResourceLimits>,
    /// Network configuration.
    pub network: NetworkConfig,
    /// Storage configuration.
    pub storage: StorageConfig,
    /// Task configuration.
    pub task: TaskConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resource_mode: ResourceMode::Medium,
            resource_limits: None,
            network: NetworkConfig {
                port: 4001,
                bootstrap_nodes: vec![],
                enable_nat_traversal: true,
                max_connections: 50,
            },
            storage: StorageConfig {
                db_path: "./catp2p-db".to_string(),
                max_size: 1024 * 1024 * 1024, // 1 GB
            },
            task: TaskConfig {
                max_concurrent_tasks: 10,
                task_timeout: 3600, // 1 hour
            },
        }
    }
}

impl Config {
    /// Checks if the configuration is valid.
    pub fn is_valid(&self) -> bool {
        // Basic validation
        if self.resource_mode == ResourceMode::Custom && self.resource_limits.is_none() {
            return false;
        }

        if let Some(limits) = &self.resource_limits {
            if limits.cpu_limit < 0.0 || limits.cpu_limit > 1.0 {
                return false;
            }
            if let Some(gpu_limit) = limits.gpu_limit {
                if gpu_limit < 0.0 || gpu_limit > 1.0 {
                    return false;
                }
            }
        }

        true
    }
}
