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
use std::sync::{Arc, Mutex};

/// Contribution type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    /// CPU contribution.
    Cpu,
    /// Memory contribution.
    Memory,
    /// Disk contribution.
    Disk,
    /// GPU contribution.
    Gpu,
    /// Network contribution.
    Network,
}

/// Contribution record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    /// Total CPU time contributed in seconds.
    pub cpu_time_secs: f64,
    /// Total memory contributed in bytes.
    pub memory_bytes: u64,
    /// Total disk space contributed in bytes.
    pub disk_bytes: u64,
    /// Total GPU time contributed in seconds.
    pub gpu_time_secs: f64,
    /// Total network bandwidth contributed in bytes.
    pub network_bytes: u64,
    /// Total points earned.
    pub points: u64,
}

impl Default for Contribution {
    fn default() -> Self {
        Self {
            cpu_time_secs: 0.0,
            memory_bytes: 0,
            disk_bytes: 0,
            gpu_time_secs: 0.0,
            network_bytes: 0,
            points: 0,
        }
    }
}

/// Scoring system for tracking peer contributions.
pub struct ScoringSystem {
    contributions: Arc<Mutex<HashMap<String, Contribution>>>,
    cpu_points_per_hour: u64,
    memory_points_per_gb_hour: u64,
    disk_points_per_gb_hour: u64,
    gpu_points_per_hour: u64,
    network_points_per_gb: u64,
}

impl ScoringSystem {
    /// Creates a new ScoringSystem with default point values.
    pub fn new() -> Self {
        Self {
            contributions: Arc::new(Mutex::new(HashMap::new())),
            cpu_points_per_hour: 100,
            memory_points_per_gb_hour: 50,
            disk_points_per_gb_hour: 20,
            gpu_points_per_hour: 200,
            network_points_per_gb: 10,
        }
    }
    
    /// Creates a new ScoringSystem with custom point values.
    pub fn new_with_custom_points(
        cpu_points_per_hour: u64,
        memory_points_per_gb_hour: u64,
        disk_points_per_gb_hour: u64,
        gpu_points_per_hour: u64,
        network_points_per_gb: u64,
    ) -> Self {
        Self {
            contributions: Arc::new(Mutex::new(HashMap::new())),
            cpu_points_per_hour,
            memory_points_per_gb_hour,
            disk_points_per_gb_hour,
            gpu_points_per_hour,
            network_points_per_gb,
        }
    }
    
    /// Records a contribution from a peer.
    pub fn record_contribution(
        &self,
        peer_id: &str,
        contribution_type: ContributionType,
        amount: f64,
    ) -> Result<u64, Error> {
        let mut contributions = self.contributions.lock()
            .map_err(|_| Error::Other("Failed to lock contributions".to_string()))?;
        
        let contribution = contributions.entry(peer_id.to_string()).or_default();
        
        let points = match contribution_type {
            ContributionType::Cpu => {
                contribution.cpu_time_secs += amount;
                (amount / 3600.0 * self.cpu_points_per_hour as f64) as u64
            },
            ContributionType::Memory => {
                let gb_hours = amount / (1024.0 * 1024.0 * 1024.0) * (1.0 / 3600.0);
                contribution.memory_bytes += amount as u64;
                (gb_hours * self.memory_points_per_gb_hour as f64) as u64
            },
            ContributionType::Disk => {
                let gb_hours = amount / (1024.0 * 1024.0 * 1024.0) * (1.0 / 3600.0);
                contribution.disk_bytes += amount as u64;
                (gb_hours * self.disk_points_per_gb_hour as f64) as u64
            },
            ContributionType::Gpu => {
                contribution.gpu_time_secs += amount;
                (amount / 3600.0 * self.gpu_points_per_hour as f64) as u64
            },
            ContributionType::Network => {
                let gb = amount / (1024.0 * 1024.0 * 1024.0);
                contribution.network_bytes += amount as u64;
                (gb * self.network_points_per_gb as f64) as u64
            },
        };
        
        contribution.points += points;
        
        Ok(points)
    }
    
    /// Records a task contribution from a peer.
    pub fn record_task_contribution(
        &self,
        peer_id: &str,
        cpu_time_secs: f64,
        memory_used: u64,
        disk_used: u64,
        gpu_time_secs: f64,
    ) -> Result<u64, Error> {
        // Calculate the task score first, before acquiring the lock
        let task_score = self.calculate_task_score(cpu_time_secs, memory_used, disk_used, gpu_time_secs);
        
        // Then update the contribution
        let mut contributions = self.contributions.lock()
            .map_err(|_| Error::Other("Failed to lock contributions".to_string()))?;
        
        let contribution = contributions.entry(peer_id.to_string()).or_default();
        
        // Now update the contribution
        contribution.cpu_time_secs += cpu_time_secs;
        contribution.memory_bytes += memory_used;
        contribution.disk_bytes += disk_used;
        contribution.gpu_time_secs += gpu_time_secs;
        contribution.points += task_score;
        
        Ok(task_score)
    }
    
    /// Calculates the score for a task.
    fn calculate_task_score(
        &self,
        cpu_time_secs: f64,
        memory_used: u64,
        disk_used: u64,
        gpu_time_secs: f64,
    ) -> u64 {
        let cpu_points = (cpu_time_secs / 3600.0 * self.cpu_points_per_hour as f64) as u64;
        
        let memory_gb_hours = (memory_used as f64) / (1024.0 * 1024.0 * 1024.0) * (1.0 / 3600.0);
        let memory_points = (memory_gb_hours * self.memory_points_per_gb_hour as f64) as u64;
        
        let disk_gb_hours = (disk_used as f64) / (1024.0 * 1024.0 * 1024.0) * (1.0 / 3600.0);
        let disk_points = (disk_gb_hours * self.disk_points_per_gb_hour as f64) as u64;
        
        let gpu_points = (gpu_time_secs / 3600.0 * self.gpu_points_per_hour as f64) as u64;
        
        cpu_points + memory_points + disk_points + gpu_points
    }
    
    /// Gets the contribution for a peer.
    pub fn get_contribution(&self, peer_id: &str) -> Result<Option<Contribution>, Error> {
        let contributions = self.contributions.lock()
            .map_err(|_| Error::Other("Failed to lock contributions".to_string()))?;
        
        Ok(contributions.get(peer_id).cloned())
    }
    
    /// Gets all contributions.
    pub fn get_all_contributions(&self) -> Result<HashMap<String, Contribution>, Error> {
        let contributions = self.contributions.lock()
            .map_err(|_| Error::Other("Failed to lock contributions".to_string()))?;
        
        Ok(contributions.clone())
    }
}

impl Default for ScoringSystem {
    fn default() -> Self {
        Self::new()
    }
}
