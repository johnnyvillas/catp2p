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

//! Benchmarking functionality for assessing system capabilities.

pub mod cpu;
pub mod disk;
pub mod gpu;
pub mod memory;
pub mod network;

use crate::error::Error;
use serde::{Deserialize, Serialize};

/// Benchmark result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// CPU benchmark score.
    pub cpu_score: f64,
    /// Memory benchmark score.
    pub memory_score: f64,
    /// Disk benchmark score.
    pub disk_score: f64,
    /// GPU benchmark score, if available.
    pub gpu_score: Option<f64>,
    /// Network benchmark score.
    pub network_score: Option<f64>,
    /// Overall benchmark score.
    pub overall_score: f64,
}

/// Runs all benchmarks and returns a combined result.
pub async fn run_all_benchmarks() -> Result<BenchmarkResult, Error> {
    // Run CPU benchmark
    let cpu_score = cpu::run_cpu_benchmark()?;
    
    // Run memory benchmark
    let memory_score = memory::run_memory_benchmark()?;
    
    // Run disk benchmark
    let disk_score = disk::run_disk_benchmark()?;
    
    // Run GPU benchmark if available
    let gpu_score = if gpu::is_gpu_available().await {
        match gpu::run_gpu_benchmark().await {
            Ok(score) => Some(score),
            Err(_) => None,
        }
    } else {
        None
    };
    
    // Network benchmark is optional and requires a server
    let network_score = None;
    
    // Calculate overall score
    let mut overall_score = (cpu_score + memory_score + disk_score) / 3.0;
    
    // Include GPU score if available
    if let Some(gpu) = gpu_score {
        overall_score = (overall_score * 3.0 + gpu) / 4.0;
    }
    
    // Include network score if available
    if let Some(network) = network_score {
        let divisor = if gpu_score.is_some() { 5.0 } else { 4.0 };
        overall_score = (overall_score * (divisor - 1.0) + network) / divisor;
    }
    
    Ok(BenchmarkResult {
        cpu_score,
        memory_score,
        disk_score,
        gpu_score,
        network_score,
        overall_score,
    })
}
