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

//! Points system for tracking and rewarding contributions.

use crate::error::Error;
use serde::{Deserialize, Serialize};

/// A point transaction record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointTransaction {
    /// The transaction ID.
    pub id: String,
    /// The peer ID.
    pub peer_id: String,
    /// The number of points.
    pub points: f64,
    /// The reason for the transaction.
    pub reason: String,
    /// The timestamp of the transaction.
    pub timestamp: u64,
}

/// The points manager for tracking and rewarding contributions.
pub struct PointsManager {
    transactions: Vec<PointTransaction>,
}

impl PointsManager {
    /// Creates a new PointsManager.
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    /// Awards points to a peer.
    pub fn award_points(&mut self, peer_id: &str, points: f64, reason: &str) -> Result<String, Error> {
        let transaction_id = format!("txn-{}-{}", peer_id, self.transactions.len());
        
        let transaction = PointTransaction {
            id: transaction_id.clone(),
            peer_id: peer_id.to_string(),
            points,
            reason: reason.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        self.transactions.push(transaction);
        
        Ok(transaction_id)
    }

    /// Gets the total points for a peer.
    pub fn get_total_points(&self, peer_id: &str) -> f64 {
        self.transactions.iter()
            .filter(|txn| txn.peer_id == peer_id)
            .map(|txn| txn.points)
            .sum()
    }

    /// Gets all transactions for a peer.
    pub fn get_peer_transactions(&self, peer_id: &str) -> Vec<&PointTransaction> {
        self.transactions.iter()
            .filter(|txn| txn.peer_id == peer_id)
            .collect()
    }

    /// Gets all transactions.
    pub fn get_all_transactions(&self) -> &[PointTransaction] {
        &self.transactions
    }
}

impl Default for PointsManager {
    fn default() -> Self {
        Self::new()
    }
}
