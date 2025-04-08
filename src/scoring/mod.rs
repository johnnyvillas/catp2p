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

//! Scoring and rewards system for tracking contributions.

pub mod points;

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A record of a peer's contributions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerContribution {
    /// The peer's ID.
    pub peer_id: String,
    /// The total number of tasks completed.
    pub tasks_completed: u64,
    /// The total number of tasks failed.
    pub tasks_failed: u64,
    /// The total CPU time contributed in seconds.
    pub cpu_time_contributed: f64,
    /// The total memory contributed in byte-seconds.
    pub memory_contributed: u64,
    /// The total disk space contributed in byte-seconds.
    pub disk_contributed: u64,
    /// The total GPU time contributed in seconds, if applicable.
    pub gpu_time_contributed: Option<f64>,
    /// The total score earned.
    pub total_score: f64,
}

/// The main scoring manager for CatP2P.
pub struct ScoringManager {
    contributions: HashMap<String, PeerContribution>,
}

impl ScoringManager {
    /// Creates a new ScoringManager.
    pub fn new() -> Self {
        Self {
            contributions: HashMap::new(),
        }
    }

    /// Records a completed task for a peer.
    pub fn record_task_completion(
        &mut self,
        peer_id: &str,
        cpu_time: f64,
        memory_used: u64,
        disk_used: u64,
        gpu_time: Option<f64>,
    ) -> Result<f64, Error> {
        let contribution = self.contributions
            .entry(peer_id.to_string())
            .or_insert_with(|| PeerContribution {
                peer_id: peer_id.to_string(),
                tasks_completed: 0,
                tasks_failed: 0,
                cpu_time_contributed: 0.0,
                memory_contributed: 0,
                disk_contributed: 0,
                gpu_time_contributed: None,
                total_score: 0.0,
            });
        
        contribution.tasks_completed += 1;
        contribution.cpu_time_contributed += cpu_time;
        contribution.memory_contributed += memory_used;
        contribution.disk_contributed += disk_used;
        
        if let Some(gpu_time_value) = gpu_time {
            contribution.gpu_time_contributed = Some(
                contribution.gpu_time_contributed.unwrap_or(0.0) + gpu_time_value
            );
        }
        
        // Calculate score for this task
        let task_score = self.calculate_task_score(cpu_time, memory_used, disk_used, gpu_time);
        contribution.total_score += task_score;
        
        Ok(task_score)
    }

    /// Records a failed task for a peer.
    pub fn record_task_failure(&mut self, peer_id: &str) -> Result<(), Error> {
        let contribution = self.contributions
            .entry(peer_id.to_string())
            .or_insert_with(|| PeerContribution {
                peer_id: peer_id.to_string(),
                tasks_completed: 0,
                tasks_failed: 0,
                cpu_time_contributed: 0.0,
                memory_contributed: 0,
                disk_contributed: 0,
                gpu_time_contributed: None,
                total_score: 0.0,
            });
        
        contribution.tasks_failed += 1;
        
        Ok(())
    }

    /// Gets a peer's contribution record.
    pub fn get_peer_contribution(&self, peer_id: &str) -> Option<&PeerContribution> {
        self.contributions.get(peer_id)
    }

    /// Gets all peer contribution records.
    pub fn get_all_contributions(&self) -> Vec<&PeerContribution> {
        self.contributions.values().collect()
    }

    /// Calculates the score for a task based on resource usage.
    fn calculate_task_score(
        &self,
        cpu_time: f64,
        memory_used: u64,
        disk_used: u64,
        gpu_time: Option<f64>,
    ) -> f64 {
        // Simple scoring formula for now
        // We can make this more sophisticated later
        let cpu_score = cpu_time * 1.0;
        let memory_score = (memory_used as f64) / (1024.0 * 1024.0 * 1024.0) * 0.5; // GB-seconds
        let disk_score = (disk_used as f64) / (1024.0 * 1024.0 * 1024.0) * 0.3; // GB-seconds
        let gpu_score = gpu_time.unwrap_or(0.0) * 2.0; // GPU time is weighted more heavily
        
        cpu_score + memory_score + disk_score + gpu_score
    }
}

impl Default for ScoringManager {
    fn default() -> Self {
        Self::new()
    }
}
